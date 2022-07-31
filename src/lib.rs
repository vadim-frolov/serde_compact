//! Compact structs and enums serialized with [serde](https://crates.io/crates/serde)
//! Field names and enum tags are shortened and mapped with #[serde(rename ="")] macro.
//! 
//! ```
//! use serde_compact::compact;
//! use serde::{Serialize, Deserialize};

//! #[derive(Serialize, Deserialize, PartialEq, Debug)]
//! enum CallbackQuery {
//!     ConfirmEventReservation { event_id: i32, user_id: i32, ticket_type: i32 },
//!     CancelEventReservation { event_id: i32, user_id: i32, ticket_type: i32 },
//! }

//! #[compact] // <= add before deriving Serialize
//! #[derive(Serialize, Deserialize, PartialEq, Debug)]
//! enum CompactCallbackQuery {
//!     ConfirmEventReservation { event_id: i32, user_id: i32, ticket_type: i32 },
//!     CancelEventReservation { event_id: i32, user_id: i32, ticket_type: i32 },
//! }

//! fn main() {
//!     let s = CallbackQuery::ConfirmEventReservation {event_id: 1, user_id: 1, ticket_type: 1};
//!     let ser_s = serde_json::to_string(&s).unwrap();
//!     assert_eq!(ser_s, r#"{"ConfirmEventReservation":{"event_id":1,"user_id":1,"ticket_type":1}}"#);
//!     assert_eq!(ser_s.len(), 70);

//!     let compact_s = CompactCallbackQuery::ConfirmEventReservation {event_id: 1, user_id: 1, ticket_type: 1};
//!     let ser_compact_s = serde_json::to_string(&compact_s).unwrap();
//!     assert_eq!(ser_compact_s, r#"{"b":{"c":1,"e":1,"d":1}}"#);
//!     assert_eq!(ser_compact_s.len(), 25);
//! 
//!     let de: CompactCallbackQuery = serde_json::from_str(&ser_compact_s).unwrap();
//!     assert_eq!(compact_s, de);
//! }
//! ```
use proc_macro::TokenStream;
use quote::quote;
use syn::parse::Parser;
use std::collections::{HashSet, HashMap};
use syn::{
    fold::{self, Fold},
    visit::{self, Visit},
    parse_macro_input, Attribute, Item, Variant, Field,
};

const ALPHABET: [char; 52] = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z',
                                'A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];

/// Compact structs and enums serialized with [serde](https://crates.io/crates/serde)
/// Field names and enum tags are shortened and mapped with #[serde(rename ="")] macro.
#[proc_macro_attribute]
pub fn compact(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as Item);

    // Collect field names and tags.
    let mut collector = NameCollector {names: HashSet::new()};
    collector.visit_item(&input);

    // Map.
    let mut mapper = NameMapper::new(collector.names);
    let output = mapper.fold_item(input);
    TokenStream::from(quote!(#output))
}

struct NameCollector {
    names: HashSet<String>,
}

impl<'ast> Visit<'ast> for NameCollector {
    fn visit_field(&mut self, node: &'ast Field) {
        if let Some(ident) = &node.ident {
            self.names.insert(ident.to_string());
        }
        visit::visit_field(self, node);
    }
    fn visit_variant(&mut self, node: &'ast Variant) {
        self.names.insert(node.ident.to_string());
        visit::visit_variant(self, node);
    }
}


struct NameMapper {
    map: HashMap<String, String>,
}

impl NameMapper {
    fn new(names: HashSet<String>) -> Self {
        let mut sorted_names = names.into_iter().collect::<Vec<String>>();
        sorted_names.sort();
        let mut map: HashMap<String, String> = HashMap::new();
        for (idx, name) in sorted_names.into_iter().enumerate() {
            map.insert(name, Self::get_name(idx));
        }
        Self{map}
    }

    /// Encode field names
    /// Convert name vocabulary index to the base of ALPHABET 
    fn get_name(mut value: usize) -> String {

        let base = ALPHABET.len();
        let mut name = "".to_string();

        loop {
            name.push(ALPHABET[(value % base) as usize]);
            value /= base;
            if value == 0 {
                break;
            }
        }
        name.chars().rev().collect()
    }
}

impl Fold for NameMapper {    
    fn fold_field(&mut self, node: Field) -> Field {
        let mut node = node;
        if let Some(ident) = &node.ident {
            let rename = self.map.get(&ident.to_string()).expect("Failed to find mapping");
            if let Ok(mut attrs) = Attribute::parse_outer.parse_str(&format!("#[serde(rename = \"{}\")]", rename)) {
                if let Some(attr) = attrs.pop() {
                    node.attrs.push(attr);
                }    
            }
            fold::fold_field(self, node)
        } else {
            fold::fold_field(self, node)
        }
    }

    fn fold_variant(&mut self, node: Variant) -> Variant {
        let mut node = node;
        let rename = self.map.get(&node.ident.to_string()).expect("Failed to find mapping");
        if let Ok(mut attrs) = Attribute::parse_outer.parse_str(&format!("#[serde(rename = \"{}\")]", rename)) {
            if let Some(attr) = attrs.pop() {
                node.attrs.push(attr);
            }    
        }
        fold::fold_variant(self, node)
    }
}

