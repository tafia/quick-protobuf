use std::str;
use std::path::{Path, PathBuf};

use types::{Frequency, Field, Message, Enumerator, FileDescriptor, Syntax};
use nom::{multispace, digit};

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
                 value: word_ref >> many0!(br) >> tag!("]") >> many0!(br) >>
                 ((key, value)) ));

named!(frequency<Frequency>,
       alt!(tag!("optional") => { |_| Frequency::Optional } |
            tag!("repeated") => { |_| Frequency::Repeated } |
            tag!("required") => { |_| Frequency::Required } ));

named!(message_field<Field>, 
       do_parse!(frequency: opt!(frequency) >> many1!(br) >>
                 typ: word >> many1!(br) >>
                 name: word >> many0!(br) >> tag!("=") >> many0!(br) >>
                 number: map_res!(map_res!(digit, str::from_utf8), str::FromStr::from_str) >> many0!(br) >> 
                 key_vals: many0!(key_val) >> tag!(";") >>
                 (Field {
                    name: name,
                    frequency: frequency.unwrap_or(Frequency::Optional),
                    typ: typ,
                    number: number,
                    default: key_vals.iter().find(|&&(k, _)| k == "default")
                                      .map(|&(_, v)| v.to_string()),
                    packed: key_vals.iter().find(|&&(k, _)| k == "packed")
                                    .map(|&(_, v)| str::FromStr::from_str(v)
                                         .expect("Cannot parse Packed value")),
                    boxed: false,
                    deprecated: key_vals.iter().find(|&&(k, _)| k == "deprecated")
                                        .map_or(false, |&(_, v)| str::FromStr::from_str(v)
                                                .expect("Cannot parse Deprecated value")),
                 }) ));

enum MessageEvent {
    Messages(Vec<Message>),
    Field(Field),
    ReservedNums(Vec<i32>),
    ReservedNames(Vec<String>),
    Ignore,
}

named!(message_event<MessageEvent>, alt!(reserved_nums => { |r| MessageEvent::ReservedNums(r) } |
                                         reserved_names => { |r| MessageEvent::ReservedNames(r) } |
                                         message_field => { |f| MessageEvent::Field(f) } |
                                         message => { |m| MessageEvent::Messages(m) } |
                                         br => { |_| MessageEvent::Ignore }));

named!(message_events<(String, Vec<MessageEvent>)>, 
       do_parse!(tag!("message") >> many0!(br) >> 
                 name: word >> many0!(br) >> 
                 tag!("{") >> many0!(br) >>
                 events: many0!(message_event) >>
                 many0!(br) >> tag!("}") >>
                 ((name, events)) ));

named!(message<Vec<Message>>,
       map!(message_events, |(name, events): (String, Vec<MessageEvent>)| {
           let mut messages = Vec::new();
           let mut msg = Message { name: name.clone(), .. Message::default() };
           for e in events {
               match e {
                   MessageEvent::Field(f) => msg.fields.push(f),
                   MessageEvent::ReservedNums(r) => msg.reserved_nums = Some(r),
                   MessageEvent::ReservedNames(r) => msg.reserved_names = Some(r),
                   MessageEvent::Messages(ms) => {
                       for mut m in ms {
                           m.parents.push(name.clone());
                           messages.push(m)
                       }
                   },
                   MessageEvent::Ignore => (),
               }
           }
           messages.push(msg);
           messages
       }));

named!(enum_field<(String, i32)>, 
       do_parse!(name: word >> many0!(br) >>
                 tag!("=") >> many0!(br) >>
                 number: map_res!(map_res!(digit, str::from_utf8), str::FromStr::from_str) >> many0!(br) >>
                 tag!(";") >> many0!(br) >>
                 ((name, number))));
    
named!(enumerator<Enumerator>, 
       do_parse!(tag!("enum") >> many1!(br) >> name: word >> many0!(br) >>
                 tag!("{") >> many0!(br) >> fields: many0!(enum_field) >> many0!(br) >> tag!("}") >>
                 (Enumerator { 
                     name: name, 
                     fields: fields,
                     imported: false,
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
    Messages(Vec<Message>),
    Enum(Enumerator),
    Ignore
}

named!(event<Event>,
       alt!(syntax => { |s| Event::Syntax(s) } |
            import => { |i| Event::Import(i) } |
            package => { |p| Event::Package(p) } |
            message => { |m| Event::Messages(m) } | 
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
                   Event::Package(p) => desc.package = p.split('.').map(|s| s.to_string()).collect(),
                   Event::Messages(m) => desc.messages.extend(m),
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
            assert!(mess.len() == 1);
            assert_eq!(10, mess[0].fields.len());
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
        assert_eq!(vec!("foo".to_string(), "bar".to_string()), desc.package);
    }
}
