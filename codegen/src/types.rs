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
pub struct Field {
    pub name: String,
    pub frequency: Frequency,
    pub typ: String,
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

    fn is_numeric(&self) -> bool {
        match &*self.typ {
            "int32" | "sint32" | "sfixed32" |
            "int64" | "sint64" | "sfixed64" |
            "uint32" | "fixed32" |
            "uint64" | "fixed64" | 
            "float" | "double" => true,
            _ => false,
        }
    }

    /// searches if the message must be boxed
    fn is_leaf(&self, leaf_messages: &[&str], msgs: &[Message]) -> bool {
        match self.frequency {
            Frequency::Repeated | Frequency::Required => return true,
            Frequency::Optional if !self.is_message(msgs) => true,
            _ => leaf_messages.iter().any(|m| m == &self.typ),
        }
    }

    fn is_message(&self, msgs: &[Message]) -> bool {
        msgs.iter().any(|m| m.name == self.typ)
    }

    fn is_enum(&self, msgs: &[Message]) -> bool {
        self.get_type(msgs) == "enum"
    }

    fn is_fixed_size(&self, msgs: &[Message]) -> bool {
        match self.wire_type_num_non_packed(msgs) {
            1 | 5 => true,
            _ => false,
        }
    }

    fn is_cow(&self) -> bool {
        match &*self.typ {
            "bytes" | "string" => true,
            _ => false,
        }
    }

    fn has_unregular_default(&self, enums: &[Enumerator], msgs: &[Message]) -> bool {
        match self.default {
            None => false,
            Some(ref d) => match &*self.rust_type(msgs) {
                "i32" | "i64" | "u32" | "u64" | "f32" | "f64" => d.parse::<f32>().unwrap() != 0.,
                "bool" => *d != "false",
                "Cow<'a, str>" => *d != "\"\"",
                "Cow<'a, [u8]>" => *d != "[]",
                t => match enums.iter().find(|e| e.name == self.typ) {
                    Some(e) => t != e.fields[0].0,
                    None => false, // Messages are regular defaults
                }
            } 
        }
    }

    fn has_lifetime(&self, msgs: &[Message]) -> bool {
        // borrow bytes and string
        if self.is_cow() { return true; }

        // borrow messages that have lifetime (ie they have at least one borrowed field)
        match msgs.iter().find(|m| m.name == self.typ) {
            Some(ref m) if m.has_lifetime(msgs) => true,
            _ => false,
        }
    }

    fn rust_type(&self, msgs: &[Message]) -> String {
        match &*self.typ {
            "int32" | "sint32" | "sfixed32" => "i32".to_string(),
            "int64" | "sint64" | "sfixed64" => "i64".to_string(),
            "uint32" | "fixed32" => "u32".to_string(),
            "uint64" | "fixed64" => "u64".to_string(),
            "float" => "f32".to_string(),
            "double" => "f64".to_string(),
            "string" => "Cow<'a, str>".to_string(),
            "bytes" => "Cow<'a, [u8]>".to_string(),
            t => msgs.iter().find(|m| m.name == t).map_or(t.to_string(), |m| if m.has_lifetime(msgs) {
                format!("{}<'a>", t.to_string())
            } else {
                t.to_string()
            })
        }
    }

    fn wire_type_num(&self, msgs: &[Message]) -> u32 {
        if self.packed() {
            2
        } else {
            self.wire_type_num_non_packed(msgs)
        }
    }

    fn wire_type_num_non_packed(&self, msgs: &[Message]) -> u32 {
        match &*self.typ {
            "int32" | "sint32" | "int64" | "sint64" | 
                "uint32" | "uint64" | "bool" | "enum" => 0,
            "fixed64" | "sfixed64" | "double" => 1,
            "fixed32" | "sfixed32" | "float" => 5,
            "string" | "bytes" => 2,
            t => if msgs.iter().any(|m| m.name == t) { 2 } else { 0 /* enum */ }
        }
    }

    fn get_type(&self, msgs: &[Message]) -> &str {
        match &*self.typ {
            "int32" | "sint32" | "int64" | "sint64" | 
                "uint32" | "uint64" | "bool" | "fixed64" | 
                "sfixed64" | "double" | "fixed32" | "sfixed32" | 
                "float" | "bytes" | "string" => &self.typ,
            _ => if self.is_message(msgs) { "message" } else { "enum" },
        }
    }

