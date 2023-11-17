extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn maybe_derive_debug(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let expanded = quote! {
        #[cfg_attr(feature="log", derive(core::fmt::Debug))]
        #input
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn maybe_derive_format(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let expanded = quote! {
        #[cfg_attr(feature="defmt", derive(defmt::Format))]
        #input
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn derive_format_or_debug(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let expanded = quote! {
        #[cfg_attr(feature="defmt", derive(defmt::Format))]
        #[cfg_attr(feature="log", derive(core::fmt::Debug))]
        #input

        #[cfg(not(any(feature = "defmt", feature = "log")))]
        impl FormatOrDebug for #name{}
    };

    TokenStream::from(expanded)
}
