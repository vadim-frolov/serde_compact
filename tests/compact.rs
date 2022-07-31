#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use serde_compact::compact;

    macro_rules! test_serde {
        ($t:ty, $data:expr) => {
            let ser = serde_json::to_string(&$data).unwrap();
            let de: $t = serde_json::from_str(&ser).unwrap();
            assert_eq!($data, de);
        };
    }

    #[test]
    fn test() {
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        enum Message {
            Request {
                id: String,
                method: String,
                parameter: i32,
            },
            Response {
                id: String,
                result: i32,
            },
        }

        test_serde!(
            Message,
            Message::Request {
                id: "".to_string(),
                method: "".to_string(),
                parameter: 0,
            }
        );

        #[compact]
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        struct BigStruct {
            field1: i32,
            field2: i32,
            field3: i32,
            field4: i32,
            field5: i32,
            field6: i32,
            field7: i32,
            field8: i32,
            field9: i32,
            field10: i32,
            field11: i32,
            field12: i32,
            field13: i32,
            field14: i32,
            field15: i32,
            field16: i32,
            field17: i32,
            field18: i32,
            field19: i32,
            field20: i32,
            field21: i32,
            field22: i32,
            field23: i32,
            field24: i32,
            field25: i32,
            field26: i32,
            field27: i32,
            field28: i32,
            field29: i32,
            field30: i32,
        }

        test_serde!(
            BigStruct,
            BigStruct {
                field1: 1,
                field2: 2,
                field3: 3,
                field4: 4,
                field5: 5,
                field6: 6,
                field7: 7,
                field8: 8,
                field9: 9,
                field10: 10,
                field11: 11,
                field12: 12,
                field13: 13,
                field14: 14,
                field15: 15,
                field16: 16,
                field17: 17,
                field18: 18,
                field19: 19,
                field20: 20,
                field21: 21,
                field22: 22,
                field23: 23,
                field24: 24,
                field25: 25,
                field26: 26,
                field27: 27,
                field28: 28,
                field29: 29,
                field30: 30,
            }
        );

        #[compact]
        #[derive(Serialize, Deserialize, Debug)]
        enum CompactCallbackQuery {
            ConfirmEventReservation {
                event_id: i32,
                user_id: i32,
                ticket_type: i32,
            },
            CancelEventReservation {
                event_id: i32,
                user_id: i32,
                ticket_type: i32,
            },
        }

        let s = CompactCallbackQuery::ConfirmEventReservation {
            event_id: 1,
            user_id: 1,
            ticket_type: 0,
        };
        let ser = serde_json::to_string(&s).unwrap();
        assert_eq!(ser, r#"{"b":{"c":1,"e":1,"d":0}}"#);
    }
}
