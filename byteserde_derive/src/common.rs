use quote::{__private::TokenStream, quote};
use syn::{
    ConstParam, Data, Expr, Fields, GenericParam, Generics, Ident, Index,
    Member, Path, Type, TypeArray, TypePath, TypeParam,
};

use crate::{
    named::{ser_arr_num_named}, 
    unnamed::{ ser_arr_num_unnamed}, 
    struct_shared::{des_arr_num_vars, get_effective_endian, get_length_attribute, Length, ser_overrides, ser_num, des_num_vars}
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
    ast: &syn::DeriveInput,
) -> (Vec<FieldSerializerDeserializerTokens>, StructType) {
    let ty: StructType;

    let tokens = match &ast.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(flds) => {
                ty = StructType::Regular;
                flds
                .named
                .iter()
                .map(|fld| {
                    let var_name = fld.ident.as_ref().unwrap();
                    let fld_type = map_field_type(&fld.ty);
                    match fld_type {
                        FieldType::Numeric | FieldType::Byte =>{ // TODO optimize for byte
                            let effective_endian = &get_effective_endian(&ast.attrs, &fld.attrs);
                            FieldSerializerDeserializerTokens {
                                ser_vars: quote! ( let #var_name = self.#var_name; ),// let #var_name = self.#var_name;
                                ser_over: ser_overrides(fld, var_name),              // let #var_name = #[derive(replace( **** ))];
                                ser_uses_stack: ser_num(effective_endian, var_name), // ser.serialize_[be|le|ne]( #var_name )?;
                                ser_uses_heap: ser_num(effective_endian, var_name),  // ser.serialize_[be|le|ne]( #var_name )?;
                                des_vars: des_num_vars(effective_endian, var_name),  // let #var_name = des.deserialize_[be|le|ne]()?;
                                des_uses: quote!( #var_name, ),
                            }
                        },
                        FieldType::NumericArray{ty, len} | FieldType::ByteArray{ty, len}=>{ // TODO optimize for byte arrays to avoid endian
                            let effective_endian = &get_effective_endian(&ast.attrs, &fld.attrs);
                            FieldSerializerDeserializerTokens {
                                ser_vars: quote! (), // not supported
                                ser_over: quote! (), // not supported
                                ser_uses_stack: ser_arr_num_named(effective_endian, var_name), // for e in self.fld_name { ser.serialize_[be|le|ne](e)?; }
                                ser_uses_heap: ser_arr_num_named(effective_endian, var_name),
                                des_vars: des_arr_num_vars(effective_endian, var_name, ty, len), // let mut fld_name: [u16: len] = [0; len]; for e in fld_name.iter_mut() {*e = des.deserialize_[be|le|ne]()?;} },
                                des_uses: quote!( #var_name, ),
                            }
                        },
                        _ =>{
                            let length = get_length_attribute(&fld.attrs);
                            FieldSerializerDeserializerTokens {
                                ser_vars: quote! ( let #var_name = &self.#var_name; ),
                                ser_over: quote! (),  // not supported
                                ser_uses_stack: quote!( #var_name.byte_serialize_stack(ser)?; ),
                                ser_uses_heap: quote!( #var_name.byte_serialize_heap(ser)?; ),
                                des_vars: match length { 
                                    Length::Len(len) => quote!( let #var_name = des.deserialize_take(#len)?;), 
                                    Length::NotSet => quote!( let #var_name = des.deserialize()?;)
                                },
                                des_uses: quote!( #var_name, ),
                            }
                        }
                }
                })
                .collect::<Vec<_>>()
            }
            Fields::Unnamed(flds) => {
                ty = StructType::Tuple;
                flds
                .unnamed
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
                        FieldType::Numeric | FieldType::Byte =>{
                            let effective_endian = &get_effective_endian(&ast.attrs, &fld.attrs);
                            FieldSerializerDeserializerTokens {
                                ser_vars: quote! { let #var_name = self.#fld_index; }, // let #var_name = self.#fld_index;
                                ser_over: ser_overrides(fld, var_name),                // let #var_name = #[derive(replace( **** ))];
                                ser_uses_stack: ser_num(effective_endian, var_name),   // ser.serialize_[be|le|ne](#var_name)?;
                                ser_uses_heap: ser_num(effective_endian, var_name),    // ser.serialize_[be|le|ne](#var_name)?;
                                des_vars: des_num_vars( effective_endian, var_name),   // let #var_name = des.deserialize_[be|le|ne]()?;
                                des_uses: quote!( #var_name, ),                        // #var_name,
                            }
                        },
                        FieldType::NumericArray{ty, len} | FieldType::ByteArray{ty, len } =>{
                            let effective_endian = &get_effective_endian(&ast.attrs, &fld.attrs);
                            FieldSerializerDeserializerTokens{
                                ser_vars: quote! (), // not supported
                                ser_over: quote! (), // not supported
                                ser_uses_stack: ser_arr_num_unnamed(effective_endian, fld_index), // for n in self.fld_index { ser.serialize_[be|le|ne](n)?; })),
                                ser_uses_heap: ser_arr_num_unnamed(effective_endian, fld_index),
                                des_vars: des_arr_num_vars(effective_endian, var_name, ty, len), // { let mut fld_index: [u16; len] = [0; len]; for e in fld_index.iter_mut() {*e = des.deserialize_[be|le|ne]()?;} fld_index},
                                des_uses: quote!( #var_name, ),
                            }
                        },
                        _ => {
                            let length = get_length_attribute(&fld.attrs);
                            FieldSerializerDeserializerTokens {
                                ser_vars: quote! ( let #var_name = &self.#fld_index; ),
                                ser_over: quote! (),
                                ser_uses_stack: quote!( #var_name.byte_serialize_stack(ser)?; ),
                                ser_uses_heap: quote!( #var_name.byte_serialize_heap(ser)?; ),
                                des_vars: match length {
                                    Length::Len(len) => quote!( let #var_name = des.deserialize_take(#len)?; ),
                                    Length::NotSet => quote!( let #var_name = des.deserialize()?; ),
                                },
                                des_uses: quote!( #var_name, ),
                            }
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

#[derive(PartialEq)]
enum FieldType<'a> {
    Byte,
    Numeric,
    ByteArray{ty: &'a Type, len: &'a Expr},
    NumericArray{ty: &'a Type, len: &'a Expr},
    Other,
}

fn map_field_type(ty: &Type) -> FieldType {
    match ty {
        Type::Path(TypePath { path, .. }) => path_2_type(path),
        Type::Array(TypeArray { elem: ty, len, .. }) => match ty.as_ref() {
            Type::Path(TypePath { path, .. }) => match path_2_type(path) {
                FieldType::Numeric => FieldType::NumericArray{ty, len},
                FieldType::Byte => FieldType::ByteArray{ ty, len},
                _ => FieldType::Other,
            },
            _ => FieldType::Other,
        },
        _ => FieldType::Other,
    }
}

fn path_2_type(path: &Path) -> FieldType {

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

    FieldType::Other
    
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
                GenericParam::Type(TypeParam {ident, ..}) => {
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
pub fn get_crate_name() -> TokenStream{
    let cargo_crate_name = std::env::var("CARGO_CRATE_NAME").unwrap();
    let crate_name = 
    match cargo_crate_name.as_str(){
        "byteserde" => quote! (crate),
        _ => quote! (::byteserde),
    };
    crate_name
}