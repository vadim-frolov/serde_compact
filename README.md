[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/smartlike-org/smartlike/LICENSE)
[![License: Apache](https://img.shields.io/badge/license-APACHE-blue.svg)](https://github.com/smartlike-org/smartlike/LICENSE)

# Macros to compact structs and enums serialized with [serde](https://crates.io/crates/serde).

Field names and enum tags are shortened and mapped with #[serde(rename ="")] macro.

```rust
use serde_compact::compact;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
enum CallbackQuery {
    ConfirmEventReservation { event_id: i32, user_id: i32, ticket_type: i32 },
    CancelEventReservation { event_id: i32, user_id: i32, ticket_type: i32 },
}

#[compact] // <= add before deriving Serialize
#[derive(Serialize, Deserialize, Debug)]
enum CompactCallbackQuery {
    ConfirmEventReservation { event_id: i32, user_id: i32, ticket_type: i32 },
    CancelEventReservation { event_id: i32, user_id: i32, ticket_type: i32 },
}

fn main() {
    let s = CallbackQuery::ConfirmEventReservation {event_id: 1, user_id: 1, ticket_type: 1};
    let ser_s = serde_json::to_string(&s).unwrap();
    assert_eq!(ser_s, r#"{"ConfirmEventReservation":{"event_id":1,"user_id":1,"ticket_type":1}}"#);
    assert_eq!(ser_s.len(), 70);

    let compact_s = CompactCallbackQuery::ConfirmEventReservation {event_id: 1, user_id: 1, ticket_type: 1};
    let ser_compact_s = serde_json::to_string(&compact_s).unwrap();
    assert_eq!(ser_compact_s, r#"{"b":{"c":1,"e":1,"d":1}}"#);
    assert_eq!(ser_compact_s.len(), 25);

    let de: CompactCallbackQuery = serde_json::from_str(&ser_compact_s).unwrap();
    assert_eq!(compact_s, de);
}
```
