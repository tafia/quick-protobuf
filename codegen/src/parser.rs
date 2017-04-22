use std::str;
use std::path::{Path, PathBuf};

use types::{Frequency, Field, Message, Enumerator, OneOf, FileDescriptor, Syntax, FieldType};
use nom::{multispace, digit, hex_digit};

fn is_word(b: u8) -> bool {
    match b {
        b'a'...b'z' | b'A'...b'Z' | b'0'...b'9' | b'_' | b'.' => true,
        _ => false
    }
}

named!(word<String>, map_res!(take_while!(is_word), |b: &[u8]| String::from_utf8(b.to_vec())));
named!(word_ref<&str>, map_res!(take_while!(is_word), str::from_utf8));

named!(comment<()>, do_parse!(tag!("//") >> take_until_and_consume!("\n") >> ()));
named!(block_comment<()>, do_parse!(tag!("/*") >> take_until_and_consume!("*/") >> ()));

/// word break: multispace or comment
named!(br<()>, alt!(map!(multispace, |_| ()) | comment | block_comment));

named!(syntax<Syntax>,
       do_parse!(tag!("syntax") >> many0!(br) >> tag!("=") >> many0!(br) >>
                 proto: alt!(tag!("\"proto2\"") => { |_| Syntax::Proto2 } |
                             tag!("\"proto3\"") => { |_| Syntax::Proto3 }) >>
                 many0!(br) >> tag!(";") >>
                 (proto) ));

named!(import<PathBuf>,
       do_parse!(tag!("import")>> many1!(br) >> tag!("\"") >>
                 path: map!(map_res!(take_until!("\""), str::from_utf8), |s| Path::new(s).into()) >> tag!("\"") >>
                 many0!(br) >> tag!(";") >>
                 (path) ));

named!(package<String>,
       do_parse!(tag!("package") >> many1!(br) >> package: word >> many0!(br) >> tag!(";") >>
                 (package) ));

named!(reserved_nums<Vec<i32>>,
       do_parse!(tag!("reserved") >> many1!(br) >>
                 nums: many1!(do_parse!(num: map_res!(map_res!(digit, str::from_utf8), str::FromStr::from_str) >>
                                        many0!(alt!(br | tag!(",") => { |_| () })) >> (num))) >>
                (nums) ));

named!(reserved_names<Vec<String>>,
       do_parse!(tag!("reserved") >> many1!(br) >>
                 names: many1!(do_parse!(tag!("\"") >> name: word >> tag!("\"") >>
                                        many0!(alt!(br | tag!(",") => { |_| () })) >>
                                        (name))) >>
                (names) ));

named!(key_val<(&str, &str)>,
       do_parse!(tag!("[") >> many0!(br) >>
                 key: word_ref >> many0!(br) >> tag!("=") >> many0!(br) >>
                 value: map_res!(is_not!("]"), str::from_utf8) >> tag!("]") >> many0!(br) >>
                 ((key, value.trim())) ));

named!(frequency<Frequency>,
       alt!(tag!("optional") => { |_| Frequency::Optional } |
            tag!("repeated") => { |_| Frequency::Repeated } |
            tag!("required") => { |_| Frequency::Required } ));

named!(field_type<FieldType>,
       alt!(tag!("int32") => { |_| FieldType::Int32 } |
            tag!("int64") => { |_| FieldType::Int64 } |
            tag!("uint32") => { |_| FieldType::Uint32 } |
            tag!("uint64") => { |_| FieldType::Uint64 } |
            tag!("sint32") => { |_| FieldType::Sint32 } |
            tag!("sint64") => { |_| FieldType::Sint64 } |
            tag!("fixed32") => { |_| FieldType::Fixed32 } |
            tag!("sfixed32") => { |_| FieldType::Sfixed32 } |
            tag!("fixed64") => { |_| FieldType::Fixed64 } |
            tag!("sfixed64") => { |_| FieldType::Sfixed64 } |
            tag!("bool") => { |_| FieldType::Bool } |
            tag!("string") => { |_| FieldType::String_ } |
            tag!("bytes") => { |_| FieldType::Bytes } |
            tag!("float") => { |_| FieldType::Float } |
            tag!("double") => { |_| FieldType::Double } |
            map_field => { |(k, v)| FieldType::Map(Box::new((k, v))) } |
            word => { |w| FieldType::Message(w) }));

named!(map_field<(FieldType, FieldType)>,
       do_parse!(tag!("map") >> many0!(br) >> tag!("<") >> many0!(br) >>
                 key: field_type >> many0!(br) >> tag!(",") >> many0!(br) >>
                 value: field_type >> tag!(">") >>
                 ((key, value)) ));

named!(one_of<OneOf>,
       do_parse!(tag!("oneof") >> many1!(br) >>
                 name: word >> many0!(br) >> tag!("{") >>
                 fields: many1!(message_field) >> many0!(br) >> tag!("}") >> many0!(br) >>
                 (OneOf {
                     name: name,
                     fields: fields,
                     package: "".to_string(),
                     module: "".to_string()
                 }) ));

