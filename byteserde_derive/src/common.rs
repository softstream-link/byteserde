use quote::{__private::TokenStream, quote};
use syn::{
    ConstParam, Data, DeriveInput, Expr, Field, Fields, GenericParam, Generics, Ident, Index,
    Member, Path, Type, TypeArray, TypeParam, TypePath,
};

use crate::{
    struct_shared::{
        des_arr_num_vars, des_num_vars, get_effective_endian, get_length_attribute, ser_arr_num,
        ser_num, ser_overrides, Length, MemberIdent,
    },
};

pub enum StructType {
    Regular,
    Tuple,
}
#[derive(Debug)]
pub struct FieldSerializerDeserializerTokens {
    pub ser_vars: TokenStream,
    pub ser_over: TokenStream,
    pub ser_uses_stack: TokenStream,
    pub ser_uses_heap: TokenStream,
    pub des_vars: TokenStream,
    pub des_uses: TokenStream,
}
pub fn get_struct_ser_des_tokens(
    ast: &DeriveInput,
) -> (Vec<FieldSerializerDeserializerTokens>, StructType) {
    let ty: StructType;

    let tokens = match &ast.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(flds) => {
                ty = StructType::Regular;
                flds.named
                    .iter()
                    .map(|fld| {
                        let var_name = fld.ident.as_ref().unwrap();
                        let fld_type = map_field_type(&fld.ty);
                        match fld_type {
                            FieldType::Numeric | FieldType::Byte => {
                                // TODO optimize for byte & byte arrays to avoid endian
                                setup_numeric(ast, fld, var_name, MemberIdent::Named(var_name))
                            }
                            FieldType::NumericArray { ty, len }
                            | FieldType::ByteArray { ty, len } => setup_numeric_array(
                                ast,
                                fld,
                                ty,
                                var_name,
                                len,
                                &MemberIdent::Named(var_name),
                            ),
                            FieldType::Other { ty } => {
                                setup_other(fld, var_name, ty, MemberIdent::Named(var_name))
                            }
                        }
                    })
                    .collect::<Vec<_>>()
            }
            Fields::Unnamed(flds) => {
                ty = StructType::Tuple;
                flds.unnamed
                    .iter()
                    .enumerate()
                    .map(|(i, fld)| {
                        let fld_index = &Member::Unnamed(Index {
                            index: i as u32,
                            span: ast.ident.span(),
                        });
                        let var_name = &Ident::new(&format!("_{}", i), ast.ident.span());
                        let fld_type = map_field_type(&fld.ty);
                        match fld_type {
                            FieldType::Numeric | FieldType::Byte => {
                                setup_numeric(ast, fld, var_name, MemberIdent::Unnamed(fld_index))
                            }
                            FieldType::NumericArray { ty, len }
                            | FieldType::ByteArray { ty, len } => setup_numeric_array(
                                ast,
                                fld,
                                ty,
                                var_name,
                                len,
                                &MemberIdent::Unnamed(fld_index),
                            ),
                            FieldType::Other { ty } => {
                                setup_other(fld, var_name, ty, MemberIdent::Unnamed(fld_index))
                            }
                        }
                    })
                    .collect::<Vec<_>>()
            }
            Fields::Unit => panic!(
                "Unit struct type is not supported, found '{struct_name}'",
                struct_name = &ast.ident
            ),
        },
        _ => {
            panic!(
                "Only struct types supported, found '{struct_name}' of type '{ty}'",
                ty = match ast.data {
                    Data::Enum(_) => "enum",
                    Data::Union(_) => "union",
                    _ => "unknow",
                },
                struct_name = &ast.ident
            )
        }
    };
    (tokens, ty)
}

