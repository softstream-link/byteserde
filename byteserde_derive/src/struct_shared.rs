use quote::{
    __private::{Span},
    quote,
};
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

pub enum Deplete {
    NotSet,
    Size(Expr),
}
pub fn deplete_attr(attrs: &[Attribute]) -> Deplete {
    let (_, deplete, _) = get_attrs(attrs);
    deplete
}

#[derive(Debug)]
pub enum Replace {
    NotSet,
    Set(Expr),
}
pub fn replace_attr(attrs: &[Attribute]) -> Replace {
    let (_, _, replace) = get_attrs(attrs);
    replace
}

#[derive(Debug)]
pub enum Endian {
    Lit,
    Big,
    Native,
    NotSet,
}

pub fn endian_attr(struc_attrs: &[Attribute], fld_attrs: &[Attribute]) -> Endian {
    let (fld_endian, _, _) = get_attrs(fld_attrs);
    match fld_endian {
        Endian::NotSet => {
            let (struct_endian, _, _) = get_attrs(struc_attrs);
            struct_endian
        }
        _ => fld_endian,
    }
}

fn get_attrs(attrs: &[Attribute]) -> (Endian, Deplete, Replace) {
    let byteserde_attrs = attrs
        .iter()
        .filter(|atr| atr.meta.path().is_ident("byteserde"))
        .collect::<Vec<_>>();

    let mut endian = Endian::NotSet;
    let mut deplete = Deplete::NotSet;
    let mut replace = Replace::NotSet;

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
                replace = Replace::Set(content.parse::<Expr>()?);
                return Ok(());
            }
            // only affects variable length fields like String
            if meta.path.is_ident("deplete") {
                let content;
                parenthesized!(content in meta.input);
                deplete = Deplete::Size(content.parse::<Expr>()?);
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

    (endian, deplete, replace)
}
