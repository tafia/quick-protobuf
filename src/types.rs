//! A module to handle basic protobuf types

pub const TAG_TYPE_BITS: u32 = 3;
pub const TAG_TYPE_MASK: u32 = (1u32 << TAG_TYPE_BITS as usize) - 1;
pub const FIELD_NUMBER_MAX: u32 = 0x1fffffff;

/// An enum to define to type of field to be parsed next
#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub enum WireType {
    Varint          = 0,
    Fixed64         = 1,
    LengthDelimited = 2,
    StartGroup      = 3,
    EndGroup        = 4,
    Fixed32         = 5,
    Unknown         = 6,
}

impl From<u32> for WireType {
    fn from(n: u32) -> WireType {
        match n {
            0 => WireType::Varint,
            1 => WireType::Fixed64,
            2 => WireType::LengthDelimited,
            3 => WireType::StartGroup,
            4 => WireType::EndGroup,
            5 => WireType::Fixed32,
            _ => WireType::Unknown,
        }
    }
}

impl WireType {
    fn is_valid(&self) -> bool {
        if let WireType::Unknown = *self {
            return false;
        }
        true
    }
}

/// A struct to define which property is to be parsed next
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tag {
    field_number: u32,
    wire_type: WireType,
}

impl Tag {
    pub fn value(&self) -> u32 {
        (self.field_number << TAG_TYPE_BITS) | (self.wire_type as u32)
    }

    pub fn unpack(&self) -> (u32, WireType) {
        (self.field_number(), self.wire_type())
    }

    pub fn wire_type(&self) -> WireType {
        self.wire_type
    }

    pub fn field_number(&self) -> u32 {
        self.field_number
    }

    pub fn is_valid(&self) -> bool {
        self.field_number != 0 && self.wire_type.is_valid()
    }
}

impl From<u32> for Tag {
    fn from(value: u32) -> Tag {
        let wire_type = (value & TAG_TYPE_MASK).into();
        let field_number = value >> TAG_TYPE_BITS;
        Tag {
            field_number: field_number,
            wire_type: wire_type,
        }
    }
}

#[test]
fn test_parse_tag() {
    assert_eq!(::types::Tag {
        field_number: 1,
        wire_type: ::types::WireType::Varint,
    }, 0b00001000.into())
}