named!(message_field<Field>,
       do_parse!(frequency: opt!(frequency) >> many0!(br) >>
                 typ: field_type >> many1!(br) >>
                 name: word >> many0!(br) >> tag!("=") >> many0!(br) >>
                 number: map_res!(map_res!(digit, str::from_utf8), str::FromStr::from_str) >> many0!(br) >>
                 key_vals: many0!(key_val) >> tag!(";") >>
                 (Field {
                      name: name,
                      frequency: frequency.unwrap_or(Frequency::Optional),
                      typ: typ,
                      number: number,
                      default: key_vals.iter()
                               .find(|&&(k, _)| k == "default")
                               .map(|&(_, v)| v.to_string()),
                      packed: key_vals.iter()
                              .find(|&&(k, _)| k == "packed")
                              .map(|&(_, v)| str::FromStr::from_str(v)
                                   .expect("Cannot parse Packed value")),
                      boxed: false,
                      deprecated: key_vals.iter()
                                  .find(|&&(k, _)| k == "deprecated")
                                  .map_or(false, |&(_, v)| str::FromStr::from_str(v)
                                          .expect("Cannot parse Deprecated value")),
                 }) ));

enum MessageEvent {
    Message(Message),
    Enumerator(Enumerator),
    Field(Field),
    ReservedNums(Vec<i32>),
    ReservedNames(Vec<String>),
    OneOf(OneOf),
    Ignore,
}

named!(message_event<MessageEvent>, alt!(reserved_nums => { |r| MessageEvent::ReservedNums(r) } |
                                         reserved_names => { |r| MessageEvent::ReservedNames(r) } |
                                         message_field => { |f| MessageEvent::Field(f) } |
                                         message => { |m| MessageEvent::Message(m) } |
                                         enumerator => { |e| MessageEvent::Enumerator(e) } |
                                         one_of => { |o| MessageEvent::OneOf(o) } |
                                         br => { |_| MessageEvent::Ignore }));

named!(message_events<(String, Vec<MessageEvent>)>,
       do_parse!(tag!("message") >> many0!(br) >>
                 name: word >> many0!(br) >>
                 tag!("{") >> many0!(br) >>
                 events: many0!(message_event) >>
                 many0!(br) >> tag!("}") >>
                 many0!(br) >> many0!(tag!(";")) >>
                 ((name, events)) ));

named!(message<Message>,
       map!(message_events, |(name, events): (String, Vec<MessageEvent>)| {
           let mut msg = Message { name: name.clone(), .. Message::default() };
           for e in events {
               match e {
                   MessageEvent::Field(f) => msg.fields.push(f),
                   MessageEvent::ReservedNums(r) => msg.reserved_nums = Some(r),
                   MessageEvent::ReservedNames(r) => msg.reserved_names = Some(r),
                   MessageEvent::Message(m) => msg.messages.push(m),
                   MessageEvent::Enumerator(e) => msg.enums.push(e),
                   MessageEvent::OneOf(o) => msg.oneofs.push(o),
                   MessageEvent::Ignore => (),
               }
           }
           msg
       }));

named!(hex_integer<i32>,
    do_parse!(
        tag!("0x") >>
        num: map_res!(
            map_res!(hex_digit,str::from_utf8),
            |s| i32::from_str_radix(s,16)
        ) >> (num)
    ));

named!(integer<i32>,  map_res!( map_res!(digit,str::from_utf8), str::FromStr::from_str));

named!(enum_field<(String, i32)>,
       do_parse!(name: word >> many0!(br) >>
                 tag!("=") >> many0!(br) >>
                 number: alt!(hex_integer | integer) >> many0!(br) >>
                 tag!(";") >> many0!(br) >>
                 ((name, number))));


named!(enumerator<Enumerator>,
       do_parse!(tag!("enum") >> many1!(br) >> name: word >> many0!(br) >>
                 tag!("{") >> many0!(br) >> fields: many0!(enum_field) >> many0!(br) >> tag!("}") >>
                 many0!(br) >> many0!(tag!(";")) >>
                 (Enumerator {
                     name: name,
                     fields: fields,
                     imported: false,
                     package: "".to_string(),
                     module: "".to_string()
                 })));

named!(option_ignore<()>,
       do_parse!(tag!("option") >> many1!(br) >> take_until_and_consume!(";") >> ()));

named!(service_ignore<()>,
       do_parse!(tag!("service") >> many1!(br) >> word >> many0!(br) >> tag!("{") >>
                 take_until_and_consume!("}") >> ()));

enum Event {
    Syntax(Syntax),
    Import(PathBuf),
    Package(String),
    Message(Message),
    Enum(Enumerator),
    Ignore
}

