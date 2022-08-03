//! # Serde Compact
//! Compact structs and enums serialized with [serde](https://crates.io/crates/serde).
//! Field names and enum tags are shortened and mapped with #[serde(rename ="")] macro trading-off serialized data external interoperability for up to 50% size reduction.
//! Use when both serialization and deserialization happens in Rust.
//!
//! ```
//! use serde_compact::compact;
//! use serde::{Serialize, Deserialize};

//! #[derive(Serialize, Deserialize, Debug)]
//! enum CallbackQuery {
//!     ReservationConfirmation { event_id: i32, user_id: i32, ticket_type: i32 },
//!     // ...
//! }

//! #[compact] // <= add before deriving Serialize
//! #[derive(Serialize, Deserialize, PartialEq, Debug)]
//! enum CompactCallbackQuery {
//!     ReservationConfirmation { event_id: i32, user_id: i32, ticket_type: i32 },
//!     // ...
//! }

//! fn main() {
//!     // Original serialization
//!     let s = CallbackQuery::ReservationConfirmation {event_id: 1, user_id: 1, ticket_type: 1};
//!     let ser_s = serde_json::to_string(&s).unwrap();
//!     assert_eq!(ser_s, r#"{"ReservationConfirmation":{"event_id":1,"user_id":1,"ticket_type":1}}"#);
//!     assert_eq!(ser_s.len(), 70);
//!
//!     // Compacted
//!     let cs = CompactCallbackQuery::ReservationConfirmation {event_id: 1, user_id: 1, ticket_type: 1};
//!     let ser_cs = serde_json::to_string(&cs).unwrap();
//!     assert_eq!(ser_cs, r#"{"a":{"b":1,"d":1,"c":1}}"#);
//!     assert_eq!(ser_cs.len(), 25);
//!
//!     let de: CompactCallbackQuery = serde_json::from_str(&ser_cs).unwrap();
//!     assert_eq!(cs, de);
//! }
//! ```
use proc_macro::TokenStream;
use quote::quote;
use std::collections::{HashMap, HashSet};
use syn::parse::Parser;
use syn::{
    fold::{self, Fold},
    parse_macro_input,
    visit::{self, Visit},
    Attribute, Field, Item, Variant,
};

const ALPHABET: [char; 52] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

/// Compact structs and enums serialized with [serde](https://crates.io/crates/serde).
/// Field names and enum tags are shortened and mapped with #[serde(rename ="")] macro.
/// Example:
/// ```
/// use serde_compact::compact;
/// use serde::{Serialize, Deserialize};
///
/// #[compact] // <= add before deriving Serialize
/// #[derive(Serialize, Deserialize)]
/// enum CompactCallbackQuery {
///     ReservationConfirmation { event_id: i32, user_id: i32, ticket_type: i32 },
///     // ...
/// }
/// // Serialized to: "{"a":{"b":1,"d":1,"c":1}}"
/// //    instead of: "{"ReservationConfirmation":{"event_id":1,"user_id":1,"ticket_type":1}}"
/// ```
#[proc_macro_attribute]
pub fn compact(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as Item);

    // Collect field names and tags.
    let mut collector = NameCollector {
        names: HashSet::new(),
    };
    collector.visit_item(&input);

    // Map.
    let mut mapper = NameMapper::new(collector.names);
    let output = mapper.fold_item(input);
    TokenStream::from(quote!(#output))
}

/// Collect all names before mapping
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

/// Sort collected names and insert map macros
struct NameMapper {
    map: HashMap<String, String>,
}

impl NameMapper {
    fn new(names: HashSet<String>) -> Self {
        let mut sorted_names: Vec<String> = names.into_iter().collect();
        sorted_names.sort();
        let mut map: HashMap<String, String> = HashMap::new();
        for (idx, name) in sorted_names.into_iter().enumerate() {
            map.insert(name, Self::get_name(idx));
        }
        Self { map }
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
            let rename = self
                .map
                .get(&ident.to_string())
                .expect("Failed to find mapping");
            if let Ok(mut attrs) =
                Attribute::parse_outer.parse_str(&format!("#[serde(rename = \"{}\")]", rename))
            {
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
        let rename = self
            .map
            .get(&node.ident.to_string())
            .expect("Failed to find mapping");
        if let Ok(mut attrs) =
            Attribute::parse_outer.parse_str(&format!("#[serde(rename = \"{}\")]", rename))
        {
            if let Some(attr) = attrs.pop() {
                node.attrs.push(attr);
            }
        }
        fold::fold_variant(self, node)
    }
}
