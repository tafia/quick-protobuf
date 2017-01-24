use std::io::Result as IoResult;
use std::io::Write;

use nom::IError;
use parser::file_descriptor;

fn sizeof_varint(v: u32) -> usize {
    match v {
        0x0...0x7F => 1,
        0x80...0x3FFF => 2,
        0x4000...0x1FFFFF => 3,
        0x200000...0xFFFFFFF => 4,
        _ => 5,
    }
}

#[derive(Debug)]
pub enum Frequency {
    Optional,
    Repeated,
    Required,
}

#[derive(Debug)]
pub struct Field<'a> {
    pub name: &'a str,
    pub frequency: Frequency,
    pub typ: &'a str,
    pub number: i32,
    pub default: Option<&'a str>,
    pub packed: bool,
    pub boxed: bool,
}

impl<'a> Field<'a> {

    fn rust_type(&self) -> &str {
        match self.typ {
            "int32" | "sint32" | "sfixed32" => "i32",
            "int64" | "sint64" | "sfixed64" => "i64",
            "uint32" | "fixed32" => "u32",
            "uint64" | "fixed64" => "u64",
            "float" => "f32",
            "double" => "f64",
            "string" => "String",
            "bytes" => "Vec<u8>",
            t => t,
        }
    }

    fn wire_type_num(&self, enums: &[&str]) -> u32 {
        match self.typ {
            "int32" | "sint32" | "int64" | "sint64" | 
                "uint32" | "uint64" | "bool" | "enum" => 0,
            "fixed64" | "sfixed64" | "double" => 1,
            "fixed32" | "sfixed32" | "float" => 5,
            t if enums.contains(&t) => 0,
            _ => 2,
        }
    }

    fn read_fn(&self, enums: &[&str]) -> &str {
        match self.typ {
            "int32" | "sint32" | "int64" | "sint64" | 
                "uint32" | "uint64" | "bool" | "fixed64" | 
                "sfixed64" | "double" | "fixed32" | "sfixed32" | 
                "float" | "string" | "bytes" => self.typ,
            t if enums.contains(&t) => "enum",
            _ => "message",
        }
    }

    fn tag(&self, enums: &[&str]) -> u32 {
        (self.number as u32) << 3 | self.wire_type_num(enums)
    }

    fn write_definition<W: Write>(&self, w: &mut W) -> IoResult<()> {
        match self.frequency {
            Frequency::Optional => {
                if self.boxed {
                    writeln!(w, "    pub {}: Option<Box<{}>>,", self.name, self.rust_type())
                } else {
                    writeln!(w, "    pub {}: Option<{}>,", self.name, self.rust_type())
                }
            }
            Frequency::Repeated => writeln!(w, "    pub {}: Vec<{}>,", self.name, self.rust_type()),
            Frequency::Required => {
                if self.boxed {
                    writeln!(w, "    pub {}: Box<{}>,", self.name, self.rust_type())
                } else {
                    writeln!(w, "    pub {}: {},", self.name, self.rust_type())
                }
            }
        }
    }

    fn write_match_tag<W: Write>(&self, w: &mut W, enums: &[&str]) -> IoResult<()> {
        match self.frequency {
            Frequency::Optional => {
                if self.boxed {
                    writeln!(w, "Ok({}) => msg.{} = Some(Box::new(r.read_{}()?)),",
                             self.tag(enums), self.name, self.read_fn(enums))
                } else {
                    writeln!(w, "Ok({}) => msg.{} = Some(r.read_{}()?),",
                             self.tag(enums), self.name, self.read_fn(enums))
                }
            }
            Frequency::Repeated => {
                if self.packed {
                    writeln!(w, "Ok({}) => msg.{} = r.read_packed_repeated_field(|r| r.read_{}())?,",
                             self.tag(enums), self.name, self.read_fn(enums))
                } else {
                    writeln!(w, "Ok({}) => msg.{}.push(r.read_{}()?),",
                             self.tag(enums), self.name, self.read_fn(enums))
                }
            }
            Frequency::Required => {
                if self.boxed {
                    writeln!(w, "Ok({}) => msg.{} = Box::new(r.read_{}()?),",
                             self.tag(enums), self.name, self.read_fn(enums))
                } else {
                    writeln!(w, "Ok({}) => msg.{} = r.read_{}()?,",
                             self.tag(enums), self.name, self.read_fn(enums))
                }
            }
        }
    }

