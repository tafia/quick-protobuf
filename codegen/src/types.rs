use std::io::{Read, Write, BufReader, BufWriter};
use std::path::{Path, PathBuf, Component};
use std::fs::File;

use errors::{Result, ErrorKind};
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

#[derive(Debug, Clone, Copy)]
pub enum Syntax {
    Proto2,
    Proto3,
}

impl Default for Syntax {
    fn default() -> Syntax {
        Syntax::Proto2
    }
}

#[derive(Debug, Clone)]
pub enum Frequency {
    Optional,
    Repeated,
    Required,
}

#[derive(Debug, Clone)]
pub enum FieldType {
    Int32,
    Int64,
    Uint32,
    Uint64,
    Sint32,
    Sint64,
    Bool,
    Enum(String),
    Fixed64,
    Sfixed64,
    Double,
    String_,
    Bytes,
    Message(String),
    Fixed32,
    Sfixed32,
    Float,
    Map(Box<(FieldType, FieldType)>),
}

impl FieldType {

    fn is_numeric(&self) -> bool {
        match *self {
            FieldType::Int32 |
            FieldType::Int64 |
            FieldType::Uint32 |
            FieldType::Uint64 |
            FieldType::Sint32 |
            FieldType::Sint64 |
            FieldType::Fixed64 |
            FieldType::Sfixed64 |
            FieldType::Double |
            FieldType::Fixed32 |
            FieldType::Sfixed32 |
            FieldType::Float => true,
            _ => false,
        }
    }

    fn is_message(&self) -> bool {
        match *self {
            FieldType::Message(_) => true,
            _ => false,
        }
    }

    fn is_enum(&self) -> bool {
        match *self {
            FieldType::Enum(_) => true,
            _ => false,
        }
    }

    fn is_cow(&self) -> bool {
        match *self {
            FieldType::Bytes | FieldType::String_ => true,
            _ => false,
        }
    }

    fn wire_type_num(&self, packed: bool) -> u32 {
        if packed {
            2
        } else {
            self.wire_type_num_non_packed()
        }
    }

    fn wire_type_num_non_packed(&self) -> u32 {
        match *self {
            FieldType::Int32 | FieldType::Sint32 | FieldType::Int64 |
            FieldType::Sint64 | FieldType::Uint32 | FieldType::Uint64 |
            FieldType::Bool | FieldType::Enum(_) => 0,
            FieldType::Fixed32 | FieldType::Sfixed32 | FieldType::Float => 5,
            FieldType::Fixed64 | FieldType::Sfixed64 | FieldType::Double => 1,
            FieldType::String_ | FieldType::Bytes | FieldType::Message(_) => 2,
            FieldType::Map(..) => unimplemented!(),
        }
    }

    fn proto_type(&self) -> &str {
        match *self {
            FieldType::Int32 => "int32",
            FieldType::Sint32 => "sint32",
            FieldType::Int64 => "int64",
            FieldType::Sint64 => "sint64",
            FieldType::Uint32 => "uint32",
            FieldType::Uint64 => "uint64",
            FieldType::Bool => "bool",
            FieldType::Enum(_) => "enum",
            FieldType::Fixed32 => "fixed32",
            FieldType::Sfixed32 => "sfixed32",
            FieldType::Float => "float",
            FieldType::Fixed64 => "fixed64",
            FieldType::Sfixed64 => "sfixed64",
            FieldType::Double => "double",
            FieldType::String_ => "string",
            FieldType::Bytes => "bytes",
            FieldType::Message(_) => "message",
            FieldType::Map(..) => unimplemented!(),
        }
    }

    fn is_fixed_size(&self) -> bool {
        match self.wire_type_num_non_packed() {
            1 | 5 => true,
            _ => false,
        }
    }

