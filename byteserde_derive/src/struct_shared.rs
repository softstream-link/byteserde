use quote::{__private::{Span, TokenStream}, quote, ToTokens};
use syn::{parenthesized, Attribute, Expr, Ident, LitStr, Member};

pub fn ser_endian_method_xx(endian: &Endian) -> Ident {
    match endian {
        Endian::Big => Ident::new("serialize_be", Span::call_site()),
        Endian::Lit => Ident::new("serialize_le", Span::call_site()),
        _ => Ident::new("serialize_ne", Span::call_site()),
    }
}
pub fn des_endian_method_xx(endian: &Endian) -> Ident {
    match endian {
        Endian::Big => Ident::new("deserialize_be", Span::call_site()),
        Endian::Lit => Ident::new("deserialize_le", Span::call_site()),
        _ => Ident::new("deserialize_ne", Span::call_site()),
    }
}
pub enum MemberIdent<'a> {
    Named(&'a Ident),
    Unnamed(&'a Member),
}

pub enum Length {
    NotSet,
    Len(Expr),
}
pub fn get_length_attribute(attrs: &[Attribute]) -> Length {
    let (_, length, _, _, _) = get_attributes(attrs);
    length
}

#[derive(Debug)]
pub enum Replace {
    NotSet,
    Set(Expr),
}
pub fn get_replace_attribute(attrs: &[Attribute]) -> Replace {
    let (_, _, replace, _, _) = get_attributes(attrs);
    replace
}

#[derive(Debug)]
pub enum Endian {
    Lit,
    Big,
    Native,
    NotSet,
}

pub fn get_endian_attribute(struc_attrs: &[Attribute], fld_attrs: &[Attribute]) -> Endian {
    let (fld_endian, _, _, _, _) = get_attributes(fld_attrs);
    match fld_endian {
        Endian::NotSet => {
            let (struct_endian, _, _, _, _) = get_attributes(struc_attrs);
            struct_endian
        }
        _ => fld_endian,
    }
}

pub enum Bind {
    NotSet,
    Set(Ident),
}
pub fn get_bind_attribute(struct_attrs: &[Attribute]) -> Bind {
    let (_, _, _, bind, _) = get_attributes(struct_attrs);
    bind
}
pub struct From(Expr);
impl ToTokens for From {
    fn to_token_stream(&self) -> TokenStream {
        self.0.to_token_stream()
    }
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens)
    }
}
pub fn get_from_attributes(struct_attrs: &[Attribute]) -> Vec<From> {
    let (_, _, _, _, from) = get_attributes(struct_attrs);
    from
}
fn get_attributes(attrs: &[Attribute]) -> (Endian, Length, Replace, Bind, Vec<From>) {
    let byteserde_attrs = attrs
        .iter()
        .filter(|atr| atr.meta.path().is_ident("byteserde"))
        .collect::<Vec<_>>();

    let mut endian = Endian::NotSet;
    let mut length = Length::NotSet;
    let mut over = Replace::NotSet;
    let mut bind = Bind::NotSet;
    // let mut from = From::NotSet;
    let mut from = Vec::<From>::new();

    // https://docs.rs/syn/latest/syn/meta/struct.ParseNestedMeta.html

    for attr in byteserde_attrs {
        let res = attr.parse_nested_meta(|meta| {
            // only affects numeric fields
            if meta.path.is_ident("endian") {
                let value = meta.value()?;
                let s: LitStr = value.parse()?;
                if s.value() == "be" {
                    endian = Endian::Big;
                } else if s.value() == "le" {
                    endian = Endian::Lit;
                } else if s.value() == "ne" {
                    endian = Endian::Native;
                } else {
                    return Err(meta.error("Expected \"be\", \"le\", or \"ne\""));
                }
                return Ok(());
            }
            if meta.path.is_ident("replace") {
                let content;
                parenthesized!(content in meta.input);
                over = Replace::Set(content.parse::<Expr>()?);
                return Ok(());
            }
            // only affects variable length fields like String
            if meta.path.is_ident("deplete") {
                let content;
                parenthesized!(content in meta.input);
                length = Length::Len(content.parse::<Expr>()?);
                return Ok(());
            }
            // only affects enums
            if meta.path.is_ident("bind") {
                let content;
                parenthesized!(content in meta.input);
                bind = Bind::Set(content.parse::<Ident>()?);
                return Ok(());
            }
            if meta.path.is_ident("from") {
                let content;
                parenthesized!(content in meta.input);
                from.push(From(content.parse::<Expr>()?));
                return Ok(());
            }

            Err(meta.error(format!("Unexpected attribute. {}", quote!(#attr))))
        });

        if res.is_err() {
            panic!(
                "Failed to process attrs: {} {}",
                quote!( #(#attrs)* ),
                res.unwrap_err()
            );
        }
    }

    (endian, length, over, bind, from)
}