named!(event<Event>,
       alt!(syntax => { |s| Event::Syntax(s) } |
            import => { |i| Event::Import(i) } |
            package => { |p| Event::Package(p) } |
            message => { |m| Event::Message(m) } |
            enumerator => { |e| Event::Enum(e) } |
            option_ignore => { |_| Event::Ignore } |
            service_ignore => { |_| Event::Ignore } |
            br => { |_| Event::Ignore }));

named!(pub file_descriptor<FileDescriptor>,
       map!(many0!(event), |events: Vec<Event>| {
           let mut desc = FileDescriptor::default();
           for event in events {
               match event {
                   Event::Syntax(s) => desc.syntax = s,
                   Event::Import(i) => desc.import_paths.push(i),
                   Event::Package(p) => desc.package = p,
                   Event::Message(m) => desc.messages.push(m),
                   Event::Enum(e) => desc.enums.push(e),
                   Event::Ignore => (),
               }
           }
           desc
       }));

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_message() {
        let msg = r#"message ReferenceData
    {
        repeated ScenarioInfo  scenarioSet = 1;
        repeated CalculatedObjectInfo calculatedObjectSet = 2;
        repeated RiskFactorList riskFactorListSet = 3;
        repeated RiskMaturityInfo riskMaturitySet = 4;
        repeated IndicatorInfo indicatorSet = 5;
        repeated RiskStrikeInfo riskStrikeSet = 6;
        repeated FreeProjectionList freeProjectionListSet = 7;
        repeated ValidationProperty ValidationSet = 8;
        repeated CalcProperties calcPropertiesSet = 9;
        repeated MaturityInfo maturitySet = 10;
    }"#;

        let mess = message(msg.as_bytes());
        if let ::nom::IResult::Done(_, mess) = mess {
            assert_eq!(10, mess.fields.len());
        }
    }

    #[test]
    fn test_enum() {
        let msg = r#"enum PairingStatus {
                DEALPAIRED        = 0;
                INVENTORYORPHAN   = 1;
                CALCULATEDORPHAN  = 2;
                CANCELED          = 3;
    }"#;

        let mess = enumerator(msg.as_bytes());
        if let ::nom::IResult::Done(_, mess) = mess {
            assert_eq!(4, mess.fields.len());
        }
    }

    #[test]
    fn test_ignore() {
        let msg = r#"option optimize_for = SPEED;"#;

        match option_ignore(msg.as_bytes()) {
            ::nom::IResult::Done(_, _) => (),
            e => panic!("Expecting done {:?}", e),
        }
    }

    #[test]
    fn test_import() {
        let msg = r#"syntax = "proto3";

    import "test_import_nested_imported_pb.proto";

    message ContainsImportedNested {
        optional ContainerForNested.NestedMessage m = 1;
        optional ContainerForNested.NestedEnum e = 2;
    }
    "#;
        let desc = file_descriptor(msg.as_bytes()).to_full_result().unwrap();
        assert_eq!(vec![Path::new("test_import_nested_imported_pb.proto")], desc.import_paths);
    }

    #[test]
    fn test_package() {
        let msg = r#"
        package foo.bar;

    message ContainsImportedNested {
        optional ContainerForNested.NestedMessage m = 1;
        optional ContainerForNested.NestedEnum e = 2;
    }
    "#;
        let desc = file_descriptor(msg.as_bytes()).to_full_result().unwrap();
        assert_eq!("foo.bar".to_string(), desc.package);
    }

    #[test]
    fn test_nested_message() {
        let msg = r#"message A
    {
        message B {
            repeated int32 a = 1;
            optional string b = 2;
        }
        optional b = 1;
    }"#;

        let mess = message(msg.as_bytes());
        if let ::nom::IResult::Done(_, mess) = mess {
            assert!(mess.messages.len() == 1);
        }
    }

    #[test]
    fn test_map() {
        let msg = r#"message A
    {
        optional map<string, int32> b = 1;
    }"#;

        let mess = message(msg.as_bytes());
        if let ::nom::IResult::Done(_, mess) = mess {
            assert_eq!(1, mess.fields.len());
            match mess.fields[0].typ {
                FieldType::Map(ref f) => match &**f {
                    &(FieldType::String_, FieldType::Int32) => (),
                    ref f => panic!("Expecting Map<String, Int32> found {:?}", f),
                },
                ref f => panic!("Expecting map, got {:?}", f),
            }
        } else {
            panic!("Could not parse map message");
        }
    }

    #[test]
    fn test_oneof() {
        let msg = r#"message A
    {
        optional int32 a1 = 1;
        oneof a_oneof {
            string a2 = 2;
            int32 a3 = 3;
            bytes a4 = 4;
        }
        repeated bool a5 = 5;
    }"#;

        let mess = message(msg.as_bytes());
        if let ::nom::IResult::Done(_, mess) = mess {
            assert_eq!(1, mess.oneofs.len());
            assert_eq!(3, mess.oneofs[0].fields.len());
        }
    }
}
