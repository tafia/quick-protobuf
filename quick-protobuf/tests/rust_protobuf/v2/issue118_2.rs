use super::*;
use quick_protobuf::sizeofs::*;
use quick_protobuf::{BytesReader, MessageRead, MessageWrite, Result, Writer};
use std::io::Write;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct First {
    pub first_field: Option<Box<Fourth>>,
}

impl<'a> MessageRead<'a> for First {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.first_field = Some(Box::new(r.read_message::<Fourth>(bytes)?)),
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for First {
    fn get_size(&self) -> usize {
        0 + self
            .first_field
            .as_ref()
            .map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.first_field {
            w.write_with_tag(10, |w| w.write_message(&**s))?;
        }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Second {
    pub second_field: Option<Box<First>>,
}

impl<'a> MessageRead<'a> for Second {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.second_field = Some(Box::new(r.read_message::<First>(bytes)?)),
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Second {
    fn get_size(&self) -> usize {
        0 + self
            .second_field
            .as_ref()
            .map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.second_field {
            w.write_with_tag(10, |w| w.write_message(&**s))?;
        }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Third {
    pub third_field: Option<Box<First>>,
}

impl<'a> MessageRead<'a> for Third {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.third_field = Some(Box::new(r.read_message::<First>(bytes)?)),
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Third {
    fn get_size(&self) -> usize {
        0 + self
            .third_field
            .as_ref()
            .map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.third_field {
            w.write_with_tag(10, |w| w.write_message(&**s))?;
        }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Fourth {
    pub fourth_field: Option<Box<Third>>,
    pub fourth_field_2: Option<Box<Fifth>>,
}

impl<'a> MessageRead<'a> for Fourth {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.fourth_field = Some(Box::new(r.read_message::<Third>(bytes)?)),
                Ok(18) => msg.fourth_field_2 = Some(Box::new(r.read_message::<Fifth>(bytes)?)),
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Fourth {
    fn get_size(&self) -> usize {
        0 + self
            .fourth_field
            .as_ref()
            .map_or(0, |m| 1 + sizeof_len((m).get_size()))
            + self
                .fourth_field_2
                .as_ref()
                .map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.fourth_field {
            w.write_with_tag(10, |w| w.write_message(&**s))?;
        }
        if let Some(ref s) = self.fourth_field_2 {
            w.write_with_tag(18, |w| w.write_message(&**s))?;
        }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Fifth {
    pub fifth_field: Option<Box<Second>>,
}

impl<'a> MessageRead<'a> for Fifth {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.fifth_field = Some(Box::new(r.read_message::<Second>(bytes)?)),
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Fifth {
    fn get_size(&self) -> usize {
        0 + self
            .fifth_field
            .as_ref()
            .map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.fifth_field {
            w.write_with_tag(10, |w| w.write_message(&**s))?;
        }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Sixth {
    pub sixth_field: Option<Box<Sixth>>,
}

impl<'a> MessageRead<'a> for Sixth {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.sixth_field = Some(Box::new(r.read_message::<Sixth>(bytes)?)),
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Sixth {
    fn get_size(&self) -> usize {
        0 + self
            .sixth_field
            .as_ref()
            .map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.sixth_field {
            w.write_with_tag(10, |w| w.write_message(&**s))?;
        }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Seventh {
    pub seventh_field: Option<Box<Eighth>>,
}

impl<'a> MessageRead<'a> for Seventh {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.seventh_field = Some(Box::new(r.read_message::<Eighth>(bytes)?)),
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Seventh {
    fn get_size(&self) -> usize {
        0 + self
            .seventh_field
            .as_ref()
            .map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.seventh_field {
            w.write_with_tag(10, |w| w.write_message(&**s))?;
        }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Eighth {
    pub eighth_field: Option<Box<Seventh>>,
}

impl<'a> MessageRead<'a> for Eighth {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.eighth_field = Some(Box::new(r.read_message::<Seventh>(bytes)?)),
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Eighth {
    fn get_size(&self) -> usize {
        0 + self
            .eighth_field
            .as_ref()
            .map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.eighth_field {
            w.write_with_tag(10, |w| w.write_message(&**s))?;
        }
        Ok(())
    }
}
