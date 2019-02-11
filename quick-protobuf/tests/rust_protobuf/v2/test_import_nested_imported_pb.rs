use super::*;
use quick_protobuf::{BytesReader, MessageRead, MessageWrite, Result};

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ContainerForNested {}

impl<'a> MessageRead<'a> for ContainerForNested {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ContainerForNested {}

pub mod mod_ContainerForNested {

    use super::*;

    #[derive(Debug, Default, PartialEq, Clone)]
    pub struct NestedMessage {}

    impl<'a> MessageRead<'a> for NestedMessage {
        fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
            r.read_to_end();
            Ok(Self::default())
        }
    }

    impl MessageWrite for NestedMessage {}

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum NestedEnum {
        RED = 1,
    }

    impl Default for NestedEnum {
        fn default() -> Self {
            NestedEnum::RED
        }
    }

    impl From<i32> for NestedEnum {
        fn from(i: i32) -> Self {
            match i {
                1 => NestedEnum::RED,
                _ => Self::default(),
            }
        }
    }

    impl<'a> From<&'a str> for NestedEnum {
        fn from(s: &'a str) -> Self {
            match s {
                "RED" => NestedEnum::RED,
                _ => Self::default(),
            }
        }
    }

}
