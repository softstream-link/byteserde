use quote::{
    __private::{Span, TokenStream},
    quote, ToTokens,
};
use syn::{parenthesized, Attribute, Expr};


pub enum Peek {
    NotSet,
    Set(Expr),
}
pub fn get_peek_attribute(struct_attrs: &[Attribute]) -> Peek {
    let (peek, _) = get_option_attributes(struct_attrs);
    peek
}

pub enum Eq {
    NotSet,
    Set(Expr),
}
pub fn get_eq_attribute(struct_attrs: &[Attribute]) -> Eq {
    let (_, eq) = get_option_attributes(struct_attrs);
    eq
}

fn get_option_attributes(attrs: &[Attribute]) -> (Peek, Eq) {
    let byteserde_attrs = attrs
        .iter()
        .filter(|atr| atr.meta.path().is_ident("byteserde"))
        .collect::<Vec<_>>();

    let mut peek = Peek::NotSet;
    let mut eq = Eq::NotSet;

    // https://docs.rs/syn/latest/syn/meta/struct.ParseNestedMeta.html

    for attr in byteserde_attrs {
        let res = attr.parse_nested_meta(|meta| {
            // Option only
            if meta.path.is_ident("peek") {
                let content;
                parenthesized!(content in meta.input);
                peek = Peek::Set(content.parse::<Expr>()?);
                return Ok(());
            }
            // Option only
            if meta.path.is_ident("eq") {
                let content;
                parenthesized!(content in meta.input);
                eq = Eq::Set(content.parse::<Expr>()?);
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

    (peek, eq)
}
