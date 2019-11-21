extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(GcLayout)]
pub fn gclayout_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let expanded = quote! {
        impl #impl_generics ::abgc::GcLayout for #ident #ty_generics #where_clause {
            fn layout(&self) -> ::std::alloc::Layout {
                ::std::alloc::Layout::new::<#ident #ty_generics>()
            }
        }
    };

    TokenStream::from(expanded)
}
