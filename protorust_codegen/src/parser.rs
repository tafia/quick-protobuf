use std::str;
use types::{Frequency, Field, Message, Enumerator, MessageOrEnum, FileDescriptor, Syntax};
use nom::{multispace, digit};

fn is_word(b: u8) -> bool {
    match b {
        b'a'...b'z' | b'A'...b'Z' | b'0'...b'9' | b'_' => true,
        _ => false
    }
}

named!(word<&str>, map_res!(take_while!(is_word), str::from_utf8));

named!(comment<()>, do_parse!(tag!("//") >> take_until_and_consume!("\n") >> ()));
named!(block_comment<()>, do_parse!(tag!("/*") >> take_until_and_consume!("*/") >> ()));

/// word break: multispace or comment
named!(br<()>, alt!(map!(multispace, |_| ()) | comment | block_comment));

named!(syntax<Syntax>, do_parse!(tag!("syntax") >> many0!(br) >> tag!("=") >>
    proto: alt!(tag!("\"proto2\"") => { |_| Syntax::Proto2 } |
                tag!("\"proto3\"") => { |_| Syntax::Proto3 }) >> (proto)));

named!(default_value<&str>, do_parse!(
    tag!("[") >> many0!(br) >> tag!("default") >> many0!(br) >> tag!("=") >> many0!(br) >> 
    default: word >> many0!(br) >> tag!("]") >>
    (default)));

named!(packed<bool>, do_parse!(
    tag!("[") >> many0!(br) >> tag!("packed") >> many0!(br) >> tag!("=") >> many0!(br) >> 
    packed: map_res!(word, str::FromStr::from_str) >> many0!(br) >> tag!("]") >>
    (packed)));

named!(frequency<Frequency>,
       alt!(tag!("optional") => { |_| Frequency::Optional } |
            tag!("repeated") => { |_| Frequency::Repeated } |
            tag!("required") => { |_| Frequency::Required } ));

named!(message_field<Field>, do_parse!(
    frequency: opt!(frequency) >> many1!(br) >>
    typ: word >> many1!(br) >>
    name: word >> many0!(br) >>
    tag!("=") >> many0!(br) >>
    number: map_res!(map_res!(digit, str::from_utf8), str::FromStr::from_str) >> many0!(br) >> 
    default: opt!(default_value) >> many0!(br) >> 
    packed: opt!(packed) >> many0!(br) >> tag!(";") >> many0!(br) >>
    (Field {
       name: name,
       frequency: frequency.unwrap_or(Frequency::Optional),
       typ: typ,
       number: number,
       default: default,
       packed: packed,
       boxed: false,
    })));

named!(message<Message>, do_parse!(
    tag!("message") >> many0!(br) >> 
    name: word >> many0!(br) >> 
    tag!("{") >> many0!(br) >>
    fields: many0!(message_field) >> 
    tag!("}") >> many0!(br) >>
    (Message { name: name, fields: fields })));

named!(enum_field<(&str, i32)>, do_parse!(
    name: word >> many0!(br) >>
    tag!("=") >> many0!(br) >>
    number: map_res!(map_res!(digit, str::from_utf8), str::FromStr::from_str) >> many0!(br) >>
    tag!(";") >> many0!(br) >>
    ((name, number))));
    
named!(enumerator<Enumerator>, do_parse!(
    tag!("enum") >> many1!(br) >>
    name: word >> many0!(br) >>
    tag!("{") >> many0!(br) >>
    fields: many0!(enum_field) >> 
    tag!("}") >> many0!(br) >>
    (Enumerator { name: name, fields: fields })));

named!(ignore<()>, do_parse!(
    alt!(tag!("package") | tag!("option")) >> many1!(br) >> 
    take_until_and_consume!(";") >> many0!(br) >> ()));

named!(message_or_enum<MessageOrEnum>, alt!(
         message => { |m| MessageOrEnum::Msg(m) } | 
         enumerator => { |e| MessageOrEnum::Enum(e) } |
         ignore => { |_| MessageOrEnum::Ignore } ));

named!(pub file_descriptor<FileDescriptor>, do_parse!(
    many0!(br) >> syntax: opt!(syntax) >> many0!(br) >>
    message_and_enums: many0!(message_or_enum) >>
    (FileDescriptor {
        syntax: syntax.unwrap_or(Syntax::Proto2),
        message_and_enums: message_and_enums,
        messages: Vec::new(),
        enums: Vec::new(),
    })));

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
    let msg = r#"package com.test.v0;

option optimize_for = SPEED;
"#;

    match ignore(msg.as_bytes()) {
        ::nom::IResult::Done(_, _) => (),
        e => panic!("Expecting done {:?}", e),
    }
}
