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
pub struct Test1 {
    // message fields
    value: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Test1 {}

impl Test1 {
    pub fn new() -> Test1 {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Test1 {
        static mut instance: ::protobuf::lazy::Lazy<Test1> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Test1,
        };
        unsafe {
            instance.get(Test1::new)
        }
    }

    // optional int32 value = 1;

    pub fn clear_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: i32) {
        self.value = ::std::option::Option::Some(v);
    }

    pub fn get_value(&self) -> i32 {
        self.value.unwrap_or(0)
    }

    fn get_value_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.value
    }
}

impl ::protobuf::Message for Test1 {
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
                    }
                    let tmp = is.read_int32()?;
                    self.value = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.value {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.value {
            os.write_int32(1, v)?;
        }
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

impl ::protobuf::MessageStatic for Test1 {
    fn new() -> Test1 {
        Test1::new()
    }

    fn descriptor_static(_: ::std::option::Option<Test1>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "value",
                    Test1::get_value_for_reflect,
                    Test1::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Test1>(
                    "Test1",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Test1 {
    fn clear(&mut self) {
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Test1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Test1 {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TestRepeatedBool {
    // message fields
    values: ::std::vec::Vec<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TestRepeatedBool {}

impl TestRepeatedBool {
    pub fn new() -> TestRepeatedBool {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TestRepeatedBool {
        static mut instance: ::protobuf::lazy::Lazy<TestRepeatedBool> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TestRepeatedBool,
        };
        unsafe {
            instance.get(TestRepeatedBool::new)
        }
    }

    // repeated bool values = 1;

    pub fn clear_values(&mut self) {
        self.values.clear();
    }

    // Param is passed by value, moved
    pub fn set_values(&mut self, v: ::std::vec::Vec<bool>) {
        self.values = v;
    }

    // Mutable pointer to the field.
    pub fn mut_values(&mut self) -> &mut ::std::vec::Vec<bool> {
        &mut self.values
    }

    // Take field
    pub fn take_values(&mut self) -> ::std::vec::Vec<bool> {
        ::std::mem::replace(&mut self.values, ::std::vec::Vec::new())
    }

    pub fn get_values(&self) -> &[bool] {
        &self.values
    }

    fn get_values_for_reflect(&self) -> &::std::vec::Vec<bool> {
        &self.values
    }

    fn mut_values_for_reflect(&mut self) -> &mut ::std::vec::Vec<bool> {
        &mut self.values
    }
}

impl ::protobuf::Message for TestRepeatedBool {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_bool_into(wire_type, is, &mut self.values)?;
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
        my_size += 2 * self.values.len() as u32;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.values {
            os.write_bool(1, *v)?;
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

impl ::protobuf::MessageStatic for TestRepeatedBool {
    fn new() -> TestRepeatedBool {
        TestRepeatedBool::new()
    }

    fn descriptor_static(_: ::std::option::Option<TestRepeatedBool>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "values",
                    TestRepeatedBool::get_values_for_reflect,
                    TestRepeatedBool::mut_values_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TestRepeatedBool>(
                    "TestRepeatedBool",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TestRepeatedBool {
    fn clear(&mut self) {
        self.clear_values();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TestRepeatedBool {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TestRepeatedBool {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TestRepeatedPackedInt32 {
    // message fields
    values: ::std::vec::Vec<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TestRepeatedPackedInt32 {}

impl TestRepeatedPackedInt32 {
    pub fn new() -> TestRepeatedPackedInt32 {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TestRepeatedPackedInt32 {
        static mut instance: ::protobuf::lazy::Lazy<TestRepeatedPackedInt32> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TestRepeatedPackedInt32,
        };
        unsafe {
            instance.get(TestRepeatedPackedInt32::new)
        }
    }

    // repeated int32 values = 1;

    pub fn clear_values(&mut self) {
        self.values.clear();
    }

    // Param is passed by value, moved
    pub fn set_values(&mut self, v: ::std::vec::Vec<i32>) {
        self.values = v;
    }

    // Mutable pointer to the field.
    pub fn mut_values(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.values
    }

    // Take field
    pub fn take_values(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.values, ::std::vec::Vec::new())
    }

    pub fn get_values(&self) -> &[i32] {
        &self.values
    }

    fn get_values_for_reflect(&self) -> &::std::vec::Vec<i32> {
        &self.values
    }

    fn mut_values_for_reflect(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.values
    }
}

impl ::protobuf::Message for TestRepeatedPackedInt32 {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.values)?;
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
        if !self.values.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(1, &self.values);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.values.is_empty() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.values))?;
            for v in &self.values {
                os.write_int32_no_tag(*v)?;
            };
        }
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

impl ::protobuf::MessageStatic for TestRepeatedPackedInt32 {
    fn new() -> TestRepeatedPackedInt32 {
        TestRepeatedPackedInt32::new()
    }

    fn descriptor_static(_: ::std::option::Option<TestRepeatedPackedInt32>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "values",
                    TestRepeatedPackedInt32::get_values_for_reflect,
                    TestRepeatedPackedInt32::mut_values_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TestRepeatedPackedInt32>(
                    "TestRepeatedPackedInt32",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TestRepeatedPackedInt32 {
    fn clear(&mut self) {
        self.clear_values();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TestRepeatedPackedInt32 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TestRepeatedPackedInt32 {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TestRepeatedMessages {
    // message fields
    messages1: ::protobuf::RepeatedField<TestRepeatedMessages>,
    messages2: ::protobuf::RepeatedField<TestRepeatedMessages>,
    messages3: ::protobuf::RepeatedField<TestRepeatedMessages>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TestRepeatedMessages {}

impl TestRepeatedMessages {
    pub fn new() -> TestRepeatedMessages {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TestRepeatedMessages {
        static mut instance: ::protobuf::lazy::Lazy<TestRepeatedMessages> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TestRepeatedMessages,
        };
        unsafe {
            instance.get(TestRepeatedMessages::new)
        }
    }

    // repeated .perftest_data.TestRepeatedMessages messages1 = 1;

    pub fn clear_messages1(&mut self) {
        self.messages1.clear();
    }

    // Param is passed by value, moved
    pub fn set_messages1(&mut self, v: ::protobuf::RepeatedField<TestRepeatedMessages>) {
        self.messages1 = v;
    }

    // Mutable pointer to the field.
    pub fn mut_messages1(&mut self) -> &mut ::protobuf::RepeatedField<TestRepeatedMessages> {
        &mut self.messages1
    }

    // Take field
    pub fn take_messages1(&mut self) -> ::protobuf::RepeatedField<TestRepeatedMessages> {
        ::std::mem::replace(&mut self.messages1, ::protobuf::RepeatedField::new())
    }

    pub fn get_messages1(&self) -> &[TestRepeatedMessages] {
        &self.messages1
    }

    fn get_messages1_for_reflect(&self) -> &::protobuf::RepeatedField<TestRepeatedMessages> {
        &self.messages1
    }

    fn mut_messages1_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<TestRepeatedMessages> {
        &mut self.messages1
    }

    // repeated .perftest_data.TestRepeatedMessages messages2 = 2;

    pub fn clear_messages2(&mut self) {
        self.messages2.clear();
    }

    // Param is passed by value, moved
    pub fn set_messages2(&mut self, v: ::protobuf::RepeatedField<TestRepeatedMessages>) {
        self.messages2 = v;
    }

    // Mutable pointer to the field.
    pub fn mut_messages2(&mut self) -> &mut ::protobuf::RepeatedField<TestRepeatedMessages> {
        &mut self.messages2
    }

    // Take field
    pub fn take_messages2(&mut self) -> ::protobuf::RepeatedField<TestRepeatedMessages> {
        ::std::mem::replace(&mut self.messages2, ::protobuf::RepeatedField::new())
    }

    pub fn get_messages2(&self) -> &[TestRepeatedMessages] {
        &self.messages2
    }

    fn get_messages2_for_reflect(&self) -> &::protobuf::RepeatedField<TestRepeatedMessages> {
        &self.messages2
    }

    fn mut_messages2_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<TestRepeatedMessages> {
        &mut self.messages2
    }

    // repeated .perftest_data.TestRepeatedMessages messages3 = 3;

    pub fn clear_messages3(&mut self) {
        self.messages3.clear();
    }

    // Param is passed by value, moved
    pub fn set_messages3(&mut self, v: ::protobuf::RepeatedField<TestRepeatedMessages>) {
        self.messages3 = v;
    }

    // Mutable pointer to the field.
    pub fn mut_messages3(&mut self) -> &mut ::protobuf::RepeatedField<TestRepeatedMessages> {
        &mut self.messages3
    }

    // Take field
    pub fn take_messages3(&mut self) -> ::protobuf::RepeatedField<TestRepeatedMessages> {
        ::std::mem::replace(&mut self.messages3, ::protobuf::RepeatedField::new())
    }

    pub fn get_messages3(&self) -> &[TestRepeatedMessages] {
        &self.messages3
    }

    fn get_messages3_for_reflect(&self) -> &::protobuf::RepeatedField<TestRepeatedMessages> {
        &self.messages3
    }

    fn mut_messages3_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<TestRepeatedMessages> {
        &mut self.messages3
    }
}

impl ::protobuf::Message for TestRepeatedMessages {
    fn is_initialized(&self) -> bool {
        for v in &self.messages1 {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.messages2 {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.messages3 {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.messages1)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.messages2)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.messages3)?;
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
        for value in &self.messages1 {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.messages2 {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.messages3 {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.messages1 {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.messages2 {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.messages3 {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for TestRepeatedMessages {
    fn new() -> TestRepeatedMessages {
        TestRepeatedMessages::new()
    }

    fn descriptor_static(_: ::std::option::Option<TestRepeatedMessages>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TestRepeatedMessages>>(
                    "messages1",
                    TestRepeatedMessages::get_messages1_for_reflect,
                    TestRepeatedMessages::mut_messages1_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TestRepeatedMessages>>(
                    "messages2",
                    TestRepeatedMessages::get_messages2_for_reflect,
                    TestRepeatedMessages::mut_messages2_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TestRepeatedMessages>>(
                    "messages3",
                    TestRepeatedMessages::get_messages3_for_reflect,
                    TestRepeatedMessages::mut_messages3_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TestRepeatedMessages>(
                    "TestRepeatedMessages",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TestRepeatedMessages {
    fn clear(&mut self) {
        self.clear_messages1();
        self.clear_messages2();
        self.clear_messages3();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TestRepeatedMessages {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TestRepeatedMessages {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TestOptionalMessages {
    // message fields
    message1: ::protobuf::SingularPtrField<TestOptionalMessages>,
    message2: ::protobuf::SingularPtrField<TestOptionalMessages>,
    message3: ::protobuf::SingularPtrField<TestOptionalMessages>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TestOptionalMessages {}

impl TestOptionalMessages {
    pub fn new() -> TestOptionalMessages {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TestOptionalMessages {
        static mut instance: ::protobuf::lazy::Lazy<TestOptionalMessages> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TestOptionalMessages,
        };
        unsafe {
            instance.get(TestOptionalMessages::new)
        }
    }

    // optional .perftest_data.TestOptionalMessages message1 = 1;

    pub fn clear_message1(&mut self) {
        self.message1.clear();
    }

    pub fn has_message1(&self) -> bool {
        self.message1.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message1(&mut self, v: TestOptionalMessages) {
        self.message1 = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message1(&mut self) -> &mut TestOptionalMessages {
        if self.message1.is_none() {
            self.message1.set_default();
        }
        self.message1.as_mut().unwrap()
    }

    // Take field
    pub fn take_message1(&mut self) -> TestOptionalMessages {
        self.message1.take().unwrap_or_else(|| TestOptionalMessages::new())
    }

    pub fn get_message1(&self) -> &TestOptionalMessages {
        self.message1.as_ref().unwrap_or_else(|| TestOptionalMessages::default_instance())
    }

    fn get_message1_for_reflect(&self) -> &::protobuf::SingularPtrField<TestOptionalMessages> {
        &self.message1
    }

    fn mut_message1_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<TestOptionalMessages> {
        &mut self.message1
    }

    // optional .perftest_data.TestOptionalMessages message2 = 2;

    pub fn clear_message2(&mut self) {
        self.message2.clear();
    }

    pub fn has_message2(&self) -> bool {
        self.message2.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message2(&mut self, v: TestOptionalMessages) {
        self.message2 = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message2(&mut self) -> &mut TestOptionalMessages {
        if self.message2.is_none() {
            self.message2.set_default();
        }
        self.message2.as_mut().unwrap()
    }

    // Take field
    pub fn take_message2(&mut self) -> TestOptionalMessages {
        self.message2.take().unwrap_or_else(|| TestOptionalMessages::new())
    }

    pub fn get_message2(&self) -> &TestOptionalMessages {
        self.message2.as_ref().unwrap_or_else(|| TestOptionalMessages::default_instance())
    }

    fn get_message2_for_reflect(&self) -> &::protobuf::SingularPtrField<TestOptionalMessages> {
        &self.message2
    }

    fn mut_message2_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<TestOptionalMessages> {
        &mut self.message2
    }

    // optional .perftest_data.TestOptionalMessages message3 = 3;

    pub fn clear_message3(&mut self) {
        self.message3.clear();
    }

    pub fn has_message3(&self) -> bool {
        self.message3.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message3(&mut self, v: TestOptionalMessages) {
        self.message3 = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message3(&mut self) -> &mut TestOptionalMessages {
        if self.message3.is_none() {
            self.message3.set_default();
        }
        self.message3.as_mut().unwrap()
    }

    // Take field
    pub fn take_message3(&mut self) -> TestOptionalMessages {
        self.message3.take().unwrap_or_else(|| TestOptionalMessages::new())
    }

    pub fn get_message3(&self) -> &TestOptionalMessages {
        self.message3.as_ref().unwrap_or_else(|| TestOptionalMessages::default_instance())
    }

    fn get_message3_for_reflect(&self) -> &::protobuf::SingularPtrField<TestOptionalMessages> {
        &self.message3
    }

    fn mut_message3_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<TestOptionalMessages> {
        &mut self.message3
    }
}

impl ::protobuf::Message for TestOptionalMessages {
    fn is_initialized(&self) -> bool {
        for v in &self.message1 {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.message2 {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.message3 {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.message1)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.message2)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.message3)?;
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
        if let Some(ref v) = self.message1.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.message2.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.message3.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.message1.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.message2.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.message3.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
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

impl ::protobuf::MessageStatic for TestOptionalMessages {
    fn new() -> TestOptionalMessages {
        TestOptionalMessages::new()
    }

    fn descriptor_static(_: ::std::option::Option<TestOptionalMessages>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TestOptionalMessages>>(
                    "message1",
                    TestOptionalMessages::get_message1_for_reflect,
                    TestOptionalMessages::mut_message1_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TestOptionalMessages>>(
                    "message2",
                    TestOptionalMessages::get_message2_for_reflect,
                    TestOptionalMessages::mut_message2_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TestOptionalMessages>>(
                    "message3",
                    TestOptionalMessages::get_message3_for_reflect,
                    TestOptionalMessages::mut_message3_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TestOptionalMessages>(
                    "TestOptionalMessages",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TestOptionalMessages {
    fn clear(&mut self) {
        self.clear_message1();
        self.clear_message2();
        self.clear_message3();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TestOptionalMessages {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TestOptionalMessages {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TestStrings {
    // message fields
    s1: ::protobuf::SingularField<::std::string::String>,
    s2: ::protobuf::SingularField<::std::string::String>,
    s3: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TestStrings {}

impl TestStrings {
    pub fn new() -> TestStrings {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TestStrings {
        static mut instance: ::protobuf::lazy::Lazy<TestStrings> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TestStrings,
        };
        unsafe {
            instance.get(TestStrings::new)
        }
    }

    // optional string s1 = 1;

    pub fn clear_s1(&mut self) {
        self.s1.clear();
    }

    pub fn has_s1(&self) -> bool {
        self.s1.is_some()
    }

    // Param is passed by value, moved
    pub fn set_s1(&mut self, v: ::std::string::String) {
        self.s1 = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_s1(&mut self) -> &mut ::std::string::String {
        if self.s1.is_none() {
            self.s1.set_default();
        }
        self.s1.as_mut().unwrap()
    }

    // Take field
    pub fn take_s1(&mut self) -> ::std::string::String {
        self.s1.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_s1(&self) -> &str {
        match self.s1.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_s1_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.s1
    }

    fn mut_s1_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.s1
    }

    // optional string s2 = 2;

    pub fn clear_s2(&mut self) {
        self.s2.clear();
    }

    pub fn has_s2(&self) -> bool {
        self.s2.is_some()
    }

    // Param is passed by value, moved
    pub fn set_s2(&mut self, v: ::std::string::String) {
        self.s2 = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_s2(&mut self) -> &mut ::std::string::String {
        if self.s2.is_none() {
            self.s2.set_default();
        }
        self.s2.as_mut().unwrap()
    }

    // Take field
    pub fn take_s2(&mut self) -> ::std::string::String {
        self.s2.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_s2(&self) -> &str {
        match self.s2.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_s2_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.s2
    }

    fn mut_s2_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.s2
    }

    // optional string s3 = 3;

    pub fn clear_s3(&mut self) {
        self.s3.clear();
    }

    pub fn has_s3(&self) -> bool {
        self.s3.is_some()
    }

    // Param is passed by value, moved
    pub fn set_s3(&mut self, v: ::std::string::String) {
        self.s3 = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_s3(&mut self) -> &mut ::std::string::String {
        if self.s3.is_none() {
            self.s3.set_default();
        }
        self.s3.as_mut().unwrap()
    }

    // Take field
    pub fn take_s3(&mut self) -> ::std::string::String {
        self.s3.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_s3(&self) -> &str {
        match self.s3.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_s3_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.s3
    }

    fn mut_s3_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.s3
    }
}

impl ::protobuf::Message for TestStrings {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.s1)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.s2)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.s3)?;
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
        if let Some(ref v) = self.s1.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.s2.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.s3.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.s1.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.s2.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.s3.as_ref() {
            os.write_string(3, &v)?;
        }
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

impl ::protobuf::MessageStatic for TestStrings {
    fn new() -> TestStrings {
        TestStrings::new()
    }

    fn descriptor_static(_: ::std::option::Option<TestStrings>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "s1",
                    TestStrings::get_s1_for_reflect,
                    TestStrings::mut_s1_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "s2",
                    TestStrings::get_s2_for_reflect,
                    TestStrings::mut_s2_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "s3",
                    TestStrings::get_s3_for_reflect,
                    TestStrings::mut_s3_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TestStrings>(
                    "TestStrings",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TestStrings {
    fn clear(&mut self) {
        self.clear_s1();
        self.clear_s2();
        self.clear_s3();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TestStrings {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TestStrings {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TestBytes {
    // message fields
    b1: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TestBytes {}

impl TestBytes {
    pub fn new() -> TestBytes {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TestBytes {
        static mut instance: ::protobuf::lazy::Lazy<TestBytes> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TestBytes,
        };
        unsafe {
            instance.get(TestBytes::new)
        }
    }

    // optional bytes b1 = 1;

    pub fn clear_b1(&mut self) {
        self.b1.clear();
    }

    pub fn has_b1(&self) -> bool {
        self.b1.is_some()
    }

    // Param is passed by value, moved
    pub fn set_b1(&mut self, v: ::std::vec::Vec<u8>) {
        self.b1 = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_b1(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.b1.is_none() {
            self.b1.set_default();
        }
        self.b1.as_mut().unwrap()
    }

    // Take field
    pub fn take_b1(&mut self) -> ::std::vec::Vec<u8> {
        self.b1.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_b1(&self) -> &[u8] {
        match self.b1.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_b1_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.b1
    }

    fn mut_b1_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.b1
    }
}

impl ::protobuf::Message for TestBytes {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.b1)?;
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
        if let Some(ref v) = self.b1.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.b1.as_ref() {
            os.write_bytes(1, &v)?;
        }
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

impl ::protobuf::MessageStatic for TestBytes {
    fn new() -> TestBytes {
        TestBytes::new()
    }

    fn descriptor_static(_: ::std::option::Option<TestBytes>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "b1",
                    TestBytes::get_b1_for_reflect,
                    TestBytes::mut_b1_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TestBytes>(
                    "TestBytes",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TestBytes {
    fn clear(&mut self) {
        self.clear_b1();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TestBytes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TestBytes {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PerftestData {
    // message fields
    test1: ::protobuf::RepeatedField<Test1>,
    test_repeated_bool: ::protobuf::RepeatedField<TestRepeatedBool>,
    test_repeated_messages: ::protobuf::RepeatedField<TestRepeatedMessages>,
    test_optional_messages: ::protobuf::RepeatedField<TestOptionalMessages>,
    test_strings: ::protobuf::RepeatedField<TestStrings>,
    test_repeated_packed_int32: ::protobuf::RepeatedField<TestRepeatedPackedInt32>,
    test_small_bytearrays: ::protobuf::RepeatedField<TestBytes>,
    test_large_bytearrays: ::protobuf::RepeatedField<TestBytes>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PerftestData {}

impl PerftestData {
    pub fn new() -> PerftestData {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PerftestData {
        static mut instance: ::protobuf::lazy::Lazy<PerftestData> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PerftestData,
        };
        unsafe {
            instance.get(PerftestData::new)
        }
    }

    // repeated .perftest_data.Test1 test1 = 1;

    pub fn clear_test1(&mut self) {
        self.test1.clear();
    }

    // Param is passed by value, moved
    pub fn set_test1(&mut self, v: ::protobuf::RepeatedField<Test1>) {
        self.test1 = v;
    }

    // Mutable pointer to the field.
    pub fn mut_test1(&mut self) -> &mut ::protobuf::RepeatedField<Test1> {
        &mut self.test1
    }

    // Take field
    pub fn take_test1(&mut self) -> ::protobuf::RepeatedField<Test1> {
        ::std::mem::replace(&mut self.test1, ::protobuf::RepeatedField::new())
    }

    pub fn get_test1(&self) -> &[Test1] {
        &self.test1
    }

    fn get_test1_for_reflect(&self) -> &::protobuf::RepeatedField<Test1> {
        &self.test1
    }

    fn mut_test1_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Test1> {
        &mut self.test1
    }

    // repeated .perftest_data.TestRepeatedBool test_repeated_bool = 2;

    pub fn clear_test_repeated_bool(&mut self) {
        self.test_repeated_bool.clear();
    }

    // Param is passed by value, moved
    pub fn set_test_repeated_bool(&mut self, v: ::protobuf::RepeatedField<TestRepeatedBool>) {
        self.test_repeated_bool = v;
    }

    // Mutable pointer to the field.
    pub fn mut_test_repeated_bool(&mut self) -> &mut ::protobuf::RepeatedField<TestRepeatedBool> {
        &mut self.test_repeated_bool
    }

    // Take field
    pub fn take_test_repeated_bool(&mut self) -> ::protobuf::RepeatedField<TestRepeatedBool> {
        ::std::mem::replace(&mut self.test_repeated_bool, ::protobuf::RepeatedField::new())
    }

    pub fn get_test_repeated_bool(&self) -> &[TestRepeatedBool] {
        &self.test_repeated_bool
    }

    fn get_test_repeated_bool_for_reflect(&self) -> &::protobuf::RepeatedField<TestRepeatedBool> {
        &self.test_repeated_bool
    }

    fn mut_test_repeated_bool_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<TestRepeatedBool> {
        &mut self.test_repeated_bool
    }

    // repeated .perftest_data.TestRepeatedMessages test_repeated_messages = 3;

    pub fn clear_test_repeated_messages(&mut self) {
        self.test_repeated_messages.clear();
    }

    // Param is passed by value, moved
    pub fn set_test_repeated_messages(&mut self, v: ::protobuf::RepeatedField<TestRepeatedMessages>) {
        self.test_repeated_messages = v;
    }

    // Mutable pointer to the field.
    pub fn mut_test_repeated_messages(&mut self) -> &mut ::protobuf::RepeatedField<TestRepeatedMessages> {
        &mut self.test_repeated_messages
    }

    // Take field
    pub fn take_test_repeated_messages(&mut self) -> ::protobuf::RepeatedField<TestRepeatedMessages> {
        ::std::mem::replace(&mut self.test_repeated_messages, ::protobuf::RepeatedField::new())
    }

    pub fn get_test_repeated_messages(&self) -> &[TestRepeatedMessages] {
        &self.test_repeated_messages
    }

    fn get_test_repeated_messages_for_reflect(&self) -> &::protobuf::RepeatedField<TestRepeatedMessages> {
        &self.test_repeated_messages
    }

    fn mut_test_repeated_messages_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<TestRepeatedMessages> {
        &mut self.test_repeated_messages
    }

    // repeated .perftest_data.TestOptionalMessages test_optional_messages = 4;

    pub fn clear_test_optional_messages(&mut self) {
        self.test_optional_messages.clear();
    }

    // Param is passed by value, moved
    pub fn set_test_optional_messages(&mut self, v: ::protobuf::RepeatedField<TestOptionalMessages>) {
        self.test_optional_messages = v;
    }

    // Mutable pointer to the field.
    pub fn mut_test_optional_messages(&mut self) -> &mut ::protobuf::RepeatedField<TestOptionalMessages> {
        &mut self.test_optional_messages
    }

    // Take field
    pub fn take_test_optional_messages(&mut self) -> ::protobuf::RepeatedField<TestOptionalMessages> {
        ::std::mem::replace(&mut self.test_optional_messages, ::protobuf::RepeatedField::new())
    }

    pub fn get_test_optional_messages(&self) -> &[TestOptionalMessages] {
        &self.test_optional_messages
    }

    fn get_test_optional_messages_for_reflect(&self) -> &::protobuf::RepeatedField<TestOptionalMessages> {
        &self.test_optional_messages
    }

    fn mut_test_optional_messages_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<TestOptionalMessages> {
        &mut self.test_optional_messages
    }

    // repeated .perftest_data.TestStrings test_strings = 5;

    pub fn clear_test_strings(&mut self) {
        self.test_strings.clear();
    }

    // Param is passed by value, moved
    pub fn set_test_strings(&mut self, v: ::protobuf::RepeatedField<TestStrings>) {
        self.test_strings = v;
    }

    // Mutable pointer to the field.
    pub fn mut_test_strings(&mut self) -> &mut ::protobuf::RepeatedField<TestStrings> {
        &mut self.test_strings
    }

    // Take field
    pub fn take_test_strings(&mut self) -> ::protobuf::RepeatedField<TestStrings> {
        ::std::mem::replace(&mut self.test_strings, ::protobuf::RepeatedField::new())
    }

    pub fn get_test_strings(&self) -> &[TestStrings] {
        &self.test_strings
    }

    fn get_test_strings_for_reflect(&self) -> &::protobuf::RepeatedField<TestStrings> {
        &self.test_strings
    }

    fn mut_test_strings_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<TestStrings> {
        &mut self.test_strings
    }

    // repeated .perftest_data.TestRepeatedPackedInt32 test_repeated_packed_int32 = 6;

    pub fn clear_test_repeated_packed_int32(&mut self) {
        self.test_repeated_packed_int32.clear();
    }

    // Param is passed by value, moved
    pub fn set_test_repeated_packed_int32(&mut self, v: ::protobuf::RepeatedField<TestRepeatedPackedInt32>) {
        self.test_repeated_packed_int32 = v;
    }

    // Mutable pointer to the field.
    pub fn mut_test_repeated_packed_int32(&mut self) -> &mut ::protobuf::RepeatedField<TestRepeatedPackedInt32> {
        &mut self.test_repeated_packed_int32
    }

    // Take field
    pub fn take_test_repeated_packed_int32(&mut self) -> ::protobuf::RepeatedField<TestRepeatedPackedInt32> {
        ::std::mem::replace(&mut self.test_repeated_packed_int32, ::protobuf::RepeatedField::new())
    }

    pub fn get_test_repeated_packed_int32(&self) -> &[TestRepeatedPackedInt32] {
        &self.test_repeated_packed_int32
    }

    fn get_test_repeated_packed_int32_for_reflect(&self) -> &::protobuf::RepeatedField<TestRepeatedPackedInt32> {
        &self.test_repeated_packed_int32
    }

    fn mut_test_repeated_packed_int32_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<TestRepeatedPackedInt32> {
        &mut self.test_repeated_packed_int32
    }

    // repeated .perftest_data.TestBytes test_small_bytearrays = 7;

    pub fn clear_test_small_bytearrays(&mut self) {
        self.test_small_bytearrays.clear();
    }

    // Param is passed by value, moved
    pub fn set_test_small_bytearrays(&mut self, v: ::protobuf::RepeatedField<TestBytes>) {
        self.test_small_bytearrays = v;
    }

    // Mutable pointer to the field.
    pub fn mut_test_small_bytearrays(&mut self) -> &mut ::protobuf::RepeatedField<TestBytes> {
        &mut self.test_small_bytearrays
    }

    // Take field
    pub fn take_test_small_bytearrays(&mut self) -> ::protobuf::RepeatedField<TestBytes> {
        ::std::mem::replace(&mut self.test_small_bytearrays, ::protobuf::RepeatedField::new())
    }

    pub fn get_test_small_bytearrays(&self) -> &[TestBytes] {
        &self.test_small_bytearrays
    }

    fn get_test_small_bytearrays_for_reflect(&self) -> &::protobuf::RepeatedField<TestBytes> {
        &self.test_small_bytearrays
    }

    fn mut_test_small_bytearrays_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<TestBytes> {
        &mut self.test_small_bytearrays
    }

    // repeated .perftest_data.TestBytes test_large_bytearrays = 8;

    pub fn clear_test_large_bytearrays(&mut self) {
        self.test_large_bytearrays.clear();
    }

    // Param is passed by value, moved
    pub fn set_test_large_bytearrays(&mut self, v: ::protobuf::RepeatedField<TestBytes>) {
        self.test_large_bytearrays = v;
    }

    // Mutable pointer to the field.
    pub fn mut_test_large_bytearrays(&mut self) -> &mut ::protobuf::RepeatedField<TestBytes> {
        &mut self.test_large_bytearrays
    }

    // Take field
    pub fn take_test_large_bytearrays(&mut self) -> ::protobuf::RepeatedField<TestBytes> {
        ::std::mem::replace(&mut self.test_large_bytearrays, ::protobuf::RepeatedField::new())
    }

    pub fn get_test_large_bytearrays(&self) -> &[TestBytes] {
        &self.test_large_bytearrays
    }

    fn get_test_large_bytearrays_for_reflect(&self) -> &::protobuf::RepeatedField<TestBytes> {
        &self.test_large_bytearrays
    }

    fn mut_test_large_bytearrays_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<TestBytes> {
        &mut self.test_large_bytearrays
    }
}

impl ::protobuf::Message for PerftestData {
    fn is_initialized(&self) -> bool {
        for v in &self.test1 {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.test_repeated_bool {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.test_repeated_messages {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.test_optional_messages {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.test_strings {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.test_repeated_packed_int32 {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.test_small_bytearrays {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.test_large_bytearrays {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.test1)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.test_repeated_bool)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.test_repeated_messages)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.test_optional_messages)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.test_strings)?;
                },
                6 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.test_repeated_packed_int32)?;
                },
                7 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.test_small_bytearrays)?;
                },
                8 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.test_large_bytearrays)?;
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
        for value in &self.test1 {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.test_repeated_bool {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.test_repeated_messages {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.test_optional_messages {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.test_strings {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.test_repeated_packed_int32 {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.test_small_bytearrays {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.test_large_bytearrays {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.test1 {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.test_repeated_bool {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.test_repeated_messages {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.test_optional_messages {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.test_strings {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.test_repeated_packed_int32 {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.test_small_bytearrays {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.test_large_bytearrays {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for PerftestData {
    fn new() -> PerftestData {
        PerftestData::new()
    }

    fn descriptor_static(_: ::std::option::Option<PerftestData>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Test1>>(
                    "test1",
                    PerftestData::get_test1_for_reflect,
                    PerftestData::mut_test1_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TestRepeatedBool>>(
                    "test_repeated_bool",
                    PerftestData::get_test_repeated_bool_for_reflect,
                    PerftestData::mut_test_repeated_bool_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TestRepeatedMessages>>(
                    "test_repeated_messages",
                    PerftestData::get_test_repeated_messages_for_reflect,
                    PerftestData::mut_test_repeated_messages_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TestOptionalMessages>>(
                    "test_optional_messages",
                    PerftestData::get_test_optional_messages_for_reflect,
                    PerftestData::mut_test_optional_messages_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TestStrings>>(
                    "test_strings",
                    PerftestData::get_test_strings_for_reflect,
                    PerftestData::mut_test_strings_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TestRepeatedPackedInt32>>(
                    "test_repeated_packed_int32",
                    PerftestData::get_test_repeated_packed_int32_for_reflect,
                    PerftestData::mut_test_repeated_packed_int32_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TestBytes>>(
                    "test_small_bytearrays",
                    PerftestData::get_test_small_bytearrays_for_reflect,
                    PerftestData::mut_test_small_bytearrays_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TestBytes>>(
                    "test_large_bytearrays",
                    PerftestData::get_test_large_bytearrays_for_reflect,
                    PerftestData::mut_test_large_bytearrays_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PerftestData>(
                    "PerftestData",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PerftestData {
    fn clear(&mut self) {
        self.clear_test1();
        self.clear_test_repeated_bool();
        self.clear_test_repeated_messages();
        self.clear_test_optional_messages();
        self.clear_test_strings();
        self.clear_test_repeated_packed_int32();
        self.clear_test_small_bytearrays();
        self.clear_test_large_bytearrays();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PerftestData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PerftestData {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13perftest_data.proto\x12\rperftest_data\"\x1d\n\x05Test1\x12\x14\n\
    \x05value\x18\x01\x20\x01(\x05R\x05value\"*\n\x10TestRepeatedBool\x12\
    \x16\n\x06values\x18\x01\x20\x03(\x08R\x06values\"5\n\x17TestRepeatedPac\
    kedInt32\x12\x1a\n\x06values\x18\x01\x20\x03(\x05R\x06valuesB\x02\x10\
    \x01\"\xdf\x01\n\x14TestRepeatedMessages\x12A\n\tmessages1\x18\x01\x20\
    \x03(\x0b2#.perftest_data.TestRepeatedMessagesR\tmessages1\x12A\n\tmessa\
    ges2\x18\x02\x20\x03(\x0b2#.perftest_data.TestRepeatedMessagesR\tmessage\
    s2\x12A\n\tmessages3\x18\x03\x20\x03(\x0b2#.perftest_data.TestRepeatedMe\
    ssagesR\tmessages3\"\xd9\x01\n\x14TestOptionalMessages\x12?\n\x08message\
    1\x18\x01\x20\x01(\x0b2#.perftest_data.TestOptionalMessagesR\x08message1\
    \x12?\n\x08message2\x18\x02\x20\x01(\x0b2#.perftest_data.TestOptionalMes\
    sagesR\x08message2\x12?\n\x08message3\x18\x03\x20\x01(\x0b2#.perftest_da\
    ta.TestOptionalMessagesR\x08message3\"=\n\x0bTestStrings\x12\x0e\n\x02s1\
    \x18\x01\x20\x01(\tR\x02s1\x12\x0e\n\x02s2\x18\x02\x20\x01(\tR\x02s2\x12\
    \x0e\n\x02s3\x18\x03\x20\x01(\tR\x02s3\"\x1b\n\tTestBytes\x12\x0e\n\x02b\
    1\x18\x01\x20\x01(\x0cR\x02b1\"\xff\x04\n\x0cPerftestData\x12*\n\x05test\
    1\x18\x01\x20\x03(\x0b2\x14.perftest_data.Test1R\x05test1\x12M\n\x12test\
    _repeated_bool\x18\x02\x20\x03(\x0b2\x1f.perftest_data.TestRepeatedBoolR\
    \x10testRepeatedBool\x12Y\n\x16test_repeated_messages\x18\x03\x20\x03(\
    \x0b2#.perftest_data.TestRepeatedMessagesR\x14testRepeatedMessages\x12Y\
    \n\x16test_optional_messages\x18\x04\x20\x03(\x0b2#.perftest_data.TestOp\
    tionalMessagesR\x14testOptionalMessages\x12=\n\x0ctest_strings\x18\x05\
    \x20\x03(\x0b2\x1a.perftest_data.TestStringsR\x0btestStrings\x12c\n\x1at\
    est_repeated_packed_int32\x18\x06\x20\x03(\x0b2&.perftest_data.TestRepea\
    tedPackedInt32R\x17testRepeatedPackedInt32\x12L\n\x15test_small_bytearra\
    ys\x18\x07\x20\x03(\x0b2\x18.perftest_data.TestBytesR\x13testSmallBytear\
    rays\x12L\n\x15test_large_bytearrays\x18\x08\x20\x03(\x0b2\x18.perftest_\
    data.TestBytesR\x13testLargeBytearraysJ\xef\r\n\x06\x12\x04\0\0/\x01\n\
    \x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x02\x08\x15\n\n\n\
    \x02\x04\0\x12\x04\x04\0\x06\x01\n\n\n\x03\x04\0\x01\x12\x03\x04\x08\r\n\
    \x0b\n\x04\x04\0\x02\0\x12\x03\x05\x04\x1d\n\x0c\n\x05\x04\0\x02\0\x04\
    \x12\x03\x05\x04\x0c\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x05\r\x12\n\x0c\
    \n\x05\x04\0\x02\0\x01\x12\x03\x05\x13\x18\n\x0c\n\x05\x04\0\x02\0\x03\
    \x12\x03\x05\x1b\x1c\n\n\n\x02\x04\x01\x12\x04\x08\0\n\x01\n\n\n\x03\x04\
    \x01\x01\x12\x03\x08\x08\x18\n\x0b\n\x04\x04\x01\x02\0\x12\x03\t\x04\x1d\
    \n\x0c\n\x05\x04\x01\x02\0\x04\x12\x03\t\x04\x0c\n\x0c\n\x05\x04\x01\x02\
    \0\x05\x12\x03\t\r\x11\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\t\x12\x18\n\
    \x0c\n\x05\x04\x01\x02\0\x03\x12\x03\t\x1b\x1c\n\n\n\x02\x04\x02\x12\x04\
    \x0c\0\x0e\x01\n\n\n\x03\x04\x02\x01\x12\x03\x0c\x08\x1f\n\x0b\n\x04\x04\
    \x02\x02\0\x12\x03\r\x040\n\x0c\n\x05\x04\x02\x02\0\x04\x12\x03\r\x04\
    \x0c\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x03\r\r\x12\n\x0c\n\x05\x04\x02\
    \x02\0\x01\x12\x03\r\x13\x19\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\r\x1c\
    \x1d\n\x0c\n\x05\x04\x02\x02\0\x08\x12\x03\r\x1e/\n\x0f\n\x08\x04\x02\
    \x02\0\x08\xe7\x07\0\x12\x03\r\x20-\n\x10\n\t\x04\x02\x02\0\x08\xe7\x07\
    \0\x02\x12\x03\r\x20&\n\x11\n\n\x04\x02\x02\0\x08\xe7\x07\0\x02\0\x12\
    \x03\r\x20&\n\x12\n\x0b\x04\x02\x02\0\x08\xe7\x07\0\x02\0\x01\x12\x03\r\
    \x20&\n\x10\n\t\x04\x02\x02\0\x08\xe7\x07\0\x03\x12\x03\r)-\n\n\n\x02\
    \x04\x03\x12\x04\x10\0\x14\x01\n\n\n\x03\x04\x03\x01\x12\x03\x10\x08\x1c\
    \n\x0b\n\x04\x04\x03\x02\0\x12\x03\x11\x040\n\x0c\n\x05\x04\x03\x02\0\
    \x04\x12\x03\x11\x04\x0c\n\x0c\n\x05\x04\x03\x02\0\x06\x12\x03\x11\r!\n\
    \x0c\n\x05\x04\x03\x02\0\x01\x12\x03\x11\"+\n\x0c\n\x05\x04\x03\x02\0\
    \x03\x12\x03\x11./\n\x0b\n\x04\x04\x03\x02\x01\x12\x03\x12\x040\n\x0c\n\
    \x05\x04\x03\x02\x01\x04\x12\x03\x12\x04\x0c\n\x0c\n\x05\x04\x03\x02\x01\
    \x06\x12\x03\x12\r!\n\x0c\n\x05\x04\x03\x02\x01\x01\x12\x03\x12\"+\n\x0c\
    \n\x05\x04\x03\x02\x01\x03\x12\x03\x12./\n\x0b\n\x04\x04\x03\x02\x02\x12\
    \x03\x13\x040\n\x0c\n\x05\x04\x03\x02\x02\x04\x12\x03\x13\x04\x0c\n\x0c\
    \n\x05\x04\x03\x02\x02\x06\x12\x03\x13\r!\n\x0c\n\x05\x04\x03\x02\x02\
    \x01\x12\x03\x13\"+\n\x0c\n\x05\x04\x03\x02\x02\x03\x12\x03\x13./\n\n\n\
    \x02\x04\x04\x12\x04\x16\0\x1a\x01\n\n\n\x03\x04\x04\x01\x12\x03\x16\x08\
    \x1c\n\x0b\n\x04\x04\x04\x02\0\x12\x03\x17\x04/\n\x0c\n\x05\x04\x04\x02\
    \0\x04\x12\x03\x17\x04\x0c\n\x0c\n\x05\x04\x04\x02\0\x06\x12\x03\x17\r!\
    \n\x0c\n\x05\x04\x04\x02\0\x01\x12\x03\x17\"*\n\x0c\n\x05\x04\x04\x02\0\
    \x03\x12\x03\x17-.\n\x0b\n\x04\x04\x04\x02\x01\x12\x03\x18\x04/\n\x0c\n\
    \x05\x04\x04\x02\x01\x04\x12\x03\x18\x04\x0c\n\x0c\n\x05\x04\x04\x02\x01\
    \x06\x12\x03\x18\r!\n\x0c\n\x05\x04\x04\x02\x01\x01\x12\x03\x18\"*\n\x0c\
    \n\x05\x04\x04\x02\x01\x03\x12\x03\x18-.\n\x0b\n\x04\x04\x04\x02\x02\x12\
    \x03\x19\x04/\n\x0c\n\x05\x04\x04\x02\x02\x04\x12\x03\x19\x04\x0c\n\x0c\
    \n\x05\x04\x04\x02\x02\x06\x12\x03\x19\r!\n\x0c\n\x05\x04\x04\x02\x02\
    \x01\x12\x03\x19\"*\n\x0c\n\x05\x04\x04\x02\x02\x03\x12\x03\x19-.\n\n\n\
    \x02\x04\x05\x12\x04\x1c\0\x20\x01\n\n\n\x03\x04\x05\x01\x12\x03\x1c\x08\
    \x13\n\x0b\n\x04\x04\x05\x02\0\x12\x03\x1d\x04\x1b\n\x0c\n\x05\x04\x05\
    \x02\0\x04\x12\x03\x1d\x04\x0c\n\x0c\n\x05\x04\x05\x02\0\x05\x12\x03\x1d\
    \r\x13\n\x0c\n\x05\x04\x05\x02\0\x01\x12\x03\x1d\x14\x16\n\x0c\n\x05\x04\
    \x05\x02\0\x03\x12\x03\x1d\x19\x1a\n\x0b\n\x04\x04\x05\x02\x01\x12\x03\
    \x1e\x04\x1b\n\x0c\n\x05\x04\x05\x02\x01\x04\x12\x03\x1e\x04\x0c\n\x0c\n\
    \x05\x04\x05\x02\x01\x05\x12\x03\x1e\r\x13\n\x0c\n\x05\x04\x05\x02\x01\
    \x01\x12\x03\x1e\x14\x16\n\x0c\n\x05\x04\x05\x02\x01\x03\x12\x03\x1e\x19\
    \x1a\n\x0b\n\x04\x04\x05\x02\x02\x12\x03\x1f\x04\x1b\n\x0c\n\x05\x04\x05\
    \x02\x02\x04\x12\x03\x1f\x04\x0c\n\x0c\n\x05\x04\x05\x02\x02\x05\x12\x03\
    \x1f\r\x13\n\x0c\n\x05\x04\x05\x02\x02\x01\x12\x03\x1f\x14\x16\n\x0c\n\
    \x05\x04\x05\x02\x02\x03\x12\x03\x1f\x19\x1a\n\n\n\x02\x04\x06\x12\x04\"\
    \0$\x01\n\n\n\x03\x04\x06\x01\x12\x03\"\x08\x11\n\x0b\n\x04\x04\x06\x02\
    \0\x12\x03#\x04\x1a\n\x0c\n\x05\x04\x06\x02\0\x04\x12\x03#\x04\x0c\n\x0c\
    \n\x05\x04\x06\x02\0\x05\x12\x03#\r\x12\n\x0c\n\x05\x04\x06\x02\0\x01\
    \x12\x03#\x13\x15\n\x0c\n\x05\x04\x06\x02\0\x03\x12\x03#\x18\x19\n\n\n\
    \x02\x04\x07\x12\x04&\0/\x01\n\n\n\x03\x04\x07\x01\x12\x03&\x08\x14\n\
    \x0b\n\x04\x04\x07\x02\0\x12\x03'\x04\x1d\n\x0c\n\x05\x04\x07\x02\0\x04\
    \x12\x03'\x04\x0c\n\x0c\n\x05\x04\x07\x02\0\x06\x12\x03'\r\x12\n\x0c\n\
    \x05\x04\x07\x02\0\x01\x12\x03'\x13\x18\n\x0c\n\x05\x04\x07\x02\0\x03\
    \x12\x03'\x1b\x1c\n\x0b\n\x04\x04\x07\x02\x01\x12\x03(\x045\n\x0c\n\x05\
    \x04\x07\x02\x01\x04\x12\x03(\x04\x0c\n\x0c\n\x05\x04\x07\x02\x01\x06\
    \x12\x03(\r\x1d\n\x0c\n\x05\x04\x07\x02\x01\x01\x12\x03(\x1e0\n\x0c\n\
    \x05\x04\x07\x02\x01\x03\x12\x03(34\n\x0b\n\x04\x04\x07\x02\x02\x12\x03)\
    \x04=\n\x0c\n\x05\x04\x07\x02\x02\x04\x12\x03)\x04\x0c\n\x0c\n\x05\x04\
    \x07\x02\x02\x06\x12\x03)\r!\n\x0c\n\x05\x04\x07\x02\x02\x01\x12\x03)\"8\
    \n\x0c\n\x05\x04\x07\x02\x02\x03\x12\x03);<\n\x0b\n\x04\x04\x07\x02\x03\
    \x12\x03*\x04=\n\x0c\n\x05\x04\x07\x02\x03\x04\x12\x03*\x04\x0c\n\x0c\n\
    \x05\x04\x07\x02\x03\x06\x12\x03*\r!\n\x0c\n\x05\x04\x07\x02\x03\x01\x12\
    \x03*\"8\n\x0c\n\x05\x04\x07\x02\x03\x03\x12\x03*;<\n\x0b\n\x04\x04\x07\
    \x02\x04\x12\x03+\x04*\n\x0c\n\x05\x04\x07\x02\x04\x04\x12\x03+\x04\x0c\
    \n\x0c\n\x05\x04\x07\x02\x04\x06\x12\x03+\r\x18\n\x0c\n\x05\x04\x07\x02\
    \x04\x01\x12\x03+\x19%\n\x0c\n\x05\x04\x07\x02\x04\x03\x12\x03+()\n\x0b\
    \n\x04\x04\x07\x02\x05\x12\x03,\x04D\n\x0c\n\x05\x04\x07\x02\x05\x04\x12\
    \x03,\x04\x0c\n\x0c\n\x05\x04\x07\x02\x05\x06\x12\x03,\r$\n\x0c\n\x05\
    \x04\x07\x02\x05\x01\x12\x03,%?\n\x0c\n\x05\x04\x07\x02\x05\x03\x12\x03,\
    BC\n\x0b\n\x04\x04\x07\x02\x06\x12\x03-\x041\n\x0c\n\x05\x04\x07\x02\x06\
    \x04\x12\x03-\x04\x0c\n\x0c\n\x05\x04\x07\x02\x06\x06\x12\x03-\r\x16\n\
    \x0c\n\x05\x04\x07\x02\x06\x01\x12\x03-\x17,\n\x0c\n\x05\x04\x07\x02\x06\
    \x03\x12\x03-/0\n\x0b\n\x04\x04\x07\x02\x07\x12\x03.\x041\n\x0c\n\x05\
    \x04\x07\x02\x07\x04\x12\x03.\x04\x0c\n\x0c\n\x05\x04\x07\x02\x07\x06\
    \x12\x03.\r\x16\n\x0c\n\x05\x04\x07\x02\x07\x01\x12\x03.\x17,\n\x0c\n\
    \x05\x04\x07\x02\x07\x03\x12\x03./0\
";

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
