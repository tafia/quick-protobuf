#[derive(Clone, Debug, PartialEq, Message)]
pub struct Test1 {
    #[prost(int32, optional, tag="1")]
    pub value: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Message)]
pub struct TestRepeatedBool {
    #[prost(bool, repeated, packed="false", tag="1")]
    pub values: Vec<bool>,
}
#[derive(Clone, Debug, PartialEq, Message)]
pub struct TestRepeatedPackedInt32 {
    #[prost(int32, repeated, tag="1")]
    pub values: Vec<i32>,
}
#[derive(Clone, Debug, PartialEq, Message)]
pub struct TestRepeatedMessages {
    #[prost(message, repeated, tag="1")]
    pub messages1: Vec<TestRepeatedMessages>,
    #[prost(message, repeated, tag="2")]
    pub messages2: Vec<TestRepeatedMessages>,
    #[prost(message, repeated, tag="3")]
    pub messages3: Vec<TestRepeatedMessages>,
}
#[derive(Clone, Debug, PartialEq, Message)]
pub struct TestOptionalMessages {
    #[prost(message, optional, boxed, tag="1")]
    pub message1: Option<Box<TestOptionalMessages>>,
    #[prost(message, optional, boxed, tag="2")]
    pub message2: Option<Box<TestOptionalMessages>>,
    #[prost(message, optional, boxed, tag="3")]
    pub message3: Option<Box<TestOptionalMessages>>,
}
#[derive(Clone, Debug, PartialEq, Message)]
pub struct TestStrings {
    #[prost(string, optional, tag="1")]
    pub s1: Option<String>,
    #[prost(string, optional, tag="2")]
    pub s2: Option<String>,
    #[prost(string, optional, tag="3")]
    pub s3: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Message)]
pub struct TestBytes {
    #[prost(bytes, optional, tag="1")]
    pub b1: Option<Vec<u8>>,
}
#[derive(Clone, Debug, PartialEq, Message)]
pub struct PerftestData {
    #[prost(message, repeated, tag="1")]
    pub test1: Vec<Test1>,
    #[prost(message, repeated, tag="2")]
    pub test_repeated_bool: Vec<TestRepeatedBool>,
    #[prost(message, repeated, tag="3")]
    pub test_repeated_messages: Vec<TestRepeatedMessages>,
    #[prost(message, repeated, tag="4")]
    pub test_optional_messages: Vec<TestOptionalMessages>,
    #[prost(message, repeated, tag="5")]
    pub test_strings: Vec<TestStrings>,
    #[prost(message, repeated, tag="6")]
    pub test_repeated_packed_int32: Vec<TestRepeatedPackedInt32>,
    #[prost(message, repeated, tag="7")]
    pub test_small_bytearrays: Vec<TestBytes>,
    #[prost(message, repeated, tag="8")]
    pub test_large_bytearrays: Vec<TestBytes>,
}