    /// searches if the message must be boxed
    fn is_leaf(&self, leaf_messages: &[&str], enums: &[&str]) -> bool {
        match self.frequency {
            Frequency::Repeated | Frequency::Required => return true,
            Frequency::Optional => (),
        }
        if self.read_fn(enums) != "message" {
            return true;
        }
        leaf_messages.iter().any(|m| m == &self.typ)
    }

    fn write_get_size<W: Write>(&self, w: &mut W, enums: &[&str], is_first: bool) -> IoResult<()> {
        match self.frequency {
            Frequency::Required => {
                if is_first { write!(w, "        ")? } else { write!(w, "        + ")? }
                self.write_inner_get_size(w, enums, &format!("self.{}", self.name), "")?;
                writeln!(w, "")?;
            }
            Frequency::Optional => {
                if is_first {
                    write!(w, "        self.{}.as_ref().map_or(0, |m| ", self.name)?
                } else {
                    write!(w, "        + self.{}.as_ref().map_or(0, |m| ", self.name)?
                }
                self.write_inner_get_size(w, enums, "m", "*")?;
                writeln!(w, ")")?;
            }
            Frequency::Repeated => {
                let tag_size = sizeof_varint(self.tag(enums));
                if is_first { write!(w, "        ")? } else { write!(w, "        + ")? }
                if self.packed {
                    match self.wire_type_num(enums) {
                        0 => writeln!(w, "{} + sizeof_var_length(self.{}.iter().map(|s| sizeof_{}(s)).sum::<usize>())", 
                                      tag_size, self.name, self.read_fn(enums))?,
                        1 => writeln!(w, "{} + sizeof_var_length(self.{}.len() * 8)", tag_size, self.name)?,
                        5 => writeln!(w, "{} + sizeof_var_length(self.{}.len() * 4)", tag_size, self.name)?,
                        2 => {
                            let len = if self.read_fn(enums) == "message" { "get_size" } else { "len" };
                            writeln!(w, "{} + sizeof_var_length(self.{}.iter().map(|s| sizeof_var_length(s.{}())).sum::<usize>())", 
                                     tag_size, self.name, len)?;
                        }
                        e => panic!("expecting wire type number, got: {}", e),
                    }
                } else {
                    match self.wire_type_num(enums) {
                        0 => writeln!(w, "self.{}.iter().map(|s| {} + sizeof_{}(s)).sum::<usize>()", 
                                      self.name, tag_size, self.read_fn(enums))?,
                        1 => writeln!(w, "({} + 8) * self.{}.len()", tag_size, self.name)?,
                        5 => writeln!(w, "({} + 4) * self.{}.len()", tag_size, self.name)?,
                        2 => {
                            let len = if self.read_fn(enums) == "message" { "get_size" } else { "len" };
                            writeln!(w, "self.{}.iter().map(|s| {} + sizeof_var_length(s.{}())).sum::<usize>()", 
                                     self.name, tag_size, len)?;
                        }
                        e => panic!("expecting wire type number, got: {}", e),
                    }
                }
            }
        }
        Ok(())
    }

    fn write_inner_get_size<W: Write>(&self, w: &mut W, enums: &[&str], s: &str, as_ref: &str) -> IoResult<()> {
        let tag_size = sizeof_varint(self.tag(enums));
        match self.wire_type_num(enums) {
            0 => {
                let read_fn = self.read_fn(enums);
                let as_enum = if read_fn == "enum" { " as i32" } else { "" };
                write!(w, "{} + sizeof_{}({}{}{})", tag_size, read_fn, as_ref, s, as_enum)?
            },
            1 => write!(w, "{} + 8", tag_size)?,
            5 => write!(w, "{} + 4", tag_size)?,
            2 => {
                let len = if self.read_fn(enums) == "message" { "get_size" } else { "len" };
                write!(w, "{} + sizeof_var_length({}.{}())", tag_size, s, len)?;
            }
            e => panic!("expecting wire type number, got: {}", e),
        }
        Ok(())
    }

