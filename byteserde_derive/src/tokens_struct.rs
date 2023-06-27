use core::panic;

use quote::{
    __private::{Span, TokenStream},
    quote,
};
use syn::{
    AngleBracketedGenericArguments, ConstParam, Data, DeriveInput, Expr, Field, Fields,
    GenericArgument, GenericParam, Generics, Ident, Index, Member, Path, PathArguments, Type,
    TypeArray, TypeGroup, TypeParam, TypePath,
};

use crate::{
    attr_struct::{
        deplete_attr, des_endian_method_xx, endian_attr, eq_attr, replace_attr,
        ser_endian_method_xx, Deplete, MemberIdent, PeekEq, Replace,
    },
    common::{FldSerDesTokens, SerDesTokens, StructType},
};

pub fn get_struct_tokens(ast: &DeriveInput) -> SerDesTokens {
    let ty: StructType;
    let id = &ast.ident;
    let flds_tokens = match &ast.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(flds) => {
                ty = StructType::Regular(format!("{}", id), id.clone());
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
                            FieldType::OptionStructs { .. } => {
                                setup_option(ast, fld, &fld.ty, var_name, member, &fld_type)
                            }
                            FieldType::Struct { ty } => setup_struct(fld, var_name, ty, member),
                        }
                    })
                    .collect::<Vec<_>>()
            }
            Fields::Unnamed(flds) => {
                ty = StructType::Tuple(format!("{}", id), id.clone());
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
                            FieldType::OptionStructs { .. } => {
                                setup_option(ast, fld, &fld.ty, var_name, member, &fld_type)
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
        Data::Enum(data) => {
            ty = StructType::Enum(format!("{}", id), id.clone());
            let mut tokens = Vec::<FldSerDesTokens>::new();

            for variant in data.variants.pairs() {
                let variant_id = variant.value().ident.clone();
                let eq = match eq_attr(&variant.value().attrs){
                    PeekEq::Set(eq) => eq,
                    PeekEq::NotSet => panic!("enum '{id}' variant '{variant_id}' missing required #[byteserde(eq( ... ))] attribute. It is matched vs #[byteserde(peek(start, len))] to determin deserilization struct."), 
                };
                let default = FldSerDesTokens {
                    ser_vars: quote!(),
                    ser_repl: quote!(),
                    ser_uses_stck: quote!(),
                    ser_uses_heap: quote!(),
                    des_vars: quote!(),
                    des_peeked: quote!(),
                    des_uses: quote!(),
                    des_errors: vec![],
                    size_of: quote!(), //TODO enum size of support
                    size_errors: vec![],
                    len_of: quote!(), //TODO enum len support
                };
                // deserializer
                match &variant.value().fields {
                    Fields::Unnamed(flds) => {
                        let des_uses = flds
                            .unnamed
                            .iter()
                            .map(|fld| fld.ty.clone())
                            .collect::<Vec<_>>();
                        let des_peeked = quote!( if __peeked == #eq { return Ok( Self::#variant_id( #( #des_uses::byte_deserialize(des)? ),* ) )} );
                        tokens.push(FldSerDesTokens {
                            des_peeked,
                            ..default.clone()
                        });
                    }
                    _ => {
                        panic!("enum '{}' has an unsupported variant '{}'. Only tuple-like style variants are supported", id, quote!(#variant))
                    }
                }
                // serializer
                match &variant.value().fields {
                    Fields::Unnamed(flds) => {
                        let unnamed_members = flds
                            .unnamed
                            .iter()
                            .enumerate()
                            .map(|(i, _)| Ident::new(&format!("_{}", i), ast.ident.span()))
                            .collect::<Vec<_>>();
                        let ser_uses_stck = quote!( Self::#variant_id(#(#unnamed_members),*) => { #(#unnamed_members.byte_serialize_stack(ser)?;)*}, );
                        let ser_uses_heap = quote!( Self::#variant_id(#(#unnamed_members),*) => { #( #unnamed_members.byte_serialize_heap(ser)?;)*}, );

                        tokens.push(FldSerDesTokens {
                            ser_uses_stck: quote!( #ser_uses_stck ),
                            ser_uses_heap: quote!( #ser_uses_heap ),
                            len_of: quote!(),
                            ..default.clone()
                        });
                    }
                    _ => {} // will panic during deserializer
                }
            }
            tokens
        }
        _ => {
            panic!(
                "Only struct types supported, found '{struct_name}' of type '{ty}'",
                ty = match ast.data {
                    Data::Union(_) => "union",
                    _ => "unknow",
                },
                struct_name = &ast.ident
            )
        }
    };
    SerDesTokens {
        struct_type: ty,
        flds: flds_tokens,
    }
}

fn setup_numeric(
    ast: &DeriveInput,
    fld: &Field,
    ty: &Type,
    var_name: &Ident,
    member: &MemberIdent,
    option: &FieldType,
) -> FldSerDesTokens {
    let replace = replace_attr(&fld.attrs);
    let endian = endian_attr(&ast.attrs, &fld.attrs);
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
        FieldType::Byte { signed, .. } => match signed {
            true => quote!( let #var_name: #ty = des.deserialize_i8()?; ),
            false => quote!( let #var_name: #ty = des.deserialize_u8()?; ),
        },
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
        des_peeked: quote!(), // does not apply here
        des_uses: quote!( #var_name, ),
        des_errors: vec![],
        size_of: size,
        size_errors: vec![],
        len_of: len,
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
    let replace = replace_attr(&fld.attrs);
    let endian = endian_attr(&ast.attrs, &fld.attrs);
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
            false => {
                quote!( let #var_name: [#arr_ty; #len] = *des.deserialize_bytes_array_ref()?; )
            }
            true => {
                quote!( let #var_name: [u8; #len] = *des.deserialize_bytes_array_ref()?; 
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
    let size = match option {
        FieldType::ArrBytes { .. } | FieldType::ArrNumerics { .. } => {
            quote!( ::std::mem::size_of::<#arr_ty>() * #len )
        }
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
    let len = match option {
        FieldType::ArrBytes { .. } | FieldType::ArrNumerics { .. } => {
            quote!( (::std::mem::size_of::<#arr_ty>() * #len) )
        }
        FieldType::ArrStructs { .. } => {
            quote!( ({ let mut len = 0; for e in #len_var.iter() { len += e.byte_len(); } len }) )
        }
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
        des_peeked: quote!(), // does not apply here
        des_uses: quote!( #var_name, ),
        des_errors: vec![],
        size_of: size,
        size_errors: vec![],
        len_of: len,
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
    let deplete = deplete_attr(&fld.attrs);
    let replace = replace_attr(&fld.attrs);
    let endian = endian_attr(&ast.attrs, &fld.attrs);
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
            format!(
                "{}.{} field #[byteserde(deplete( .. ))] set higther then length of Vec instance",
                quote!(#struct_name),
                quote!( #fld_name )
            )
        }
        MemberIdent::Unnamed(fld_index) => {
            format!(
                "{}.{} field #[byteserde(deplete( .. ))] set higther then length of Vec instance",
                quote!(#struct_name),
                quote!( #fld_index )
            )
        }
    };
    let assert_vec_len_gt_then_deplete = match deplete {
        Deplete::Size(ref size) => {
            quote!(assert!(#var_name.len() >= #size, #assert_error))
        }
        Deplete::NotSet => quote!(),
    };
    let vec_deplete_len = match deplete {
        Deplete::Size(ref size) => quote!( #size ),
        Deplete::NotSet => quote!( #member_name.len() ),
    };
    let ser_uses_xxx = |byte_serialize_xxx: &Ident| match option {
        FieldType::VecBytes { .. } => {
            quote!( #assert_vec_len_gt_then_deplete; ser.serialize_bytes_slice(&#var_name[..#vec_deplete_len])?; )
        }
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

    let len = match option {
        FieldType::VecBytes { vec_ty } | FieldType::VecNumerics { vec_ty } => {
            quote!( (::std::mem::size_of::<#vec_ty>() * #vec_deplete_len) )
        }
        FieldType::VecStructs { .. } => match replace {
            Replace::Set(ref value) => {
                quote!( ({ let mut len = 0; for (idx, e) in #value.iter().enumerate() { if idx >= #vec_deplete_len {break} len += e.byte_len(); } len }) )
            }
            Replace::NotSet => {
                quote!( ({ let mut len = 0; for (idx, e) in #member_name.iter().enumerate() { if idx >= #vec_deplete_len {break} len += e.byte_len(); } len }) )
            }
        },
        _ => panic!(
            "this method should only be called ArrayBytes, ArrayNumerics, ArrayStructs types"
        ),
    };
    let size_error: Vec<String> = vec![format!("trait ByteSerializedLenOf can't be implemented for struct {} because it has a member {} of Vec type whose size is not know at compile time", &ast.ident, member_name)];
    FldSerDesTokens {
        ser_vars,
        ser_repl,
        ser_uses_stck: ser_uses_xxx(&Ident::new("byte_serialize_stack", Span::call_site())),
        ser_uses_heap: ser_uses_xxx(&Ident::new("byte_serialize_heap", Span::call_site())),
        des_vars: des_vars_xxx,
        des_peeked: quote!(), // does not apply here
        des_uses: quote!( #var_name, ),
        des_errors: vec![],
        size_of: quote!(0),
        size_errors: size_error,
        len_of: len,
    }
}
fn setup_struct(fld: &Field, var_name: &Ident, ty: &Type, member: &MemberIdent) -> FldSerDesTokens {
    let length = deplete_attr(&fld.attrs);
    let replace = replace_attr(&fld.attrs);
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
        des_peeked: quote!(), // does not apply here
        des_uses: quote!( #var_name, ),
        des_errors: vec![],
        size_of: quote!( #ty.byte_size() ),
        size_errors: vec![],
        len_of: quote!( self.#var_name.byte_len() ),
    }
}

fn setup_option(
    ast: &DeriveInput,
    fld: &Field,
    fld_ty: &Type,
    var_name: &Ident,
    member: &MemberIdent,
    option: &FieldType,
) -> FldSerDesTokens {
    let mut des_errors = vec![];
    let struct_name = &ast.ident;
    let fld_name = match member {
        MemberIdent::Named(fld_name) => quote!(#fld_name),
        MemberIdent::Unnamed(fld_idx) => quote!(#fld_idx),
    };
    let replace = replace_attr(&fld.attrs);
    let eq = match eq_attr(&fld.attrs) {
        PeekEq::Set(value) => quote!(#value),
        PeekEq::NotSet => {
            des_errors.push(format!(
                "{struct_name}.{fld_name} is Option<T> type and hence requires `#[byteserde(eq( ... ))] attribute it that evalutes to a byte slice and complared with &[u8] of `#[byteserde(peek( start, len ))]` expression",
            ));
            quote!()
        }
    };

    // serializer
    let ser_vars = match member {
        MemberIdent::Named(_) => quote!( let #var_name: &#fld_ty = &self.#var_name; ),
        MemberIdent::Unnamed(fld_index) => quote! { let #var_name: &#fld_ty = &self.#fld_index; },
    };

    let ser_repl = match replace {
        Replace::Set(ref value) => quote!( let #var_name = &#value; ),
        Replace::NotSet => quote!(),
    };

    let ser_uses_xxx = |byte_serialize_xxx: &Ident| match option {
        FieldType::OptionStructs { .. } => {
            quote!( match #var_name { Some(v) => v.#byte_serialize_xxx(ser)?, None => {}, } )
        }
        _ => panic!("this method should only be called with OptionStructs types"),
    };

    // TODO does it make sense to default Option size to Some size?
    let size_of = match option {
        FieldType::OptionStructs { opt_ty } => quote!( Option::<#opt_ty>::byte_size() ),
        _ => panic!("this method should only be called with Option types"),
    };

    // eprintln!("opt_ty: {:?}", format!("{}", quote!(#fld_ty)));
    FldSerDesTokens {
        ser_vars,
        ser_repl,
        ser_uses_stck: ser_uses_xxx(&Ident::new("byte_serialize_stack", Span::call_site())),
        ser_uses_heap: ser_uses_xxx(&Ident::new("byte_serialize_heap", Span::call_site())),
        des_vars: quote!( let mut #var_name: #fld_ty = None; ),
        des_peeked: quote!(if __peeked == #eq { #var_name = Some(des.deserialize()?); continue; }),
        des_uses: quote!( #var_name, ),
        des_errors,
        size_of,
        size_errors: vec![],
        len_of: quote!( self.#var_name.byte_len() ),
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
    OptionStructs {
        opt_ty: Type,
    },
}

fn map_field_type(ty: &Type) -> FieldType {
    // eprintln!("\tmap_field_type: {:?}", ty);
    match ty {
        Type::Path(TypePath { path, .. }) => path_2_byte_numeric_vec_struct(path, ty),
        Type::Array(TypeArray {
            elem: arr_ty, len, ..
        }) => match arr_ty.as_ref() {
            Type::Path(TypePath { path, .. }) => match path_2_byte_numeric_vec_struct(path, arr_ty)
            {
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

fn path_2_byte_numeric_vec_struct<'a>(path: &'a Path, ty: &'a Type) -> FieldType<'a> {
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
                return match path_2_byte_numeric_vec_struct(&path.path, &vec_ty) {
                    FieldType::Numeric { .. } => FieldType::VecNumerics { vec_ty },
                    FieldType::Byte { .. } => FieldType::VecBytes { vec_ty },
                    _ => FieldType::VecStructs { vec_ty },
                };
            }
        };
    }
    // Option
    if path.segments.len() == 1 && path.segments[0].ident == "Option" {
        let opt_args = &path.segments[0].arguments;
        if let PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) = opt_args
        {
            if let GenericArgument::Type(Type::Path(path, ..)) = &args[0] {
                // eprintln!("\t\tOption: {:?}", path);
                let opt_ty = Type::Path(path.clone());
                return match path_2_byte_numeric_vec_struct(&path.path, &opt_ty) {
                    FieldType::Struct { .. } => FieldType::OptionStructs { opt_ty },
                    // FieldType::Byte { .. } => FieldType::OptionBytes { opt_ty } ,
                    // FieldType::Numeric { .. } => FieldType::OptionNumerics { opt_ty },
                    _ => panic!("Option of Byte & Numerics are not supported only of other struct types. Ex: Option<SomeStruct>"),
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
        None => quote!(),
    };
    match generics.params.len() {
        0 => (quote!(), quote!(), where_clause),
        _ => (
            quote! ( #generics ),
            quote! ( < #(#type_alias),* > ),
            where_clause,
        ),
    }
}
