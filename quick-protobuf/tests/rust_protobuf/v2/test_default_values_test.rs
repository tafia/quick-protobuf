use std::borrow::Cow;
use std::f32;
use std::f64;

use super::test_default_values::*;
use super::test_default_values_dont_use_cow::*;
use quick_protobuf::*;

#[test]
fn test_default_value_simple() {
    let bytes = &[];
    let mut reader = BytesReader::from_bytes(bytes);
    let d = TestDefaultValuesOptional::from_reader(&mut reader, bytes).unwrap();
    assert_eq!(1.0, d.double_field.unwrap_or(TestDefaultValuesOptional::DEFAULT_DOUBLE_FIELD));
    assert_eq!(2.0, d.float_field.unwrap_or(TestDefaultValuesOptional::DEFAULT_FLOAT_FIELD));
    assert_eq!(3, d.int32_field.unwrap_or(TestDefaultValuesOptional::DEFAULT_INT32_FIELD));
    assert_eq!(4, d.int64_field.unwrap_or(TestDefaultValuesOptional::DEFAULT_INT64_FIELD));
    assert_eq!(5, d.uint32_field.unwrap_or(TestDefaultValuesOptional::DEFAULT_UINT32_FIELD));
    assert_eq!(6, d.uint64_field.unwrap_or(TestDefaultValuesOptional::DEFAULT_UINT64_FIELD));
    assert_eq!(7, d.sint32_field.unwrap_or(TestDefaultValuesOptional::DEFAULT_SINT32_FIELD));
    assert_eq!(8, d.sint64_field.unwrap_or(TestDefaultValuesOptional::DEFAULT_SINT64_FIELD));
    assert_eq!(9, d.fixed32_field.unwrap_or(TestDefaultValuesOptional::DEFAULT_FIXED32_FIELD));
    assert_eq!(10, d.fixed64_field.unwrap_or(TestDefaultValuesOptional::DEFAULT_FIXED64_FIELD));
    assert_eq!(11, d.sfixed32_field.unwrap_or(TestDefaultValuesOptional::DEFAULT_SFIXED32_FIELD));
    assert_eq!(12, d.sfixed64_field.unwrap_or(TestDefaultValuesOptional::DEFAULT_SFIXED64_FIELD));
    assert_eq!(true, d.bool_field.unwrap_or(TestDefaultValuesOptional::DEFAULT_BOOL_FIELD));
    assert_eq!("abc\n22", d.string_field.unwrap_or(TestDefaultValuesOptional::DEFAULT_STRING_FIELD));
    assert_eq!(b"cde\n33".to_vec(), d.bytes_field.unwrap_or(TestDefaultValuesOptional::DEFAULT_BYTES_FIELD).to_vec());
    assert!(EnumForDefaultValue::TWO.eq(&d.enum_field.unwrap_or(TestDefaultValuesOptional::DEFAULT_ENUM_FIELD)));
    assert!(d
        .enum_field_without_default
        .map_or(true, |e| e.eq(&EnumForDefaultValue::ONE)));
}

#[test]
fn test_default_required_init() {
    let a = TestDefaultValuesRequired::default();
    assert_eq!(
        a,
        TestDefaultValuesRequired {
            double_field: 1f64,
            float_field: 2f32,
            int32_field: 3i32,
            int64_field: 4i64,
            uint32_field: 5u32,
            uint64_field: 6u64,
            sint32_field: 7i32,
            sint64_field: 8i64,
            fixed32_field: 9u32,
            fixed64_field: 10u64,
            sfixed32_field: 11i32,
            sfixed64_field: 12i64,
            bool_field: true,
            string_field: Cow::Borrowed("abc\n22"),
            bytes_field: Cow::Borrowed(b"cde\n33"),
            enum_field: EnumForDefaultValue::TWO,
            enum_field_without_default: EnumForDefaultValue::ONE,
        }
    );
}

#[test]
fn test_default_value_extreme() {
    let bytes = &[];
    let mut reader = BytesReader::from_bytes(bytes);
    let d = TestExtremeDefaultValues::from_reader(&mut reader, bytes).unwrap();
    assert_eq!(f64::INFINITY, d.inf_double.unwrap_or(TestExtremeDefaultValues::DEFAULT_INF_DOUBLE));
    assert_eq!(f64::NEG_INFINITY, d.neg_inf_double.unwrap_or(TestExtremeDefaultValues::DEFAULT_NEG_INF_DOUBLE));
    assert!(d.nan_double.unwrap_or(TestExtremeDefaultValues::DEFAULT_NAN_DOUBLE).is_nan());
    assert_eq!(f32::INFINITY, d.inf_float.unwrap_or(TestExtremeDefaultValues::DEFAULT_INF_FLOAT));
    assert_eq!(f32::NEG_INFINITY, d.neg_inf_float.unwrap_or(TestExtremeDefaultValues::DEFAULT_NEG_INF_FLOAT));
    assert!(d.nan_float.unwrap_or(TestExtremeDefaultValues::DEFAULT_NAN_FLOAT).is_nan());
}

#[test]
fn test_default_value_simple_dont_use_cow() {
    let bytes = &[];
    let mut reader = BytesReader::from_bytes(bytes);
    let d = TestDefaultValuesDontUseCowOptional::from_reader(&mut reader, bytes).unwrap();
    assert_eq!("abc\n22".to_owned(), d.string_field.unwrap_or(TestDefaultValuesDontUseCowOptional::DEFAULT_STRING_FIELD.to_owned()));
    assert_eq!(b"cde\n33".to_vec(), d.bytes_field.unwrap_or(TestDefaultValuesDontUseCowOptional::DEFAULT_BYTES_FIELD.to_vec()));
}

#[test]
fn test_default_required_init_dont_use_cow() {
    let a = TestDefaultValuesDontUseCowRequired::default();
    assert_eq!(
        a,
        TestDefaultValuesDontUseCowRequired {
            string_field: "abc\n22".to_owned(),
            bytes_field: b"cde\n33".to_vec(),
        }
    );
}
