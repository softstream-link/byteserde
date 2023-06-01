use quote::{
    __private::{TokenStream},
    quote, ToTokens,
};
use syn::{parenthesized, Attribute, Expr, Ident};

#[derive(Debug)]
pub enum Replace {
    NotSet,
    Set(Expr),
}
pub fn enum_replace_attr(attrs: &[Attribute]) -> Replace {
    let (replace, _, _) = get_attrs(attrs);
    replace
}

pub enum Bind {
    NotSet,
    Set(Ident),
}
pub fn enum_bind_attr(struct_attrs: &[Attribute]) -> Bind {
    let (_, bind, _) = get_attrs(struct_attrs);
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
pub fn enum_from_attr(struct_attrs: &[Attribute]) -> Vec<From> {
    let (_, _, from) = get_attrs(struct_attrs);
    from
}

fn get_attrs(attrs: &[Attribute]) -> (Replace, Bind, Vec<From>) {
    let byteserde_attrs = attrs
        .iter()
        .filter(|atr| atr.meta.path().is_ident("byteserde"))
        .collect::<Vec<_>>();

    let mut replace = Replace::NotSet;
    let mut bind = Bind::NotSet;
    let mut from = Vec::<From>::new();

    // https://docs.rs/syn/latest/syn/meta/struct.ParseNestedMeta.html

    for attr in byteserde_attrs {
        let res = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("replace") {
                let content;
                parenthesized!(content in meta.input);
                replace = Replace::Set(content.parse::<Expr>()?);
                return Ok(());
            }
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

    (replace, bind, from)
}
