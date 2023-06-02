use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;
use tokens_enum::get_enum_from_tokens;
use tokens_struct::{get_generics, get_struct_ser_des_tokens};

use crate::{
    attr_struct::{peek_attr, Peek},
    common::{StructType}, validate_struct::has_option_flds,
};
// test only
#[cfg(test)]
pub mod unittest;

mod attr_enum;
mod attr_struct;
mod common;
mod tokens_enum;
mod tokens_struct;
mod validate_struct;
#[proc_macro_derive(ByteSerializeStack, attributes(byteserde))]
pub fn byte_serialize_stack(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    // get struct name
    let struct_name = &ast.ident;
    let (generics_declaration, generics_alias, where_clause) = get_generics(&ast.generics);
    let res = get_struct_ser_des_tokens(&ast);
    // grap just stack presets
    let ser_vars = res.ser_vars();
    let ser_relp = res.ser_repl();
    let ser_uses_stck = res.ser_uses_stck();

    // generate stack serializer
    let output = quote! {
        #[automatically_derived]
        impl #generics_declaration ::byteserde::ser::ByteSerializeStack for #struct_name #generics_alias #where_clause{
        // impl byteserde::ser::ByteSerializeStack for #struct_name {
            fn byte_serialize_stack<const CAP: usize>(&self, ser: &mut ::byteserde::ser::ByteSerializerStack<CAP>) -> ::byteserde::error::Result<()>{
                // numerics
                //      ser.serialize_[be|le|ne](self.field_name)?; -- for regular
                //      ser.serialize_[be|le|ne](self.0         )?; -- for tuple
                // trait ByteSerializeStack
                //      self.field_name.byte_serialize_stack(ser)?;     -- for regular
                //      self.0         .byte_serialize_stack(ser)?;     -- for tuple
                #( #ser_vars )*
                #( #ser_relp )*
                #( #ser_uses_stck )*
                Ok(())
            }
        }
    };
    output.into()
}

#[proc_macro_derive(ByteSerializeHeap, attributes(byteserde))]
pub fn byte_serialize_heap(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    // get struct name
    let struct_name = &ast.ident;
    let (generics_declaration, generics_alias, where_clause) = get_generics(&ast.generics);
    // get ser & des quote presets
    let res = get_struct_ser_des_tokens(&ast);
    // grap just heap presets
    let ser_vars = res.ser_vars();
    let ser_repl = res.ser_repl();
    let ser_uses_heap = res.ser_uses_heap();

    // generate heap serializer
    let output = quote! {
        #[automatically_derived]
        impl #generics_declaration ::byteserde::ser::ByteSerializeHeap for #struct_name #generics_alias #where_clause{
            fn byte_serialize_heap(&self, ser: &mut ::byteserde::ser::ByteSerializerHeap) -> ::byteserde::error::Result<()>{
                // numerics
                //      ser.serialize_[be|le|ne](self.field_name)?;         -- for regular
                //      ser.serialize_[be|le|ne](self.0         )?;         -- for tuple
                // trait ByteSerializeStack
                //      self.field_name.byte_serialize_heap(ser)?;          -- for regular
                //      self.0         .byte_serialize_heap(ser)?;          -- for tuple
                #( #ser_vars)*
                #( #ser_repl)*
                #( #ser_uses_heap)*
                Ok(())
            }
        }
    };
    output.into()
}

