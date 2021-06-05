use proc_macro::TokenStream;
use proc_macro2::{
    Ident,
    Span,
};
use quote::quote;
use syn;

#[proc_macro_derive(box_shorthand)]
pub fn arbitrary_name(input: TokenStream) -> TokenStream {
    let derive_input: syn::DeriveInput = syn::parse(input).unwrap();
    
    // Create a module name to contain the shorthand functions
    let type_name = derive_input.ident;
    let mod_name = Ident::new(&format!("{}B", type_name), Span::call_site());

    // Obtain generics information
    let generics = derive_input.generics;
    let generics_where = generics.where_clause.as_ref().map(|w| quote!(#w)).unwrap_or(quote!());

    // Obtain enum information
    let data_enum = match derive_input.data {
        syn::Data::Enum(data_enum) => data_enum,
        _ => panic!("Cannot create box shorthand for non-enum."),
    };

    // Get and process variant information
    let variant_fns = data_enum.variants.into_iter().map(|variant| {
        let ident = variant.ident;
        
        // syn::Type is the type of the syntax representation of the field types
        let out = match variant.fields {
            // struct-like enum variant
            syn::Fields::Named(fields_named) => {
                let parameters = fields_named.named.iter().map(|field| {
                    let ident = field.ident.as_ref();
                    let ty = &field.ty;
                    quote!(#ident: #ty)
                });
                let arguments = fields_named.named.iter().map(|field| {
                    let ident = field.ident.as_ref();
                    quote!(#ident)
                });
                quote!(
                    pub(super) fn #ident #generics (#(#parameters),*) -> Box<#type_name #generics> #generics_where {
                        Box::new(#type_name::#ident {#(#arguments),*})
                    }
                )
            }

            // tuple-like enum variant
            syn::Fields::Unnamed(fields_unnamed) => {
                let parameters = fields_unnamed.unnamed.iter().enumerate().map(|(index, field)| {
                    let ident = Ident::new(&format!("field{}", index), Span::call_site());
                    let ty = &field.ty;
                    quote!(#ident: #ty)
                });
                let arguments = (0..fields_unnamed.unnamed.len()).into_iter().map(|index| Ident::new(
                    &format!("field{}", index),
                    Span::call_site(),
                ));
                quote!(
                    pub(super) fn #ident #generics (#(#parameters),*) -> Box<#type_name #generics> #generics_where {
                        Box::new(#type_name::#ident(#(#arguments),*))
                    }
                )
            }

            // unit enum variant, no fields
            syn::Fields::Unit => quote!(
                pub(super) fn #ident #generics() -> Box<#type_name #generics> #generics_where {
                    Box::new(#type_name::#ident)
                }
            ),
        };
        out
    });

    let out = quote!(
        #[allow(proc_macro_derive_resolution_fallback)]
        #[allow(non_snake_case)]
        mod #mod_name {
            use super::*;
            
            #(#variant_fns)*
        }
    );

    // Use this to check for syntax errors since rust just says "compile error" with no further explanation in many cases
    // dbg!(out.to_string());

    out.into()
}
