use core::panic;

use quote::{
    __private::{Span, TokenStream},
    quote,
};
use syn::{
    AngleBracketedGenericArguments, ConstParam, Data, DeriveInput, Expr, Field, Fields,
    GenericArgument, GenericParam, Generics, Ident, Index, Member, Path, PathArguments, Type,
    TypeArray, TypeParam, TypePath, TypeGroup,
};

use crate::struct_shared::{
    des_endian_method_xx, get_endian_attribute, get_bind_attribute, get_deplete_attribute,
    get_replace_attribute, ser_endian_method_xx, Bind, Deplete, MemberIdent, Replace,
};

pub enum StructType {
    Regular,
    Tuple,
    Enum,
}
#[derive(Debug)]
pub struct FldSerDesTokens {
    // only used to create serailizer
    pub ser_vars: TokenStream,
    pub ser_repl: TokenStream,
    pub ser_uses_stck: TokenStream,
    pub ser_uses_heap: TokenStream,

    // only used to create deserailizer
    pub des_vars: TokenStream,
    pub des_uses: TokenStream,

    // only used to create size
    pub size: TokenStream,
    pub size_error: Option<String>,
    pub len: TokenStream,
}
pub fn get_struct_ser_des_tokens(
    ast: &DeriveInput,
) -> (Vec<FldSerDesTokens>, StructType) {
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
                            FieldType::VecBytes { .. }
                            | FieldType::VecNumerics { .. }
                            | FieldType::VecStructs { .. } => {
                                setup_vec(ast, fld, &fld.ty, var_name, member, &fld_type)
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
                            FieldType::VecBytes { .. }
                            | FieldType::VecNumerics { .. }
                            | FieldType::VecStructs { .. } => {
                                setup_vec(ast, fld, &fld.ty, var_name, member, &fld_type)
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
        Data::Enum(_) => {
            let enum_type = &ast.ident;
            let bind = get_bind_attribute(&ast.attrs);
            let struct_type = match bind {
                Bind::Set(value) => {
                    quote!(#value)
                }
                _ => panic!("Enum {enum_type} needs to be bound to a struct type using bind attribute. Example: `#[byteserde(bind( MyStructName ))]`"),
            };
            ty = StructType::Enum;
            let mut tokens = Vec::<FldSerDesTokens>::new();
            tokens.push(FldSerDesTokens {
                ser_vars: quote!(let _from_enum = #struct_type::from(self);),
                ser_repl: quote!(),
                ser_uses_stck: quote!(ser.serialize(&_from_enum)?;),
                ser_uses_heap: quote!(ser.serialize(&_from_enum)?;),
                des_vars: quote!( let _struct = des.deserialize::<#struct_type>()?; ),
                des_uses: quote!(),
                size: quote!( todo!("Please imlement") ), //TODO
                size_error: None,
                len: quote!( todo!("Please imlement") ), //TODO
            });

            tokens
        }
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
) -> FldSerDesTokens {
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

    let size = quote!( ::std::mem::size_of::<#ty>() );
    let len = quote!( ::std::mem::size_of::<#ty>() );
    FldSerDesTokens {
        ser_vars,
        ser_repl,
        ser_uses_stck: ser_uses_xxx.clone(),
        ser_uses_heap: ser_uses_xxx,
        des_vars,
        des_uses: quote!( #var_name, ),
        size,
        size_error: None,
        len,
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
) -> FldSerDesTokens {
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
    let size = match option{
        FieldType::ArrBytes { .. } | FieldType::ArrNumerics { .. } =>  quote!( ::std::mem::size_of::<#arr_ty>() * #len ),
        FieldType::ArrStructs { .. } => quote!( #arr_ty::byte_size() * #len ),
        _ => panic!(
            "this method should only be called ArrayBytes, ArrayNumerics, ArrayStructs types"
        ),
    };

    let len_var = match member {
        MemberIdent::Named(fld_name) => {
            quote!( self.#fld_name )
        }
        MemberIdent::Unnamed(fld_index) => {
            quote!( self.#fld_index )
        }
    };
    let len = match option{
        FieldType::ArrBytes { .. } | FieldType::ArrNumerics { .. } =>  quote!( (::std::mem::size_of::<#arr_ty>() * #len) ),
        FieldType::ArrStructs { .. } => quote!( ({ let mut len = 0; for e in #len_var.iter() { len += e.byte_len(); } len }) ),
        _ => panic!(
            "this method should only be called ArrayBytes, ArrayNumerics, ArrayStructs types"
        ),
    };
    
    FldSerDesTokens {
        ser_vars,
        ser_repl,
        ser_uses_stck: ser_uses_xxx(&Ident::new("byte_serialize_stack", Span::call_site())),
        ser_uses_heap: ser_uses_xxx(&Ident::new("byte_serialize_heap", Span::call_site())),
        des_vars,
        des_uses: quote!( #var_name, ),
        size,
        size_error: None,
        len,
    }
}

fn setup_vec(
    ast: &DeriveInput,
    fld: &Field,
    ty: &Type,
    var_name: &Ident,
    member: &MemberIdent,
    option: &FieldType,
) -> FldSerDesTokens {
    let deplete = get_deplete_attribute(&fld.attrs);
    let replace = get_replace_attribute(&fld.attrs);
    let endian = get_endian_attribute(&ast.attrs, &fld.attrs);
    let ser_endian_method_xx = ser_endian_method_xx(&endian);
    let des_endian_method_xx = des_endian_method_xx(&endian);

    let member_name = match member {
        MemberIdent::Named(fld_name) => {
            quote!( self.#fld_name )
        }
        MemberIdent::Unnamed(fld_index) => {
            quote!( self.#fld_index )
        }
    };
    let ser_vars = match member {
        MemberIdent::Named(_) => quote!( let #var_name: &#ty = &self.#var_name; ),
        MemberIdent::Unnamed(fld_index) => quote! { let #var_name: &#ty = &self.#fld_index; },
    };

    let ser_repl = match replace {
        Replace::Set(ref value) => quote!( let #var_name = &#value; ),
        Replace::NotSet => quote!(),
    };

    let struct_name = &ast.ident;
    let assert_error = match member {
        MemberIdent::Named(fld_name) => {
            format!("{}.{} field #[byteserde(deplete( .. ))] set higther then length of Vec instance", quote!(#struct_name), quote!( #fld_name ))
        }
        MemberIdent::Unnamed(fld_index) => {
            format!("{}.{} field #[byteserde(deplete( .. ))] set higther then length of Vec instance", quote!(#struct_name), quote!( #fld_index ))
        }
    };
    let assert_vec_len_gt_then_deplete = match deplete {
        Deplete::Size(ref size) => {
            quote!( assert!(#var_name.len() >= #size, #assert_error) )
        }
        Deplete::NotSet => quote!(),
    };
    let vec_deplete_len = match deplete{
        Deplete::Size(ref size) => quote!( #size ),
        Deplete::NotSet => quote!( #member_name.len() ),
    };
    let ser_uses_xxx = |byte_serialize_xxx: &Ident| match option {
        FieldType::VecBytes { .. } => quote!( #assert_vec_len_gt_then_deplete; ser.serialize_bytes_slice(&#var_name[..#vec_deplete_len])?; ),
        FieldType::VecNumerics { .. } => {
            quote!( #assert_vec_len_gt_then_deplete; for (idx, n) in #var_name.iter().enumerate() { if idx >= #vec_deplete_len {break;} ser.#ser_endian_method_xx(*n)?; })
        }
        FieldType::VecStructs { .. } => {
            quote!( #assert_vec_len_gt_then_deplete; for (idx, n) in #var_name.iter().enumerate() { if idx >= #vec_deplete_len {break;} n.#byte_serialize_xxx(ser)?; })
        }
        _ => panic!("this method should only be called with Vec[Bytes|Numerics|Structs] types"),
    };

    let des_vars_byte = match deplete {
        Deplete::Size(ref size) => {
            quote!( let #var_name: #ty = des.deserialize_take::<#ty>( (#size) as usize )?.into(); )
        }
        Deplete::NotSet => {
            quote!( let #var_name: #ty = des.deserialize_bytes_slice_remaining().into(); )
        }
    };
    let des_vars_numerics = match deplete {
        Deplete::Size(ref size) => {
            quote!( let mut #var_name: #ty = vec![]; for _ in 0..#size { #var_name.push(des.#des_endian_method_xx()?); })
        }
        Deplete::NotSet => {
            quote!( let mut #var_name: #ty = vec![]; while des.is_empty() == false { #var_name.push(des.#des_endian_method_xx()?); })
        }
    };
    let des_vars_other = match deplete {
        Deplete::Size(ref size) => {
            quote!( let mut #var_name: #ty = vec![]; for _ in 0..#size { #var_name.push(des.deserialize()?); })
        }
        Deplete::NotSet => {
            quote!( let mut #var_name: #ty = vec![]; while des.is_empty() == false { #var_name.push(des.deserialize()?); })
        }
    };
    let des_vars_xxx = match option {
        FieldType::VecBytes { .. } => des_vars_byte,
        FieldType::VecNumerics { .. } => des_vars_numerics,
        FieldType::VecStructs { .. } => des_vars_other,
        _ => panic!("this method should only be called with Vec types"),
    };
    


    let len = match option{
        FieldType::VecBytes { vec_ty } | FieldType::VecNumerics { vec_ty } =>  quote!( (::std::mem::size_of::<#vec_ty>() * #vec_deplete_len) ),
        FieldType::VecStructs { .. } => 
                    match replace {
                        Replace::Set(ref value) => quote!( ({ let mut len = 0; for (idx, e) in #value.iter().enumerate() { if idx >= #vec_deplete_len {break} len += e.byte_len(); } len }) ),
                        Replace::NotSet => quote!( ({ let mut len = 0; for (idx, e) in #member_name.iter().enumerate() { if idx >= #vec_deplete_len {break} len += e.byte_len(); } len }) ),
                    },
        _ => panic!(
            "this method should only be called ArrayBytes, ArrayNumerics, ArrayStructs types"
        ),
    };
    let size_error = Some(format!("trait ByteSerializedLenOf can't be implemented for struct {} because it has a member {} of Vec type whose size is not know at compile time", &ast.ident, member_name));
    FldSerDesTokens {
        ser_vars,
        ser_repl,
        ser_uses_stck: ser_uses_xxx(&Ident::new("byte_serialize_stack", Span::call_site())),
        ser_uses_heap: ser_uses_xxx(&Ident::new("byte_serialize_heap", Span::call_site())),
        des_vars: des_vars_xxx,
        des_uses: quote!( #var_name, ),
        size: quote!( 0 ), 
        size_error,
        len,
    }
}
fn setup_struct(
    fld: &Field,
    var_name: &Ident,
    ty: &Type,
    member: &MemberIdent,
) -> FldSerDesTokens {
    let length = get_deplete_attribute(&fld.attrs);
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
        Deplete::Size(len) => {
            quote!( let #var_name: #ty = des.deserialize_take( (#len) as usize )?; )
        }
        Deplete::NotSet => quote!( let #var_name: #ty = des.deserialize()?; ),
    };
    FldSerDesTokens {
        ser_vars,
        ser_repl,
        ser_uses_stck: quote!( #var_name.byte_serialize_stack(ser)?; ),
        ser_uses_heap: quote!( #var_name.byte_serialize_heap(ser)?; ),
        des_vars,
        des_uses: quote!( #var_name, ),
        size: quote!( todo!("Please imlement") ), //TODO
        size_error: None,
        len: quote!( todo!("Please imlement") ), //TODO
    }
}

#[derive(Debug)]
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
        vec_ty: Type,
    },
    VecNumerics {
        vec_ty: Type,
    },
    #[allow(dead_code)] // TODO might be used later for now disable warning
    VecStructs {
        vec_ty: Type,
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
        // for some reason when usig macro_rules! to create a tuple struct ex: struct Me(u32) the type of the tuple comes in the TypeGroup instead of TypePath so we need to handle it here
        Type::Group(TypeGroup { elem, .. }) => map_field_type(elem),
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
                let vec_ty = Type::Path(path.clone());
                return match path_2_numeric_byte_or_other(&path.path, &vec_ty) {
                    FieldType::Numeric { .. } => FieldType::VecNumerics { vec_ty  },
                    FieldType::Byte { .. } => FieldType::VecBytes { vec_ty },
                    _ => FieldType::VecStructs { vec_ty },
                };
        
            }
        };
    }

    FieldType::Struct { ty }
}

pub fn get_generics(generics: &Generics) -> (TokenStream, TokenStream, TokenStream) {
    let type_alias = generics
        .params
        .iter()
        .map(|param| {
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
    let where_clause = match &generics.where_clause {
        Some(where_clause) => quote! ( #where_clause ),
        None => quote! (),
    };
    match generics.params.len() {
        0 => (quote! (), quote! (), where_clause),
        _ => (quote! ( #generics ), quote! ( < #(#type_alias),* > ), where_clause ),
    }
}
