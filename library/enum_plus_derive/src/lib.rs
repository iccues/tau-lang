extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DataEnum, DeriveInput};

#[proc_macro_derive(EnumPlus)]
pub fn enum_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);
    let id: &syn::Ident = &ast.ident;
    let Data::Enum(ref data_enum) = ast.data else {
        panic!("EnumPlus derive macro must use in enum!");
    };

    let try_from_u8 = enum_try_from_u8_derive(id, data_enum);
    let try_from_usize = enum_try_from_usize_derive(id, data_enum);
    let try_from_str = enum_try_from_str_derive(id, data_enum);
    let into_str = enum_into_str_derive(id, data_enum);
    quote! {
        #try_from_u8
        #try_from_usize
        #try_from_str
        #into_str
    }
    .into()
}

#[proc_macro_derive(EnumPlusStr)]
pub fn enum_derive_str(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);
    let id: &syn::Ident = &ast.ident;
    let Data::Enum(ref data_enum) = ast.data else {
        panic!("EnumPlus derive macro must use in enum!");
    };

    let try_from_str = enum_try_from_str_derive(id, data_enum);
    let into_str = enum_into_str_derive(id, data_enum);
    quote! {
        #try_from_str
        #into_str
    }
    .into()
}


fn enum_try_from_u8_derive(id: &syn::Ident, data_enum: &DataEnum) -> TokenStream2 {
    let mut match_ast = quote!();
    for item in data_enum.variants.iter() {
        let item_id = &item.ident;
        let Some((_, discriminant)) = &item.discriminant else {
            panic!("EnumPlus derive macro must use with discriminant");
        };
        match_ast.extend(quote! {
            #discriminant => Ok(#id::#item_id),
        });
    }

    quote! {
        impl TryFrom<u8> for #id {
            type Error = ();
            fn try_from(value: u8) -> Result<Self, Self::Error> {
                #[allow(unreachable_patterns)]
                match value {
                    #match_ast
                    _ => Err(())
                }
            }
        }
    }
}

fn enum_try_from_usize_derive(id: &syn::Ident, data_enum: &DataEnum) -> TokenStream2 {
    let mut match_ast = quote!();
    for item in data_enum.variants.iter() {
        let item_id = &item.ident;
        let Some((_, discriminant)) = &item.discriminant else {
            panic!("EnumPlus derive macro must use with discriminant");
        };
        match_ast.extend(quote! {
            #discriminant => Ok(#id::#item_id),
        });
    }

    quote! {
        impl TryFrom<usize> for #id {
            type Error = ();
            fn try_from(value: usize) -> Result<Self, Self::Error> {
                #[allow(unreachable_patterns)]
                match value {
                    #match_ast
                    _ => Err(())
                }
            }
        }
    }
}

fn enum_try_from_str_derive(id: &syn::Ident, data_enum: &DataEnum) -> TokenStream2 {
    let mut match_ast = quote!();
    for item in data_enum.variants.iter() {
        let item_id = &item.ident;
        match_ast.extend(quote! {
            stringify!(#item_id) => Ok(#id::#item_id),
        });
    }

    quote! {
        impl TryFrom<&str> for #id {
            type Error = ();
            fn try_from(value: &str) -> Result<Self, Self::Error> {
                #[allow(unreachable_patterns)]
                match value {
                    #match_ast
                    _ => Err(())
                }
            }
        }
    }
}

fn enum_into_str_derive(id: &syn::Ident, data_enum: &DataEnum) -> TokenStream2 {
    let mut match_ast = quote!();
    for item in data_enum.variants.iter() {
        let item_id = &item.ident;
        match_ast.extend(quote! {
            #id::#item_id => stringify!(#item_id),
        });
    }

    quote! {
        impl From<#id> for &str {
            fn from(value: #id) -> Self {
                match value {
                    #match_ast
                }
            }
        }
    }
}
