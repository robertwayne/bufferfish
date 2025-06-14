#[cfg(feature = "derive")]
pub mod compiler;

pub use bufferfish_core::{decodable::Decodable, encodable::Encodable, *};
#[cfg(feature = "derive")]
pub use bufferfish_derive::{Decode, Encode};
#[cfg(feature = "derive")]
pub use compiler::generate;

#[cfg(feature = "derive")]
#[cfg(test)]
mod tests {
    use bufferfish_core::Bufferfish;
    use bufferfish_derive::{Decode, Encode};

    #[test]
    fn test_peek_one() {
        let mut bf = Bufferfish::new();
        bf.write_u8(0).unwrap();

        assert_eq!(bf.peek().unwrap(), 0);
        assert_eq!(bf.peek().unwrap(), 0);
    }

    #[test]
    fn peek_n() {
        let mut bf = Bufferfish::new();
        bf.write_u8(0).unwrap();
        bf.write_u8(1).unwrap();
        bf.write_u8(2).unwrap();

        assert_eq!(bf.peek_n(2).unwrap(), &[0, 1]);
        assert_eq!(bf.peek_n(2).unwrap(), &[0, 1]);
    }

    #[test]
    fn peek_one_past_capacity() {
        let mut bf = Bufferfish::new();

        let result = bf.peek();

        assert!(result.is_err());
    }

    #[test]
    fn peek_n_past_capacity() {
        let mut bf = Bufferfish::new();
        bf.write_u8(0).unwrap();

        let result = bf.peek_n(2);

        assert!(result.is_err());
    }

    #[test]
    fn test_extends_bufferfish() {
        let mut bf = Bufferfish::new();
        bf.write_u8(0).unwrap();

        let mut buf2 = Bufferfish::new();
        buf2.write_u8(1).unwrap();

        bf.extend(buf2);

        assert_eq!(bf.as_ref(), &[0, 1]);
    }

    #[test]
    fn test_extends_impls() {
        let mut bf = Bufferfish::new();
        bf.write_u8(0).unwrap();

        let slice: &[u8] = &[1];
        let vec = Vec::from([2]);

        bf.extend(slice);
        bf.extend(vec);

        assert_eq!(bf.as_ref(), &[0, 1, 2]);
    }

    #[test]
    fn test_write_u8() {
        let mut bf = Bufferfish::new();
        bf.write_u8(0).unwrap();
        bf.write_u8(255).unwrap();

        assert_eq!(bf.as_ref(), &[0, 255]);
    }

    #[test]
    fn test_write_u16() {
        let mut bf = Bufferfish::new();
        bf.write_u16(0).unwrap();
        bf.write_u16(12345).unwrap();
        bf.write_u16(65535).unwrap();

        assert_eq!(bf.as_ref(), &[0, 0, 48, 57, 255, 255]);
    }

    #[test]
    fn test_write_u32() {
        let mut bf = Bufferfish::new();
        bf.write_u32(0).unwrap();
        bf.write_u32(1234567890).unwrap();
        bf.write_u32(u32::MAX).unwrap();

        assert_eq!(
            bf.as_ref(),
            &[0, 0, 0, 0, 73, 150, 2, 210, 255, 255, 255, 255]
        );
    }

    #[test]
    fn test_write_u64() {
        let mut bf = Bufferfish::new();
        bf.write_u64(0).unwrap();
        bf.write_u64(4611686018427387904).unwrap();
        bf.write_u64(u64::MAX).unwrap();

        assert_eq!(
            bf.as_ref(),
            &[
                0, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255,
                255
            ]
        );
    }

    #[test]
    fn test_write_u128() {
        let mut bf = Bufferfish::new();
        bf.write_u128(0).unwrap();
        bf.write_u128(123456789012345678901234567890).unwrap();
        bf.write_u128(u128::MAX).unwrap();

        assert_eq!(
            bf.as_ref(),
            &[
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 142, 233, 15, 246, 195,
                115, 224, 238, 78, 63, 10, 210, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                255, 255, 255, 255, 255, 255
            ]
        );
    }

    #[test]
    fn test_read_u8() {
        let mut bf = Bufferfish::new();
        bf.write_u8(0).unwrap();
        bf.write_u8(255).unwrap();

        assert_eq!(bf.read_u8().unwrap(), 0);
        assert_eq!(bf.read_u8().unwrap(), 255);
    }

