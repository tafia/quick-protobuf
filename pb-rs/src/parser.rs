use std::path::PathBuf;
use std::str;

use crate::types::{
    Enumerator, Field, FieldType, FileDescriptor, Frequency, Message, OneOf,
    RpcFunctionDeclaration, RpcService, Syntax,
};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alphanumeric1, digit1, hex_digit1, multispace1, not_line_ending},
    combinator::{map, map_res, opt, recognize, value},
    multi::{many0, many1, separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    IResult,
};

#[derive(Debug, Clone)]
enum MessageEvent {
    Message(Message),
    Enumerator(Enumerator),
    Field(Field),
    ReservedNums(Vec<i32>),
    ReservedNames(Vec<String>),
    OneOf(OneOf),
    Ignore,
}

#[derive(Debug, Clone)]
enum Event {
    Syntax(Syntax),
    Import(PathBuf),
    Package(String),
    Message(Message),
    Enum(Enumerator),
    RpcService(RpcService),
    Ignore,
}

fn word_ref(input: &str) -> IResult<&str, &str> {
    recognize(many1(alt((alphanumeric1, tag("_"), tag(".")))))(input)
}

fn word(input: &str) -> IResult<&str, String> {
    map(word_ref, |word| word.to_owned())(input)
}

fn hex_integer(input: &str) -> IResult<&str, i32> {
    preceded(
        tag("0x"),
        map_res(hex_digit1, |s: &str| i32::from_str_radix(s, 16)),
    )(input)
}

fn integer(input: &str) -> IResult<&str, i32> {
    map_res(digit1, |s: &str| s.parse())(input)
}

fn comment(input: &str) -> IResult<&str, ()> {
    value((), pair(tag("//"), not_line_ending))(input)
}

fn block_comment(input: &str) -> IResult<&str, ()> {
    value((), tuple((tag("/*"), take_until("*/"), tag("*/"))))(input)
}

fn string(input: &str) -> IResult<&str, String> {
    map(
        delimited(tag("\""), take_until("\""), tag("\"")),
        |s: &str| s.to_owned(),
    )(input)
}

// word break: multispace or comment
fn br(input: &str) -> IResult<&str, ()> {
    value((), many1(alt((value((), multispace1), comment, block_comment))))(input)
}

fn syntax(input: &str) -> IResult<&str, Syntax> {
    delimited(
        tuple((tag("syntax"), many0(br), tag("="), many0(br))),
        alt((
            value(Syntax::Proto2, tag("\"proto2\"")),
            value(Syntax::Proto3, tag("\"proto3\"")),
        )),
        pair(many0(br), tag(";")),
    )(input)
}

fn import(input: &str) -> IResult<&str, PathBuf> {
    delimited(
        pair(tag("import"), many1(br)),
        map(string, PathBuf::from),
        pair(many0(br), tag(";")),
    )(input)
}

fn package(input: &str) -> IResult<&str, String> {
    delimited(
        pair(tag("package"), many1(br)),
        word,
        pair(many0(br), tag(";")),
    )(input)
}

fn extensions(input: &str) -> IResult<&str, ()> {
    value((), delimited(tag("extensions"), take_until(";"), tag(";")))(input)
}

fn num_range(input: &str) -> IResult<&str, Vec<i32>> {
    map(
        separated_pair(integer, tuple((many1(br), tag("to"), many1(br))), integer),
        |(from_, to)| (from_..=to).collect(),
    )(input)
}

fn reserved_nums(input: &str) -> IResult<&str, Vec<i32>> {
    map(
        delimited(
            pair(tag("reserved"), many1(br)),
            separated_list1(
                tuple((many0(br), tag(","), many0(br))),
                alt((num_range, map(integer, |i| vec![i]))),
            ),
            pair(many0(br), tag(";")),
        ),
        |nums| nums.into_iter().flat_map(|v| v.into_iter()).collect(),
    )(input)
}

fn reserved_names(input: &str) -> IResult<&str, Vec<String>> {
    delimited(
        pair(tag("reserved"), many1(br)),
        separated_list1(tuple((many0(br), tag(","), many0(br))), string),
        pair(many0(br), tag(";")),
    )(input)
}