    /// Searches for message corresponding to the current type
    ///
    /// Searches first basic name then within nested messages
    fn find_message<'a, 'b>(&'a self, msgs: &'b [Message]) -> Option<&'b Message> {
        match *self {
            FieldType::Message(ref m) => {
                let mut found = match m.rfind('.') {
                    Some(p) => {
                        let package = &m[..p];
                        let name = &m[(p + 1)..];
                        msgs.iter().find(|m| m.package == package && m.name == name)
                    },
                    None => msgs.iter().find(|m2| m2.name == &m[..]),
                };

                if found.is_none() {
                    // recursively search into nested messages
                    for m in msgs {
                        found = self.find_message(&m.messages);
                        if found.is_some() { break; }
                    }
                }

                found
            },
            _ => None,
        }
    }

    fn find_enum<'a, 'b>(&'a self, enums: &'b [Enumerator]) -> Option<&'b Enumerator> {
        match *self {
            FieldType::Enum(ref e) => enums.iter().find(|m| m.name == &e[..]),
            _ => None,
        }
    }

    fn has_lifetime(&self, msgs: &[Message]) -> bool {
        if self.is_cow() { return true; }
        self.find_message(msgs).map_or(false, |m| m.has_lifetime(msgs))
    }

    fn rust_type(&self, msgs: &[Message]) -> String {
        match *self {
            FieldType::Int32 | FieldType::Sint32 | FieldType::Sfixed32 => "i32".to_string(),
            FieldType::Int64 | FieldType::Sint64 | FieldType::Sfixed64 => "i64".to_string(),
            FieldType::Uint32 | FieldType::Fixed32 => "u32".to_string(),
            FieldType::Uint64 | FieldType::Fixed64 => "u64".to_string(),
            FieldType::Double => "f64".to_string(),
            FieldType::Float => "f32".to_string(),
            FieldType::String_ => "Cow<'a, str>".to_string(),
            FieldType::Bytes => "Cow<'a, [u8]>".to_string(),
            FieldType::Bool => "bool".to_string(),
            FieldType::Enum(ref e) => e.replace(".", "::"),
            FieldType::Message(ref msg) => match self.find_message(msgs) {
                Some(m) => {
                    let lifetime = if m.has_lifetime(msgs) { "<'a>" } else { "" };
                    let package = m.package.split('.').filter(|p| !p.is_empty())
                        .map(|p| format!("mod_{}::", p)).collect::<String>();
                    format!("{}{}{}", package, m.name, lifetime)
                },
                None => unreachable!(format!("Could not find message {}", msg)),
            },
            FieldType::Map(ref t) => {
                let &(ref key, ref value) = &**t;
                format!("HashMap<{}, {}>", key.rust_type(msgs), value.rust_type(msgs))
            }
        }
    }

    fn read_fn(&self, msgs: &[Message]) -> String {
        match self.find_message(msgs) {
            Some(m) if m.package.is_empty()=> {
                format!("read_message(bytes, {}::from_reader)", m.name)
            },
            Some(m) => {
                format!("read_message(bytes, {}{}::from_reader)", 
                        m.package.split('.').map(|p| format!("mod_{}::", p)).collect::<String>(), m.name)
            }
            None => {
                format!("read_{}(bytes)", self.proto_type())
            }
        }
    }

}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub frequency: Frequency,
    pub typ: FieldType,
    pub number: i32,
    pub default: Option<String>,
    pub packed: Option<bool>,
    pub boxed: bool,
    pub deprecated: bool,
}

impl Field {

    fn packed(&self) -> bool {
        self.packed.unwrap_or(false)
    }

    /// searches if the message must be boxed
    fn is_leaf(&self, leaf_messages: &[String]) -> bool {
        match self.typ {
            FieldType::Message(ref s) => {
                match self.frequency {
                    Frequency::Repeated | Frequency::Required => true,
                    _ => {
                        let typ = match s.rfind('.') {
                            Some(p) => &s[p + 1..],
                            None => &s[..],
                        };
                        leaf_messages.iter().any(|m| &*m == &typ)
                    }
                }
            }
            _ => true,
        }
    }

    fn has_unregular_default(&self, enums: &[Enumerator], msgs: &[Message]) -> bool {
        match self.default {
            None => false,
            Some(ref d) => match &*self.typ.rust_type(msgs) {
                "i32" | "i64" | "u32" | "u64" | "f32" | "f64" => d.parse::<f32>().unwrap() != 0.,
                "bool" => *d != "false",
                "Cow<'a, str>" => *d != "\"\"",
                "Cow<'a, [u8]>" => *d != "[]",
                t => self.typ.find_enum(enums).map_or(false, |e| t != e.fields[0].0),
            } 
        }
    }

