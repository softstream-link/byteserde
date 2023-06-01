use core::panic;

use quote::{
    __private::{TokenStream},
    quote,
};
use syn::{
    Data, DeriveInput, Fields,
};

use crate::attr_enum::{enum_bind_attr, enum_from_attr,enum_replace_attr, Bind, Replace,From,};

pub fn get_enum_from_tokens(
    ast: &DeriveInput,
) -> Vec<TokenStream>
{

    let enum_type = &ast.ident;
    let enum_type_str = format!("{}", quote!(#enum_type));
    let enum_ref_type_str = format!("{}", quote!(&#enum_type));

    let bind = enum_bind_attr(&ast.attrs);
    let bind_type = match bind {
        Bind::Set(value) => quote!(#value),
        _ => panic!("Enum {enum_type} needs to be bound to a struct type using bind attribute. Example: `#[byteserde(bind( MyStructName ))]`"),
    };    
    let bind_type_str = format!("{}", quote!(#bind_type));
    let bind_ref_type_str = format!("{}", quote!(&#bind_type));

    let from_types: Vec<From> = enum_from_attr(&ast.attrs);
    if from_types.len() == 0 {
        panic!(
            "Enum {enum_type} is missing at least one from attribute,
                Example: 
                    `#[byteserde(from(&{enum_type_str}))] - used by serializers`
                    `#[byteserde(from({enum_type_str}))] - convenient for testings`
                    "
        );
    }
    let mut tokens = Vec::<TokenStream>::new();
    for from_type in from_types{
        let from_type_str = format!("{}", quote!(#from_type));
        if ![&bind_type_str, &bind_ref_type_str, &enum_type_str, &enum_ref_type_str].contains(&&from_type_str){
             panic!("`#[byteserde(from({from_type_str}))]` needs to match one of 
                 `{bind_type_str}`
                 `{bind_ref_type_str}`
                 `{enum_type_str}`
                 `{enum_ref_type_str}`
                 ")
         }

        
        let from_impl = match &ast.data {
            Data::Enum(data) => {
                let match_arms = data.variants
                    .iter()
                    .map(|var| {
                        let replace = match enum_replace_attr(&var.attrs){
                            Replace::Set(value) => {
                                quote!(#value)
                            }
                            _ => panic!("`{enum_type}::{}` variant with missing replace, consider adding `#[byteserde(replace( <instance of type {bind_type_str}> ))]`",
                                &var.ident
                            ),
                        
                        };
                        let enum_variant = &var.ident;
                        match var.fields{
                            Fields::Unit =>{
                                match from_type_str.contains(&enum_type_str){
                                   true => quote!(#enum_type::#enum_variant => #replace,),
                                   false => quote!(#replace => #enum_type::#enum_variant,),
                                }
                            },
                            _ => panic!("Only unit fields for enum types supported"),
                        }
                    }
                    )
                    .collect::<Vec<_>>();
                // eprintln!("match_arms: {}", quote!(#(#match_arms)*));
                let impl_from = match from_type_str.contains(&enum_type_str){
                    true => {
                        quote! {
                            #[automatically_derived]
                            impl From<#from_type> for #bind_type{
                                fn from(v: #from_type) -> #bind_type{
                                    match v{
                                         #(#match_arms)*
                                    }
                                }
                            }
                        }
                    },
                    false => {
                        quote! {
                            #[automatically_derived]
                            impl From<#from_type> for #enum_type{
                                fn from(v: #from_type) -> #enum_type{
                                    match v{
                                         #(#match_arms)*
                                         _ => panic!("{:?} is not mapped to enum variant", v),
                                    }
                                }
                            }
                        }
                    }
                };
                // eprintln!("impl_from: {}", quote!(#impl_from));
                impl_from
                
            }
            _ => panic!("This feature is only supported for enum types")
            };
        tokens.push(from_impl);
    }
    tokens
}