fn key_val(input: &str) -> IResult<&str, (&str, &str)> {
    delimited(
        pair(tag("["), many0(br)),
        separated_pair(
            word_ref,
            delimited(many0(br), tag("="), many0(br)),
            map(take_until("]"), |v: &str| v.trim()),
        ),
        tag("]"),
    )(input)
}

fn frequency(input: &str) -> IResult<&str, Frequency> {
    alt((
        value(Frequency::Optional, tag("optional")),
        value(Frequency::Repeated, tag("repeated")),
        value(Frequency::Required, tag("required")),
    ))(input)
}

fn field_type(input: &str) -> IResult<&str, FieldType> {
    alt((
        value(FieldType::Int32, tag("int32")),
        value(FieldType::Int64, tag("int64")),
        value(FieldType::Uint32, tag("uint32")),
        value(FieldType::Uint64, tag("uint64")),
        value(FieldType::Sint32, tag("sint32")),
        value(FieldType::Sint64, tag("sint64")),
        value(FieldType::Fixed32, tag("fixed32")),
        value(FieldType::Sfixed32, tag("sfixed32")),
        value(FieldType::Fixed64, tag("fixed64")),
        value(FieldType::Sfixed64, tag("sfixed64")),
        value(FieldType::Bool, tag("bool")),
        value(FieldType::StringCow, tag("string")),
        value(FieldType::BytesCow, tag("bytes")),
        value(FieldType::Float, tag("float")),
        value(FieldType::Double, tag("double")),
        map(map_field, |(k, v)| FieldType::Map(Box::new(k), Box::new(v))),
        map(word, |w| FieldType::MessageOrEnum(w)),
    ))(input)
}

fn map_field(input: &str) -> IResult<&str, (FieldType, FieldType)> {
    delimited(
        tuple((tag("map"), many0(br), tag("<"), many0(br))),
        separated_pair(
            field_type,
            delimited(many0(br), tag(","), many0(br)),
            field_type,
        ),
        pair(many0(br), tag(">")),
    )(input)
}

fn message_field(input: &str) -> IResult<&str, Field> {
    map(
        tuple((
            opt(terminated(frequency, many1(br))),
            terminated(field_type, many1(br)),
            separated_pair(
                word,
                delimited(many0(br), tag("="), many0(br)),
                alt((integer, hex_integer)),
            ),
            delimited(many0(br), many0(key_val), pair(many0(br), tag(";"))),
        )),
        |(freq, typ, (name, number), key_vals)| Field {
            name,
            frequency: freq.unwrap_or(Frequency::Optional),
            number,
            default: key_vals.iter().find_map(|&(k, v)| {
                if k == "default" {
                    Some(v.to_string())
                } else {
                    None
                }
            }),
            packed: key_vals.iter().find_map(|&(k, v)| {
                if k == "packed" {
                    Some(v.parse().expect("Cannot parse Packed value"))
                } else {
                    None
                }
            }),
            boxed: false,
            typ,
            deprecated: key_vals
                .iter()
                .find_map(|&(k, v)| {
                    if k == "deprecated" {
                        Some(v.parse().expect("Cannot parse Deprecated value"))
                    } else {
                        None
                    }
                })
                .unwrap_or(false),
        },
    )(input)
}

fn one_of(input: &str) -> IResult<&str, OneOf> {
    map(
        pair(
            preceded(pair(tag("oneof"), many1(br)), word),
            delimited(
                pair(many0(br), tag("{")),
                many1(delimited(many0(br), message_field, many0(br))),
                tag("}"),
            ),
        ),
        |(name, fields)| OneOf {
            name,
            fields,
            package: "".to_string(),
            module: "".to_string(),
            imported: false,
        },
    )(input)
}