    #[test]
    fn test_read_u16() {
        let mut bf = Bufferfish::new();
        bf.write_u16(0).unwrap();
        bf.write_u16(12345).unwrap();
        bf.write_u16(65535).unwrap();

        assert_eq!(bf.read_u16().unwrap(), 0);
        assert_eq!(bf.read_u16().unwrap(), 12345);
        assert_eq!(bf.read_u16().unwrap(), 65535);
    }

    #[test]
    fn test_read_u32() {
        let mut bf = Bufferfish::new();
        bf.write_u32(0).unwrap();
        bf.write_u32(1234567890).unwrap();
        bf.write_u32(u32::MAX).unwrap();

        assert_eq!(bf.read_u32().unwrap(), 0);
        assert_eq!(bf.read_u32().unwrap(), 1234567890);
        assert_eq!(bf.read_u32().unwrap(), u32::MAX);
    }

    #[test]
    fn test_read_u64() {
        let mut bf = Bufferfish::new();
        bf.write_u64(0).unwrap();
        bf.write_u64(4611686018427387904).unwrap();
        bf.write_u64(u64::MAX).unwrap();

        assert_eq!(bf.read_u64().unwrap(), 0);
        assert_eq!(bf.read_u64().unwrap(), 4611686018427387904);
        assert_eq!(bf.read_u64().unwrap(), u64::MAX);
    }

    #[test]
    fn test_read_u128() {
        let mut bf = Bufferfish::new();
        bf.write_u128(0).unwrap();
        bf.write_u128(123456789012345678901234567890).unwrap();
        bf.write_u128(u128::MAX).unwrap();

        assert_eq!(bf.read_u128().unwrap(), 0);
        assert_eq!(bf.read_u128().unwrap(), 123456789012345678901234567890);
        assert_eq!(bf.read_u128().unwrap(), u128::MAX);
    }

    #[test]
    fn test_write_i8() {
        let mut bf = Bufferfish::new();
        bf.write_i8(0).unwrap();
        bf.write_i8(127).unwrap();
        bf.write_i8(-128).unwrap();

        assert_eq!(bf.as_ref(), &[0, 127, 128]);
    }

    #[test]
    fn test_write_i16() {
        let mut bf = Bufferfish::new();
        bf.write_i16(0).unwrap();
        bf.write_i16(12345).unwrap();
        bf.write_i16(32767).unwrap();
        bf.write_i16(-32768).unwrap();

        assert_eq!(bf.as_ref(), &[0, 0, 48, 57, 127, 255, 128, 0]);
    }

    #[test]
    fn test_write_i32() {
        let mut bf = Bufferfish::new();
        bf.write_i32(0).unwrap();
        bf.write_i32(1234567890).unwrap();
        bf.write_i32(2147483647).unwrap();
        bf.write_i32(-2147483648).unwrap();

        assert_eq!(
            bf.as_ref(),
            &[
                0, 0, 0, 0, 73, 150, 2, 210, 127, 255, 255, 255, 128, 0, 0, 0
            ]
        );
    }

    #[test]
    fn test_write_i64() {
        let mut bf = Bufferfish::new();
        bf.write_i64(0).unwrap();
        bf.write_i64(4611686018427387904).unwrap();
        bf.write_i64(i64::MAX).unwrap();
        bf.write_i64(i64::MIN).unwrap();

        assert_eq!(
            bf.as_ref(),
            &[
                0, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 127, 255, 255, 255, 255, 255, 255,
                255, 128, 0, 0, 0, 0, 0, 0, 0
            ]
        );
    }

    #[test]
    fn test_write_i128() {
        let mut bf = Bufferfish::new();
        bf.write_i128(0).unwrap();
        bf.write_i128(123456789012345678901234567890).unwrap();
        bf.write_i128(i128::MAX).unwrap();
        bf.write_i128(i128::MIN).unwrap();

        assert_eq!(
            bf.as_ref(),
            &[
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 142, 233, 15, 246, 195,
                115, 224, 238, 78, 63, 10, 210, 127, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                255, 255, 255, 255, 255, 255, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ]
        );
    }