    fn read_fn(&self, msgs: &[Message]) -> String {
        if self.is_message(msgs) {
            format!("read_message(bytes, {}::from_reader)", self.typ)
        } else {
            format!("read_{}(bytes)", self.get_type(msgs))
        }
    }

    fn tag(&self, msgs: &[Message]) -> u32 {
        (self.number as u32) << 3 | self.wire_type_num(msgs)
    }

    fn write_definition<W: Write>(&self, w: &mut W, msgs: &[Message]) -> Result<()> {
        match self.frequency {
            Frequency::Optional => {
                if self.boxed {
                    writeln!(w, "    pub {}: Option<Box<{}>>,", self.name, self.rust_type(msgs))?
                } else {
                    if self.default.is_none() {
                        writeln!(w, "    pub {}: Option<{}>,", self.name, self.rust_type(msgs))?
                    } else {
                        writeln!(w, "    pub {}: {},", self.name, self.rust_type(msgs))?
                    }
                }
            }
            Frequency::Repeated => writeln!(w, "    pub {}: Vec<{}>,", self.name, self.rust_type(msgs))?,
            Frequency::Required => writeln!(w, "    pub {}: {},", self.name, self.rust_type(msgs))?,
        }
        Ok(())
    }

    fn write_match_tag_owned<W: Write>(&self, w: &mut W, msgs: &[Message]) -> Result<()> {
        match self.frequency {
            Frequency::Optional => {
                if self.boxed {
                    writeln!(w, "Ok({}) => msg.{} = Some(Box::new(r.{}?)),",
                             self.tag(msgs), self.name, self.read_fn(msgs))?
                } else {
                    if self.default.is_none() {
                        writeln!(w, "Ok({}) => msg.{} = Some(r.{}?),",
                                 self.tag(msgs), self.name, self.read_fn(msgs))?
                    } else {
                        writeln!(w, "Ok({}) => msg.{} = r.{}?,",
                                 self.tag(msgs), self.name, self.read_fn(msgs))?
                    }
                }
            }
            Frequency::Repeated => {
                if self.packed() {
                    writeln!(w, "Ok({}) => msg.{} = r.read_packed(bytes, |r, bytes| r.{})?,",
                             self.tag(msgs), self.name, self.read_fn(msgs))?
                } else {
                    writeln!(w, "Ok({}) => msg.{}.push(r.{}?),",
                             self.tag(msgs), self.name, self.read_fn(msgs))?
                }
            }
            Frequency::Required => {
                writeln!(w, "Ok({}) => msg.{} = r.{}?,",
                         self.tag(msgs), self.name, self.read_fn(msgs))?
            }
        }
        Ok(())
    }

    fn write_match_tag_borrowed<W: Write>(&self, w: &mut W, msgs: &[Message]) -> Result<()> {
        match self.frequency {
            Frequency::Optional => {
                if self.boxed {
                    writeln!(w, "Ok({}) => msg.{} = Some(Box::new(Cow::Borrowed(r.{}?))),",
                             self.tag(msgs), self.name, self.read_fn(msgs))?
                } else {
                    if self.default.is_none() {
                        writeln!(w, "Ok({}) => msg.{} = Some(Cow::Borrowed(r.{}?)),",
                                 self.tag(msgs), self.name, self.read_fn(msgs))?
                    } else {
                        writeln!(w, "Ok({}) => msg.{} = Cow::Borrowed(r.{}?),",
                                 self.tag(msgs), self.name, self.read_fn(msgs))?
                    }
                }
            }
            Frequency::Repeated => {
                if self.packed() {
                    writeln!(w, "Ok({}) => msg.{} = r.read_packed(bytes, |r, bytes| r.{})?,",
                             self.tag(msgs), self.name, self.read_fn(msgs))?
                } else {
                    writeln!(w, "Ok({}) => msg.{}.push(Cow::Borrowed(r.{}?)),",
                             self.tag(msgs), self.name, self.read_fn(msgs))?
                }
            }
            Frequency::Required => {
                writeln!(w, "Ok({}) => msg.{} = Cow::Borrowed(r.{}?),",
                         self.tag(msgs), self.name, self.read_fn(msgs))?
            }
        }
        Ok(())
    }