fn rpc_function_declaration(input: &str) -> IResult<&str, RpcFunctionDeclaration> {
    map(
        tuple((
            delimited(pair(tag("rpc"), many1(br)), word, many0(br)),
            delimited(pair(tag("("), many0(br)), word, pair(many0(br), tag(")"))),
            delimited(
                tuple((many1(br), tag("returns"), many0(br), tag("("), many0(br))),
                word,
                pair(many0(br), tag(")")),
            ),
            preceded(
                many0(br),
                alt((
                    value(
                        (),
                        delimited(
                            pair(tag("{"), many0(br)),
                            many0(alt((option_ignore, value((), tag(";"))))),
                            pair(many0(br), tag("}")),
                        ),
                    ),
                    value((), tag(";")),
                )),
            ),
        )),
        |(name, arg, ret, _)| RpcFunctionDeclaration { name, arg, ret },
    )(input)
}

fn rpc_service(input: &str) -> IResult<&str, RpcService> {
    map(
        pair(
            delimited(pair(tag("service"), many1(br)), word, many0(br)),
            delimited(
                tag("{"),
                many0(delimited(many0(br), rpc_function_declaration, many0(br))),
                tag("}"),
            ),
        ),
        |(service_name, functions)| RpcService {
            service_name,
            functions,
        },
    )(input)
}

fn message_event(input: &str) -> IResult<&str, MessageEvent> {
    alt((
        map(reserved_nums, |r| MessageEvent::ReservedNums(r)),
        map(reserved_names, |r| MessageEvent::ReservedNames(r)),
        map(message_field, |f| MessageEvent::Field(f)),
        map(message, |m| MessageEvent::Message(m)),
        map(enumerator, |e| MessageEvent::Enumerator(e)),
        map(one_of, |o| MessageEvent::OneOf(o)),
        value(MessageEvent::Ignore, extensions),
        value(MessageEvent::Ignore, br),
    ))(input)
}

fn message(input: &str) -> IResult<&str, Message> {
    map(
        terminated(
            pair(
                delimited(pair(tag("message"), many1(br)), word, many0(br)),
                delimited(tag("{"), many0(message_event), tag("}")),
            ),
            opt(pair(many0(br), tag(";"))),
        ),
        |(name, events)| {
            let mut msg = Message {
                name,
                ..Default::default()
            };
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
        },
    )(input)
}

fn enum_field(input: &str) -> IResult<&str, (String, i32)> {
    terminated(
        separated_pair(
            word,
            tuple((many0(br), tag("="), many0(br))),
            alt((hex_integer, integer)),
        ),
        pair(many0(br), tag(";")),
    )(input)
}

fn enumerator(input: &str) -> IResult<&str, Enumerator> {
    map(
        terminated(
            pair(
                delimited(pair(tag("enum"), many1(br)), word, many0(br)),
                delimited(
                    pair(tag("{"), many0(br)),
                    separated_list0(many0(br), enum_field),
                    pair(many0(br), tag("}")),
                ),
            ),
            opt(pair(many0(br), tag(";"))),
        ),
        |(name, fields)| Enumerator {
            name,
            fields,
            ..Default::default()
        },
    )(input)
}

fn option_ignore(input: &str) -> IResult<&str, ()> {
    value((), delimited(tag("option"), take_until(";"), tag(";")))(input)
}

pub fn file_descriptor(input: &str) -> IResult<&str, FileDescriptor, nom::error::Error<String>> {
    map(
        many0(alt((
            map(syntax, |s| Event::Syntax(s)),
            map(import, |i| Event::Import(i)),
            map(package, |p| Event::Package(p)),
            map(message, |m| Event::Message(m)),
            map(enumerator, |e| Event::Enum(e)),
            map(rpc_service, |r| Event::RpcService(r)),
            value(Event::Ignore, option_ignore),
            value(Event::Ignore, br),
        ))),
        |events| {
            let mut desc = FileDescriptor::default();
            for event in events {
                match event {
                    Event::Syntax(s) => desc.syntax = s,
                    Event::Import(i) => desc.import_paths.push(i),
                    Event::Package(p) => desc.package = p,
                    Event::Message(m) => desc.messages.push(m),
                    Event::Enum(e) => desc.enums.push(e),
                    Event::RpcService(r) => desc.rpc_services.push(r),
                    Event::Ignore => (),
                }
            }
            desc
        },
    )(input).map_err(|e: nom::Err<nom::error::Error<&str>>| e.to_owned())
}