    fn tag(&self) -> u32 {
        (self.number as u32) << 3 | self.typ.wire_type_num(self.packed())
    }

    fn write_definition<W: Write>(&self, w: &mut W, msgs: &[Message]) -> Result<()> {
        write!(w, "    pub {}: ", self.name)?;
        match self.frequency {
            Frequency::Optional if self.boxed => writeln!(w, "Option<Box<{}>>,", self.typ.rust_type(msgs))?,
            Frequency::Optional if self.default.is_some() => writeln!(w, "{},", self.typ.rust_type(msgs))?,
            Frequency::Optional => writeln!(w, "Option<{}>,", self.typ.rust_type(msgs))?,
            Frequency::Repeated => writeln!(w, "Vec<{}>,", self.typ.rust_type(msgs))?,
            Frequency::Required => writeln!(w, "{},", self.typ.rust_type(msgs))?,
        }
        Ok(())
    }

    fn write_match_tag<W: Write>(&self, w: &mut W, msgs: &[Message]) -> Result<()> {
        let val = if self.typ.is_cow() {
            format!("Cow::Borrowed(r.{}?)", self.typ.read_fn(msgs))
        } else {
            format!("r.{}?", self.typ.read_fn(msgs))
        };

        write!(w, "                Ok({}) => msg.{}", self.tag(), self.name)?;
        match self.frequency {
            Frequency::Required => writeln!(w, " = {},", val)?,
            Frequency::Optional if self.default.is_some() => writeln!(w, " = {},", val)?,
            Frequency::Optional if self.boxed => writeln!(w, " = Some(Box::new({})),", val)?,
            Frequency::Optional => writeln!(w, " = Some({}),", val)?,
            Frequency::Repeated if self.packed() => {
                writeln!(w, " = r.read_packed(bytes, |r, bytes| r.{})?,", self.typ.read_fn(msgs))?
            }
            Frequency::Repeated => writeln!(w, ".push({}),", val)?,
        }
        Ok(())
    }

    fn write_get_size<W: Write>(&self, w: &mut W, is_first: bool) -> Result<()> {
        if is_first { 
            write!(w, "        ")?;
        } else { 
            write!(w, "        + ")?;
        }
        match self.frequency {
            Frequency::Required => {
                self.write_inner_get_size(w, &format!("self.{}", self.name), "")?;
                writeln!(w, "")?;
            }
            Frequency::Optional => {
                match self.default.as_ref() {
                    None => {
                        if self.typ.is_fixed_size() {
                            write!(w, "self.{}.as_ref().map_or(0, |_| ", self.name)?;
                        } else {
                            write!(w, "self.{}.as_ref().map_or(0, |m| ", self.name)?;
                        }
                        self.write_inner_get_size(w, "m", "*")?;
                        writeln!(w, ")")?;
                    }
                    Some(d) => {
                        write!(w, "if self.{} == {} {{ 0 }} else {{", self.name, d)?;
                        self.write_inner_get_size(w, &format!("self.{}", self.name), "")?;
                        writeln!(w, "}}")?;
                    }
                }
            }
            Frequency::Repeated => {
                let tag_size = sizeof_varint(self.tag());
                let get_type = self.typ.proto_type();
                let as_enum = if self.typ.is_enum() { " as i32" } else { "" };
                if self.packed() {
                    write!(w, "if self.{}.is_empty() {{ 0 }} else {{ ", self.name)?;
                    match self.typ.wire_type_num_non_packed() {
                        0 => write!(w, "{} + sizeof_var_length(self.{}.iter().map(|s| sizeof_{}(*s{})).sum::<usize>())", 
                                    tag_size, self.name, get_type, as_enum)?,
                        1 => write!(w, "{} + sizeof_var_length(self.{}.len() * 8)", tag_size, self.name)?,
                        5 => write!(w, "{} + sizeof_var_length(self.{}.len() * 4)", tag_size, self.name)?,
                        2 => {
                            let len = if self.typ.is_message() { "get_size" } else { "len" };
                            write!(w, "{} + sizeof_var_length(self.{}.iter().map(|s| sizeof_var_length(s.{}())).sum::<usize>())", 
                                   tag_size, self.name, len)?;
                        }
                        e => panic!("expecting wire type number, got: {}", e),
                    }
                    writeln!(w, " }}")?;
                } else {
                    match self.typ.wire_type_num_non_packed() {
                        0 => writeln!(w, "self.{}.iter().map(|s| {} + sizeof_{}(*s{})).sum::<usize>()", 
                                      self.name, tag_size, get_type, as_enum)?,
                        1 => writeln!(w, "({} + 8) * self.{}.len()", tag_size, self.name)?,
                        5 => writeln!(w, "({} + 4) * self.{}.len()", tag_size, self.name)?,
                        2 => {
                            let len = if self.typ.is_message() { "get_size" } else { "len" };
                            writeln!(w, "self.{}.iter().map(|s| {} + sizeof_var_length(s.{}())).sum::<usize>()", 
                                     self.name, tag_size, len)?;
                        }
                        e => return Err(ErrorKind::InvalidWireType(e).into()),
                    }
                }
            }
        }
        Ok(())
    }