    fn write_get_size<W: Write>(&self, w: &mut W, msgs: &[Message], is_first: bool) -> Result<()> {
        if is_first { 
            write!(w, "        ")?;
        } else { 
            write!(w, "        + ")?;
        }
        match self.frequency {
            Frequency::Required => {
                self.write_inner_get_size(w, msgs, &format!("self.{}", self.name), "")?;
                writeln!(w, "")?;
            }
            Frequency::Optional => {
                match self.default.as_ref() {
                    None => {
                        if self.is_fixed_size(msgs) {
                            write!(w, "self.{}.as_ref().map_or(0, |_| ", self.name)?;
                        } else {
                            write!(w, "self.{}.as_ref().map_or(0, |m| ", self.name)?;
                        }
                        self.write_inner_get_size(w, msgs, "m", "*")?;
                        writeln!(w, ")")?;
                    }
                    Some(d) => {
                        write!(w, "if self.{} == {} {{ 0 }} else {{", self.name, d)?;
                        self.write_inner_get_size(w, msgs, &format!("self.{}", self.name), "")?;
                        writeln!(w, "}}")?;
                    }
                }
            }
            Frequency::Repeated => {
                let tag_size = sizeof_varint(self.tag(msgs));
                let get_type = self.get_type(msgs);
                let as_enum = if self.is_enum(msgs) { " as i32" } else { "" };
                if self.packed() {
                    write!(w, "if self.{}.is_empty() {{ 0 }} else {{ ", self.name)?;
                    match self.wire_type_num_non_packed(msgs) {
                        0 => write!(w, "{} + sizeof_var_length(self.{}.iter().map(|s| sizeof_{}(*s{})).sum::<usize>())", 
                                    tag_size, self.name, get_type, as_enum)?,
                        1 => write!(w, "{} + sizeof_var_length(self.{}.len() * 8)", tag_size, self.name)?,
                        5 => write!(w, "{} + sizeof_var_length(self.{}.len() * 4)", tag_size, self.name)?,
                        2 => {
                            let len = if self.is_message(msgs) { "get_size" } else { "len" };
                            write!(w, "{} + sizeof_var_length(self.{}.iter().map(|s| sizeof_var_length(s.{}())).sum::<usize>())", 
                                   tag_size, self.name, len)?;
                        }
                        e => panic!("expecting wire type number, got: {}", e),
                    }
                    writeln!(w, " }}")?;
                } else {
                    match self.wire_type_num_non_packed(msgs) {
                        0 => writeln!(w, "self.{}.iter().map(|s| {} + sizeof_{}(*s{})).sum::<usize>()", 
                                      self.name, tag_size, get_type, as_enum)?,
                        1 => writeln!(w, "({} + 8) * self.{}.len()", tag_size, self.name)?,
                        5 => writeln!(w, "({} + 4) * self.{}.len()", tag_size, self.name)?,
                        2 => {
                            let len = if self.is_message(msgs) { "get_size" } else { "len" };
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

    fn write_inner_get_size<W: Write>(&self, w: &mut W, msgs: &[Message], s: &str, as_ref: &str) -> Result<()> {
        let tag_size = sizeof_varint(self.tag(msgs));
        match self.wire_type_num_non_packed(msgs) {
            0 => {
                let get_type = self.get_type(msgs);
                let as_enum = if self.is_enum(msgs) { " as i32" } else { "" };
                write!(w, "{} + sizeof_{}({}{}{})", tag_size, get_type, as_ref, s, as_enum)?
            },
            1 => write!(w, "{} + 8", tag_size)?,
            5 => write!(w, "{} + 4", tag_size)?,
            2 => {
                let len = if self.is_message(msgs) { "get_size" } else { "len" };
                if self.packed() {
                    write!(w, "if s.is_empty() {{ 0 }} else {{ {} + sizeof_var_length({}.{}()) }}", tag_size, s, len)?;
                } else {
                    write!(w, "{} + sizeof_var_length({}.{}())", tag_size, s, len)?;
                }
            }
            e => panic!("expecting wire type number, got: {}", e),
        }
        Ok(())
    }

    fn write_write<W: Write>(&self, w: &mut W, msgs: &[Message]) -> Result<()> {
        let tag = self.tag(msgs);
        let use_ref = self.wire_type_num_non_packed(msgs) == 2;
        let get_type = self.get_type(msgs);
        let as_enum = if self.is_enum(msgs) { " as i32" } else { "" };
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
            Frequency::Repeated => {
                if self.packed() {
                    match get_type {
                        "message" => {
                            writeln!(w, "        r.write_packed_repeated_field_with_tag({}, &self.{}, |r, m| r.write_{}({}m{}), \
                                        &|m| sizeof_var_length(m.get_size()))?;", 
                                     tag, self.name, get_type, if use_ref { "" } else { "*" }, as_enum)?
                        },
                        "bytes" | "string" => {
                            writeln!(w, "        r.write_packed_repeated_field_with_tag({}, &self.{}, |r, m| r.write_{}({}m{}), \
                                        &|m| sizeof_var_length(m.len()))?;", 
                                     tag, self.name, get_type, if use_ref { "" } else { "*" }, as_enum)?
                        },
                        t => {
                            writeln!(w, "        r.write_packed_repeated_field_with_tag({}, &self.{}, |r, m| r.write_{}({}m{}), \
                                        &|m| sizeof_{}(*m))?;", 
                                     tag, self.name, get_type, if use_ref { "" } else { "*" }, as_enum, t)?
                        },
                    }
                } else {
                    writeln!(w, "        for s in &self.{} {{ r.write_{}_with_tag({}, {}s{})? }}", 
                             self.name, get_type, tag, if use_ref { "" } else { "*" }, as_enum)?;
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Message {
    pub name: String,
    pub fields: Vec<Field>,
    pub reserved_nums: Option<Vec<i32>>,
    pub reserved_names: Option<Vec<String>>,
    pub imported: bool,
}

impl Message {

    fn is_leaf(&self, leaf_messages: &[&str], msgs: &[Message]) -> bool {
        self.fields.iter().all(|f| f.is_leaf(leaf_messages, msgs) || f.deprecated)
    }

    fn has_lifetime(&self, msgs: &[Message]) -> bool {
        self.fields.iter().any(|f| f.typ != self.name && f.has_lifetime(msgs))
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

    fn write_impl_message_read<W: Write>(&self, w: &mut W, enums: &[Enumerator], msgs: &[Message]) -> Result<()> {
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
            write!(w, "                ")?;
            if f.is_cow() {
                f.write_match_tag_borrowed(w, msgs)?;
            } else {
                f.write_match_tag_owned(w, msgs)?;
            }
        }
        writeln!(w, "                Ok(t) => {{ r.read_unknown(bytes, t)?; }}")?;
        writeln!(w, "                Err(e) => return Err(e),")?;
        writeln!(w, "            }}")?;
        writeln!(w, "        }}")?;
        writeln!(w, "        Ok(msg)")?;
        writeln!(w, "    }}")?;
        writeln!(w, "}}")?;

        if !self.can_derive_default(enums, msgs) {
//             writeln!(w, "")?;
//             self.write_impl_default(w, msgs)?;
        }
        Ok(())
    }

    fn write_impl_message_write<W: Write>(&self, w: &mut W, msgs: &[Message]) -> Result<()> {
        if self.has_lifetime(msgs) {
            writeln!(w, "impl<'a> MessageWrite for {}<'a> {{", self.name)?;
        } else {
            writeln!(w, "impl MessageWrite for {} {{", self.name)?;
        }
        self.write_get_size(w, msgs)?;
        writeln!(w, "")?;
        self.write_write_message(w, msgs)?;
        writeln!(w, "}}")?;
        Ok(())
    }

    fn write_get_size<W: Write>(&self, w: &mut W, msgs: &[Message]) -> Result<()> {
        writeln!(w, "    fn get_size(&self) -> usize {{")?;
        for (i, f) in self.fields.iter().filter(|f| !f.deprecated).enumerate() {
            f.write_get_size(w, msgs, i == 0)?;
        }
        writeln!(w, "    }}")?;
        Ok(())
    }

    fn write_write_message<W: Write>(&self, w: &mut W, msgs: &[Message]) -> Result<()> {
        writeln!(w, "    fn write_message<W: Write>(&self, r: &mut Writer<W>) -> Result<()> {{")?;
        for f in self.fields.iter().filter(|f| !f.deprecated) {
            f.write_write(w, msgs)?;
        }
        writeln!(w, "        Ok(())")?;
        writeln!(w, "    }}")?;
        Ok(())
    }

//     fn write_impl_default<W: Write>(&self, w: &mut W, msgs: &[Message]) -> IoResult<()> {
//         writeln!(w, "impl Default for {} {{", self.name)?;
//         writeln!(w, "    fn default() -> Self {{")?;
//         writeln!(w, "        {} {{", self.name)?;
//         for f in self.fields.iter().filter(|f| !f.deprecated) {
//             match f.default {
//                 None => writeln!(w, "            {}::default(),", f.rust_type())?,
//                 Some(ref d) => if msgs.iter().any(|m| m.name == f.typ) {
//                     writeln!(w, "            {}: {},", f.name, d)?
//                 } else {
//                     writeln!(w, "            {}: {}::{},", f.name, f.typ, d)?
//                 }
//             }
//         }
//         writeln!(w, "        }}")?;
//         writeln!(w, "    }}")?;
//         writeln!(w, "}}")
//     }

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
}

#[derive(Debug, Clone)]
pub struct Enumerator {
    pub name: String,
    pub fields: Vec<(String, i32)>,
    pub imported: bool,
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
    pub package: Option<String>,
    pub syntax: Syntax,
    pub messages: Vec<Message>,
    pub enums: Vec<Enumerator>,
}

impl FileDescriptor {

    pub fn write_proto<P: AsRef<Path>>(in_file: P, out_file: P) -> Result<()> {
        let mut desc = FileDescriptor::read_proto(&in_file)?;
        desc.fetch_imports(in_file.as_ref())?;
        desc.break_cycles();
        desc.sanity_checks(in_file.as_ref())?;
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
                    if f.default.is_none() && f.is_numeric() { 
                        f.default = Some("0".to_string());
                    }
                }
            }
        }
    }

    pub fn write<W: Write>(&self, w: &mut W, filename: &str) -> Result<()> {
        
        println!("Found {} messages, and {} enums", self.messages.len(), self.enums.len());

        writeln!(w, "//! Automatically generated rust module for '{}' file", filename)?;
        writeln!(w, "")?;
        writeln!(w, "#![allow(non_snake_case)]")?;
        writeln!(w, "#![allow(non_upper_case_globals)]")?;
        writeln!(w, "#![allow(non_camel_case_types)]")?;
        writeln!(w, "")?;
        writeln!(w, "use std::io::{{Write}};")?;
        if self.messages.iter().any(|m| m.has_lifetime(&self.messages)) {
            writeln!(w, "use std::borrow::Cow;")?;
        }
        writeln!(w, "use quick_protobuf::{{MessageWrite, BytesReader, Writer, Result}};")?;
        writeln!(w, "use quick_protobuf::sizeofs::*;")?;

        self.write_imports(w)?;

        for m in self.enums.iter().filter(|e| !e.imported) {
            println!("Writing enum {}", m.name);
            writeln!(w, "")?;
            m.write_definition(w)?;
            writeln!(w, "")?;
            m.write_impl_default(w)?;
            writeln!(w, "")?;
            m.write_from_i32(w)?;
        }
        for m in self.messages.iter().filter(|m| !m.imported) {
            println!("Writing message {}", m.name);
            writeln!(w, "")?;
            m.write_definition(w, &self.enums, &self.messages)?;
            writeln!(w, "")?;
            m.write_impl_message_read(w, &self.enums, &self.messages)?;
            writeln!(w, "")?;
            m.write_impl_message_write(w, &self.messages)?;
        }
        println!("Done processing {}", filename);
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

    /// Get messages and enums from imports
    fn fetch_imports(&mut self, in_file: &Path) -> Result<()> {
        for p in &self.import_paths {
            let import_path = get_imported_path(&in_file, p);
            let mut f = FileDescriptor::read_proto(&import_path)?;
            f.fetch_imports(&import_path)?;
            self.messages.extend(f.messages.drain(..).map(|mut m| {
                m.imported = true;
                m
            }));
            self.enums.extend(f.enums.drain(..).map(|mut e| {
                e.imported = true;
                e
            }));
        }
        Ok(())
    }

    fn break_cycles(&mut self) {

        let message_names = self.messages.iter().map(|m| m.name.to_string()).collect::<Vec<_>>();

        let mut leaf_messages = Vec::new();
        let mut undef_messages = (0..self.messages.len()).collect::<Vec<_>>();
        while !undef_messages.is_empty() {
            let len = undef_messages.len();
            let mut new_undefs = Vec::new();
            for i in undef_messages.into_iter() {
                if self.messages[i].is_leaf(&leaf_messages, &self.messages) {
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
                    let mut m = self.messages[k].clone();
                    for f in m.fields.iter_mut() {
                        if !f.is_leaf(&leaf_messages, &self.messages) {
                            f.boxed = true;
                        }
                    }
                    self.messages[k] = m;
                }
            }
        }
    }
}

fn get_imported_path<P: AsRef<Path>, Q: AsRef<Path>>(in_file: P, import: Q) -> PathBuf {
    in_file.as_ref().parent().map_or_else(|| import.as_ref().into(), |p| p.join(import.as_ref()))
}