#[cfg(test)]
mod test {
    use super::*;

    use std::path::Path;

    fn assert_complete<T>(parse: nom::IResult<&str, T>) -> T {
        let (rem, obj) = parse.expect("valid parse");
        assert_eq!(rem, "", "expected no trailing data");
        obj
    }

    fn assert_desc(msg: &str) -> FileDescriptor {
        let (rem, obj) = file_descriptor(msg).expect("valid parse");
        assert_eq!("", rem, "expected no trailing data");
        obj
    }

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

        let mess = message(msg);
        if let ::nom::IResult::Ok((_, mess)) = mess {
            assert_eq!(10, mess.fields.len());
        }
    }

    #[test]
    fn empty_message() {
        let msg = r#"message Vec { }"#;
        let mess = assert_complete(message(msg));
        assert_eq!("Vec", mess.name);
        assert_eq!(0, mess.fields.len());
        assert_desc(msg);
    }

    #[test]
    fn test_enum() {
        let msg = r#"enum PairingStatus {
                DEALPAIRED        = 0;
                INVENTORYORPHAN   = 1;
                CALCULATEDORPHAN  = 2;
                CANCELED          = 3;
    }"#;

        let mess = enumerator(msg);
        if let ::nom::IResult::Ok((_, mess)) = mess {
            assert_eq!(4, mess.fields.len());
        }
        assert_desc(msg);
    }

    #[test]
    fn comment_vs_other_whitespace_behaviour() {
        // I think it would be nice to not combine parsing comments with parsing
        // other types of whitespace (like \n) at this level, but admittedly
        // it's a small matter
        assert_eq!("\n", comment("// foo\n").unwrap().0);
    }

    #[test]
    fn test_ignore() {
        let msg = r#"option optimize_for = SPEED;"#;

        match option_ignore(msg) {
            ::nom::IResult::Ok(_) => (),
            e => panic!("Expecting done {:?}", e),
        }
        assert_desc(msg);
    }

    #[test]
    fn test_comments() {
        assert_eq!("\nb", comment("// BOOM\nb").unwrap().0);
        assert_eq!("\nb", comment("//\nb").unwrap().0);
        assert_eq!("", comment("//").unwrap().0);
        assert_eq!("", block_comment("/**/").unwrap().0);
        assert_eq!("\nb", block_comment("/* BOOM */\nb").unwrap().0);
        let msg = r#"
            // BOOM
            //
            /* BOOM */
            package foo.bar;//
            //
            // BOOM

            /* BOOM */
            message A {
                // BOOM
                enum E1 {
                    // BOOM

                    // BOOM
                    V1 = 1;

                    // BOOM
                    v2 = 2;
                }
                // BOOM

                enum E2 {
                    // BOOM
                }

                enum E3 { /* BOOM */ }
                // BOOM
                message B {
                    // BOOM
                    // BOOM
                    optional string b = 1;
                }
                message C {
                    // BOOM
                }
                message D { /* BOOM */ }
                required string a = 1;
            }
            "#;
        let desc = file_descriptor(msg).unwrap().1;
        assert_eq!("foo.bar".to_string(), desc.package);
        assert_eq!(1, desc.messages.len());
        assert_eq!(3, desc.messages[0].messages.len());
        assert_eq!(3, desc.messages[0].enums.len());
        assert_desc(msg);
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
        let desc = file_descriptor(msg).unwrap().1;
        assert_eq!(
            vec![Path::new("test_import_nested_imported_pb.proto")],
            desc.import_paths
        );
        assert_desc(msg);
    }

    #[test]
    fn leading_comment() {
        let msg = r#"//foo
        message Bar {}"#;
        let desc = assert_desc(msg);
        assert_eq!(1, desc.messages.len());
        assert_eq!("Bar", desc.messages[0].name);
        assert_eq!(0, desc.messages[0].fields.len());
        assert_desc(msg);
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
        let desc = file_descriptor(msg).unwrap().1;
        assert_eq!("foo.bar".to_string(), desc.package);
        assert_desc(msg);
    }

    #[test]
    fn test_nested_message() {
        let msg = r#"message A
    {
        message B {
            repeated int32 a = 1;
            optional string b = 2;
        }
        message C {
        }
        message D {}
        optional int32 b = 1;
    }"#;

        let desc = file_descriptor(msg).unwrap().1;
        dbg!(&desc);
        assert_eq!(1, desc.messages.len());
        assert_eq!(3, desc.messages[0].messages.len());
        assert_desc(msg);
    }

    #[test]
    fn test_map() {
        let msg = r#"message A
    {
        optional map<string, int32> b = 1;
    }"#;

        let mess = message(msg);
        if let ::nom::IResult::Ok((_, mess)) = mess {
            assert_eq!(1, mess.fields.len());
            match mess.fields[0].typ {
                FieldType::Map(ref key, ref value) => match (&**key, &**value) {
                    (&FieldType::String_, &FieldType::Int32) => (),
                    (&FieldType::StringCow, &FieldType::Int32) => (),
                    _ => panic!(
                        "Expecting Map<String, Int32> found Map<{:?}, {:?}>",
                        key, value
                    ),
                },
                ref f => panic!("Expecting map, got {:?}", f),
            }
        } else {
            panic!("Could not parse map message");
        }
        assert_desc(msg);
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

        let mess = message(msg);
        if let ::nom::IResult::Ok((_, mess)) = mess {
            assert_eq!(1, mess.oneofs.len());
            assert_eq!(3, mess.oneofs[0].fields.len());
        }
        assert_desc(msg);
    }

    #[test]
    fn test_reserved() {
        let msg = r#"message Sample {
       reserved 4, 15, 17 to 20, 30;
       reserved "foo", "bar";
       uint64 age =1;
       bytes name =2;
    }"#;

        let mess = message(msg);
        dbg!(&mess);
        if let ::nom::IResult::Ok((_, mess)) = mess {
            assert_eq!(Some(vec![4, 15, 17, 18, 19, 20, 30]), mess.reserved_nums);
            assert_eq!(
                Some(vec!["foo".to_string(), "bar".to_string()]),
                mess.reserved_names
            );
            assert_eq!(2, mess.fields.len());
        } else {
            panic!("Could not parse reserved fields message");
        }
        assert_desc(msg);
    }

    #[test]
    fn enum_comments() {
        let msg = r#"enum Turn {
            UP = 1;
            // for what?
            // for what, you ask?
            DOWN = 2;
          }"#;
        let en = assert_complete(enumerator(msg));
        assert_eq!(2, en.fields.len());
        assert_desc(msg);
    }

    #[test]
    fn enum_semicolon() {
        let msg = r#"message Foo { enum Bar { BAZ = 1; }; Bar boop = 1; }"#;
        let desc = assert_desc(msg);
        assert_eq!(1, desc.messages.len());
        assert_eq!(
            FieldType::MessageOrEnum("Bar".to_owned()),
            desc.messages[0].fields[0].typ
        );

        let msg2 = r#"
        message TestMessage {
            enum EnumWithSemicolon {
                val_1                     = 0;
                val_2                     = 1;
                val_3                     = 2;
            };
            enum EnumWithoutSemicolon {
                val_1                     = 0;
                val_2                     = 1;
                val_3                     = 2;
            }
            required EnumWithSemicolon       a    = 1;
            required EnumWithoutSemicolon    b    = 2;
            optional string                  c    = 3;
            optional uint32                  d    = 4;
        }
        "#;
        assert_desc(msg2);
    }

    #[test]
    fn test_rpc_service() {
        let msg = r#"
            service RpcService {
                rpc function0(InStruct0) returns (OutStruct0);
                rpc function1(InStruct1) returns (OutStruct1);
                rpc function2  (  InStruct2  ) returns (  OutStruct2  ) {  }
            }
        "#;

        match file_descriptor(msg) {
            ::nom::IResult::Ok((_, descriptor)) => {
                println!("Services found: {:?}", descriptor.rpc_services);
                let service = &descriptor.rpc_services.get(0).expect("Service not found!");
                let func0 = service.functions.get(0).expect("Function 0 not returned!");
                let func1 = service.functions.get(1).expect("Function 1 not returned!");
                let func2 = service.functions.get(2).expect("Function 2 not returned!");
                assert_eq!("RpcService", service.service_name);
                assert_eq!("function0", func0.name);
                assert_eq!("InStruct0", func0.arg);
                assert_eq!("OutStruct0", func0.ret);
                assert_eq!("function1", func1.name);
                assert_eq!("InStruct1", func1.arg);
                assert_eq!("OutStruct1", func1.ret);
                assert_eq!("function2", func2.name);
                assert_eq!("InStruct2", func2.arg);
                assert_eq!("OutStruct2", func2.ret);
            }
            other => panic!("Could not parse RPC Service: {:?}", other),
        }
        assert_desc(msg);
    }

    #[test]
    fn test_rpc_function() {
        let msg = r#"rpc function_name(Arg) returns (Ret);"#;

        match rpc_function_declaration(msg) {
            ::nom::IResult::Ok((_, declaration)) => {
                assert_eq!("function_name", declaration.name);
                assert_eq!("Arg", declaration.arg);
                assert_eq!("Ret", declaration.ret);
            }
            other => panic!("Could not parse RPC Function Declaration: {:?}", other),
        }
    }

    #[test]
    fn test_missing_tokens() {
        fn test_missing_tokens_message() {
            /* Check that base case is actually correct before we start deleting tokens */
            assert!(message(
                r#"message A {
                    optional int32 a = 1;
                }"#
            )
            .is_ok());

            /* Empty message name */
            assert!(message(
                r#"message {
                    optional int32 a = 1;
                }"#
            )
            .is_err());

            /* Empty message field name */
            assert!(message(
                r#"message A {
                    optional int32 = 1;
                }"#
            )
            .is_err());

            /* Empty message field type */
            assert!(message(
                r#"message A {
                    optional a = 1;
                }"#
            )
            .is_err());

            /* Empty message field number */
            assert!(message(
                r#"message A {
                    optional int32 a = ;
                }"#
            )
            .is_err());
        }

        fn test_missing_tokens_enum() {
            /* Check that base case is actually correct before we start deleting tokens */
            assert!(enumerator(
                r#"enum A {
                    FIELD_A = 0;
                }"#
            )
            .is_ok());

            /* Empty enum name */
            assert!(enumerator(
                r#"enum {
                    FIELD_A = 0;
                }"#
            )
            .is_err());

            /* Empty enum field name */
            assert!(enumerator(
                r#"enum A {
                    = 0;
                }"#
            )
            .is_err());

            /* Empty enum field number */
            assert!(enumerator(
                r#"enum A {
                    FIELD_A = ;
                }"#
            )
            .is_err());
        }

        fn test_missing_tokens_one_of() {
            /* Check that base case is actually correct before we start deleting tokens */
            assert!(one_of(
                r#"oneof a {
                    int32 field_a = 0;
                }"#
            )
            .is_ok());

            /* Empty oneof name */
            assert!(one_of(
                r#"oneof {
                    int32 field_a = 0;
                }"#
            )
            .is_err());

            /* Empty oneof field name */
            assert!(one_of(
                r#"oneof a {
                    int32 = 0;
                }"#
            )
            .is_err());

            /* Empty oneof field type */
            assert!(one_of(
                r#"oneof a {
                    field_a = 0;
                }"#
            )
            .is_err());

            /* Empty oneof field number */
            assert!(one_of(
                r#"oneof a {
                    int32 field_a = ;
                }"#
            )
            .is_err());
        }

        // remember to actually call the test functions!
        test_missing_tokens_message();
        test_missing_tokens_enum();
        test_missing_tokens_one_of();
    }
}
