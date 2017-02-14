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
pub struct TestMap {
    // message fields
    m: ::std::collections::HashMap<::std::string::String, u32>,
    mm: ::std::collections::HashMap<::std::string::String, TestMapEntry>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TestMap {}

impl TestMap {
    pub fn new() -> TestMap {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TestMap {
        static mut instance: ::protobuf::lazy::Lazy<TestMap> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TestMap,
        };
        unsafe {
            instance.get(TestMap::new)
        }
    }

    // repeated .TestMap.MEntry m = 1;

    pub fn clear_m(&mut self) {
        self.m.clear();
    }

    // Param is passed by value, moved
    pub fn set_m(&mut self, v: ::std::collections::HashMap<::std::string::String, u32>) {
        self.m = v;
    }

    // Mutable pointer to the field.
    pub fn mut_m(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, u32> {
        &mut self.m
    }

    // Take field
    pub fn take_m(&mut self) -> ::std::collections::HashMap<::std::string::String, u32> {
        ::std::mem::replace(&mut self.m, ::std::collections::HashMap::new())
    }

    pub fn get_m(&self) -> &::std::collections::HashMap<::std::string::String, u32> {
        &self.m
    }

    fn get_m_for_reflect(&self) -> &::std::collections::HashMap<::std::string::String, u32> {
        &self.m
    }

    fn mut_m_for_reflect(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, u32> {
        &mut self.m
    }

    // repeated .TestMap.MmEntry mm = 2;

    pub fn clear_mm(&mut self) {
        self.mm.clear();
    }

    // Param is passed by value, moved
    pub fn set_mm(&mut self, v: ::std::collections::HashMap<::std::string::String, TestMapEntry>) {
        self.mm = v;
    }

    // Mutable pointer to the field.
    pub fn mut_mm(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, TestMapEntry> {
        &mut self.mm
    }

    // Take field
    pub fn take_mm(&mut self) -> ::std::collections::HashMap<::std::string::String, TestMapEntry> {
        ::std::mem::replace(&mut self.mm, ::std::collections::HashMap::new())
    }

    pub fn get_mm(&self) -> &::std::collections::HashMap<::std::string::String, TestMapEntry> {
        &self.mm
    }

    fn get_mm_for_reflect(&self) -> &::std::collections::HashMap<::std::string::String, TestMapEntry> {
        &self.mm
    }

    fn mut_mm_for_reflect(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, TestMapEntry> {
        &mut self.mm
    }
}

impl ::protobuf::Message for TestMap {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeUint32>(wire_type, is, &mut self.m)?;
                },
                2 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<TestMapEntry>>(wire_type, is, &mut self.mm)?;
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
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeUint32>(1, &self.m);
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<TestMapEntry>>(2, &self.mm);
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeUint32>(1, &self.m, os)?;
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<TestMapEntry>>(2, &self.mm, os)?;
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

impl ::protobuf::MessageStatic for TestMap {
    fn new() -> TestMap {
        TestMap::new()
    }

    fn descriptor_static(_: ::std::option::Option<TestMap>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeUint32>(
                    "m",
                    TestMap::get_m_for_reflect,
                    TestMap::mut_m_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<TestMapEntry>>(
                    "mm",
                    TestMap::get_mm_for_reflect,
                    TestMap::mut_mm_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TestMap>(
                    "TestMap",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TestMap {
    fn clear(&mut self) {
        self.clear_m();
        self.clear_mm();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TestMap {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TestMap {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TestMapEntry {
    // message fields
    pub v: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TestMapEntry {}

impl TestMapEntry {
    pub fn new() -> TestMapEntry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TestMapEntry {
        static mut instance: ::protobuf::lazy::Lazy<TestMapEntry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TestMapEntry,
        };
        unsafe {
            instance.get(TestMapEntry::new)
        }
    }

    // int64 v = 1;

    pub fn clear_v(&mut self) {
        self.v = 0;
    }

    // Param is passed by value, moved
    pub fn set_v(&mut self, v: i64) {
        self.v = v;
    }

    pub fn get_v(&self) -> i64 {
        self.v
    }

    fn get_v_for_reflect(&self) -> &i64 {
        &self.v
    }

    fn mut_v_for_reflect(&mut self) -> &mut i64 {
        &mut self.v
    }
}

impl ::protobuf::Message for TestMapEntry {
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
                    let tmp = is.read_int64()?;
                    self.v = tmp;
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
        if self.v != 0 {
            my_size += ::protobuf::rt::value_size(1, self.v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.v != 0 {
            os.write_int64(1, self.v)?;
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

impl ::protobuf::MessageStatic for TestMapEntry {
    fn new() -> TestMapEntry {
        TestMapEntry::new()
    }

    fn descriptor_static(_: ::std::option::Option<TestMapEntry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "v",
                    TestMapEntry::get_v_for_reflect,
                    TestMapEntry::mut_v_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TestMapEntry>(
                    "TestMapEntry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TestMapEntry {
    fn clear(&mut self) {
        self.clear_v();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TestMapEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TestMapEntry {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x11, 0x74, 0x65, 0x73, 0x74, 0x5f, 0x6d, 0x61, 0x70, 0x5f, 0x70, 0x62, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x22, 0xc6, 0x01, 0x0a, 0x07, 0x54, 0x65, 0x73, 0x74, 0x4d, 0x61, 0x70, 0x12,
    0x1d, 0x0a, 0x01, 0x6d, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x54, 0x65, 0x73,
    0x74, 0x4d, 0x61, 0x70, 0x2e, 0x4d, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x52, 0x01, 0x6d, 0x12, 0x20,
    0x0a, 0x02, 0x6d, 0x6d, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x54, 0x65, 0x73,
    0x74, 0x4d, 0x61, 0x70, 0x2e, 0x4d, 0x6d, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x52, 0x02, 0x6d, 0x6d,
    0x1a, 0x34, 0x0a, 0x06, 0x4d, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65,
    0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x12, 0x14, 0x0a, 0x05,
    0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x05, 0x76, 0x61, 0x6c,
    0x75, 0x65, 0x3a, 0x02, 0x38, 0x01, 0x1a, 0x44, 0x0a, 0x07, 0x4d, 0x6d, 0x45, 0x6e, 0x74, 0x72,
    0x79, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03,
    0x6b, 0x65, 0x79, 0x12, 0x23, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x54, 0x65, 0x73, 0x74, 0x4d, 0x61, 0x70, 0x45, 0x6e, 0x74, 0x72,
    0x79, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x3a, 0x02, 0x38, 0x01, 0x22, 0x1c, 0x0a, 0x0c,
    0x54, 0x65, 0x73, 0x74, 0x4d, 0x61, 0x70, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x12, 0x0c, 0x0a, 0x01,
    0x76, 0x18, 0x01, 0x20, 0x01, 0x28, 0x03, 0x52, 0x01, 0x76, 0x4a, 0x94, 0x02, 0x0a, 0x06, 0x12,
    0x04, 0x00, 0x00, 0x09, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x02, 0x00, 0x05, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x00, 0x01, 0x12, 0x03, 0x02, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x03, 0x04, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x03,
    0x04, 0x02, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x03, 0x04,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x03, 0x18, 0x19, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x03, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x04, 0x04, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x04, 0x12, 0x04, 0x04, 0x04, 0x03, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x06, 0x12, 0x03, 0x04, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x04, 0x1e, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x04, 0x23, 0x24, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x07, 0x00, 0x09, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x07, 0x08, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x00, 0x12, 0x03, 0x08, 0x02, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x04, 0x12, 0x04, 0x08, 0x02, 0x07, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x08, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x08, 0x08, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x08, 0x0c,
    0x0d, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
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
