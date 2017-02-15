use quick_protobuf::*;

use std::borrow::Cow;
use rust_protobuf::hex::{encode_hex, decode_hex};
use super::test_oneof_pb::*;

#[test]
fn test_simple() {
    let mut test_message = TestOneof::default();
    test_message.one = mod_TestOneof::OneOfone::uint32_field(150);
    test_serialize_deserialize!("28 96 01", &test_message, TestOneof);
}

#[test]
fn test_types() {
    fn t<F>(f: F)
        where F : Fn(&mut TestOneof)
    {
        let mut o = TestOneof::default();
        f(&mut o);
        test_serialize_deserialize_length_delimited!(&o, TestOneof);
    }

    t(|o| o.one = mod_TestOneof::OneOfone::double_field(10.0));
    t(|o| o.one = mod_TestOneof::OneOfone::float_field(11.0));
    t(|o| o.one = mod_TestOneof::OneOfone::int32_field(12));
    t(|o| o.one = mod_TestOneof::OneOfone::int64_field(13));
    t(|o| o.one = mod_TestOneof::OneOfone::uint32_field(14));
    t(|o| o.one = mod_TestOneof::OneOfone::uint64_field(15));
    t(|o| o.one = mod_TestOneof::OneOfone::sint32_field(16));
    t(|o| o.one = mod_TestOneof::OneOfone::sint64_field(17));
    t(|o| o.one = mod_TestOneof::OneOfone::fixed32_field(18));
    t(|o| o.one = mod_TestOneof::OneOfone::fixed64_field(19));
    t(|o| o.one = mod_TestOneof::OneOfone::sfixed32_field(20));
    t(|o| o.one = mod_TestOneof::OneOfone::sfixed64_field(21));
    t(|o| o.one = mod_TestOneof::OneOfone::bool_field(true));
    t(|o| o.one = mod_TestOneof::OneOfone::string_field(Cow::Borrowed("asas")));
    t(|o| o.one = mod_TestOneof::OneOfone::bytes_field(Cow::Owned(vec![99, 100])));
    t(|o| o.one = mod_TestOneof::OneOfone::enum_field(EnumForOneof::A));
    t(|o| {
        let mut msg = MessageForOneof::default();
        msg.f = Some(22);
        o.one = mod_TestOneof::OneOfone::message_field(msg);
    })
}
