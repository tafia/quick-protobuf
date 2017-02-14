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
pub struct TestExposeOneof {
    // message oneof groups
    foobar: ::std::option::Option<TestExposeOneof_oneof_foobar>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TestExposeOneof {}

#[derive(Clone,PartialEq)]
pub enum TestExposeOneof_oneof_foobar {
    s(::std::string::String),
    i(i32),
}

impl TestExposeOneof {
    pub fn new() -> TestExposeOneof {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TestExposeOneof {
        static mut instance: ::protobuf::lazy::Lazy<TestExposeOneof> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TestExposeOneof,
        };
        unsafe {
            instance.get(TestExposeOneof::new)
        }
    }

    // optional string s = 1;

    pub fn clear_s(&mut self) {
        self.foobar = ::std::option::Option::None;
    }

    pub fn has_s(&self) -> bool {
        match self.foobar {
            ::std::option::Option::Some(TestExposeOneof_oneof_foobar::s(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_s(&mut self, v: ::std::string::String) {
        self.foobar = ::std::option::Option::Some(TestExposeOneof_oneof_foobar::s(v))
    }

    // Mutable pointer to the field.
    pub fn mut_s(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(TestExposeOneof_oneof_foobar::s(_)) = self.foobar {
        } else {
            self.foobar = ::std::option::Option::Some(TestExposeOneof_oneof_foobar::s(::std::string::String::new()));
        }
        match self.foobar {
            ::std::option::Option::Some(TestExposeOneof_oneof_foobar::s(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_s(&mut self) -> ::std::string::String {
        if self.has_s() {
            match self.foobar.take() {
                ::std::option::Option::Some(TestExposeOneof_oneof_foobar::s(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_s(&self) -> &str {
        match self.foobar {
            ::std::option::Option::Some(TestExposeOneof_oneof_foobar::s(ref v)) => v,
            _ => "",
        }
    }

    // optional int32 i = 2;

    pub fn clear_i(&mut self) {
        self.foobar = ::std::option::Option::None;
    }

    pub fn has_i(&self) -> bool {
        match self.foobar {
            ::std::option::Option::Some(TestExposeOneof_oneof_foobar::i(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_i(&mut self, v: i32) {
        self.foobar = ::std::option::Option::Some(TestExposeOneof_oneof_foobar::i(v))
    }

    pub fn get_i(&self) -> i32 {
        match self.foobar {
            ::std::option::Option::Some(TestExposeOneof_oneof_foobar::i(v)) => v,
            _ => 0,
        }
    }
}

impl ::protobuf::Message for TestExposeOneof {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.foobar = ::std::option::Option::Some(TestExposeOneof_oneof_foobar::s(is.read_string()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.foobar = ::std::option::Option::Some(TestExposeOneof_oneof_foobar::i(is.read_int32()?));
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
        if let ::std::option::Option::Some(ref v) = self.foobar {
            match v {
                &TestExposeOneof_oneof_foobar::s(ref v) => {
                    my_size += ::protobuf::rt::string_size(1, &v);
                },
                &TestExposeOneof_oneof_foobar::i(v) => {
                    my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.foobar {
            match v {
                &TestExposeOneof_oneof_foobar::s(ref v) => {
                    os.write_string(1, v)?;
                },
                &TestExposeOneof_oneof_foobar::i(v) => {
                    os.write_int32(2, v)?;
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

impl ::protobuf::MessageStatic for TestExposeOneof {
    fn new() -> TestExposeOneof {
        TestExposeOneof::new()
    }

    fn descriptor_static(_: ::std::option::Option<TestExposeOneof>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "s",
                    TestExposeOneof::has_s,
                    TestExposeOneof::get_s,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor::<_>(
                    "i",
                    TestExposeOneof::has_i,
                    TestExposeOneof::get_i,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TestExposeOneof>(
                    "TestExposeOneof",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TestExposeOneof {
    fn clear(&mut self) {
        self.clear_s();
        self.clear_i();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TestExposeOneof {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TestExposeOneof {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x1a, 0x74, 0x65, 0x73, 0x74, 0x5f, 0x65, 0x78, 0x70, 0x6f, 0x73, 0x65, 0x5f, 0x6f, 0x6e,
    0x65, 0x6f, 0x66, 0x5f, 0x70, 0x62, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0f, 0x72, 0x75,
    0x73, 0x74, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x3b, 0x0a,
    0x0f, 0x54, 0x65, 0x73, 0x74, 0x45, 0x78, 0x70, 0x6f, 0x73, 0x65, 0x4f, 0x6e, 0x65, 0x6f, 0x66,
    0x12, 0x0e, 0x0a, 0x01, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x01, 0x73,
    0x12, 0x0e, 0x0a, 0x01, 0x69, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x48, 0x00, 0x52, 0x01, 0x69,
    0x42, 0x08, 0x0a, 0x06, 0x66, 0x6f, 0x6f, 0x62, 0x61, 0x72, 0x42, 0x04, 0xc8, 0xa6, 0x08, 0x00,
    0x4a, 0x87, 0x02, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x0a, 0x01, 0x0a, 0x09, 0x0a, 0x02, 0x03,
    0x00, 0x12, 0x03, 0x00, 0x07, 0x18, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x03, 0x00, 0x28,
    0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x03, 0x00, 0x28, 0x0a, 0x0c, 0x0a,
    0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x03, 0x07, 0x1f, 0x0a, 0x0d, 0x0a, 0x06, 0x08,
    0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x03, 0x07, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x03, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7,
    0x07, 0x00, 0x03, 0x12, 0x03, 0x03, 0x22, 0x27, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04,
    0x05, 0x00, 0x0a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x05, 0x08, 0x17,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x08, 0x00, 0x12, 0x04, 0x06, 0x04, 0x09, 0x05, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x08, 0x00, 0x01, 0x12, 0x03, 0x06, 0x0a, 0x10, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x07, 0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x07, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x07, 0x0f, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x07, 0x13, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x08, 0x08, 0x14,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x08, 0x08, 0x0d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x08, 0x0e, 0x0f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x08, 0x12, 0x13,
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