    #[test]
    fn test_read_i8() {
        let mut bf = Bufferfish::new();
        bf.write_i8(0).unwrap();
        bf.write_i8(127).unwrap();
        bf.write_i8(-128).unwrap();

        assert_eq!(bf.read_i8().unwrap(), 0);
        assert_eq!(bf.read_i8().unwrap(), 127);
        assert_eq!(bf.read_i8().unwrap(), -128);
    }

    #[test]
    fn test_read_i16() {
        let mut bf = Bufferfish::new();
        bf.write_i16(0).unwrap();
        bf.write_i16(12345).unwrap();
        bf.write_i16(32767).unwrap();
        bf.write_i16(-32768).unwrap();

        assert_eq!(bf.read_i16().unwrap(), 0);
        assert_eq!(bf.read_i16().unwrap(), 12345);
        assert_eq!(bf.read_i16().unwrap(), 32767);
        assert_eq!(bf.read_i16().unwrap(), -32768);
    }

    #[test]
    fn test_read_i32() {
        let mut bf = Bufferfish::new();
        bf.write_i32(0).unwrap();
        bf.write_i32(1234567890).unwrap();
        bf.write_i32(2147483647).unwrap();
        bf.write_i32(-2147483648).unwrap();

        assert_eq!(bf.read_i32().unwrap(), 0);
        assert_eq!(bf.read_i32().unwrap(), 1234567890);
        assert_eq!(bf.read_i32().unwrap(), 2147483647);
        assert_eq!(bf.read_i32().unwrap(), -2147483648);
    }

    #[test]
    fn test_read_i64() {
        let mut bf = Bufferfish::new();
        bf.write_i64(0).unwrap();
        bf.write_i64(4611686018427387904).unwrap();
        bf.write_i64(9223372036854775807).unwrap();
        bf.write_i64(-9223372036854775808).unwrap();

        assert_eq!(bf.read_i64().unwrap(), 0);
        assert_eq!(bf.read_i64().unwrap(), 4611686018427387904);
        assert_eq!(bf.read_i64().unwrap(), 9223372036854775807);
        assert_eq!(bf.read_i64().unwrap(), -9223372036854775808);
    }

    #[test]
    fn test_read_i128() {
        let mut bf = Bufferfish::new();
        bf.write_i128(0).unwrap();
        bf.write_i128(123456789012345678901234567890).unwrap();
        bf.write_i128(i128::MAX).unwrap();
        bf.write_i128(i128::MIN).unwrap();

        assert_eq!(bf.read_i128().unwrap(), 0);
        assert_eq!(bf.read_i128().unwrap(), 123456789012345678901234567890);
        assert_eq!(bf.read_i128().unwrap(), i128::MAX);
        assert_eq!(bf.read_i128().unwrap(), i128::MIN);
    }

    #[test]
    fn test_read_reset() {
        let mut bf = Bufferfish::new();
        bf.write_u8(0).unwrap();
        bf.read_u8().unwrap();
        bf.write_u8(255).unwrap();

        assert_eq!(bf.read_u8().unwrap(), 0);
    }

    #[test]
    fn test_bufferfish_overflow() {
        let mut bf = Bufferfish::new();
        bf.write_raw_bytes(&[0u8; 1023]).unwrap();

        let result = bf.write_u8(0);
        assert!(result.is_ok());

        let result = bf.write_u8(0);
        assert!(result.is_err());
    }

    #[test]
    fn test_oversized_write_buffer() {
        let mut bf = Bufferfish::new();

        assert!(bf.write_raw_bytes(&[0u8; 1025]).is_err());
    }

    #[test]
    fn test_unbounded_capacity() {
        let mut bf = Bufferfish::with_capacity(0);

        assert!(bf.write_raw_bytes(&[0u8; 2000]).is_ok());
    }

    #[test]
    fn test_write_string() {
        let mut bf = Bufferfish::new();
        bf.write_string("Bufferfish").unwrap();

        assert_eq!(
            bf.as_ref(),
            &[0, 10, 66, 117, 102, 102, 101, 114, 102, 105, 115, 104]
        );
    }

