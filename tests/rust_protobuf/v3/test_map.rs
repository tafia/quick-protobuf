use quick_protobuf::*;

use std::borrow::Cow;
use rust_protobuf::hex::{encode_hex, decode_hex};
use super::test_map_pb::*;

#[test]
fn test_map() {
    let mut map = TestMap::default();
    let mut entry = TestMapEntry::default();
    entry.v = Some(10);

    test_serialize_deserialize!("", &map, TestMap);

    map.m.insert(Cow::Borrowed("two"), 2);
    test_serialize_deserialize!("0a 07 0a 03 74 77 6f 10 02", &map, TestMap);

    map.m.insert(Cow::Borrowed("sixty six"), 66);
    // Insert map entry sub message
    map.mm.insert(Cow::Borrowed("map"), entry);
    // cannot (easily) test hex, because order is not specified
    test_serialize_deserialize_length_delimited!(&map, TestMap);
}

#[test]
fn test_map_with_object() {
    let mut map = TestMap::default();

    let mut entry = TestMapEntry::default();
    entry.v = Some(10);

    test_serialize_deserialize!("", &map, TestMap);

    map.mm.insert(Cow::Borrowed("map"), entry);
    // cannot (easily) test hex, because order is not specified
    test_serialize_deserialize_length_delimited!(&map, TestMap);
}

// #[test]
// fn text_format() {
//     let mut map = TestMap::new();
// 
//     assert_eq!(&*print_to_string(&map), "");
// 
//     map.mut_m().insert("two".to_owned(), 2);
// 
//     assert_eq!(&*print_to_string(&map), "m {key: \"two\" value: 2}")
// }
