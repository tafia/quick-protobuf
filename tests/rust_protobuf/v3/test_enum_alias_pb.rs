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
pub struct TestEnumWithAlias {
    // message fields
    pub en: EnumWithAlias,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TestEnumWithAlias {}

impl TestEnumWithAlias {
    pub fn new() -> TestEnumWithAlias {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TestEnumWithAlias {
        static mut instance: ::protobuf::lazy::Lazy<TestEnumWithAlias> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TestEnumWithAlias,
        };
        unsafe {
            instance.get(TestEnumWithAlias::new)
        }
    }

    // .test_enum_alias.EnumWithAlias en = 1;

    pub fn clear_en(&mut self) {
        self.en = EnumWithAlias::UNKNOWN;
    }

    // Param is passed by value, moved
    pub fn set_en(&mut self, v: EnumWithAlias) {
        self.en = v;
    }

    pub fn get_en(&self) -> EnumWithAlias {
        self.en
    }

    fn get_en_for_reflect(&self) -> &EnumWithAlias {
        &self.en
    }

    fn mut_en_for_reflect(&mut self) -> &mut EnumWithAlias {
        &mut self.en
    }
}

impl ::protobuf::Message for TestEnumWithAlias {
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
                    let tmp = is.read_enum()?;
                    self.en = tmp;
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
        if self.en != EnumWithAlias::UNKNOWN {
            my_size += ::protobuf::rt::enum_size(1, self.en);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.en != EnumWithAlias::UNKNOWN {
            os.write_enum(1, self.en.value())?;
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

impl ::protobuf::MessageStatic for TestEnumWithAlias {
    fn new() -> TestEnumWithAlias {
        TestEnumWithAlias::new()
    }

    fn descriptor_static(_: ::std::option::Option<TestEnumWithAlias>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<EnumWithAlias>>(
                    "en",
                    TestEnumWithAlias::get_en_for_reflect,
                    TestEnumWithAlias::mut_en_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TestEnumWithAlias>(
                    "TestEnumWithAlias",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TestEnumWithAlias {
    fn clear(&mut self) {
        self.clear_en();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TestEnumWithAlias {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TestEnumWithAlias {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

// Note: you cannot use pattern matching for enums with allow_alias option
#[derive(Clone,Eq,Debug)]
pub enum EnumWithAlias {
    UNKNOWN, // 0
    A, // 10
    B, // 20
    A_AGAIN, // 10
}

impl ::std::cmp::PartialEq for EnumWithAlias {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl ::std::hash::Hash for EnumWithAlias {
    fn hash<H : ::std::hash::Hasher>(&self, state: &mut H) {
        state.write_i32(self.value())
    }
}

impl ::protobuf::ProtobufEnum for EnumWithAlias {
    fn value(&self) -> i32 {
        match *self {
            EnumWithAlias::UNKNOWN => 0,
            EnumWithAlias::A => 10,
            EnumWithAlias::B => 20,
            EnumWithAlias::A_AGAIN => 10,
        }
    }

    fn from_i32(value: i32) -> ::std::option::Option<EnumWithAlias> {
        match value {
            0 => ::std::option::Option::Some(EnumWithAlias::UNKNOWN),
            10 => ::std::option::Option::Some(EnumWithAlias::A),
            20 => ::std::option::Option::Some(EnumWithAlias::B),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EnumWithAlias] = &[
            EnumWithAlias::UNKNOWN,
            EnumWithAlias::A,
            EnumWithAlias::B,
            EnumWithAlias::A_AGAIN,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<EnumWithAlias>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EnumWithAlias", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EnumWithAlias {
}

impl ::std::default::Default for EnumWithAlias {
    fn default() -> Self {
        EnumWithAlias::UNKNOWN
    }
}

impl ::protobuf::reflect::ProtobufValue for EnumWithAlias {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x18, 0x74, 0x65, 0x73, 0x74, 0x5f, 0x65, 0x6e, 0x75, 0x6d, 0x5f, 0x61, 0x6c, 0x69, 0x61,
    0x73, 0x5f, 0x70, 0x62, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0f, 0x74, 0x65, 0x73, 0x74,
    0x5f, 0x65, 0x6e, 0x75, 0x6d, 0x5f, 0x61, 0x6c, 0x69, 0x61, 0x73, 0x22, 0x43, 0x0a, 0x11, 0x54,
    0x65, 0x73, 0x74, 0x45, 0x6e, 0x75, 0x6d, 0x57, 0x69, 0x74, 0x68, 0x41, 0x6c, 0x69, 0x61, 0x73,
    0x12, 0x2e, 0x0a, 0x02, 0x65, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1e, 0x2e, 0x74,
    0x65, 0x73, 0x74, 0x5f, 0x65, 0x6e, 0x75, 0x6d, 0x5f, 0x61, 0x6c, 0x69, 0x61, 0x73, 0x2e, 0x45,
    0x6e, 0x75, 0x6d, 0x57, 0x69, 0x74, 0x68, 0x41, 0x6c, 0x69, 0x61, 0x73, 0x52, 0x02, 0x65, 0x6e,
    0x2a, 0x3b, 0x0a, 0x0d, 0x45, 0x6e, 0x75, 0x6d, 0x57, 0x69, 0x74, 0x68, 0x41, 0x6c, 0x69, 0x61,
    0x73, 0x12, 0x0b, 0x0a, 0x07, 0x55, 0x4e, 0x4b, 0x4e, 0x4f, 0x57, 0x4e, 0x10, 0x00, 0x12, 0x05,
    0x0a, 0x01, 0x41, 0x10, 0x0a, 0x12, 0x05, 0x0a, 0x01, 0x42, 0x10, 0x14, 0x12, 0x0b, 0x0a, 0x07,
    0x41, 0x5f, 0x41, 0x47, 0x41, 0x49, 0x4e, 0x10, 0x0a, 0x1a, 0x02, 0x10, 0x01, 0x4a, 0x94, 0x03,
    0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x0e, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00,
    0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x08, 0x17, 0x0a, 0x0a, 0x0a, 0x02,
    0x05, 0x00, 0x12, 0x04, 0x04, 0x00, 0x0a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12,
    0x03, 0x04, 0x05, 0x12, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x03, 0x12, 0x03, 0x05, 0x04, 0x1e,
    0x0a, 0x0d, 0x0a, 0x06, 0x05, 0x00, 0x03, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x05, 0x04, 0x1e, 0x0a,
    0x0e, 0x0a, 0x07, 0x05, 0x00, 0x03, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x05, 0x0b, 0x16, 0x0a,
    0x0f, 0x0a, 0x08, 0x05, 0x00, 0x03, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x05, 0x0b, 0x16,
    0x0a, 0x10, 0x0a, 0x09, 0x05, 0x00, 0x03, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x05,
    0x0b, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x05, 0x00, 0x03, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x05,
    0x19, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x06, 0x04, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x06, 0x04, 0x0b, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x06, 0x0e, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x00, 0x02, 0x01, 0x12, 0x03, 0x07, 0x04, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x07, 0x04, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12,
    0x03, 0x07, 0x08, 0x0a, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x02, 0x12, 0x03, 0x08, 0x04,
    0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x08, 0x04, 0x05, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x08, 0x08, 0x0a, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x00, 0x02, 0x03, 0x12, 0x03, 0x09, 0x04, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x09, 0x04, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03,
    0x02, 0x12, 0x03, 0x09, 0x0e, 0x10, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x0c, 0x00,
    0x0e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x0c, 0x08, 0x19, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0d, 0x04, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x0d, 0x04, 0x0c, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x06, 0x12, 0x03, 0x0d, 0x04, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x0d, 0x12, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x0d, 0x17, 0x18, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
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