    #[test]
    fn test_write_string_big_chars() {
        let mut bf = Bufferfish::new();
        bf.write_string("안녕하세요").unwrap();

        assert_eq!(
            bf.as_ref(),
            &[
                0, 15, 236, 149, 136, 235, 133, 149, 237, 149, 152, 236, 132, 184, 236, 154, 148
            ]
        )
    }

    #[test]
    fn test_write_multiple_strings() {
        let mut bf = Bufferfish::new();
        bf.write_string("Bufferfish").unwrap();
        bf.write_string("안녕하세요").unwrap();

        assert_eq!(
            bf.as_ref(),
            &[
                0, 10, 66, 117, 102, 102, 101, 114, 102, 105, 115, 104, 0, 15, 236, 149, 136, 235,
                133, 149, 237, 149, 152, 236, 132, 184, 236, 154, 148
            ]
        );
    }

    #[test]
    fn test_read_string() {
        let mut bf = Bufferfish::new();
        bf.write_string("Bufferfish").unwrap();

        assert_eq!(bf.read_string().unwrap(), "Bufferfish");
    }

    #[test]
    fn test_write_bool() {
        let mut bf = Bufferfish::new();
        bf.write_bool(true).unwrap();
        bf.write_bool(false).unwrap();

        assert_eq!(bf.as_ref(), &[1, 0]);
    }

    #[test]
    fn test_read_bool() {
        let mut bf = Bufferfish::new();
        bf.write_bool(true).unwrap();
        bf.write_bool(false).unwrap();

        assert!(bf.read_bool().unwrap());
        assert!(!bf.read_bool().unwrap());
    }

    #[test]
    // This is just a visual test for ensuring pretty-formatting on output.
    // Must be run with `cargo test -- --show-output` to see the string.
    fn test_display_trait() {
        let mut bf = Bufferfish::new();
        bf.write_u16(4).unwrap();
        bf.write_string("Bufferfish").unwrap();

        // Should look like this:
        //  Byte:  0  4  0  10  66  117  102  102  101  114  102  105  115  104
        // Index:  0  1  2   3   4    5    6    7    8    9   10   11   12   13
        println!("{bf}");
    }

    #[test]
    fn test_write_raw_bytes() {
        let mut bf = Bufferfish::new();
        bf.write_string("Bufferfish").unwrap();

        let mut buf2 = Bufferfish::new();
        buf2.write_string("안녕하세요").unwrap();

        bf.write_raw_bytes(buf2.as_ref()).unwrap();

        assert!(bf.read_string().unwrap() == "Bufferfish");
        assert!(bf.read_string().unwrap() == "안녕하세요");
    }

    #[test]
    fn test_write_packed_bools() {
        let mut bf = Bufferfish::new();
        bf.write_packed_bools(&[true, false, true, false, true, false, true, false])
            .unwrap();

        assert_eq!(bf.as_ref(), &[0b10101010]);
    }

    #[test]
    fn test_read_packed_bools() {
        let mut bf = Bufferfish::new();
        bf.write_u8(0b10101010).unwrap();

        let bools = bf.read_packed_bools(8).unwrap();

        assert_eq!(
            bools,
            vec![true, false, true, false, true, false, true, false]
        );
    }

