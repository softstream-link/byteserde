use common::{get_generics, get_struct_ser_des_tokens};
use enum_map::get_enum_from_tokens;
use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

mod common;
mod enum_map;
mod struct_shared;
#[proc_macro_derive(ByteSerializeStack, attributes(byteserde))]
pub fn byte_serialize_stack(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    // get struct name
    let struct_name = &ast.ident;
    let (generics_declaration, generics_alias, where_clause) = get_generics(&ast.generics);
    let (ser_method, _) = get_struct_ser_des_tokens(&ast);
    // grap just stack presets
    let ser_vars = ser_method.iter().map(|f| &f.ser_vars).collect::<Vec<_>>();
    let ser_over = ser_method.iter().map(|f| &f.ser_repl).collect::<Vec<_>>();
    let ser_uses_stack = ser_method
        .iter()
        .map(|f| &f.ser_uses_stck)
        .collect::<Vec<_>>();

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
                #( #ser_over )*
                #( #ser_uses_stack )*
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
    let (ser_method, _) = get_struct_ser_des_tokens(&ast);
    // grap just heap presets
    let ser_vars = ser_method.iter().map(|f| &f.ser_vars).collect::<Vec<_>>();
    let ser_over = ser_method.iter().map(|f| &f.ser_repl).collect::<Vec<_>>();
    let ser_uses_heap = ser_method
        .iter()
        .map(|f| &f.ser_uses_heap)
        .collect::<Vec<_>>();

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
                #( #ser_over)*
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
    let struct_name = &ast.ident;
    let (generics_declaration, generics_alias, where_clause) = get_generics(&ast.generics);
    // get ser & des quote presets
    let (des_method, ty) = get_struct_ser_des_tokens(&ast);
    // grap just heap presets
    let des_vars = des_method.iter().map(|f| &f.des_vars).collect::<Vec<_>>();
    let des_uses = des_method.iter().map(|f| &f.des_uses).collect::<Vec<_>>();
    let impl_body = match ty {
        common::StructType::Regular => quote!(#struct_name {#( #des_uses )*}), // NOTE {}
        common::StructType::Tuple => quote!  (#struct_name (#( #des_uses )*)), // NOTE ()
        common::StructType::Enum => quote! ( #struct_name::from( _struct )),   // NOTE ::from()
    };

    // generate deserializer
    let output = quote! {
        #[automatically_derived]
        impl #generics_declaration ::byteserde::des::ByteDeserialize<#struct_name #generics_alias> for #struct_name #generics_alias #where_clause{
            fn byte_deserialize(des: &mut ::byteserde::des::ByteDeserializer) -> ::byteserde::error::Result<#struct_name #generics_alias>{
                // let type_u16:    u16 = des.deserialize_[be|le|ne]()?; -- numerics
                // let type_String: String = des.deserialize()?;          -- trait ByteDeserialize
                // StructName { type_u16, type_String }
                //
                // let _0 = des.deserialize_[be|le|ne]()?; -- numerics
                // let _1  = des.deserialize()?;          -- trait ByteDeserialize
                // TupleName ( _0, _1 )
                #( #des_vars )*
                Ok(#impl_body)
            }
        }
    };
    output.into()
}


#[proc_macro_derive(ByteSerializedSizeOf, attributes(byteserde))]
pub fn byte_serialized_size_of(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    // get struct name
    let struct_name = &ast.ident;
    let (generics_declaration, generics_alias, where_clause) = get_generics(&ast.generics);
    // get ser & des quote presets
    let (size, _) = get_struct_ser_des_tokens(&ast);
    // grap just heap presets
    let size = size.iter().map(|f| &f.size).collect::<Vec<_>>();

    // eprintln!("size: {:?}", size);

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
    let (len, _) = get_struct_ser_des_tokens(&ast);
    // grap just heap presets
    let len = len.iter().map(|f| &f.len).collect::<Vec<_>>();

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
