use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;
use tokens_enum::get_enum_from_tokens;
use tokens_struct::{get_generics, get_struct_tokens};

use crate::{
    attr_struct::{peek_attr, Peek},
    common::StructType,
};
// test only
#[cfg(test)]
pub mod unittest;

mod attr_struct;
mod common;
mod tokens_enum;
mod tokens_struct;

#[proc_macro_derive(ByteSerializeStack, attributes(byteserde))]
pub fn byte_serialize_stack(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    // get struct name
    let struct_name = &ast.ident;
    let (generics_declaration, generics_alias, where_clause) = get_generics(&ast.generics);
    let res = get_struct_tokens(&ast);
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
    let res = get_struct_tokens(&ast);
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
    let sdt = get_struct_tokens(&ast);

    let peek = peek_attr(&ast.attrs);
    sdt.des_validate(&peek);

    let des_vars = sdt.des_vars();
    let des_option = sdt.des_option();
    let des_uses = sdt.des_uses();
    let id = sdt.struct_ident();

    let impl_body = match sdt.struct_type {
        StructType::Regular(..) => quote!(#id {#( #des_uses )*}), // NOTE {}
        StructType::Tuple(..) => quote!  (#id (#( #des_uses )*)), // NOTE ()
        StructType::Enum(..) => quote! ( #id::from( _struct )),   // NOTE ::from()
    };

    let start_len = match peek {
        Peek::Set(v) => quote!(#v),
        Peek::NotSet => quote!(),
    };

    let des_option_special = match sdt.has_option_flds() {
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
                #des_option_special
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
    let res = get_struct_tokens(&ast);
    // grap just heap presets
    res.size_validate();
    let size = res.size_of();

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
    let res = get_struct_tokens(&ast);
    // grap just heap presets
    let len = res.len_of();

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