    fn write_write<W: Write>(&self, w: &mut W, enums: &[&str]) -> IoResult<()> {
        let tag = self.tag(enums);
        let use_ref = match self.rust_type() {
            "i32" | "i64" | "u32" | "u64" | "f32" | "f64" | "bool" => false,
            t => !enums.contains(&t),
        };
        let read_fn = self.read_fn(enums);
        let as_enum = if read_fn == "enum" { " as i32" } else { "" };
        match self.frequency {
            Frequency::Required => {
                let r = if use_ref { "&" } else { "" };
                writeln!(w, "        r.write_{}_with_tag({}, {}self.{}{})?;", read_fn, tag, r, self.name, as_enum)?;
            },
            Frequency::Optional => {
                let r = if use_ref { "" } else { "*" };
                writeln!(w, "        if let Some(ref s) = self.{} {{ r.write_{}_with_tag({}, {}s{})?; }}", 
                         self.name, read_fn, tag, r, as_enum)?;
            }
            Frequency::Repeated => {
                if self.packed {
                    writeln!(w, "        r.write_tag({})?;", tag)?;
                    writeln!(w, "        r.write_packed_repeated_field(&self.{}, |r, m| r.write_{}({}m{}), |_| 1)?;", 
                             self.name, read_fn, if use_ref { "" } else { "*" }, as_enum)?;
                } else {
                    writeln!(w, "        for s in &self.{} {{ r.write_{}_with_tag({}, {}s{})? }}", 
                             self.name, read_fn, tag, if use_ref { "" } else { "*" }, as_enum)?;
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Message<'a> {
    pub name: &'a str,
    pub fields: Vec<Field<'a>>,
}

impl<'a> Message<'a> {
    fn write_definition<W: Write>(&self, w: &mut W) -> IoResult<()> {
        writeln!(w, "#[derive(Debug, Default, PartialEq, Clone)]")?;
        writeln!(w, "pub struct {} {{", self.name)?;
        for f in &self.fields {
            f.write_definition(w)?;
        }
        writeln!(w, "}}")
    }

    fn write_impl_message_read<W: Write>(&self, w: &mut W, enums: &[&str]) -> IoResult<()> {
        writeln!(w, "impl MessageRead for {} {{", self.name)?;
        self.write_from_reader(w, enums)?;
        writeln!(w, "}}")
    }

    fn write_impl_message_write<W: Write>(&self, w: &mut W, enums: &[&str]) -> IoResult<()> {
        writeln!(w, "impl MessageWrite for {} {{", self.name)?;
        self.write_get_size(w, enums)?;
        writeln!(w, "")?;
        self.write_write_message(w, enums)?;
        writeln!(w, "}}")
    }

    fn write_from_reader<W: Write>(&self, w: &mut W, enums: &[&str]) -> IoResult<()> {
        writeln!(w, "    fn from_reader<R: Read>(mut r: &mut Reader<R>) -> Result<Self> {{")?;
        writeln!(w, "        let mut msg = Self::default();")?;
        writeln!(w, "        while !r.is_eof() {{")?;
        writeln!(w, "            match r.next_tag() {{")?;
        for f in &self.fields {
            write!(w, "                ")?;
            f.write_match_tag(w, enums)?;
        }
        writeln!(w, "                Ok(t) => {{ r.read_unknown(t)?; }}")?;
        writeln!(w, "                Err(e) => return Err(e),")?;
        writeln!(w, "            }}")?;
        writeln!(w, "        }}")?;
        writeln!(w, "        Ok(msg)")?;
        writeln!(w, "    }}")
    }

    fn write_get_size<W: Write>(&self, w: &mut W, enums: &[&str]) -> IoResult<()> {
        writeln!(w, "    fn get_size(&self) -> usize {{")?;
        for (i, f) in self.fields.iter().enumerate() {
            f.write_get_size(w, enums, i == 0)?;
        }
        writeln!(w, "    }}")
    }

    fn write_write_message<W: Write>(&self, w: &mut W, enums: &[&str]) -> IoResult<()> {
        writeln!(w, "    fn write_message<W: Write>(&self, r: &mut Writer<W>) -> Result<()> {{")?;
        for f in self.fields.iter() {
            f.write_write(w, enums)?;
        }
        writeln!(w, "        Ok(())")?;
        writeln!(w, "    }}")
    }

    fn is_leaf(&self, leaf_messages: &[&str], enums: &[&str]) -> bool {
        self.fields.iter().all(|f| f.is_leaf(leaf_messages, enums))
    }
}

#[derive(Debug)]
pub struct Enumerator<'a> {
    pub name: &'a str,
    pub fields: Vec<(&'a str, i32)>,
}

impl<'a> Enumerator<'a> {
    fn write_definition<W: Write>(&self, w: &mut W) -> IoResult<()> {
        writeln!(w, "#[derive(Debug, PartialEq, Eq, Clone, Copy)]")?;
        writeln!(w, "pub enum {} {{", self.name)?;
        for &(f, number) in &self.fields {
            writeln!(w, "    {} = {},", f, number)?;
        }
        writeln!(w, "}}")
    }

    fn write_impl_default<W: Write>(&self, w: &mut W) -> IoResult<()> {
        writeln!(w, "impl Default for {} {{", self.name)?;
        writeln!(w, "    fn default() -> Self {{")?;
        // TODO: check with default field and return error if there is no field
        writeln!(w, "        {}::{}", self.name, self.fields[0].0)?;
        writeln!(w, "    }}")?;
        writeln!(w, "}}")
    }

    fn write_from_i32<W: Write>(&self, w: &mut W) -> IoResult<()> {
        writeln!(w, "impl From<i32> for {} {{", self.name)?;
        writeln!(w, "    fn from(i: i32) -> Self {{")?;
        writeln!(w, "        match i {{")?;
        for &(f, number) in &self.fields {
            writeln!(w, "            {} => {}::{},", number, self.name, f)?;
        }
        writeln!(w, "            _ => Self::default(),")?;
        writeln!(w, "        }}")?;
        writeln!(w, "    }}")?;
        writeln!(w, "}}")
    }
}

#[derive(Debug)]
pub enum MessageOrEnum<'a> {
    Msg(Message<'a>),
    Enum(Enumerator<'a>),
    Ignore,
}

#[derive(Debug)]
pub struct FileDescriptor<'a> {
    pub message_and_enums: Vec<MessageOrEnum<'a>>,
    pub messages: Vec<Message<'a>>,
    pub enums: Vec<Enumerator<'a>>,
}

impl<'a> FileDescriptor<'a> {

    pub fn from_bytes(b: &'a [u8]) -> Result<FileDescriptor<'a>, IError<u32>> {
        file_descriptor(b).to_full_result()
    }

    pub fn write<W: Write>(&self, w: &mut W, filename: &str) -> IoResult<()> {
        
        println!("Found {} messages, and {} enums", self.messages.len(), self.enums.len());

        writeln!(w, "//! Automatically generated rust module for '{}' file", filename)?;
        writeln!(w, "")?;
        writeln!(w, "#![allow(non_snake_case)]")?;
        writeln!(w, "#![allow(non_upper_case_globals)]")?;
        writeln!(w, "")?;
        writeln!(w, "use std::io::{{Read, Write}};")?;
        writeln!(w, "use quick_protobuf::{{MessageRead, MessageWrite, Reader, Writer, Result}};")?;
        writeln!(w, "use quick_protobuf::sizeofs::*;")?;

        let enums = self.enums.iter().map(|e| e.name).collect::<Vec<_>>();
        for m in &self.enums {
            writeln!(w, "")?;
            m.write_definition(w)?;
            writeln!(w, "")?;
            m.write_impl_default(w)?;
            writeln!(w, "")?;
            m.write_from_i32(w)?;
        }
        for m in &self.messages {
            writeln!(w, "")?;
            m.write_definition(w)?;
            writeln!(w, "")?;
            m.write_impl_message_read(w, &enums)?;
            writeln!(w, "")?;
            m.write_impl_message_write(w, &enums)?;
        }
        Ok(())
    }

    pub fn break_cycles(&mut self) {
        let mut messages = Vec::new();
        let mut enums = Vec::new();
        for m in self.message_and_enums.drain(..) {
            match m {
                MessageOrEnum::Msg(m) => messages.push(m),
                MessageOrEnum::Enum(e) => enums.push(e),
                _ => (),
            }
        }
        self.messages = messages;
        self.enums = enums;

        let message_names = self.messages.iter().map(|m| m.name.to_string()).collect::<Vec<_>>();
        let enum_names = self.enums.iter().map(|m| m.name.to_string()).collect::<Vec<_>>();
        let enums = enum_names.iter().map(|n| &**n).collect::<Vec<_>>();

        let mut leaf_messages = Vec::new();
        let mut undef_messages = (0..self.messages.len()).collect::<Vec<_>>();
        while !undef_messages.is_empty() {
            let len = undef_messages.len();
            let mut new_undefs = Vec::new();
            for i in undef_messages.into_iter() {
                if self.messages[i].is_leaf(&leaf_messages, &enums) {
                    leaf_messages.push(&*message_names[i])
                } else {
                    new_undefs.push(i);
                }
            }
            undef_messages = new_undefs;
            if len == undef_messages.len() {
                // try boxing messages, one by one ...
                let k = undef_messages.pop().unwrap();
                {
                    let m = self.messages.get_mut(k).unwrap();
                    for f in m.fields.iter_mut() {
                        if !f.is_leaf(&leaf_messages, &enums) {
                            f.boxed = true;
                        }
                    }
                }
            }
        }
    }
}