fn setup_numeric(
    ast: &DeriveInput,
    fld: &Field,
    var_name: &Ident,
    member: MemberIdent,
) -> FieldSerializerDeserializerTokens {
    let effective_endian = &get_effective_endian(&ast.attrs, &fld.attrs);
    FieldSerializerDeserializerTokens {
        ser_vars: match member {
            MemberIdent::Named(_) => quote! { let #var_name = self.#var_name; }, // let #var_name = self.#var_name;
            MemberIdent::Unnamed(fld_index) => quote! { let #var_name = self.#fld_index; }, // let #var_name = &self.#fld_index;
        },
        ser_over: ser_overrides(fld, var_name), // let #var_name = #[derive(replace( **** ))];
        ser_uses_stack: ser_num(effective_endian, var_name), // ser.serialize_[be|le|ne]( #var_name )?;
        ser_uses_heap: ser_num(effective_endian, var_name), // ser.serialize_[be|le|ne]( #var_name )?;
        des_vars: des_num_vars(effective_endian, var_name), // let #var_name = des.deserialize_[be|le|ne]()?;
        des_uses: quote!( #var_name, ),                     // Struct/Tuple ( #var_name, .., ..)
    }
}
fn setup_numeric_array(
    ast: &DeriveInput,
    fld: &Field,
    ty: &Type,
    var_name: &Ident,
    len: &Expr,
    member: &MemberIdent,
) -> FieldSerializerDeserializerTokens {
    let effective_endian = &get_effective_endian(&ast.attrs, &fld.attrs);
    FieldSerializerDeserializerTokens {
        ser_vars: quote!(), //"replace not implemented for numeric arrays so just injecting blank and set allowing a var to be created
        ser_over: quote!(),
        ser_uses_stack: ser_arr_num(effective_endian, member), // for e in self.fld_name { ser.serialize_[be|le|ne](e)?; }
        ser_uses_heap: ser_arr_num(effective_endian, member),  // same as stack
        des_vars: des_arr_num_vars(effective_endian, var_name, ty, len), // let mut fld_name: [u16: len] = [0; len]; for e in fld_name.iter_mut() {*e = des.deserialize_[be|le|ne]()?;} },
        des_uses: quote!( #var_name, ),
    }
}
fn setup_other(
    fld: &Field,
    var_name: &Ident,
    ty: &Type,
    member: MemberIdent,
) -> FieldSerializerDeserializerTokens {
    let length = get_length_attribute(&fld.attrs);
    FieldSerializerDeserializerTokens {
        ser_vars: match member {
            MemberIdent::Named(_) => quote! { let #var_name = &self.#var_name; }, // let #var_name = &self.#var_name;
            MemberIdent::Unnamed(fld_index) => quote! { let #var_name = &self.#fld_index; }, // let #var_name = &self.#fld_index;
        },
        ser_over: quote!(), // not supported
        ser_uses_stack: quote!( #var_name.byte_serialize_stack(ser)?; ),
        ser_uses_heap: quote!( #var_name.byte_serialize_heap(ser)?; ),
        des_vars: match length {
            Length::Len(len) => {
                quote!( let #var_name: #ty = des.deserialize_take( (#len) as usize )?; )
            }
            Length::NotSet => quote!( let #var_name: #ty = des.deserialize()?; ),
        },
        des_uses: quote!( #var_name, ),
    }
}

#[derive(PartialEq)]
enum FieldType<'a> {
    Byte,
    Numeric,
    ByteArray { ty: &'a Type, len: &'a Expr },
    NumericArray { ty: &'a Type, len: &'a Expr },
    Other { ty: &'a Type },
}

fn map_field_type(ty: &Type) -> FieldType {
    match ty {
        Type::Path(TypePath { path, .. }) => path_2_numeric_byte_or_other(path, ty),
        Type::Array(TypeArray { elem: ty, len, .. }) => match ty.as_ref() {
            Type::Path(TypePath { path, .. }) => match path_2_numeric_byte_or_other(path, ty) {
                FieldType::Numeric => FieldType::NumericArray { ty, len },
                FieldType::Byte => FieldType::ByteArray { ty, len },
                _ => FieldType::Other { ty },
            },
            _ => FieldType::Other { ty },
        },
        _ => FieldType::Other { ty },
    }
}

fn path_2_numeric_byte_or_other<'a>(path: &'a Path, ty: &'a Type) -> FieldType<'a> {
    // byte
    if path.is_ident("u8") {
        return FieldType::Byte;
    }

    // all numerics
    if path.is_ident("i8")
        || path.is_ident("i16")
        || path.is_ident("u16")
        || path.is_ident("i32")
        || path.is_ident("u32")
        || path.is_ident("i64")
        || path.is_ident("u64")
        || path.is_ident("i128")
        || path.is_ident("u128")
        || path.is_ident("f32")
        || path.is_ident("f64")
    {
        return FieldType::Numeric;
    }

    FieldType::Other { ty }
}

pub fn get_generics(generics: &Generics) -> (TokenStream, TokenStream) {
    let type_alias = generics
        .params
        .iter()
        .map(|param| {
            // eprintln!("\t\t param: {:?}", param);
            match param {
                GenericParam::Const(ConstParam { ident, .. }) => {
                    quote! ( #ident )
                }
                GenericParam::Type(TypeParam { ident, .. }) => {
                    quote! ( #ident )
                }
                GenericParam::Lifetime(_) => {
                    todo!("lifetime generics,  not implemented");
                }
            }
            // param
        })
        .collect::<Vec<_>>();
    match generics.params.len() {
        0 => (quote! {}, quote! {}),
        _ => (quote! { #generics}, quote! { < #(#type_alias),* > }),
    }
}

/// Returns a valid name to be used in when referencing structs within `byteserde` crate. Generates two options `crate` vs `::byteserde`.
/// All `./tests/*.rs` need to refer to the `crait's` structs using fully qualified name which starts with `::byteserde`.
/// All `crate's` internal references need to use relative path which when starts from root starts with `crate`. This is particularly relevant when using `#[deverive()]` macro.
pub fn get_crate_name() -> TokenStream {
    let cargo_crate_name = std::env::var("CARGO_CRATE_NAME").unwrap();
    let crate_name = match cargo_crate_name.as_str() {
        "byteserde" => quote!(crate),
        _ => quote!(::byteserde),
    };
    crate_name
}