    fn write_inner_get_size<W: Write>(&self, w: &mut W, s: &str, as_ref: &str) -> Result<()> {
        let tag_size = sizeof_varint(self.tag());
        match self.typ.wire_type_num_non_packed() {
            0 => {
                let get_type = self.typ.proto_type();
                let as_enum = if self.typ.is_enum() { " as i32" } else { "" };
                write!(w, "{} + sizeof_{}({}{}{})", tag_size, get_type, as_ref, s, as_enum)?
            },
            1 => write!(w, "{} + 8", tag_size)?,
            5 => write!(w, "{} + 4", tag_size)?,
            2 => {
                let len = if self.typ.is_message() { "get_size" } else { "len" };
                if self.packed() {
                    write!(w, "if s.is_empty() {{ 0 }} else {{ {} + sizeof_var_length({}.{}()) }}", tag_size, s, len)?;
                } else {
                    write!(w, "{} + sizeof_var_length({}.{}())", tag_size, s, len)?;
                }
            }
            e => return Err(ErrorKind::InvalidWireType(e).into()),
        }
        Ok(())
    }

    fn write_write<W: Write>(&self, w: &mut W) -> Result<()> {
        let tag = self.tag();
        let use_ref = self.typ.wire_type_num_non_packed() == 2;
        let get_type = self.typ.proto_type();
        let as_enum = if self.typ.is_enum() { " as i32" } else { "" };
        match self.frequency {
            Frequency::Required => {
                let r = if use_ref { "&" } else { "" };
                writeln!(w, "        r.write_{}_with_tag({}, {}self.{}{})?;", get_type, tag, r, self.name, as_enum)?;
            },
            Frequency::Optional => {
                let r = if use_ref { 
                    if self.boxed { "&**" } else { "" }
                } else { 
                    "*" 
                };
                match self.default.as_ref() {
                    None => {
                        writeln!(w, "        if let Some(ref s) = self.{} {{ r.write_{}_with_tag({}, {}s{})?; }}", 
                                 self.name, get_type, tag, r, as_enum)?;
                    },
                    Some(d) => {
                        writeln!(w, "        if self.{} != {} {{ r.write_{}_with_tag({}, self.{0}{})?; }}", 
                                 self.name, d, get_type, tag, as_enum)?;
                    }
                }
            }
            Frequency::Repeated if self.packed() => {
                let size_of = match self.typ {
                    FieldType::Message(_) => "&|m| sizeof_var_length(m.get_size())".to_string(),
                    FieldType::String_ | 
                        FieldType::Bytes => "&|m| sizeof_var_length(m.len())".to_string(),
                    _ => format!("&|m| sizeof_{}(*m)", get_type),

                };
                writeln!(w, "        r.write_packed_repeated_field_with_tag({}, &self.{}, |r, m| r.write_{}({}m{}), {})?;",
                         tag, self.name, get_type, if use_ref { "" } else { "*" }, as_enum, size_of)?
            }
            Frequency::Repeated => {
                writeln!(w, "        for s in &self.{} {{ r.write_{}_with_tag({}, {}s{})? }}", 
                         self.name, get_type, tag, if use_ref { "" } else { "*" }, as_enum)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct Message {
    pub name: String,
    pub fields: Vec<Field>,
    pub reserved_nums: Option<Vec<i32>>,
    pub reserved_names: Option<Vec<String>>,
    pub imported: bool,
    pub package: String,        // package from imports + nested items
    pub messages: Vec<Message>, // nested messages
    pub enums: Vec<Enumerator>, // nested enums
}

impl Message {

    fn is_leaf(&self, leaf_messages: &[String]) -> bool {
        self.imported || self.fields.iter().all(|f| f.is_leaf(leaf_messages) || f.deprecated)
    }

    fn has_lifetime(&self, msgs: &[Message]) -> bool {
        self.fields.iter().any(|f| match f.typ {
            FieldType::Message(ref m) => &m[..] != self.name && f.typ.has_lifetime(msgs),
            _ => f.typ.is_cow(),
        })
    }

    fn write_definition<W: Write>(&self, w: &mut W, enums: &[Enumerator], msgs: &[Message]) -> Result<()> {
        if self.can_derive_default(enums, msgs) {
            writeln!(w, "#[derive(Debug, Default, PartialEq, Clone)]")?;
        } else {
            writeln!(w, "#[derive(Debug, PartialEq, Clone)]")?;
        }
        if self.has_lifetime(msgs) {
            writeln!(w, "pub struct {}<'a> {{", self.name)?;
        } else {
            writeln!(w, "pub struct {} {{", self.name)?;
        }
        for f in self.fields.iter().filter(|f| !f.deprecated) {
            f.write_definition(w, msgs)?;
        }
        writeln!(w, "}}")?;
        Ok(())
    }

    fn can_derive_default(&self, enums: &[Enumerator], msgs: &[Message]) -> bool {
        self.fields.iter().all(|f| f.deprecated || !f.has_unregular_default(enums, msgs))
    }

    fn write_impl_message_read<W: Write>(&self, w: &mut W, msgs: &[Message]) -> Result<()> {
        if self.has_lifetime(msgs) {
            writeln!(w, "impl<'a> {}<'a> {{", self.name)?;
            writeln!(w, "    pub fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {{")?;
        } else {
            writeln!(w, "impl {} {{", self.name)?;
            writeln!(w, "    pub fn from_reader(r: &mut BytesReader, bytes: &[u8]) -> Result<Self> {{")?;
        }
        writeln!(w, "        let mut msg = Self::default();")?;
        writeln!(w, "        while !r.is_eof() {{")?;
        writeln!(w, "            match r.next_tag(bytes) {{")?;
        for f in self.fields.iter().filter(|f| !f.deprecated) {
            f.write_match_tag(w, msgs)?;
        }
        writeln!(w, "                Ok(t) => {{ r.read_unknown(bytes, t)?; }}")?;
        writeln!(w, "                Err(e) => return Err(e),")?;
        writeln!(w, "            }}")?;
        writeln!(w, "        }}")?;
        writeln!(w, "        Ok(msg)")?;
        writeln!(w, "    }}")?;
        writeln!(w, "}}")?;

        // TODO: write impl default when special default? 
        // alternatively set the default value directly when reading

        Ok(())
    }

    fn write_impl_message_write<W: Write>(&self, w: &mut W, msgs: &[Message]) -> Result<()> {
        if self.has_lifetime(msgs) {
            writeln!(w, "impl<'a> MessageWrite for {}<'a> {{", self.name)?;
        } else {
            writeln!(w, "impl MessageWrite for {} {{", self.name)?;
        }
        self.write_get_size(w)?;
        writeln!(w, "")?;
        self.write_write_message(w)?;
        writeln!(w, "}}")?;
        Ok(())
    }

    fn write_get_size<W: Write>(&self, w: &mut W) -> Result<()> {
        writeln!(w, "    fn get_size(&self) -> usize {{")?;
        for (i, f) in self.fields.iter().filter(|f| !f.deprecated).enumerate() {
            f.write_get_size(w, i == 0)?;
        }
        writeln!(w, "    }}")?;
        Ok(())
    }

    fn write_write_message<W: Write>(&self, w: &mut W) -> Result<()> {
        writeln!(w, "    fn write_message<W: Write>(&self, r: &mut Writer<W>) -> Result<()> {{")?;
        for f in self.fields.iter().filter(|f| !f.deprecated) {
            f.write_write(w)?;
        }
        writeln!(w, "        Ok(())")?;
        writeln!(w, "    }}")?;
        Ok(())
    }

    fn sanity_checks(&self) -> Result<()> {
        // checks for reserved fields
        for f in &self.fields {
            if self.reserved_names.as_ref().map_or(false, |names| names.contains(&f.name)) || 
                self.reserved_nums.as_ref().map_or(false, |nums| nums.contains(&f.number)) {
                return Err(ErrorKind::InvalidMessage(
                        format!("Error in message {}\nField {:?} conflict with reserved fields",
                                self.name, f)).into());
            }
        }
        Ok(())
    }

    fn set_package(&mut self, package: &str) {
        // set package = current_package.package.name to nested messages
        if package.is_empty() {
            for m in &mut self.messages { m.set_package(&self.name); }
        } else {
            self.package = package.to_string();
            let child_package = format!("{}.{}", package, self.name);
            for m in &mut self.messages { m.set_package(&child_package); }
        }
    }

    /// Searches for a matching message in all message
    ///
    /// If none is found, 
    fn set_enums(&mut self, msgs: &[Message]) {
        for f in &mut self.fields {
            if f.typ.find_message(&msgs).is_none() {
                if let FieldType::Message(m) = f.typ.clone() {
                    f.typ = FieldType::Enum(m);
                    f.boxed = false;
                }
            }
        }
        for m in &mut self.messages {
            m.set_enums(msgs);
        }
    }

}

#[derive(Debug, Clone)]
pub struct Enumerator {
    pub name: String,
    pub fields: Vec<(String, i32)>,
    pub imported: bool,
    pub package: String,
}

impl Enumerator {
    fn write_definition<W: Write>(&self, w: &mut W) -> Result<()> {
        writeln!(w, "#[derive(Debug, PartialEq, Eq, Clone, Copy)]")?;
        writeln!(w, "pub enum {} {{", self.name)?;
        for &(ref f, ref number) in &self.fields {
            writeln!(w, "    {} = {},", f, number)?;
        }
        writeln!(w, "}}")?;
        Ok(())
    }

    fn write_impl_default<W: Write>(&self, w: &mut W) -> Result<()> {
        writeln!(w, "impl Default for {} {{", self.name)?;
        writeln!(w, "    fn default() -> Self {{")?;
        // TODO: check with default field and return error if there is no field
        writeln!(w, "        {}::{}", self.name, self.fields[0].0)?;
        writeln!(w, "    }}")?;
        writeln!(w, "}}")?;
        Ok(())
    }

    fn write_from_i32<W: Write>(&self, w: &mut W) -> Result<()> {
        writeln!(w, "impl From<i32> for {} {{", self.name)?;
        writeln!(w, "    fn from(i: i32) -> Self {{")?;
        writeln!(w, "        match i {{")?;
        for &(ref f, ref number) in &self.fields {
            writeln!(w, "            {} => {}::{},", number, self.name, f)?;
        }
        writeln!(w, "            _ => Self::default(),")?;
        writeln!(w, "        }}")?;
        writeln!(w, "    }}")?;
        writeln!(w, "}}")?;
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct FileDescriptor {
    pub import_paths: Vec<PathBuf>,
    pub package: String,
    pub syntax: Syntax,
    pub messages: Vec<Message>,
    pub enums: Vec<Enumerator>,
}

impl FileDescriptor {

    pub fn write_proto<P: AsRef<Path>>(in_file: P, out_file: P) -> Result<()> {
        let mut desc = FileDescriptor::read_proto(&in_file)?;
        desc.fetch_imports(in_file.as_ref())?;

        let mut leaf_messages = Vec::new();
        break_cycles(&mut desc.messages, &mut leaf_messages);

        desc.sanity_checks(in_file.as_ref())?;
        desc.set_enums();
        desc.set_defaults();

        let name = in_file.as_ref().file_name().and_then(|e| e.to_str()).unwrap();
        let mut w = BufWriter::new(File::create(out_file)?);
        desc.write(&mut w, name)?;
        Ok(())
    }

    /// Opens a proto file, reads it and returns raw parsed data
    fn read_proto<P: AsRef<Path>>(in_file: P) -> Result<FileDescriptor> {
        let mut buf = Vec::new();
        {
            let f = File::open(&in_file)?;
            let mut reader = BufReader::new(f);
            reader.read_to_end(&mut buf)?;
        }
        let desc = file_descriptor(&buf).to_result()?;
        Ok(desc)
    }

    fn sanity_checks(&self, file: &Path) -> Result<()> {
        for i in &self.import_paths {
            // search if the corresponding file exists
            if i.is_file() {
                if !get_imported_path(file, i).exists() {
                    return Err(ErrorKind::InvalidImport(
                            format!("File {} not found", i.display())).into());
                }
            }
        }
        for m in &self.messages {
            m.sanity_checks()?;
        }
        Ok(())
    }

    /// Get messages and enums from imports
    fn fetch_imports(&mut self, in_file: &Path) -> Result<()> {
        for m in &mut self.messages {
            m.set_package("");
        }
        for p in &self.import_paths {
            let import_path = get_imported_path(&in_file, p);
            let mut f = FileDescriptor::read_proto(&import_path)?;
            f.fetch_imports(&import_path)?;

            // if the proto has a packge then the names will be prefixed
            let package = f.package.clone();
            self.messages.extend(f.messages.drain(..).map(|mut m| {
                m.set_package(&package);
                m.imported = true;
                m
            }));
            self.enums.extend(f.enums.drain(..).map(|mut e| {
                e.package = package.clone();
                e.imported = true;
                e
            }));
        }
        Ok(())
    }

    fn set_defaults(&mut self) {
        // if proto3, then changes several defaults
        if let Syntax::Proto3 = self.syntax {
            for m in &mut self.messages {
                for f in &mut m.fields {
                    if f.packed.is_none() { 
                        if let Frequency::Repeated = f.frequency { 
                            f.packed = Some(true); 
                        }
                    }
                    if f.default.is_none() && f.typ.is_numeric() { 
                        f.default = Some("0".to_string());
                    }
                }
            }
        }
    }

    fn set_enums(&mut self) {
        // this is very inefficient but we don't care ...
        let msgs = self.messages.clone();

        for m in &mut self.messages {
            m.set_enums(&msgs);
        }
    }

    fn write<W: Write>(&self, w: &mut W, filename: &str) -> Result<()> {
        println!("Found {} messages, and {} enums", self.messages.len(), self.enums.len());
        self.write_headers(w, filename)?;
        self.write_package_start(w)?;
        self.write_uses(w)?;
        self.write_imports(w)?;
        self.write_enums(w)?;
        self.write_messages(w)?;
        self.write_package_end(w)?;
        println!("Done processing {}", filename);
        Ok(())
    }

    fn write_headers<W: Write>(&self, w: &mut W, filename: &str) -> Result<()> {
        writeln!(w, "//! Automatically generated rust module for '{}' file", filename)?;
        writeln!(w, "")?;
        writeln!(w, "#![allow(non_snake_case)]")?;
        writeln!(w, "#![allow(non_upper_case_globals)]")?;
        writeln!(w, "#![allow(non_camel_case_types)]")?;
        writeln!(w, "")?;
        Ok(())
    }

    fn write_uses<W: Write>(&self, w: &mut W) -> Result<()> {
        writeln!(w, "use std::io::{{Write}};")?;
        if self.messages.iter().any(|m| m.fields.iter().any(|f| f.typ.is_cow())) {
            writeln!(w, "use std::borrow::Cow;")?;
        }
        writeln!(w, "use quick_protobuf::{{MessageWrite, BytesReader, Writer, Result}};")?;
        writeln!(w, "use quick_protobuf::sizeofs::*;")?;
        Ok(())
    }

    fn write_imports<W: Write>(&self, w: &mut W) -> Result<()> {
        if self.import_paths.is_empty() {
            return Ok(());
        }
        writeln!(w, "")?;
        for i in &self.import_paths {
            write!(w, "use super::")?;
            for c in i.components() {
                match c {
                    Component::RootDir | Component::Prefix(_) => return Err(ErrorKind::InvalidImport(
                            "Cannot import from absolute path".to_string()).into()),
                    Component::CurDir => continue,
                    Component::ParentDir => { write!(w, "super::")?; },
                    Component::Normal(path) => {
                        if path.to_str().map_or(false, |s| s.contains('.')) {
                            writeln!(w, "{}::*;", Path::new(path).file_stem().unwrap().to_string_lossy())?;
                        } else {
                            write!(w, "{}::", path.to_string_lossy())?;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn write_package_start<W: Write>(&self, w: &mut W) -> Result<()> {
        if self.package.is_empty() { return Ok(()); }
        for p in self.package.split('.') { writeln!(w, "pub mod mod_{} {{", p)?; }
        writeln!(w, "")?;
        Ok(())
    }

    fn write_package_end<W: Write>(&self, w: &mut W) -> Result<()> {
        if self.package.is_empty() { return Ok(()); }
        writeln!(w, "")?;
        for _ in self.package.split('.') { writeln!(w, "}}")?; }
        Ok(())
    }

    fn write_enums<W: Write>(&self, w: &mut W) -> Result<()> {
        for m in self.enums.iter().filter(|e| !e.imported) {
            println!("Writing enum {}", m.name);
            writeln!(w, "")?;
            m.write_definition(w)?;
            writeln!(w, "")?;
            m.write_impl_default(w)?;
            writeln!(w, "")?;
            m.write_from_i32(w)?;
        }
        Ok(())
    }

    fn write_messages<W: Write>(&self, w: &mut W) -> Result<()> {
        for m in self.messages.iter().filter(|m| !m.imported) {
            println!("Writing message {}", m.name);
            writeln!(w, "")?;
            m.write_definition(w, &self.enums, &self.messages)?;
            writeln!(w, "")?;
            m.write_impl_message_read(w, &self.messages)?;
            writeln!(w, "")?;
            m.write_impl_message_write(w, &self.messages)?;

            if !m.messages.is_empty() {
                writeln!(w, "")?;
                writeln!(w, "pub mod mod_{} {{", m.name)?;
                writeln!(w, "")?;
                if m.messages.iter().any(|m| m.fields.iter().any(|f| f.typ.is_cow())) {
                    writeln!(w, "use std::borrow::Cow;")?;
                    writeln!(w, "")?;
                }
                writeln!(w, "use super::*;")?;
                for m_sub in &m.messages {
                    println!("Writing message mod_{}::{}", m.name, m_sub.name);
                    writeln!(w, "")?;
                    m_sub.write_definition(w, &self.enums, &self.messages)?;
                    writeln!(w, "")?;
                    m_sub.write_impl_message_read(w, &self.messages)?;
                    writeln!(w, "")?;
                    m_sub.write_impl_message_write(w, &self.messages)?;
                }
                writeln!(w, "")?;
                writeln!(w, "}}")?;
            }
        }
        Ok(())
    }
}

fn get_imported_path<P: AsRef<Path>, Q: AsRef<Path>>(in_file: P, import: Q) -> PathBuf {
    in_file.as_ref().parent().map_or_else(|| import.as_ref().into(), |p| p.join(import.as_ref()))
}

/// Breaks cycles by adding boxes when necessary
///
/// Cycles means one Message calls itself at some point
fn break_cycles(messages: &mut [Message], leaf_messages: &mut Vec<String>) {
    
    for m in messages.iter_mut() {
        break_cycles(&mut m.messages, leaf_messages);
    }

    let message_names = messages.iter().map(|m| m.name.to_string()).collect::<Vec<_>>();

    let mut undef_messages = (0..messages.len()).collect::<Vec<_>>();
    while !undef_messages.is_empty() {
        let len = undef_messages.len();
        let mut new_undefs = Vec::new();
        for i in undef_messages {
            if messages[i].is_leaf(&leaf_messages) {
                leaf_messages.push(message_names[i].clone())
            } else {
                new_undefs.push(i);
            }
        }
        undef_messages = new_undefs;
        if len == undef_messages.len() {
            // try boxing messages, one by one ...
            let k = undef_messages.pop().unwrap();
            {
                let mut m = messages[k].clone();
                for f in m.fields.iter_mut() {
                    if !f.is_leaf(&leaf_messages) {
                        f.boxed = true;
                    }
                }
                messages[k] = m;
            }
        }
    }
}