    #[test]
    fn test_write_simple_array() {
        let mut bf = Bufferfish::new();
        bf.write_array(&[0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();

        assert_eq!(bf.as_ref(), &[0, 10, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_write_items_to_array() {
        use bufferfish_core as bufferfish;

        #[derive(Encode)]
        #[bufferfish(0_u16)]
        struct Object {
            a: u8,
        }

        let arr = vec![Object { a: 0 }, Object { a: 1 }];

        let mut bf = Bufferfish::new();
        bf.write_array(&arr).unwrap();

        let expected_bytes: Vec<u8> = vec![0, 2, 0, 1];

        assert_eq!(bf.into_vec(), expected_bytes);
    }

    #[test]
    fn test_read_simple_array() {
        let mut bf = Bufferfish::new();
        bf.write_array(&[0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();

        let arr = bf.read_array::<u8>().unwrap();

        assert_eq!(arr, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_read_items_from_array() {
        use bufferfish_core as bufferfish;

        #[derive(Debug, Encode, Decode, PartialEq)]
        struct Object {
            a: u8,
        }

        let arr = vec![Object { a: 0 }, Object { a: 1 }];

        let mut bf = Bufferfish::new();
        bf.write_array(&arr).unwrap();

        let result = bf.read_array::<Object>().unwrap();

        assert_eq!(result, vec![Object { a: 0 }, Object { a: 1 }]);
    }

    #[test]
    fn test_to_bufferfish_struct() {
        use bufferfish_core as bufferfish;
        use bufferfish_core::Encodable;

        #[derive(Encode)]
        #[bufferfish(0_u16)]
        struct JoinMessage {
            user: User,
        }

        #[derive(Encode)]
        struct User {
            id: u32,
            name: String,
        }

        let bf = JoinMessage {
            user: User {
                id: 0,
                name: "Bufferfish".to_string(),
            },
        }
        .to_bufferfish()
        .unwrap();

        assert_eq!(
            bf.as_ref(),
            &[
                0, 0, 0, 0, 0, 0, 0, 10, 66, 117, 102, 102, 101, 114, 102, 105, 115, 104
            ]
        );
    }

    #[test]
    fn test_to_bufferfish_array() {
        use bufferfish_core as bufferfish;
        use bufferfish_core::Encodable;

        #[derive(Encode)]
        struct User {
            id: u32,
            name: String,
        }

        let users = vec![
            User {
                id: 0,
                name: "Bufferfish".to_string(),
            },
            User {
                id: 1,
                name: "Bufferfish2".to_string(),
            },
        ];

        let bf = users.to_bufferfish().unwrap();

        assert_eq!(
            bf.as_ref(),
            &[
                0, 2, 0, 0, 0, 0, 0, 10, 66, 117, 102, 102, 101, 114, 102, 105, 115, 104, 0, 0, 0,
                1, 0, 11, 66, 117, 102, 102, 101, 114, 102, 105, 115, 104, 50,
            ]
        );
    }

    #[test]
    fn test_to_bufferfish_enums() {
        use bufferfish_core as bufferfish;
        use bufferfish_core::Encodable;

        #[derive(Encode, Clone, Copy)]
        enum MessageId {
            Join,
        }

        impl From<MessageId> for u16 {
            fn from(value: MessageId) -> Self {
                match value {
                    MessageId::Join => 0,
                }
            }
        }

        #[derive(Encode)]
        #[bufferfish(MessageId::Join)]
        struct JoinMessage {
            class: Class,
        }

        #[derive(Encode, Clone, Copy)]
        #[repr(u8)]
        enum Class {
            Warrior,
        }

        impl From<Class> for u8 {
            fn from(value: Class) -> Self {
                match value {
                    Class::Warrior => 0,
                }
            }
        }

        let bf = JoinMessage {
            class: Class::Warrior,
        }
        .to_bufferfish()
        .unwrap();

        assert_eq!(bf.as_ref(), &[0, 0, 0]);
    }

    #[test]
    fn test_encode_into_struct() {
        use bufferfish_core as bufferfish;
        use bufferfish_core::Encodable;

        #[derive(Encode)]
        #[bufferfish(0_u16)]
        struct JoinMessage {
            user: User,
        }

        #[derive(Encode)]
        struct User {
            id: u32,
            name: String,
        }

        let mut bf = Bufferfish::new();
        let user = User {
            id: 0,
            name: "Bufferfish".to_string(),
        };
        let message = JoinMessage { user };

        message.encode(&mut bf).unwrap();

        assert_eq!(
            bf.as_ref(),
            &[
                0, 0, 0, 0, 0, 0, 0, 10, 66, 117, 102, 102, 101, 114, 102, 105, 115, 104
            ]
        );
    }

    #[test]
    fn test_encode_into_array() {
        use bufferfish_core as bufferfish;
        use bufferfish_core::Encodable;

        #[derive(Encode)]
        struct User {
            id: u32,
            name: String,
        }

        let users = vec![
            User {
                id: 0,
                name: "Bufferfish".to_string(),
            },
            User {
                id: 1,
                name: "Bufferfish2".to_string(),
            },
        ];

        let mut bf = Bufferfish::new();
        users.encode(&mut bf).unwrap();

        assert_eq!(
            bf.as_ref(),
            &[
                0, 2, 0, 0, 0, 0, 0, 10, 66, 117, 102, 102, 101, 114, 102, 105, 115, 104, 0, 0, 0,
                1, 0, 11, 66, 117, 102, 102, 101, 114, 102, 105, 115, 104, 50,
            ]
        );
    }

    #[test]
    fn test_encode_into_enums() {
        use bufferfish_core as bufferfish;
        use bufferfish_core::Encodable;

        #[derive(Encode, Clone, Copy)]
        enum MessageId {
            Join,
        }

        impl From<MessageId> for u16 {
            fn from(value: MessageId) -> Self {
                match value {
                    MessageId::Join => 0,
                }
            }
        }

        #[derive(Encode)]
        #[bufferfish(MessageId::Join)]
        struct JoinMessage {
            class: Class,
        }

        #[derive(Encode, Clone, Copy)]
        #[repr(u8)]
        enum Class {
            Warrior,
        }

        impl From<Class> for u8 {
            fn from(value: Class) -> Self {
                match value {
                    Class::Warrior => 0,
                }
            }
        }

        let mut bf = Bufferfish::new();
        let message = JoinMessage {
            class: Class::Warrior,
        };

        message.encode(&mut bf).unwrap();

        assert_eq!(bf.as_ref(), &[0, 0, 0]);
    }

    #[test]
    fn test_decode_struct() {
        use bufferfish_core as bufferfish;
        use bufferfish_core::Decodable;

        #[derive(Decode)]
        struct User {
            id: u32,
            name: String,
        }

        let mut bf = Bufferfish::new();
        bf.write_u32(0).unwrap();
        bf.write_string("Bufferfish").unwrap();

        let user = User::decode(&mut bf).unwrap();

        assert_eq!(user.id, 0);
        assert_eq!(user.name, "Bufferfish");
    }

    #[test]
    fn test_decode_array() {
        use bufferfish_core as bufferfish;
        use bufferfish_core::Decodable;

        #[derive(Decode, Encode, PartialEq, Debug)]
        struct User {
            id: u32,
            name: String,
        }

        let users = vec![
            User {
                id: 0,
                name: "Bufferfish".to_string(),
            },
            User {
                id: 1,
                name: "Bufferfish2".to_string(),
            },
        ];

        let mut bf = Bufferfish::new();
        bf.write_array(&users).unwrap();

        let result = Vec::<User>::decode(&mut bf).unwrap();

        assert_eq!(
            result,
            vec![
                User {
                    id: 0,
                    name: "Bufferfish".to_string()
                },
                User {
                    id: 1,
                    name: "Bufferfish2".to_string()
                }
            ]
        );
    }

    #[test]
    fn test_decode_with_message_id() {
        use bufferfish_core as bufferfish;
        use bufferfish_core::{Decodable, Encodable};

        #[derive(Decode, Encode)]
        #[bufferfish(0_u16)]
        struct JoinMessage {
            user: String,
        }

        let mut bf = JoinMessage {
            user: "Bufferfish".to_string(),
        }
        .to_bufferfish()
        .unwrap();

        let result = JoinMessage::decode(&mut bf).unwrap();

        assert_eq!(result.user, "Bufferfish");
    }

    #[test]
    fn test_decode_nested_data() {
        use bufferfish_core as bufferfish;
        use bufferfish_core::{Decodable, Encodable};

        #[derive(Debug, Encode, Decode, PartialEq)]
        struct User {
            id: u32,
            name: String,
        }

        #[derive(Debug, Encode, Decode, Clone, Copy, PartialEq)]
        enum Role {
            User,
        }

        #[derive(Encode, Decode)]
        pub struct JoinMessage {
            user: User,
            role: Role,
        }

        let mut bf = JoinMessage {
            user: User {
                id: 0,
                name: "Bufferfish".to_string(),
            },
            role: Role::User,
        }
        .to_bufferfish()
        .unwrap();

        let result = JoinMessage::decode(&mut bf).unwrap();

        assert_eq!(
            result.user,
            User {
                id: 0,
                name: "Bufferfish".to_string()
            }
        );
        assert_eq!(result.role, Role::User);
    }

    #[test]
    fn test_decode_undersized_bufferfish() {
        use bufferfish_core as bufferfish;
        use bufferfish_core::Decodable;

        #[derive(Decode, Encode, Debug)]
        struct User {
            id: u32,
            name: String,
        }

        let mut bf = Bufferfish::new();
        bf.write_u32(0).unwrap();

        let result = User::decode(&mut bf);

        assert!(result.is_err());
    }

    #[test]
    fn test_decode_malformed_data() {
        use bufferfish_core as bufferfish;
        use bufferfish_core::Decodable;

        #[derive(Decode, Encode, Debug)]
        struct User {
            id: u32,
            name: String,
        }

        let mut bf = Bufferfish::new();
        bf.write_u32(0).unwrap();
        bf.write_u8(0).unwrap();

        let result = User::decode(&mut bf);

        assert!(result.is_err());
    }

    #[test]
    fn test_decode_into_struct() {
        use bufferfish_core as bufferfish;
        use bufferfish_core::Decodable;

        #[derive(Decode, Encode, Debug)]
        struct User {
            id: u32,
            name: String,
        }

        let mut bf = Bufferfish::new();
        bf.write_u32(0).unwrap();
        bf.write_string("Bufferfish").unwrap();

        let user = User::from_bufferfish(&mut bf).unwrap();

        assert_eq!(user.id, 0);
        assert_eq!(user.name, "Bufferfish");
    }

    #[test]
    fn test_decode_too_small_into_struct() {
        use bufferfish_core as bufferfish;
        use bufferfish_core::Decodable;

        #[derive(Decode, Encode, Debug)]
        struct User {
            id: u32,
            name: String,
        }

        // Minimum of 6 bytes: 4 bytes for u32 and 2 bytes for string length
        let mut bf_err = Bufferfish::from(vec![0, 0, 0, 0, 0]);
        let mut bf_ok = Bufferfish::from(vec![0, 0, 0, 0, 0, 0]);

        let result_should_err = User::from_bufferfish(&mut bf_err);
        let result_should_ok = User::from_bufferfish(&mut bf_ok);

        assert!(result_should_err.is_err());
        assert!(result_should_ok.is_ok());
    }

    #[test]
    fn test_decode_too_large_into_struct() {
        use bufferfish_core as bufferfish;
        use bufferfish_core::Decodable;

        #[derive(Decode, Encode, Debug)]
        struct User {
            id: u32,
        }

        let mut bf_err = Bufferfish::from(vec![0, 0, 0, 0, 0]);
        let mut bf_ok = Bufferfish::from(vec![0, 0, 0, 0]);

        let result_should_err = User::from_bufferfish(&mut bf_err);
        let result_should_ok = User::from_bufferfish(&mut bf_ok);

        assert!(result_should_err.is_err());
        assert!(result_should_ok.is_ok());
    }

    #[test]
    fn test_reset() {
        let mut bf = Bufferfish::new();
        bf.write_u8(0).unwrap();
        bf.write_u8(255).unwrap();

        bf.reset();

        assert_eq!(bf.len(), 0);
        assert_eq!(bf.as_ref(), &[]);
    }

    #[test]
    fn test_encode_decode_complex_enums() {
        use bufferfish_core as bufferfish;
        use bufferfish_core::{Decodable, Encodable};
        use bufferfish_derive::{Decode, Encode};

        #[derive(Encode, Decode)]
        enum Object {
            Variant1 { a: u32, b: String },
            Variant2 { c: i32, d: bool },
        }

        #[derive(Encode, Decode)]
        enum Complex {
            Object(Object),
            Stringly(String),
            Simple(u8),
            Other,
        }

        #[derive(Encode, Decode)]
        #[bufferfish(0_u16)]
        enum ObjectId {
            Test,
        }

        impl From<ObjectId> for u16 {
            fn from(value: ObjectId) -> Self {
                match value {
                    ObjectId::Test => 0,
                }
            }
        }

        #[derive(Encode, Decode)]
        #[bufferfish(ObjectId::Test)]
        pub struct ObjectContainer {
            complex: Complex,
        }

        let mut bf = Bufferfish::new();
        let complex = ObjectContainer {
            complex: Complex::Object(Object::Variant1 {
                a: 42,
                b: "Hello".to_string(),
            }),
        };

        complex.encode(&mut bf).unwrap();

        let decoded = ObjectContainer::decode(&mut bf).unwrap();

        match decoded.complex {
            Complex::Object(Object::Variant1 { a, b }) => {
                assert_eq!(a, 42);
                assert_eq!(b, "Hello");
            }
            _ => panic!("Decoded complex type did not match expected variant"),
        }
    }

    #[test]
    fn test_manual_encode_decode_impl() {
        use bufferfish_core as bufferfish;
        use bufferfish_core::{Decodable, Encodable};

        #[derive(Debug)]
        struct CustomType {
            value: u32,
        }

        impl bufferfish::Encodable for CustomType {
            fn encode_value(
                &self,
                bf: &mut bufferfish::Bufferfish,
            ) -> Result<(), bufferfish::BufferfishError> {
                bf.write_u32(self.value)
            }
        }

        impl bufferfish::Decodable for CustomType {
            fn decode_value(
                bf: &mut bufferfish::Bufferfish,
            ) -> Result<Self, bufferfish::BufferfishError> {
                let value = bf.read_u32()?;
                Ok(CustomType { value })
            }
        }

        let mut bf = bufferfish::Bufferfish::new();
        let custom = CustomType { value: 42 };
        custom.encode(&mut bf).unwrap();
        assert_eq!(bf.as_ref(), &[0, 0, 0, 42]);

        let decoded = CustomType::decode(&mut bf).unwrap();
        assert_eq!(decoded.value, 42);
    }

    #[test]
    fn test_optional_encode() {
        use bufferfish_core as bufferfish;
        use bufferfish_core::Encodable;

        #[derive(Debug, Encode)]
        struct OptionContainer {
            value: Option<u32>,
        }

        let mut bf = Bufferfish::new();
        let container_with_value = OptionContainer { value: Some(42) };
        let container_without_value = OptionContainer { value: None };

        container_with_value.encode(&mut bf).unwrap();
        container_without_value.encode(&mut bf).unwrap();

        assert_eq!(bf.as_ref(), &[1, 0, 0, 0, 42, 0]);
    }

    #[test]
    fn test_optional_decode_some() {
        use bufferfish_core as bufferfish;
        use bufferfish_core::Decodable;

        #[derive(Debug, Decode)]
        struct OptionContainer {
            value: Option<u32>,
        }

        let mut bf = Bufferfish::new();
        bf.write_option(&Some(42)).unwrap();

        let container_with_value = OptionContainer::decode(&mut bf).unwrap();

        assert_eq!(container_with_value.value, Some(42));
    }

    #[test]
    fn test_optional_decode_none() {
        use bufferfish_core as bufferfish;
        use bufferfish_core::Decodable;

        #[derive(Debug, Decode)]
        struct OptionContainer {
            value: Option<u32>,
        }

        let mut bf = Bufferfish::new();
        bf.write_option(&None::<u32>).unwrap();

        let container_without_value = OptionContainer::decode(&mut bf).unwrap();

        assert_eq!(container_without_value.value, None);
    }

    #[test]
    fn test_encode_decode_with_option() {
        use bufferfish_core as bufferfish;
        use bufferfish_core::{Decodable, Encodable};

        #[derive(Debug, Encode, Decode)]
        struct User {
            id: u32,
            name: String,
            age: Option<u8>,
        }

        let mut bf = Bufferfish::new();
        let user_with_age = User {
            id: 1,
            name: "Bufferfish".to_string(),
            age: Some(10),
        };
        let user_without_age = User {
            id: 2,
            name: "Bufferfish2".to_string(),
            age: None,
        };

        user_with_age.encode(&mut bf).unwrap();
        user_without_age.encode(&mut bf).unwrap();

        let decoded_user_with_age = User::decode(&mut bf).unwrap();
        let decoded_user_without_age = User::decode(&mut bf).unwrap();

        assert_eq!(decoded_user_with_age.id, 1);
        assert_eq!(decoded_user_with_age.name, "Bufferfish");
        assert_eq!(decoded_user_with_age.age, Some(10));

        assert_eq!(decoded_user_without_age.id, 2);
        assert_eq!(decoded_user_without_age.name, "Bufferfish2");
        assert_eq!(decoded_user_without_age.age, None);
    }
}
