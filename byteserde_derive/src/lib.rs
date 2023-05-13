use common::{get_crate_name, get_generics, get_struct_ser_des_tokens};
use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

mod common;
mod struct_shared;

#[proc_macro_derive(ByteSerializeStack, attributes(byteserde))]
#[allow(non_snake_case)] // keep snake name otherwise it messes up vscode refactoring
pub fn ByteSerializeStack(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    // get struct name
    let struct_name = &ast.ident;
    let (generics_declaration, generics_alias) = get_generics(&ast.generics);
    let (ser_method, _) = get_struct_ser_des_tokens(&ast);
    // grap just stack presets
    let ser_vars = ser_method.iter().map(|f| &f.ser_vars).collect::<Vec<_>>();
    let ser_over = ser_method.iter().map(|f| &f.ser_over).collect::<Vec<_>>();
    let ser_uses_stack = ser_method
        .iter()
        .map(|f| &f.ser_uses_stack)
        .collect::<Vec<_>>();

    let crate_name = get_crate_name();

    // generate stack serializer
    let output = quote! {
        #[automatically_derived]
        impl #generics_declaration #crate_name::ser::ByteSerializeStack for #struct_name #generics_alias{
        // impl byteserde::ser::ByteSerializeStack for #struct_name {
            fn byte_serialize_stack<const CAP: usize>(&self, ser: &mut #crate_name::ser::ByteSerializerStack<CAP>) -> #crate_name::error::Result<()>{
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
#[allow(non_snake_case)] // keep snake name otherwise it messes up vscode refactoring
pub fn ByteSerializeHeap(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    // get struct name
    let struct_name = &ast.ident;
    let (generics_declaration, generics_alias) = get_generics(&ast.generics);
    // get ser & des quote presets
    let (ser_method, _) = get_struct_ser_des_tokens(&ast);
    // grap just heap presets
    let ser_vars = ser_method.iter().map(|f| &f.ser_vars).collect::<Vec<_>>();
    let ser_over = ser_method.iter().map(|f| &f.ser_over).collect::<Vec<_>>();
    let ser_uses_heap = ser_method
        .iter()
        .map(|f| &f.ser_uses_heap)
        .collect::<Vec<_>>();

    let crate_name = get_crate_name();

    // generate heap serializer
    let output = quote! {
        #[automatically_derived]
        impl #generics_declaration #crate_name::ser::ByteSerializeHeap for #struct_name #generics_alias{
            fn byte_serialize_heap(&self, ser: &mut #crate_name::ser::ByteSerializerHeap) -> #crate_name::error::Result<()>{
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
#[allow(non_snake_case)] // keep snake name otherwise it messes up vscode refactoring
pub fn ByteDeserialize(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    // get struct name
    let struct_name = &ast.ident;
    let (generics_declaration, generics_alias) = get_generics(&ast.generics);
    // get ser & des quote presets
    let (des_method, ty) = get_struct_ser_des_tokens(&ast);
    // grap just heap presets
    let des_vars = des_method.iter().map(|f| &f.des_vars).collect::<Vec<_>>();
    let des_uses = des_method.iter().map(|f| &f.des_uses).collect::<Vec<_>>();
    let impl_body = match ty {
        common::StructType::Regular => quote!(#struct_name {#( #des_uses )*}),
        common::StructType::Tuple => quote!  (#struct_name (#( #des_uses )*)),
    };

    let crate_name = get_crate_name();

    // generate deserializer
    let output = quote! {
        #[automatically_derived]
        impl #generics_declaration #crate_name::des::ByteDeserialize<#struct_name #generics_alias> for #struct_name #generics_alias {
            fn byte_deserialize(des: &mut #crate_name::des::ByteDeserializer) -> #crate_name::error::Result<#struct_name #generics_alias>{
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
