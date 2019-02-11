use super::*;
use quick_protobuf::sizeofs::*;
use quick_protobuf::{BytesReader, MessageRead, MessageWrite, Result, Writer};
use std::io::Write;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ViaOneOf {
    pub Alternatives: mod_ViaOneOf::OneOfAlternatives,
}

impl<'a> MessageRead<'a> for ViaOneOf {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => {
                    msg.Alternatives = mod_ViaOneOf::OneOfAlternatives::present(Box::new(
                        r.read_message::<mod_ViaOneOf::Data>(bytes)?,
                    ))
                }
                Ok(16) => {
                    msg.Alternatives = mod_ViaOneOf::OneOfAlternatives::absent(r.read_bool(bytes)?)
                }
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ViaOneOf {
    fn get_size(&self) -> usize {
        0 + match self.Alternatives {
            mod_ViaOneOf::OneOfAlternatives::present(ref m) => 1 + sizeof_len((m).get_size()),
            mod_ViaOneOf::OneOfAlternatives::absent(ref m) => 1 + sizeof_varint(*(m) as u64),
            mod_ViaOneOf::OneOfAlternatives::None => 0,
        }
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.Alternatives {
            mod_ViaOneOf::OneOfAlternatives::present(ref m) => {
                w.write_with_tag(10, |w| w.write_message(&**m))?
            }
            mod_ViaOneOf::OneOfAlternatives::absent(ref m) => {
                w.write_with_tag(16, |w| w.write_bool(*m))?
            }
            mod_ViaOneOf::OneOfAlternatives::None => {}
        }
        Ok(())
    }
}

pub mod mod_ViaOneOf {

    use super::*;

    #[derive(Debug, Default, PartialEq, Clone)]
    pub struct Data {
        pub self_reference: Option<Box<ViaOneOf>>,
    }

    impl<'a> MessageRead<'a> for Data {
        fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
            let mut msg = Self::default();
            while !r.is_eof() {
                match r.next_tag(bytes) {
                    Ok(10) => {
                        msg.self_reference = Some(Box::new(r.read_message::<ViaOneOf>(bytes)?))
                    }
                    Ok(t) => {
                        r.read_unknown(bytes, t)?;
                    }
                    Err(e) => return Err(e),
                }
            }
            Ok(msg)
        }
    }

    impl MessageWrite for Data {
        fn get_size(&self) -> usize {
            0 + self
                .self_reference
                .as_ref()
                .map_or(0, |m| 1 + sizeof_len((m).get_size()))
        }

        fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
            if let Some(ref s) = self.self_reference {
                w.write_with_tag(10, |w| w.write_message(&**s))?;
            }
            Ok(())
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum OneOfAlternatives {
        present(Box<mod_ViaOneOf::Data>),
        absent(bool),
        None,
    }

    impl Default for OneOfAlternatives {
        fn default() -> Self {
            OneOfAlternatives::None
        }
    }

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ViaOptional {
    pub data: Option<Box<mod_ViaOptional::Data>>,
}

impl<'a> MessageRead<'a> for ViaOptional {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => {
                    msg.data = Some(Box::new(r.read_message::<mod_ViaOptional::Data>(bytes)?))
                }
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ViaOptional {
    fn get_size(&self) -> usize {
        0 + self
            .data
            .as_ref()
            .map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.data {
            w.write_with_tag(10, |w| w.write_message(&**s))?;
        }
        Ok(())
    }
}

pub mod mod_ViaOptional {

    use super::*;

    #[derive(Debug, Default, PartialEq, Clone)]
    pub struct Data {
        pub self_reference: Option<Box<ViaOptional>>,
    }

    impl<'a> MessageRead<'a> for Data {
        fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
            let mut msg = Self::default();
            while !r.is_eof() {
                match r.next_tag(bytes) {
                    Ok(10) => {
                        msg.self_reference = Some(Box::new(r.read_message::<ViaOptional>(bytes)?))
                    }
                    Ok(t) => {
                        r.read_unknown(bytes, t)?;
                    }
                    Err(e) => return Err(e),
                }
            }
            Ok(msg)
        }
    }

    impl MessageWrite for Data {
        fn get_size(&self) -> usize {
            0 + self
                .self_reference
                .as_ref()
                .map_or(0, |m| 1 + sizeof_len((m).get_size()))
        }

        fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
            if let Some(ref s) = self.self_reference {
                w.write_with_tag(10, |w| w.write_message(&**s))?;
            }
            Ok(())
        }
    }

}
