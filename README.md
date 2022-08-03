[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/smartlike-org/smartlike/LICENSE)
[![License: Apache](https://img.shields.io/badge/license-APACHE-blue.svg)](https://github.com/smartlike-org/smartlike/LICENSE)

# Serde Compact

**Macros to compact structs and enums serialized with [serde](https://crates.io/crates/serde).**

Field names and enum tags are shortened and mapped with #[serde(rename ="")] macro trading-off serialized data external interoperability for up to 50% size reduction. Use when both serialization and deserialization happens in Rust.

```rust
use serde_compact::compact;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
enum CallbackQuery {
    ReservationConfirmation { event_id: i32, user_id: i32, ticket_type: i32 },
    // ...
}

#[compact] // <= add before deriving Serialize
#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum CompactCallbackQuery {
    ReservationConfirmation { event_id: i32, user_id: i32, ticket_type: i32 },
    // ...
}

fn main() {
    // Original serialization
    let s = CallbackQuery::ReservationConfirmation {event_id: 1, user_id: 1, ticket_type: 1};
    let ser_s = serde_json::to_string(&s).unwrap();
    assert_eq!(ser_s, r#"{"ReservationConfirmation":{"event_id":1,"user_id":1,"ticket_type":1}}"#);
    assert_eq!(ser_s.len(), 70);

    // Compacted
    let cs = CompactCallbackQuery::ReservationConfirmation {event_id: 1, user_id: 1, ticket_type: 1};
    let ser_cs = serde_json::to_string(&cs).unwrap();
    assert_eq!(ser_cs, r#"{"a":{"b":1,"d":1,"c":1}}"#);
    assert_eq!(ser_cs.len(), 25);

    let de: CompactCallbackQuery = serde_json::from_str(&ser_cs).unwrap();
    assert_eq!(cs, de);
}
```
