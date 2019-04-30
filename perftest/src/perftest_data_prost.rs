#[derive(Clone, PartialEq, Message)]
pub struct Test1 {
    #[prost(int32, optional, tag="1")]
    pub value: ::std::option::Option<i32>,
}
#[derive(Clone, PartialEq, Message)]
pub struct TestRepeatedBool {
    #[prost(bool, repeated, packed="false", tag="1")]
    pub values: ::std::vec::Vec<bool>,
}
#[derive(Clone, PartialEq, Message)]
pub struct TestRepeatedPackedInt32 {
    #[prost(int32, repeated, tag="1")]
    pub values: ::std::vec::Vec<i32>,
}
#[derive(Clone, PartialEq, Message)]
pub struct TestRepeatedMessages {
    #[prost(message, repeated, tag="1")]
    pub messages1: ::std::vec::Vec<TestRepeatedMessages>,
    #[prost(message, repeated, tag="2")]
    pub messages2: ::std::vec::Vec<TestRepeatedMessages>,
    #[prost(message, repeated, tag="3")]
    pub messages3: ::std::vec::Vec<TestRepeatedMessages>,
}
#[derive(Clone, PartialEq, Message)]
pub struct TestOptionalMessages {
    #[prost(message, optional, boxed, tag="1")]
    pub message1: ::std::option::Option<::std::boxed::Box<TestOptionalMessages>>,
    #[prost(message, optional, boxed, tag="2")]
    pub message2: ::std::option::Option<::std::boxed::Box<TestOptionalMessages>>,
    #[prost(message, optional, boxed, tag="3")]
    pub message3: ::std::option::Option<::std::boxed::Box<TestOptionalMessages>>,
}
#[derive(Clone, PartialEq, Message)]
pub struct TestStrings {
    #[prost(string, optional, tag="1")]
    pub s1: ::std::option::Option<String>,
    #[prost(string, optional, tag="2")]
    pub s2: ::std::option::Option<String>,
    #[prost(string, optional, tag="3")]
    pub s3: ::std::option::Option<String>,
}
#[derive(Clone, PartialEq, Message)]
pub struct TestBytes {
    #[prost(bytes, optional, tag="1")]
    pub b1: ::std::option::Option<Vec<u8>>,
}
#[derive(Clone, PartialEq, Message)]
pub struct PerftestData {
    #[prost(message, repeated, tag="1")]
    pub test1: ::std::vec::Vec<Test1>,
    #[prost(message, repeated, tag="2")]
    pub test_repeated_bool: ::std::vec::Vec<TestRepeatedBool>,
    #[prost(message, repeated, tag="3")]
    pub test_repeated_messages: ::std::vec::Vec<TestRepeatedMessages>,
    #[prost(message, repeated, tag="4")]
    pub test_optional_messages: ::std::vec::Vec<TestOptionalMessages>,
    #[prost(message, repeated, tag="5")]
    pub test_strings: ::std::vec::Vec<TestStrings>,
    #[prost(message, repeated, tag="6")]
    pub test_repeated_packed_int32: ::std::vec::Vec<TestRepeatedPackedInt32>,
    #[prost(message, repeated, tag="7")]
    pub test_small_bytearrays: ::std::vec::Vec<TestBytes>,
    #[prost(message, repeated, tag="8")]
    pub test_large_bytearrays: ::std::vec::Vec<TestBytes>,
}
