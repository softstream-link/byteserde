use quote::{__private::TokenStream, quote};
use syn::{parenthesized, Attribute, Expr, Field, Ident, LitStr, Type};

// serialize overrides
pub fn ser_overrides(fld: &Field, var_name: &Ident) -> TokenStream {
    let over = get_replace_attribute(&fld.attrs);
    match over {
        Replace::Set(value) => quote!( let #var_name = #value; ),
        Replace::NotSet => quote!(),
    }
}
// serialize
pub fn ser_num(endian: &Endian, var_name: &Ident) -> TokenStream {
    match endian {
        Endian::Big => quote!( ser.serialize_be(#var_name)?; ),
        Endian::Lit => quote!( ser.serialize_le(#var_name)?; ),
        _ => quote!( ser.serialize_ne(#var_name)?; ),
    }
}
// deserialize
pub fn des_num_vars(endian: &Endian, var_name: &Ident) -> TokenStream {
    match endian {
        Endian::Big => quote!( let #var_name = des.deserialize_be()?; ),
        Endian::Lit => quote!( let #var_name = des.deserialize_le()?; ),
        _ => quote!( let #var_name = des.deserialize_ne()?; ),
    }
}
// deserialize
pub fn des_arr_num_vars(
    endian: &Endian,
    var_name: &Ident,
    ty: &Type,
    len: &Expr,
) -> TokenStream {
    match endian {
        Endian::Big => {
            quote!( let mut #var_name: [#ty; #len] = [0; #len]; for e in #var_name.iter_mut() {*e = des.deserialize_be()?;} )
        }
        Endian::Lit => {
            quote!( let mut #var_name: [#ty; #len] = [0; #len]; for e in #var_name.iter_mut() {*e = des.deserialize_le()?;} )
        }
        _ => {
            quote!( let mut #var_name: [#ty; #len] = [0; #len]; for e in #var_name.iter_mut() {*e = des.deserialize_ne()?;} )
        }
    }
}


#[derive(Debug)]
pub enum Length {
    NotSet,
    Len(Expr),
}
pub fn get_length_attribute(attrs: &[Attribute]) -> Length {
    let (_, length, _) = get_attributes(attrs);
    length
}

#[derive(Debug)]
pub enum Replace {
    NotSet,
    Set(Expr),
}
pub fn get_replace_attribute(attrs: &[Attribute]) -> Replace {
    let (_, _, replace) = get_attributes(attrs);
    replace
}

#[derive(Debug)]
pub enum Endian {
    Lit,
    Big,
    Native,
    NotSet,
}

pub fn get_effective_endian(struc_attrs: &[Attribute], fld_attrs: &[Attribute]) -> Endian {
    let fld_endian = get_endian_attribute(fld_attrs);
    match fld_endian {
        Endian::NotSet => get_endian_attribute(struc_attrs),
        _ => fld_endian,
    }
}

fn get_endian_attribute(attrs: &[Attribute]) -> Endian {
    // let byteserde_attrs = attrs
    //     .iter()
    //     .filter(|atr| atr.meta.path().is_ident("byteserde"))
    //     .collect::<Vec<_>>();

    // let mut endian = Endian::NotSet;

    // // https://docs.rs/syn/latest/syn/meta/struct.ParseNestedMeta.html

    // for attr in byteserde_attrs {
    //     let res = attr.parse_nested_meta(|meta| {
    //         if meta.path.is_ident("endian") {
    //             let value = meta.value()?;
    //             let s: LitStr = value.parse()?;
    //             if s.value() == "be" {
    //                 endian = Endian::Big;
    //             } else if s.value() == "le" {
    //                 endian = Endian::Lit;
    //             } else if s.value() == "ne" {
    //                 endian = Endian::Native;
    //             } else {
    //                 return Err(meta.error("Expected 'be', 'le', or 'ne'"));
    //             }
    //             return Ok(());
    //         }

    //         Err(meta.error(format!("Unexpected attribute. {}" , quote!(#attr))))
    //     });

    //     if res.is_err() {
    //         panic!("{}", res.unwrap_err());
    //     }
    // }
    let (endian, _, _) = get_attributes(attrs);
    endian
}

fn get_attributes(attrs: &[Attribute]) -> (Endian, Length, Replace) {
    let byteserde_attrs = attrs
        .iter()
        .filter(|atr| atr.meta.path().is_ident("byteserde"))
        .collect::<Vec<_>>();

    let mut endian = Endian::NotSet;
    let mut length = Length::NotSet;
    let mut over = Replace::NotSet;

    // https://docs.rs/syn/latest/syn/meta/struct.ParseNestedMeta.html

    for attr in byteserde_attrs {
        let res = attr.parse_nested_meta(|meta| {
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
            if meta.path.is_ident("length") {
                let content;
                parenthesized!(content in meta.input);
                length = Length::Len(content.parse::<Expr>()?);
                return Ok(());
            }

            Err(meta.error(format!("Unexpected attribute. {}", quote!(#attr))))
        });

        if res.is_err() {
            panic!("{}", res.unwrap_err());
        }
    }

    (endian, length, over)
}
