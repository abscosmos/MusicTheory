use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, LitInt};

#[proc_macro_attribute]
pub fn numeral(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;

    let count = if attr.is_empty() {
        panic!("Must specify number of variants: #[numeral(N)]");
    } else {
        let lit_int = parse_macro_input!(attr as LitInt);
        lit_int.base10_parse::<u8>()
            .expect("given numeral value must be a positive integer")
    };

    let roman_numerals = [
        "I", "II", "III", "IV", "V",
        "VI", "VII", "VIII", "IX", "X",
        "XI", "XII", "XIII", "XIV", "XV",
        "XVI", "XVII", "XVIII", "XIX", "XX",
    ];
    
    assert!((count as usize) < roman_numerals.len(), "Only supports up to {} variants", roman_numerals.len());

    let variants = (0..count).map(|i| {
        let ident = syn::Ident::new(roman_numerals[i as usize], proc_macro2::Span::call_site());
        if i == 0 {
            quote! { #ident = 1 }
        } else {
            quote! { #ident }
        }
    });

    let count_usize = count as usize;

    let expanded = quote! {
        #[repr(u8)]
        #[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, strum_macros::FromRepr)]
        pub enum #name {
            #(#variants,)*
        }

        impl Numeral<#count_usize> for #name {
            fn as_num(self) -> u8 {
                self as _
            }

            fn from_num(num: u8) -> Option<Self> where Self: Sized {
                Self::from_repr(num)
            }
        }
    };

    TokenStream::from(expanded)
}