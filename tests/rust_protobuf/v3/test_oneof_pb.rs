// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct MessageForOneof {
    // message fields
    pub f: i32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MessageForOneof {}

impl MessageForOneof {
    pub fn new() -> MessageForOneof {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MessageForOneof {
        static mut instance: ::protobuf::lazy::Lazy<MessageForOneof> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MessageForOneof,
        };
        unsafe {
            instance.get(MessageForOneof::new)
        }
    }

    // int32 f = 1;

    pub fn clear_f(&mut self) {
        self.f = 0;
    }

    // Param is passed by value, moved
    pub fn set_f(&mut self, v: i32) {
        self.f = v;
    }

    pub fn get_f(&self) -> i32 {
        self.f
    }

    fn get_f_for_reflect(&self) -> &i32 {
        &self.f
    }

    fn mut_f_for_reflect(&mut self) -> &mut i32 {
        &mut self.f
    }
}

impl ::protobuf::Message for MessageForOneof {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.f = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.f != 0 {
            my_size += ::protobuf::rt::value_size(1, self.f, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.f != 0 {
            os.write_int32(1, self.f)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for MessageForOneof {
    fn new() -> MessageForOneof {
        MessageForOneof::new()
    }

    fn descriptor_static(_: ::std::option::Option<MessageForOneof>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "f",
                    MessageForOneof::get_f_for_reflect,
                    MessageForOneof::mut_f_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MessageForOneof>(
                    "MessageForOneof",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MessageForOneof {
    fn clear(&mut self) {
        self.clear_f();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MessageForOneof {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MessageForOneof {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TestOneof {
    // message fields
    pub s: ::std::string::String,
    // message oneof groups
    one: ::std::option::Option<TestOneof_oneof_one>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TestOneof {}

#[derive(Clone,PartialEq)]
pub enum TestOneof_oneof_one {
    double_field(f64),
    float_field(f32),
    int32_field(i32),
    int64_field(i64),
    uint32_field(u32),
    uint64_field(u64),
    sint32_field(i32),
    sint64_field(i64),
    fixed32_field(u32),
    fixed64_field(u64),
    sfixed32_field(i32),
    sfixed64_field(i64),
    bool_field(bool),
    string_field(::std::string::String),
    bytes_field(::std::vec::Vec<u8>),
    enum_field(EnumForOneof),
    message_field(MessageForOneof),
}

impl TestOneof {
    pub fn new() -> TestOneof {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TestOneof {
        static mut instance: ::protobuf::lazy::Lazy<TestOneof> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TestOneof,
        };
        unsafe {
            instance.get(TestOneof::new)
        }
    }

    // string s = 29;

    pub fn clear_s(&mut self) {
        self.s.clear();
    }

    // Param is passed by value, moved
    pub fn set_s(&mut self, v: ::std::string::String) {
        self.s = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_s(&mut self) -> &mut ::std::string::String {
        &mut self.s
    }

    // Take field
    pub fn take_s(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.s, ::std::string::String::new())
    }

    pub fn get_s(&self) -> &str {
        &self.s
    }

    fn get_s_for_reflect(&self) -> &::std::string::String {
        &self.s
    }

    fn mut_s_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.s
    }

    // double double_field = 1;

    pub fn clear_double_field(&mut self) {
        self.one = ::std::option::Option::None;
    }

    pub fn has_double_field(&self) -> bool {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::double_field(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_double_field(&mut self, v: f64) {
        self.one = ::std::option::Option::Some(TestOneof_oneof_one::double_field(v))
    }

    pub fn get_double_field(&self) -> f64 {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::double_field(v)) => v,
            _ => 0.,
        }
    }

    // float float_field = 2;

    pub fn clear_float_field(&mut self) {
        self.one = ::std::option::Option::None;
    }

    pub fn has_float_field(&self) -> bool {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::float_field(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_float_field(&mut self, v: f32) {
        self.one = ::std::option::Option::Some(TestOneof_oneof_one::float_field(v))
    }

    pub fn get_float_field(&self) -> f32 {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::float_field(v)) => v,
            _ => 0.,
        }
    }

    // int32 int32_field = 3;

    pub fn clear_int32_field(&mut self) {
        self.one = ::std::option::Option::None;
    }

    pub fn has_int32_field(&self) -> bool {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::int32_field(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_int32_field(&mut self, v: i32) {
        self.one = ::std::option::Option::Some(TestOneof_oneof_one::int32_field(v))
    }

    pub fn get_int32_field(&self) -> i32 {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::int32_field(v)) => v,
            _ => 0,
        }
    }

    // int64 int64_field = 4;

    pub fn clear_int64_field(&mut self) {
        self.one = ::std::option::Option::None;
    }

    pub fn has_int64_field(&self) -> bool {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::int64_field(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_int64_field(&mut self, v: i64) {
        self.one = ::std::option::Option::Some(TestOneof_oneof_one::int64_field(v))
    }

    pub fn get_int64_field(&self) -> i64 {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::int64_field(v)) => v,
            _ => 0,
        }
    }

    // uint32 uint32_field = 5;

    pub fn clear_uint32_field(&mut self) {
        self.one = ::std::option::Option::None;
    }

    pub fn has_uint32_field(&self) -> bool {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::uint32_field(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_uint32_field(&mut self, v: u32) {
        self.one = ::std::option::Option::Some(TestOneof_oneof_one::uint32_field(v))
    }

    pub fn get_uint32_field(&self) -> u32 {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::uint32_field(v)) => v,
            _ => 0,
        }
    }

    // uint64 uint64_field = 6;

    pub fn clear_uint64_field(&mut self) {
        self.one = ::std::option::Option::None;
    }

    pub fn has_uint64_field(&self) -> bool {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::uint64_field(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_uint64_field(&mut self, v: u64) {
        self.one = ::std::option::Option::Some(TestOneof_oneof_one::uint64_field(v))
    }

    pub fn get_uint64_field(&self) -> u64 {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::uint64_field(v)) => v,
            _ => 0,
        }
    }

    // sint32 sint32_field = 7;

    pub fn clear_sint32_field(&mut self) {
        self.one = ::std::option::Option::None;
    }

    pub fn has_sint32_field(&self) -> bool {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::sint32_field(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_sint32_field(&mut self, v: i32) {
        self.one = ::std::option::Option::Some(TestOneof_oneof_one::sint32_field(v))
    }

    pub fn get_sint32_field(&self) -> i32 {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::sint32_field(v)) => v,
            _ => 0,
        }
    }

    // sint64 sint64_field = 8;

    pub fn clear_sint64_field(&mut self) {
        self.one = ::std::option::Option::None;
    }

    pub fn has_sint64_field(&self) -> bool {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::sint64_field(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_sint64_field(&mut self, v: i64) {
        self.one = ::std::option::Option::Some(TestOneof_oneof_one::sint64_field(v))
    }

    pub fn get_sint64_field(&self) -> i64 {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::sint64_field(v)) => v,
            _ => 0,
        }
    }

    // fixed32 fixed32_field = 9;

    pub fn clear_fixed32_field(&mut self) {
        self.one = ::std::option::Option::None;
    }

    pub fn has_fixed32_field(&self) -> bool {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::fixed32_field(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_fixed32_field(&mut self, v: u32) {
        self.one = ::std::option::Option::Some(TestOneof_oneof_one::fixed32_field(v))
    }

    pub fn get_fixed32_field(&self) -> u32 {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::fixed32_field(v)) => v,
            _ => 0,
        }
    }

    // fixed64 fixed64_field = 10;

    pub fn clear_fixed64_field(&mut self) {
        self.one = ::std::option::Option::None;
    }

    pub fn has_fixed64_field(&self) -> bool {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::fixed64_field(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_fixed64_field(&mut self, v: u64) {
        self.one = ::std::option::Option::Some(TestOneof_oneof_one::fixed64_field(v))
    }

    pub fn get_fixed64_field(&self) -> u64 {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::fixed64_field(v)) => v,
            _ => 0,
        }
    }

    // sfixed32 sfixed32_field = 11;

    pub fn clear_sfixed32_field(&mut self) {
        self.one = ::std::option::Option::None;
    }

    pub fn has_sfixed32_field(&self) -> bool {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::sfixed32_field(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_sfixed32_field(&mut self, v: i32) {
        self.one = ::std::option::Option::Some(TestOneof_oneof_one::sfixed32_field(v))
    }

    pub fn get_sfixed32_field(&self) -> i32 {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::sfixed32_field(v)) => v,
            _ => 0,
        }
    }

    // sfixed64 sfixed64_field = 12;

    pub fn clear_sfixed64_field(&mut self) {
        self.one = ::std::option::Option::None;
    }

    pub fn has_sfixed64_field(&self) -> bool {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::sfixed64_field(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_sfixed64_field(&mut self, v: i64) {
        self.one = ::std::option::Option::Some(TestOneof_oneof_one::sfixed64_field(v))
    }

    pub fn get_sfixed64_field(&self) -> i64 {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::sfixed64_field(v)) => v,
            _ => 0,
        }
    }

    // bool bool_field = 13;

    pub fn clear_bool_field(&mut self) {
        self.one = ::std::option::Option::None;
    }

    pub fn has_bool_field(&self) -> bool {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::bool_field(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_bool_field(&mut self, v: bool) {
        self.one = ::std::option::Option::Some(TestOneof_oneof_one::bool_field(v))
    }

    pub fn get_bool_field(&self) -> bool {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::bool_field(v)) => v,
            _ => false,
        }
    }

    // string string_field = 14;

    pub fn clear_string_field(&mut self) {
        self.one = ::std::option::Option::None;
    }

    pub fn has_string_field(&self) -> bool {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::string_field(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_string_field(&mut self, v: ::std::string::String) {
        self.one = ::std::option::Option::Some(TestOneof_oneof_one::string_field(v))
    }

    // Mutable pointer to the field.
    pub fn mut_string_field(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(TestOneof_oneof_one::string_field(_)) = self.one {
        } else {
            self.one = ::std::option::Option::Some(TestOneof_oneof_one::string_field(::std::string::String::new()));
        }
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::string_field(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_string_field(&mut self) -> ::std::string::String {
        if self.has_string_field() {
            match self.one.take() {
                ::std::option::Option::Some(TestOneof_oneof_one::string_field(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_string_field(&self) -> &str {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::string_field(ref v)) => v,
            _ => "",
        }
    }

    // bytes bytes_field = 15;

    pub fn clear_bytes_field(&mut self) {
        self.one = ::std::option::Option::None;
    }

    pub fn has_bytes_field(&self) -> bool {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::bytes_field(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_bytes_field(&mut self, v: ::std::vec::Vec<u8>) {
        self.one = ::std::option::Option::Some(TestOneof_oneof_one::bytes_field(v))
    }

    // Mutable pointer to the field.
    pub fn mut_bytes_field(&mut self) -> &mut ::std::vec::Vec<u8> {
        if let ::std::option::Option::Some(TestOneof_oneof_one::bytes_field(_)) = self.one {
        } else {
            self.one = ::std::option::Option::Some(TestOneof_oneof_one::bytes_field(::std::vec::Vec::new()));
        }
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::bytes_field(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_bytes_field(&mut self) -> ::std::vec::Vec<u8> {
        if self.has_bytes_field() {
            match self.one.take() {
                ::std::option::Option::Some(TestOneof_oneof_one::bytes_field(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::vec::Vec::new()
        }
    }

    pub fn get_bytes_field(&self) -> &[u8] {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::bytes_field(ref v)) => v,
            _ => &[],
        }
    }

    // .EnumForOneof enum_field = 16;

    pub fn clear_enum_field(&mut self) {
        self.one = ::std::option::Option::None;
    }

    pub fn has_enum_field(&self) -> bool {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::enum_field(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_enum_field(&mut self, v: EnumForOneof) {
        self.one = ::std::option::Option::Some(TestOneof_oneof_one::enum_field(v))
    }

    pub fn get_enum_field(&self) -> EnumForOneof {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::enum_field(v)) => v,
            _ => EnumForOneof::Z,
        }
    }

    // .MessageForOneof message_field = 17;

    pub fn clear_message_field(&mut self) {
        self.one = ::std::option::Option::None;
    }

    pub fn has_message_field(&self) -> bool {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::message_field(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_message_field(&mut self, v: MessageForOneof) {
        self.one = ::std::option::Option::Some(TestOneof_oneof_one::message_field(v))
    }

    // Mutable pointer to the field.
    pub fn mut_message_field(&mut self) -> &mut MessageForOneof {
        if let ::std::option::Option::Some(TestOneof_oneof_one::message_field(_)) = self.one {
        } else {
            self.one = ::std::option::Option::Some(TestOneof_oneof_one::message_field(MessageForOneof::new()));
        }
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::message_field(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_message_field(&mut self) -> MessageForOneof {
        if self.has_message_field() {
            match self.one.take() {
                ::std::option::Option::Some(TestOneof_oneof_one::message_field(v)) => v,
                _ => panic!(),
            }
        } else {
            MessageForOneof::new()
        }
    }

    pub fn get_message_field(&self) -> &MessageForOneof {
        match self.one {
            ::std::option::Option::Some(TestOneof_oneof_one::message_field(ref v)) => v,
            _ => MessageForOneof::default_instance(),
        }
    }
}

impl ::protobuf::Message for TestOneof {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                29 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.s)?;
                },
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.one = ::std::option::Option::Some(TestOneof_oneof_one::double_field(is.read_double()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.one = ::std::option::Option::Some(TestOneof_oneof_one::float_field(is.read_float()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.one = ::std::option::Option::Some(TestOneof_oneof_one::int32_field(is.read_int32()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.one = ::std::option::Option::Some(TestOneof_oneof_one::int64_field(is.read_int64()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.one = ::std::option::Option::Some(TestOneof_oneof_one::uint32_field(is.read_uint32()?));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.one = ::std::option::Option::Some(TestOneof_oneof_one::uint64_field(is.read_uint64()?));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.one = ::std::option::Option::Some(TestOneof_oneof_one::sint32_field(is.read_sint32()?));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.one = ::std::option::Option::Some(TestOneof_oneof_one::sint64_field(is.read_sint64()?));
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.one = ::std::option::Option::Some(TestOneof_oneof_one::fixed32_field(is.read_fixed32()?));
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.one = ::std::option::Option::Some(TestOneof_oneof_one::fixed64_field(is.read_fixed64()?));
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.one = ::std::option::Option::Some(TestOneof_oneof_one::sfixed32_field(is.read_sfixed32()?));
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.one = ::std::option::Option::Some(TestOneof_oneof_one::sfixed64_field(is.read_sfixed64()?));
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.one = ::std::option::Option::Some(TestOneof_oneof_one::bool_field(is.read_bool()?));
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.one = ::std::option::Option::Some(TestOneof_oneof_one::string_field(is.read_string()?));
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.one = ::std::option::Option::Some(TestOneof_oneof_one::bytes_field(is.read_bytes()?));
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.one = ::std::option::Option::Some(TestOneof_oneof_one::enum_field(is.read_enum()?));
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.one = ::std::option::Option::Some(TestOneof_oneof_one::message_field(is.read_message()?));
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.s.is_empty() {
            my_size += ::protobuf::rt::string_size(29, &self.s);
        };
        if let ::std::option::Option::Some(ref v) = self.one {
            match v {
                &TestOneof_oneof_one::double_field(v) => {
                    my_size += 9;
                },
                &TestOneof_oneof_one::float_field(v) => {
                    my_size += 5;
                },
                &TestOneof_oneof_one::int32_field(v) => {
                    my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &TestOneof_oneof_one::int64_field(v) => {
                    my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &TestOneof_oneof_one::uint32_field(v) => {
                    my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &TestOneof_oneof_one::uint64_field(v) => {
                    my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &TestOneof_oneof_one::sint32_field(v) => {
                    my_size += ::protobuf::rt::value_varint_zigzag_size(7, v);
                },
                &TestOneof_oneof_one::sint64_field(v) => {
                    my_size += ::protobuf::rt::value_varint_zigzag_size(8, v);
                },
                &TestOneof_oneof_one::fixed32_field(v) => {
                    my_size += 5;
                },
                &TestOneof_oneof_one::fixed64_field(v) => {
                    my_size += 9;
                },
                &TestOneof_oneof_one::sfixed32_field(v) => {
                    my_size += 5;
                },
                &TestOneof_oneof_one::sfixed64_field(v) => {
                    my_size += 9;
                },
                &TestOneof_oneof_one::bool_field(v) => {
                    my_size += 2;
                },
                &TestOneof_oneof_one::string_field(ref v) => {
                    my_size += ::protobuf::rt::string_size(14, &v);
                },
                &TestOneof_oneof_one::bytes_field(ref v) => {
                    my_size += ::protobuf::rt::bytes_size(15, &v);
                },
                &TestOneof_oneof_one::enum_field(v) => {
                    my_size += ::protobuf::rt::enum_size(16, v);
                },
                &TestOneof_oneof_one::message_field(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.s.is_empty() {
            os.write_string(29, &self.s)?;
        };
        if let ::std::option::Option::Some(ref v) = self.one {
            match v {
                &TestOneof_oneof_one::double_field(v) => {
                    os.write_double(1, v)?;
                },
                &TestOneof_oneof_one::float_field(v) => {
                    os.write_float(2, v)?;
                },
                &TestOneof_oneof_one::int32_field(v) => {
                    os.write_int32(3, v)?;
                },
                &TestOneof_oneof_one::int64_field(v) => {
                    os.write_int64(4, v)?;
                },
                &TestOneof_oneof_one::uint32_field(v) => {
                    os.write_uint32(5, v)?;
                },
                &TestOneof_oneof_one::uint64_field(v) => {
                    os.write_uint64(6, v)?;
                },
                &TestOneof_oneof_one::sint32_field(v) => {
                    os.write_sint32(7, v)?;
                },
                &TestOneof_oneof_one::sint64_field(v) => {
                    os.write_sint64(8, v)?;
                },
                &TestOneof_oneof_one::fixed32_field(v) => {
                    os.write_fixed32(9, v)?;
                },
                &TestOneof_oneof_one::fixed64_field(v) => {
                    os.write_fixed64(10, v)?;
                },
                &TestOneof_oneof_one::sfixed32_field(v) => {
                    os.write_sfixed32(11, v)?;
                },
                &TestOneof_oneof_one::sfixed64_field(v) => {
                    os.write_sfixed64(12, v)?;
                },
                &TestOneof_oneof_one::bool_field(v) => {
                    os.write_bool(13, v)?;
                },
                &TestOneof_oneof_one::string_field(ref v) => {
                    os.write_string(14, v)?;
                },
                &TestOneof_oneof_one::bytes_field(ref v) => {
                    os.write_bytes(15, v)?;
                },
                &TestOneof_oneof_one::enum_field(v) => {
                    os.write_enum(16, v.value())?;
                },
                &TestOneof_oneof_one::message_field(ref v) => {
                    os.write_tag(17, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TestOneof {
    fn new() -> TestOneof {
        TestOneof::new()
    }

    fn descriptor_static(_: ::std::option::Option<TestOneof>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "s",
                    TestOneof::get_s_for_reflect,
                    TestOneof::mut_s_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor::<_>(
                    "double_field",
                    TestOneof::has_double_field,
                    TestOneof::get_double_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor::<_>(
                    "float_field",
                    TestOneof::has_float_field,
                    TestOneof::get_float_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor::<_>(
                    "int32_field",
                    TestOneof::has_int32_field,
                    TestOneof::get_int32_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor::<_>(
                    "int64_field",
                    TestOneof::has_int64_field,
                    TestOneof::get_int64_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor::<_>(
                    "uint32_field",
                    TestOneof::has_uint32_field,
                    TestOneof::get_uint32_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor::<_>(
                    "uint64_field",
                    TestOneof::has_uint64_field,
                    TestOneof::get_uint64_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor::<_>(
                    "sint32_field",
                    TestOneof::has_sint32_field,
                    TestOneof::get_sint32_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor::<_>(
                    "sint64_field",
                    TestOneof::has_sint64_field,
                    TestOneof::get_sint64_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor::<_>(
                    "fixed32_field",
                    TestOneof::has_fixed32_field,
                    TestOneof::get_fixed32_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor::<_>(
                    "fixed64_field",
                    TestOneof::has_fixed64_field,
                    TestOneof::get_fixed64_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor::<_>(
                    "sfixed32_field",
                    TestOneof::has_sfixed32_field,
                    TestOneof::get_sfixed32_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor::<_>(
                    "sfixed64_field",
                    TestOneof::has_sfixed64_field,
                    TestOneof::get_sfixed64_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor::<_>(
                    "bool_field",
                    TestOneof::has_bool_field,
                    TestOneof::get_bool_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "string_field",
                    TestOneof::has_string_field,
                    TestOneof::get_string_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor::<_>(
                    "bytes_field",
                    TestOneof::has_bytes_field,
                    TestOneof::get_bytes_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor::<_, EnumForOneof>(
                    "enum_field",
                    TestOneof::has_enum_field,
                    TestOneof::get_enum_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, MessageForOneof>(
                    "message_field",
                    TestOneof::has_message_field,
                    TestOneof::get_message_field,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TestOneof>(
                    "TestOneof",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TestOneof {
    fn clear(&mut self) {
        self.clear_s();
        self.clear_double_field();
        self.clear_float_field();
        self.clear_int32_field();
        self.clear_int64_field();
        self.clear_uint32_field();
        self.clear_uint64_field();
        self.clear_sint32_field();
        self.clear_sint64_field();
        self.clear_fixed32_field();
        self.clear_fixed64_field();
        self.clear_sfixed32_field();
        self.clear_sfixed64_field();
        self.clear_bool_field();
        self.clear_string_field();
        self.clear_bytes_field();
        self.clear_enum_field();
        self.clear_message_field();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TestOneof {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TestOneof {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EnumForOneof {
    Z = 0,
    A = 10,
}

impl ::protobuf::ProtobufEnum for EnumForOneof {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EnumForOneof> {
        match value {
            0 => ::std::option::Option::Some(EnumForOneof::Z),
            10 => ::std::option::Option::Some(EnumForOneof::A),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EnumForOneof] = &[
            EnumForOneof::Z,
            EnumForOneof::A,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<EnumForOneof>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EnumForOneof", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EnumForOneof {
}

impl ::std::default::Default for EnumForOneof {
    fn default() -> Self {
        EnumForOneof::Z
    }
}

impl ::protobuf::reflect::ProtobufValue for EnumForOneof {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x13, 0x74, 0x65, 0x73, 0x74, 0x5f, 0x6f, 0x6e, 0x65, 0x6f, 0x66, 0x5f, 0x70, 0x62, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x1f, 0x0a, 0x0f, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x46, 0x6f, 0x72, 0x4f, 0x6e, 0x65, 0x6f, 0x66, 0x12, 0x0c, 0x0a, 0x01, 0x66, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x05, 0x52, 0x01, 0x66, 0x22, 0xb4, 0x05, 0x0a, 0x09, 0x54, 0x65, 0x73, 0x74, 0x4f,
    0x6e, 0x65, 0x6f, 0x66, 0x12, 0x0c, 0x0a, 0x01, 0x73, 0x18, 0x1d, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x01, 0x73, 0x12, 0x23, 0x0a, 0x0c, 0x64, 0x6f, 0x75, 0x62, 0x6c, 0x65, 0x5f, 0x66, 0x69, 0x65,
    0x6c, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x01, 0x48, 0x00, 0x52, 0x0b, 0x64, 0x6f, 0x75, 0x62,
    0x6c, 0x65, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x12, 0x21, 0x0a, 0x0b, 0x66, 0x6c, 0x6f, 0x61, 0x74,
    0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x02, 0x48, 0x00, 0x52, 0x0a,
    0x66, 0x6c, 0x6f, 0x61, 0x74, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x12, 0x21, 0x0a, 0x0b, 0x69, 0x6e,
    0x74, 0x33, 0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x48,
    0x00, 0x52, 0x0a, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x12, 0x21, 0x0a,
    0x0b, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x03, 0x48, 0x00, 0x52, 0x0a, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x46, 0x69, 0x65, 0x6c, 0x64,
    0x12, 0x23, 0x0a, 0x0c, 0x75, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64,
    0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x48, 0x00, 0x52, 0x0b, 0x75, 0x69, 0x6e, 0x74, 0x33, 0x32,
    0x46, 0x69, 0x65, 0x6c, 0x64, 0x12, 0x23, 0x0a, 0x0c, 0x75, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x5f,
    0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x06, 0x20, 0x01, 0x28, 0x04, 0x48, 0x00, 0x52, 0x0b, 0x75,
    0x69, 0x6e, 0x74, 0x36, 0x34, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x12, 0x23, 0x0a, 0x0c, 0x73, 0x69,
    0x6e, 0x74, 0x33, 0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x07, 0x20, 0x01, 0x28, 0x11,
    0x48, 0x00, 0x52, 0x0b, 0x73, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x12,
    0x23, 0x0a, 0x0c, 0x73, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18,
    0x08, 0x20, 0x01, 0x28, 0x12, 0x48, 0x00, 0x52, 0x0b, 0x73, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x46,
    0x69, 0x65, 0x6c, 0x64, 0x12, 0x25, 0x0a, 0x0d, 0x66, 0x69, 0x78, 0x65, 0x64, 0x33, 0x32, 0x5f,
    0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x09, 0x20, 0x01, 0x28, 0x07, 0x48, 0x00, 0x52, 0x0c, 0x66,
    0x69, 0x78, 0x65, 0x64, 0x33, 0x32, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x12, 0x25, 0x0a, 0x0d, 0x66,
    0x69, 0x78, 0x65, 0x64, 0x36, 0x34, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0a, 0x20, 0x01,
    0x28, 0x06, 0x48, 0x00, 0x52, 0x0c, 0x66, 0x69, 0x78, 0x65, 0x64, 0x36, 0x34, 0x46, 0x69, 0x65,
    0x6c, 0x64, 0x12, 0x27, 0x0a, 0x0e, 0x73, 0x66, 0x69, 0x78, 0x65, 0x64, 0x33, 0x32, 0x5f, 0x66,
    0x69, 0x65, 0x6c, 0x64, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x0f, 0x48, 0x00, 0x52, 0x0d, 0x73, 0x66,
    0x69, 0x78, 0x65, 0x64, 0x33, 0x32, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x12, 0x27, 0x0a, 0x0e, 0x73,
    0x66, 0x69, 0x78, 0x65, 0x64, 0x36, 0x34, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0c, 0x20,
    0x01, 0x28, 0x10, 0x48, 0x00, 0x52, 0x0d, 0x73, 0x66, 0x69, 0x78, 0x65, 0x64, 0x36, 0x34, 0x46,
    0x69, 0x65, 0x6c, 0x64, 0x12, 0x1f, 0x0a, 0x0a, 0x62, 0x6f, 0x6f, 0x6c, 0x5f, 0x66, 0x69, 0x65,
    0x6c, 0x64, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x08, 0x48, 0x00, 0x52, 0x09, 0x62, 0x6f, 0x6f, 0x6c,
    0x46, 0x69, 0x65, 0x6c, 0x64, 0x12, 0x23, 0x0a, 0x0c, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x5f,
    0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0e, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x0b, 0x73,
    0x74, 0x72, 0x69, 0x6e, 0x67, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x12, 0x21, 0x0a, 0x0b, 0x62, 0x79,
    0x74, 0x65, 0x73, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0f, 0x20, 0x01, 0x28, 0x0c, 0x48,
    0x00, 0x52, 0x0a, 0x62, 0x79, 0x74, 0x65, 0x73, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x12, 0x2e, 0x0a,
    0x0a, 0x65, 0x6e, 0x75, 0x6d, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x10, 0x20, 0x01, 0x28,
    0x0e, 0x32, 0x0d, 0x2e, 0x45, 0x6e, 0x75, 0x6d, 0x46, 0x6f, 0x72, 0x4f, 0x6e, 0x65, 0x6f, 0x66,
    0x48, 0x00, 0x52, 0x09, 0x65, 0x6e, 0x75, 0x6d, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x12, 0x37, 0x0a,
    0x0d, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x11,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x46, 0x6f,
    0x72, 0x4f, 0x6e, 0x65, 0x6f, 0x66, 0x48, 0x00, 0x52, 0x0c, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x42, 0x05, 0x0a, 0x03, 0x6f, 0x6e, 0x65, 0x2a, 0x1c, 0x0a,
    0x0c, 0x45, 0x6e, 0x75, 0x6d, 0x46, 0x6f, 0x72, 0x4f, 0x6e, 0x65, 0x6f, 0x66, 0x12, 0x05, 0x0a,
    0x01, 0x5a, 0x10, 0x00, 0x12, 0x05, 0x0a, 0x01, 0x41, 0x10, 0x0a, 0x4a, 0xfb, 0x09, 0x0a, 0x06,
    0x12, 0x04, 0x00, 0x00, 0x20, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12,
    0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x02, 0x00, 0x05, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x05, 0x00, 0x01, 0x12, 0x03, 0x02, 0x05, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x03, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x03, 0x04, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x03, 0x08,
    0x09, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x04, 0x04, 0x0b, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x04, 0x04, 0x05, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x04, 0x08, 0x0a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00,
    0x12, 0x04, 0x07, 0x00, 0x09, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x07,
    0x08, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x08, 0x04, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x08, 0x04, 0x07, 0x19, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x08, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x08, 0x0a, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x08, 0x0e, 0x0f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04,
    0x0b, 0x00, 0x20, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x11,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0c, 0x04, 0x12, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x04, 0x0c, 0x04, 0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0c, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x0c, 0x0b, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x0c, 0x0f, 0x11, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x01, 0x08, 0x00, 0x12, 0x04,
    0x0d, 0x04, 0x1f, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x08, 0x00, 0x01, 0x12, 0x03, 0x0d,
    0x0a, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x0e, 0x08, 0x20, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0e, 0x08, 0x0e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0e, 0x0f, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0e, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02,
    0x02, 0x12, 0x03, 0x0f, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12,
    0x03, 0x0f, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0f,
    0x0e, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0f, 0x1c, 0x1d,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x10, 0x08, 0x1e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x03, 0x05, 0x12, 0x03, 0x10, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x10, 0x0e, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x03, 0x03, 0x12, 0x03, 0x10, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x04, 0x12,
    0x03, 0x11, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x05, 0x12, 0x03, 0x11,
    0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x01, 0x12, 0x03, 0x11, 0x0e, 0x19,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x03, 0x12, 0x03, 0x11, 0x1c, 0x1d, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x01, 0x02, 0x05, 0x12, 0x03, 0x12, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x05, 0x05, 0x12, 0x03, 0x12, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x05, 0x01, 0x12, 0x03, 0x12, 0x0f, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x03,
    0x12, 0x03, 0x12, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x06, 0x12, 0x03, 0x13,
    0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x05, 0x12, 0x03, 0x13, 0x08, 0x0e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x01, 0x12, 0x03, 0x13, 0x0f, 0x1b, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x03, 0x12, 0x03, 0x13, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x07, 0x12, 0x03, 0x14, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x07, 0x05, 0x12, 0x03, 0x14, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x01,
    0x12, 0x03, 0x14, 0x0f, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x03, 0x12, 0x03,
    0x14, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x08, 0x12, 0x03, 0x15, 0x08, 0x20,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x08, 0x05, 0x12, 0x03, 0x15, 0x08, 0x0e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x08, 0x01, 0x12, 0x03, 0x15, 0x0f, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x08, 0x03, 0x12, 0x03, 0x15, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x09, 0x12, 0x03, 0x16, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x09, 0x05,
    0x12, 0x03, 0x16, 0x08, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x09, 0x01, 0x12, 0x03,
    0x16, 0x10, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x09, 0x03, 0x12, 0x03, 0x16, 0x20,
    0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x0a, 0x12, 0x03, 0x17, 0x08, 0x23, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x17, 0x08, 0x0f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x17, 0x10, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x0a, 0x03, 0x12, 0x03, 0x17, 0x20, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x0b,
    0x12, 0x03, 0x18, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0b, 0x05, 0x12, 0x03,
    0x18, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x18, 0x11,
    0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x18, 0x22, 0x24, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x0c, 0x12, 0x03, 0x19, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x0c, 0x05, 0x12, 0x03, 0x19, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x0c, 0x01, 0x12, 0x03, 0x19, 0x11, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0c,
    0x03, 0x12, 0x03, 0x19, 0x22, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x0d, 0x12, 0x03,
    0x1a, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0d, 0x05, 0x12, 0x03, 0x1a, 0x08,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0d, 0x01, 0x12, 0x03, 0x1a, 0x0d, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0d, 0x03, 0x12, 0x03, 0x1a, 0x1a, 0x1c, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x0e, 0x12, 0x03, 0x1b, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x0e, 0x05, 0x12, 0x03, 0x1b, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0e,
    0x01, 0x12, 0x03, 0x1b, 0x0f, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0e, 0x03, 0x12,
    0x03, 0x1b, 0x1e, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x0f, 0x12, 0x03, 0x1c, 0x08,
    0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0f, 0x05, 0x12, 0x03, 0x1c, 0x08, 0x0d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0f, 0x01, 0x12, 0x03, 0x1c, 0x0e, 0x19, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x0f, 0x03, 0x12, 0x03, 0x1c, 0x1c, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x10, 0x12, 0x03, 0x1d, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x10,
    0x06, 0x12, 0x03, 0x1d, 0x08, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x10, 0x01, 0x12,
    0x03, 0x1d, 0x15, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x10, 0x03, 0x12, 0x03, 0x1d,
    0x22, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x11, 0x12, 0x03, 0x1e, 0x08, 0x2b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x11, 0x06, 0x12, 0x03, 0x1e, 0x08, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x11, 0x01, 0x12, 0x03, 0x1e, 0x18, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x11, 0x03, 0x12, 0x03, 0x1e, 0x28, 0x2a, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x33,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
