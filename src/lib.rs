//! This crate provides a macro to compact struct and enum filed names and tags for Serde.
//!
//! ```edition2021
//! # use serde::{Deserialize, Serialize};
//! # use serde_compact::compact;
//! #
//! #[compact]
//! #[derive(Serialize, Deserialize)]
//! # struct S {
//! # long_field_name: String,
//! # }
//! #
//! # fn main() {}
//! ```
//! 
use proc_macro::TokenStream;
use quote::quote;
use syn::parse::Parser;
use syn::{
    fold::{self, *},
    parse::{Parse, ParseStream},
    parse_macro_input, Attribute, Item
};

const ALPHABET: [char; 52] = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z',
                                'A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];

#[proc_macro_attribute]
pub fn compact(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut args = parse_macro_input!(attr as CompactFieldNames);
    let input = parse_macro_input!(item as Item);
    let output = args.fold_item(input);
    TokenStream::from(quote!(#output))
}

struct CompactFieldNames {
    field_counter: usize,
}

impl CompactFieldNames {
    // Encode field names
    fn get_next_name(&mut self) -> String {

        let base = ALPHABET.len();
        let mut value = self.field_counter;
        let mut name = "".to_string();

        loop {
            name.push(ALPHABET[(value % base) as usize]);
            value /= base;
            if value == 0 {
                break;
            }
        }
        self.field_counter += 1;
        name.chars().rev().collect()
    }
}

impl Parse for CompactFieldNames {
    fn parse(_input: ParseStream) -> syn::parse::Result<Self> {
        Ok(CompactFieldNames {
            field_counter: 0,
        })
    }
}

impl Fold for CompactFieldNames {
    fn fold_field(&mut self, node: syn::Field) -> syn::Field {
        if node.ident.is_some() {
            let mut node = node;
            if let Ok(mut attrs) = Attribute::parse_outer.parse_str(&format!("#[serde(rename = \"{}\")]", self.get_next_name())) {
                if let Some(attr) = attrs.pop() {
                    node.attrs.push(attr);
                }    
            }
            fold::fold_field(self, node)
        } else {
            fold::fold_field(self, node)
        }
    }

    fn fold_variant(&mut self, node: syn::Variant) -> syn::Variant {
        let mut node = node;
        if let Ok(mut attrs) = Attribute::parse_outer.parse_str(&format!("#[serde(rename = \"{}\")]", self.get_next_name())) {
            if let Some(attr) = attrs.pop() {
                node.attrs.push(attr);
            }    
        }
        fold::fold_variant(self, node)
    }
}
