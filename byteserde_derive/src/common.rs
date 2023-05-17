use quote::{
    __private::{Span, TokenStream},
    quote,
};
use syn::{
    AngleBracketedGenericArguments, ConstParam, Data, DeriveInput, Expr, Field, Fields,
    GenericArgument, GenericParam, Generics, Ident, Index, Member, Path, PathArguments, Type,
    TypeArray, TypeParam, TypePath,
};

use crate::struct_shared::{
    des_endian_method_xx, get_endian_attribute, get_length_attribute, get_replace_attribute,
    ser_endian_method_xx, Length, MemberIdent, Replace,
};

pub enum StructType {
    Regular,
    Tuple,
}
#[derive(Debug)]
pub struct FieldSerializerDeserializerTokens {
    pub ser_vars: TokenStream,
    pub ser_repl: TokenStream,
    pub ser_uses_stck: TokenStream,
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
                        let member = &MemberIdent::Named(var_name);
                        let fld_type = map_field_type(&fld.ty);
                        match fld_type {
                            FieldType::Numeric { ty } | FieldType::Byte { ty, .. } => {
                                setup_numeric(ast, fld, ty, var_name, member, &fld_type)
                            }
                            FieldType::ArrBytes { arr_ty, len, .. }
                            | FieldType::ArrNumerics { arr_ty, len }
                            | FieldType::ArrStructs { arr_ty, len } => {
                                setup_array(ast, fld, arr_ty, var_name, len, member, &fld_type)
                            }
                            FieldType::VecBytes { ty }
                            | FieldType::VecNumerics { ty }
                            | FieldType::VecStructs { ty } => {
                                setup_vec(ast, fld, ty, var_name, member, &fld_type)
                            }
                            FieldType::Struct { ty } => setup_struct(fld, var_name, ty, member),
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
                        let member = &MemberIdent::Unnamed(fld_index);
                        let var_name = &Ident::new(&format!("_{}", i), ast.ident.span());
                        let fld_type = map_field_type(&fld.ty);
                        match fld_type {
                            FieldType::Numeric { ty } | FieldType::Byte { ty, .. } => {
                                setup_numeric(ast, fld, ty, var_name, member, &fld_type)
                            }
                            FieldType::ArrBytes { arr_ty, len, .. }
                            | FieldType::ArrNumerics { arr_ty, len }
                            | FieldType::ArrStructs { arr_ty, len } => {
                                setup_array(ast, fld, arr_ty, var_name, len, member, &fld_type)
                            }
                            FieldType::VecBytes { ty }
                            | FieldType::VecNumerics { ty }
                            | FieldType::VecStructs { ty } => {
                                setup_vec(ast, fld, ty, var_name, member, &fld_type)
                            }
                            FieldType::Struct { ty } => setup_struct(fld, var_name, ty, member),
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
    ty: &Type,
    var_name: &Ident,
    member: &MemberIdent,
    option: &FieldType,
) -> FieldSerializerDeserializerTokens {
    let replace = get_replace_attribute(&fld.attrs);
    let endian = get_endian_attribute(&ast.attrs, &fld.attrs);
    let ser_endian_method_xx = ser_endian_method_xx(&endian);
    let des_endian_method_xx = des_endian_method_xx(&endian);

    let ser_vars = match member {
        MemberIdent::Named(_) => quote!( let #var_name: #ty = self.#var_name; ),
        MemberIdent::Unnamed(fld_index) => quote! { let #var_name: #ty = self.#fld_index; },
    };

    let ser_repl = match replace {
        Replace::Set(value) => quote!( let #var_name = (#value) as #ty; ),
        Replace::NotSet => quote!(),
    };

    let ser_uses_xxx = match option {
        FieldType::Byte { .. } => quote!( ser.serialize_bytes_slice(&[#var_name as u8])?; ),
        FieldType::Numeric { .. } => quote!( ser.#ser_endian_method_xx(#var_name)?; ),
        _ => panic!("this method should only be called Byte, Numeric types"),
    };

    let des_vars = match option {
        FieldType::Byte { .. } => {
            quote!( let #var_name: #ty = des.deserialize_bytes_slice(1)?[0] as #ty; )
        }
        FieldType::Numeric { .. } => quote!( let #var_name: #ty = des.#des_endian_method_xx()?; ),
        _ => panic!("this method should only be called Byte, Numeric types"),
    };

    FieldSerializerDeserializerTokens {
        ser_vars: ser_vars,
        ser_repl: ser_repl,
        ser_uses_stck: ser_uses_xxx.clone(),
        ser_uses_heap: ser_uses_xxx.clone(),
        des_vars: des_vars,
        des_uses: quote!( #var_name, ),
    }
}

fn setup_array(
    ast: &DeriveInput,
    fld: &Field,
    arr_ty: &Type,
    var_name: &Ident,
    len: &Expr,
    member: &MemberIdent,
    option: &FieldType,
) -> FieldSerializerDeserializerTokens {
    let replace = get_replace_attribute(&fld.attrs);
    let endian = get_endian_attribute(&ast.attrs, &fld.attrs);
    let ser_endian_method_xx = ser_endian_method_xx(&endian);
    let des_endian_method_xx = des_endian_method_xx(&endian);

    let ser_vars = match member {
        MemberIdent::Named(fld_name) => {
            quote!( let #var_name: &[#arr_ty; #len] = &self.#fld_name; )
        }
        MemberIdent::Unnamed(fld_index) => {
            quote!( let #var_name: &[#arr_ty; #len] = &self.#fld_index; )
        }
    };
    let ser_repl = match replace {
        Replace::Set(value) => quote!( let #var_name: &[#arr_ty; #len] = &#value; ),
        Replace::NotSet => quote!(),
    };
    let ser_uses_xxx = |byte_serialize_xxx: &Ident| match option {
        FieldType::ArrBytes { signed, .. } => match signed {
            false => quote!( ser.serialize_bytes_slice(#var_name)?; ),
            true => {
                quote!( let #var_name: &[u8; #len] = unsafe { ::std::mem::transmute(#var_name) };
                        ser.serialize_bytes_slice( #var_name )?; )
            }
        },
        FieldType::ArrNumerics { .. } => {
            quote!( for n in #var_name { ser.#ser_endian_method_xx(*n)?; } )
        }
        FieldType::ArrStructs { .. } => {
            quote!( for n in #var_name { n.#byte_serialize_xxx(ser)?; } )
        }
        _ => panic!(
            "this method should only be called ArrayBytes, ArrayNumerics, ArrayStructs types"
        ),
    };

    let des_vars = match option {
        FieldType::ArrBytes { signed, .. } => match signed {
            false => quote!( let #var_name: [#arr_ty; #len] = des.deserialize_bytes_array()?; ),
            true => {
                quote!( let #var_name: [u8; #len] = des.deserialize_bytes_array()?; 
                        let #var_name: [#arr_ty; #len] = unsafe { ::std::mem::transmute(#var_name) }; )
            }
        },
        FieldType::ArrNumerics { .. } => {
            quote!( let mut #var_name: [#arr_ty; #len] = [0; #len]; for e in #var_name.iter_mut() {*e = des.#des_endian_method_xx()?;} )
        }
        FieldType::ArrStructs { .. } => {
            quote!( let mut #var_name: [#arr_ty; #len] = [#arr_ty::default(); #len]; for e in #var_name.iter_mut() {*e = des.deserialize()?;} )
        }
        _ => panic!(
            "this method should only be called ArrayBytes, ArrayNumerics, ArrayStructs types"
        ),
    };

    FieldSerializerDeserializerTokens {
        ser_vars: ser_vars,
        ser_repl: ser_repl,
        ser_uses_stck: ser_uses_xxx(&Ident::new("byte_serialize_stack", Span::call_site())),
        ser_uses_heap: ser_uses_xxx(&Ident::new("byte_serialize_heap", Span::call_site())),
        des_vars: des_vars,
        des_uses: quote!( #var_name, ),
    }
}
fn setup_vec(
    ast: &DeriveInput,
    fld: &Field,
    ty: &Type,
    var_name: &Ident,
    member: &MemberIdent,
    option: &FieldType,
) -> FieldSerializerDeserializerTokens {
    let length = get_length_attribute(&fld.attrs);
    let replace = get_replace_attribute(&fld.attrs);
    let endian = get_endian_attribute(&ast.attrs, &fld.attrs);
    let ser_endian_method_xx = ser_endian_method_xx(&endian);
    let des_endian_method_xx = des_endian_method_xx(&endian);

    let ser_vars = match member {
        MemberIdent::Named(_) => quote!( let #var_name: &#ty = &self.#var_name; ),
        MemberIdent::Unnamed(fld_index) => quote! { let #var_name: &#ty = &self.#fld_index; },
    };

    let ser_repl = match replace {
        Replace::Set(value) => quote!( let #var_name = &#value; ),
        Replace::NotSet => quote!(),
    };

    let ser_uses_xxx = |byte_serialize_xxx: &Ident| match option {
        FieldType::VecBytes { .. } => quote!( ser.serialize_bytes_slice(&#var_name)?; ),
        FieldType::VecNumerics { .. } => {
            quote!( for i in #var_name { ser.#ser_endian_method_xx(*i)?; })
        }
        FieldType::VecStructs { .. } => {
            quote!( for i in #var_name { i.#byte_serialize_xxx(ser)?; })
        }
        _ => panic!("this method should only be called with Vec[Bytes|Numerics|Structs] types"),
    };

    let des_vars_byte = match length {
        Length::Len(ref len) => {
            quote!( let #var_name: #ty = des.deserialize_take::<#ty>( (#len) as usize )?.into(); )
        }
        Length::NotSet => {
            quote!( let #var_name: #ty = des.deserialize_bytes_slice_remaining().into(); )
        }
    };
    let des_vars_numerics = match length {
        Length::Len(ref len) => {
            quote!( let mut #var_name: #ty = vec![]; for _ in 0..#len { #var_name.push(des.#des_endian_method_xx()?); })
        }
        Length::NotSet => {
            quote!( let mut #var_name: #ty = vec![]; while des.is_empty() == false { #var_name.push(des.#des_endian_method_xx()?); })
        }
    };
    let des_vars_other = match length {
        Length::Len(ref len) => {
            quote!( let mut #var_name: #ty = vec![]; for _ in 0..#len { #var_name.push(des.deserialize()?); })
        }
        Length::NotSet => {
            quote!( let mut #var_name: #ty = vec![]; while des.is_empty() == false { #var_name.push(des.deserialize()?); })
        }
    };
    let des_vars_xxx = match option {
        FieldType::VecBytes { .. } => des_vars_byte,
        FieldType::VecNumerics { .. } => des_vars_numerics,
        FieldType::VecStructs { .. } => des_vars_other,
        _ => panic!("this method should only be called with Vec types"),
    };

    FieldSerializerDeserializerTokens {
        ser_vars: ser_vars,
        ser_repl: ser_repl,
        ser_uses_stck: ser_uses_xxx(&Ident::new("byte_serialize_stack", Span::call_site())),
        ser_uses_heap: ser_uses_xxx(&Ident::new("byte_serialize_heap", Span::call_site())),
        des_vars: des_vars_xxx,
        des_uses: quote!( #var_name, ),
    }
}
fn setup_struct(
    fld: &Field,
    var_name: &Ident,
    ty: &Type,
    member: &MemberIdent,
) -> FieldSerializerDeserializerTokens {
    let length = get_length_attribute(&fld.attrs);
    let replace = get_replace_attribute(&fld.attrs);
    let ser_vars = match member {
        MemberIdent::Named(_) => quote! { let #var_name = &self.#var_name; }, // let #var_name = &self.#var_name;
        MemberIdent::Unnamed(fld_index) => quote! { let #var_name = &self.#fld_index; }, // let #var_name = &self.#fld_index;
    };
    let ser_repl = match replace {
        Replace::Set(value) => quote!( let #var_name: &#ty = &#value; ),
        Replace::NotSet => quote!(),
    };
    let des_vars = match length {
        Length::Len(len) => {
            quote!( let #var_name: #ty = des.deserialize_take( (#len) as usize )?; )
        }
        Length::NotSet => quote!( let #var_name: #ty = des.deserialize()?; ),
    };
    FieldSerializerDeserializerTokens {
        ser_vars: ser_vars,
        ser_repl: ser_repl,
        ser_uses_stck: quote!( #var_name.byte_serialize_stack(ser)?; ),
        ser_uses_heap: quote!( #var_name.byte_serialize_heap(ser)?; ),
        des_vars: des_vars,
        des_uses: quote!( #var_name, ),
    }
}

#[derive(PartialEq, Debug)]
enum FieldType<'a> {
    Byte {
        ty: &'a Type,
        signed: bool,
    },
    Numeric {
        ty: &'a Type,
    },
    Struct {
        ty: &'a Type,
    },
    ArrBytes {
        arr_ty: &'a Type,
        len: &'a Expr,
        signed: bool,
    },
    ArrNumerics {
        arr_ty: &'a Type,
        len: &'a Expr,
    },
    ArrStructs {
        arr_ty: &'a Type,
        len: &'a Expr,
    },
    VecBytes {
        ty: &'a Type,
    },
    VecNumerics {
        ty: &'a Type,
    },
    VecStructs {
        ty: &'a Type,
    },
}

fn map_field_type(ty: &Type) -> FieldType {
    match ty {
        Type::Path(TypePath { path, .. }) => path_2_numeric_byte_or_other(path, ty),
        Type::Array(TypeArray {
            elem: arr_ty, len, ..
        }) => match arr_ty.as_ref() {
            Type::Path(TypePath { path, .. }) => match path_2_numeric_byte_or_other(path, arr_ty) {
                FieldType::Byte { signed, .. } => FieldType::ArrBytes {
                    arr_ty,
                    len,
                    signed,
                },
                FieldType::Numeric { .. } => FieldType::ArrNumerics { arr_ty, len },
                FieldType::Struct { .. } => FieldType::ArrStructs { arr_ty, len },
                _ => FieldType::Struct { ty: arr_ty },
            },
            _ => FieldType::Struct { ty: arr_ty },
        },
        _ => FieldType::Struct { ty },
    }
}

fn path_2_numeric_byte_or_other<'a>(path: &'a Path, ty: &'a Type) -> FieldType<'a> {
    // byte
    if path.is_ident("u8") {
        return FieldType::Byte { ty, signed: false };
    }
    if path.is_ident("i8") {
        return FieldType::Byte { ty, signed: true };
    }

    // all non byte numerics
    if path.is_ident("i16")
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
        return FieldType::Numeric { ty };
    }

    // Vec
    if path.segments.len() == 1 && path.segments[0].ident == "Vec" {
        let vec_args = &path.segments[0].arguments;
        if let PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) = vec_args
        {
            if let GenericArgument::Type(Type::Path(path, ..)) = &args[0] {
                return match path_2_numeric_byte_or_other(&path.path, ty) {
                    FieldType::Numeric { .. } => FieldType::VecNumerics { ty },
                    FieldType::Byte { .. } => FieldType::VecBytes { ty },
                    _ => FieldType::VecStructs { ty },
                };
            }
        };
    }

    FieldType::Struct { ty }
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