#[proc_macro_derive(ByteDeserialize, attributes(byteserde))]
pub fn byte_deserialize(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    // get struct name
    let (generics_declaration, generics_alias, where_clause) = get_generics(&ast.generics);
    // get ser & des quote presets
    let sdt = get_struct_ser_des_tokens(&ast);

    if let Some(msg) = sdt.des_errs() {
        panic!("Error \n{}", msg);
    }
    let des_vars = sdt.des_vars();
    let des_option = sdt.des_option();
    let des_uses = sdt.des_uses();
    let (name, id) = match sdt.struct_type {
        StructType::Regular(ref name, ref id)
        | StructType::Tuple(ref name, ref id)
        | StructType::Enum(ref name, ref id) => (name, id),
    };

    let impl_body = match sdt.struct_type {
        StructType::Regular(..) => quote!(#id {#( #des_uses )*}), // NOTE {}
        StructType::Tuple(..) => quote!  (#id (#( #des_uses )*)), // NOTE ()
        StructType::Enum(..) => quote! ( #id::from( _struct )),   // NOTE ::from()
    };

    let has_option_flds = has_option_flds(&sdt);
    let start_len = match peek_attr(&ast.attrs) {
        Peek::Set(v) => quote!(#v),
        Peek::NotSet => {
            match has_option_flds{
                true => panic!("{name} requires `#[byteserde(peek( start, len ))]` attribute because it has option members defined with `#[byteserde(eq( .. ))]`"),
                false => quote!(),
            }
        } // panic if o
    };
    eprintln!("struct_name: {}", name);
    eprintln!("start_len: {}", start_len);
    let option = match has_option_flds {
        true => quote!(
                    while !des.is_empty() {
                        let peek = |start, len| -> Result<&[u8]> {
                            let p = des.peek_bytes_slice(len+start)?;
                            Ok(&p[start..])
                        };
                        let __peeked = peek(#start_len)?;
                        #( #des_option )*
                    }
        ),
        false => quote!(),
    };
    eprintln!("option: {}", option);
    // generate deserializer

    let output = quote!(
        #[automatically_derived]
        impl #generics_declaration ::byteserde::des::ByteDeserialize<#id #generics_alias> for #id #generics_alias #where_clause{
            fn byte_deserialize(des: &mut ::byteserde::des::ByteDeserializer) -> ::byteserde::error::Result<#id #generics_alias>{
                // let type_u16:    u16 = des.deserialize_[be|le|ne]()?; -- numerics
                // let type_String: String = des.deserialize()?;          -- trait ByteDeserialize
                // StructName { type_u16, type_String }
                //
                // let _0 = des.deserialize_[be|le|ne]()?; -- numerics
                // let _1  = des.deserialize()?;          -- trait ByteDeserialize
                // TupleName ( _0, _1 )
                #( #des_vars )*
                #option
                Ok(#impl_body)
            }
        }
    );
    output.into()
}

#[proc_macro_derive(ByteSerializedSizeOf, attributes(byteserde))]
pub fn byte_serialized_size_of(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    // get struct name
    let struct_name = &ast.ident;
    let (generics_declaration, generics_alias, where_clause) = get_generics(&ast.generics);
    // get ser & des quote presets
    let res = get_struct_ser_des_tokens(&ast);
    // grap just heap presets
    let size = res.size_of();
    let size_errors = res.size_errors();
    if let Some(msg) = size_errors {
        panic!("Error \n{}", msg);
    }

    // generate deserializer
    let output = quote! {
        #[automatically_derived]
        impl #generics_declaration ::byteserde::size::ByteSerializedSizeOf for #struct_name #generics_alias #where_clause{
            fn byte_size() -> usize{
                # ( #size )+*
            }
        }
    };
    output.into()
}
#[proc_macro_derive(ByteSerializedLenOf, attributes(byteserde))]
pub fn byte_serialized_len_of(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    // get struct name
    let struct_name = &ast.ident;
    let (generics_declaration, generics_alias, where_clause) = get_generics(&ast.generics);
    // get ser & des quote presets
    let res = get_struct_ser_des_tokens(&ast);
    // grap just heap presets
    let len = res.flds.iter().map(|f| &f.len_of).collect::<Vec<_>>();

    // eprintln!("size: {:?}", size);

    // generate deserializer
    let output = quote! {
        #[automatically_derived]
        impl #generics_declaration ::byteserde::size::ByteSerializedLenOf for #struct_name #generics_alias #where_clause{
            fn byte_len(&self) -> usize{
                # ( #len )+*
            }
        }
    };
    output.into()
}

#[proc_macro_derive(ByteEnumFromBinder, attributes(byteserde))]
pub fn byte_enum_from(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    let froms = get_enum_from_tokens(&ast);
    // generate From
    let output = quote! {
         #(#froms)*
    };
    output.into()
}
