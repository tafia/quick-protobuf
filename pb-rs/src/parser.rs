use std::convert::TryInto;
use std::str;
use std::{convert::TryFrom, path::PathBuf};

use crate::types::{
    Enumerator, Extend, Extensions, Field, FieldType, FileDescriptor, Frequency, Message, OneOf,
    Proto2Frequency, Proto3Frequency, RpcFunctionDeclaration, RpcService, Syntax,
};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{
        alpha1, alphanumeric1, anychar, digit1, hex_digit1, multispace1, not_line_ending,
    },
    combinator::{map, map_res, opt, recognize, value, verify},
    multi::{many0, many1, separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    IResult,
};

#[derive(Debug, Clone)]
enum ParsingStageFrequencyToken {
    Optional,
    Repeated,
    Required,
}

#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
enum MessageEvent {
    Message(Message),
    Enumerator(Enumerator),
    Field(Field),
    ReservedNums(Vec<i32>),
    ReservedNames(Vec<String>),
    OneOf(OneOf),
    Extensions(Extensions),
    Ignore,
}

#[derive(Debug, Clone)]
enum EnumEvent {
    Field((String, i32)),
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
    Extend(Extend),
    Ignore,
}

fn qualifiable_name(input: &str) -> IResult<&str, String> {
    map(
        verify(
            recognize(pair(opt(tag(".")), separated_list1(tag("."), word))),
            |s: &str| !s.ends_with('.') && !s.contains(".."),
        ),
        std::borrow::ToOwned::to_owned,
    )(input)
}

fn word_ref(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((
            // I would really rather just take in 1 alphabetic
            // character, but just using `alpha1()` is also technically
            // correct for our use case and is simpler to implement in
            // nom apparently
            alpha1,
            tag("_"),
        )),
        many0(alt((alphanumeric1, tag("_")))),
    ))(input)
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
    value(
        (),
        many1(alt((value((), multispace1), comment, block_comment))),
    )(input)
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
        qualifiable_name,
        pair(many0(br), tag(";")),
    )(input)
}

fn extensions(input: &str) -> IResult<&str, Extensions> {
    map(
        delimited(
            pair(tag("extensions"), many1(br)),
            pair(
                integer,
                preceded(pair(many0(br), pair(tag("to"), many1(br))), take_until(";")),
            ),
            tag(";"),
        ),
        |(from, to)| {
            // TODO: is there a better way to parse "max" or a number?
            let s = to.trim();
            let to = if s == "max" {
                Extensions::max()
            } else {
                s.parse().unwrap()
            };
            Extensions { from, to }
        },
    )(input)
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

fn frequency(input: &str) -> IResult<&str, ParsingStageFrequencyToken> {
    alt((
        value(ParsingStageFrequencyToken::Optional, tag("optional")),
        value(ParsingStageFrequencyToken::Repeated, tag("repeated")),
        value(ParsingStageFrequencyToken::Required, tag("required")),
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
        map(qualifiable_name, FieldType::MessageOrEnum),
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

fn default_check<'a>(
    syntax: Syntax,
    typ: FieldType,
    key_vals: &[(&'a str, &'a str)],
) -> Result<Option<String>, &'a str> {
    for &(k, v) in key_vals.iter() {
        if k == "default" {
            return match (syntax, typ) {
                (Syntax::Proto2, FieldType::StringCow | FieldType::BytesCow) => {
                    let remove_compulsory_inverted_commas: IResult<&str, &str> =
                        alt((
                            delimited(tag("\""), take_until("\""), tag("\"")),
                            delimited(tag("\'"), take_until("\'"), tag("\'")),
                        ))(v);
                    remove_compulsory_inverted_commas
                        .map(|(_, s)| Some(s.to_owned()))
                        .map_err(|_| "Default value must be wrapped in inverted commas!")
                }
                (Syntax::Proto2, _) => Ok(Some(v.to_owned())),
                (Syntax::Proto3, _) => Ok(Some(v.to_owned())),
            };
        }
    }
    Ok(None)
}

impl TryFrom<(Syntax, FieldType, Option<ParsingStageFrequencyToken>)> for Frequency {
    type Error = &'static str;

    fn try_from(
        value: (Syntax, FieldType, Option<ParsingStageFrequencyToken>),
    ) -> Result<Self, Self::Error> {
        match value {
            (Syntax::Proto2, FieldType::Map(..), _) => {
                Ok(Frequency::Proto2Frequency(Proto2Frequency::Map))
            }
            (Syntax::Proto2, _, Some(ParsingStageFrequencyToken::Required)) => {
                Ok(Frequency::Proto2Frequency(Proto2Frequency::Required))
            }
            (Syntax::Proto2, _, Some(ParsingStageFrequencyToken::Optional)) => {
                Ok(Frequency::Proto2Frequency(Proto2Frequency::Optional))
            }
            (Syntax::Proto2, _, Some(ParsingStageFrequencyToken::Repeated)) => {
                Ok(Frequency::Proto2Frequency(Proto2Frequency::Repeated))
            }
            (Syntax::Proto2, _, None) => Ok(Frequency::Proto2Frequency(Proto2Frequency::Optional)),
            (Syntax::Proto3, FieldType::Map(..), _) => {
                Ok(Frequency::Proto3Frequency(Proto3Frequency::Map))
            }
            (Syntax::Proto3, _, Some(ParsingStageFrequencyToken::Required)) => {
                Ok(Frequency::Proto3Frequency(Proto3Frequency::Optional))
            }
            (Syntax::Proto3, _, Some(ParsingStageFrequencyToken::Optional)) => {
                Ok(Frequency::Proto3Frequency(Proto3Frequency::Optional))
            }
            (Syntax::Proto3, _, Some(ParsingStageFrequencyToken::Repeated)) => {
                Ok(Frequency::Proto3Frequency(Proto3Frequency::Repeated))
            }
            (Syntax::Proto3, _, None) => Ok(Frequency::Proto3Frequency(Proto3Frequency::Default)),
        }
    }
}

fn field_generic<F>(syntax: Syntax, freq_func: F) -> impl FnMut(&str) -> IResult<&str, Field>
where
    F: Fn(
        (Syntax, FieldType, Option<ParsingStageFrequencyToken>),
    ) -> Result<Frequency, &'static str>,
{
    move |input| -> IResult<&str, Field> {
        map_res(
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
            |(freq, typ, (name, number), key_vals)| {
                Ok::<Field, &str>(Field {
                    name,
                    frequency: freq_func((syntax, typ.clone(), freq))?,
                    number,
                    default: default_check(syntax, typ.clone(), &key_vals)?,
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
                })
            },
        )(input)
    }
}

fn message_field(syntax: Syntax) -> impl FnMut(&str) -> IResult<&str, Field> {
    field_generic(syntax, TryInto::try_into)
}

fn oneof_message_field(syntax: Syntax) -> impl FnMut(&str) -> IResult<&str, Field> {
    field_generic(syntax, |(syntax, _, _)| {
        Ok(match syntax {
            Syntax::Proto2 => Frequency::Proto2Frequency(Proto2Frequency::Required),
            Syntax::Proto3 => Frequency::Proto3Frequency(Proto3Frequency::Optional),
        })
    })
}

fn one_of(syntax: Syntax) -> impl FnMut(&str) -> IResult<&str, OneOf> {
    move |input| {
        map(
            pair(
                preceded(pair(tag("oneof"), many1(br)), word),
                delimited(
                    pair(many0(br), tag("{")),
                    many1(delimited(many0(br), oneof_message_field(syntax), many0(br))),
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

fn message_event(syntax: Syntax) -> impl FnMut(&str) -> IResult<&str, MessageEvent> {
    move |input| {
        alt((
            map(reserved_nums, MessageEvent::ReservedNums),
            map(reserved_names, MessageEvent::ReservedNames),
            map(message_field(syntax), MessageEvent::Field),
            map(message(syntax), MessageEvent::Message),
            map(enumerator, MessageEvent::Enumerator),
            map(one_of(syntax), MessageEvent::OneOf),
            map(extensions, MessageEvent::Extensions),
            value(MessageEvent::Ignore, option_ignore),
            value(MessageEvent::Ignore, br),
        ))(input)
    }
}

fn message(syntax: Syntax) -> impl FnMut(&str) -> IResult<&str, Message> {
    move |input| {
        map(
            terminated(
                pair(
                    delimited(pair(tag("message"), many1(br)), word, many0(br)),
                    delimited(tag("{"), many0(message_event(syntax)), tag("}")),
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
                        MessageEvent::Extensions(e) => msg.extensions = Some(e),
                        MessageEvent::Ignore => (),
                    }
                }
                msg
            },
        )(input)
    }
}

fn enum_field(input: &str) -> IResult<&str, (String, i32)> {
    terminated(
        separated_pair(
            word,
            tuple((many0(br), tag("="), many0(br))),
            alt((hex_integer, integer)),
        ),
        pair(
            many0(alt((
                br,
                // TODO: add proper deprecation later. We ignore deprecated enum
                // fields for now
                value(
                    (),
                    tuple((
                        tag("["),
                        many0(multispace1),
                        tag("deprecated"),
                        many0(multispace1),
                        tag("="),
                        many0(multispace1),
                        word,
                        many0(multispace1),
                        tag("]"),
                    )),
                ),
            ))),
            tag(";"),
        ),
    )(input)
}

fn enum_event(input: &str) -> IResult<&str, EnumEvent> {
    alt((
        map(enum_field, EnumEvent::Field),
        value(EnumEvent::Ignore, option_ignore),
        value(EnumEvent::Ignore, br),
    ))(input)
}

fn enumerator(input: &str) -> IResult<&str, Enumerator> {
    map_res(
        terminated(
            pair(
                delimited(pair(tag("enum"), many1(br)), word, many0(br)),
                delimited(tag("{"), many0(enum_event), tag("}")),
            ),
            opt(pair(many0(br), tag(";"))),
        ),
        |(name, events)| {
            let mut enumerator = Enumerator {
                name,
                ..Default::default()
            };
            for event in events {
                if let EnumEvent::Field(f) = event {
                    enumerator.fields.push(f);
                }
            }
            Ok::<Enumerator, &str>(enumerator)
        },
    )(input)
}

fn option_ignore(input: &str) -> IResult<&str, ()> {
    value(
        (),
        delimited(pair(tag("option"), many1(br)), take_until(";"), tag(";")),
    )(input)
}

fn extend(syntax: Syntax) -> impl FnMut(&str) -> IResult<&str, Extend> {
    move |input| {
        map(
            terminated(
                pair(
                    delimited(pair(tag("extend"), many1(br)), qualifiable_name, many0(br)),
                    delimited(
                        tag("{"),
                        many1(delimited(many0(br), message_field(syntax), many0(br))),
                        tag("}"),
                    ),
                ),
                opt(pair(many0(br), tag(";"))),
            ),
            |(name, fields)| Extend { name, fields },
        )(input)
    }
}

fn scan_syntax(input: &str) -> IResult<&str, Syntax> {
    map_res(separated_list0(many0(anychar), syntax), |v| {
        Ok::<Syntax, &str>(if v.is_empty() { Syntax::Proto2 } else { v[0] })
    })(input)
}

pub fn file_descriptor<'a>(
    input: &'a str,
) -> IResult<&'a str, FileDescriptor, nom::error::Error<String>> {
    let got_syntax = scan_syntax(input).unwrap().1;

    let parser =
        move |input: &'a str| -> IResult<&'a str, FileDescriptor, nom::error::Error<&str>> {
            map(
                many0(alt((
                    map(syntax, Event::Syntax),
                    map(import, Event::Import),
                    map(package, Event::Package),
                    map(message(got_syntax), Event::Message),
                    map(enumerator, Event::Enum),
                    map(rpc_service, Event::RpcService),
                    map(extend(got_syntax), Event::Extend),
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
                            Event::Extend(e) => desc.message_extends.push(e),
                            Event::Ignore => (),
                        }
                    }
                    desc
                },
            )(input)
        };

    parser(input).map_err(|e: nom::Err<nom::error::Error<&str>>| e.to_owned())
}

#[cfg(test)]
mod test {
    use super::*;

    use std::path::Path;

    fn result_assert(cond: bool, msg: Option<&str>) -> Result<(), &str> {
        if cond {
            Ok(())
        } else {
            Err(msg.unwrap_or(""))
        }
    }

    fn result_assert_eq<T: PartialEq>(o1: T, o2: T, msg: Option<&str>) -> Result<(), &str> {
        result_assert(o1 == o2, msg)
    }

    fn test_syntaxes<F>(test_func: F)
    where
        F: Fn(Syntax) -> Result<(), &'static str>,
    {
        test_func(Syntax::Proto2).unwrap_or_else(|msg| panic!("Proto 2 version failed\n{}", msg));
        test_func(Syntax::Proto3).unwrap_or_else(|msg| panic!("Proto 3 version failed\n{}", msg));
    }

    fn assert_complete<T>(parse: nom::IResult<&str, T>) -> Result<T, &str> {
        let (rem, obj) = parse.expect("valid parse");
        result_assert_eq(rem, "", Some("expected no trailing data"))?;
        Ok(obj)
    }

    fn assert_desc(msg: &str) -> Result<FileDescriptor, &str> {
        let (rem, obj) = file_descriptor(msg).expect("valid parse");
        result_assert_eq("", rem, Some("expected no trailing data"))?;
        Ok(obj)
    }

    #[test]
    fn test_message() {
        test_syntaxes(move |syntax: Syntax| -> Result<(), &str> {
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

            let mess = message(syntax)(msg);
            if let ::nom::IResult::Ok((_, mess)) = mess {
                result_assert_eq(10, mess.fields.len(), None)?;
            }
            Ok(())
        });
    }

    #[test]
    fn empty_message() {
        test_syntaxes(move |syntax| -> Result<(), &str> {
            let msg = r#"message Vec { }"#;
            let mess = assert_complete(message(syntax)(msg))?;
            result_assert_eq("Vec", &mess.name, None)?;
            result_assert_eq(0, mess.fields.len(), None)?;
            assert_desc(msg)?;
            Ok(())
        });
    }

    #[test]
    fn test_enum() {
        test_syntaxes(move |_| -> Result<(), &str> {
            let msg = r#"enum PairingStatus {
                DEALPAIRED        = 0;
                INVENTORYORPHAN   = 1;
                CALCULATEDORPHAN  = 2;
                CANCELED          = 3;
            }"#;

            let mess = enumerator(msg);
            if let ::nom::IResult::Ok((_, mess)) = mess {
                result_assert_eq(4, mess.fields.len(), None)?;
            }
            assert_desc(msg)?;
            Ok(())
        })
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

        let msg2 = r#"option (parenthesized) = 123;"#;

        match option_ignore(msg2) {
            ::nom::IResult::Ok(_) => (),
            e => panic!("Expecting done {:?}", e),
        }
        assert_desc(msg).unwrap();
        assert_desc(msg2).unwrap();
    }

    #[test]
    fn test_comments() {
        assert_eq!("\nb", comment("// BOOM\nb").unwrap().0);
        assert_eq!("\nb", comment("//\nb").unwrap().0);
        assert_eq!("", comment("//").unwrap().0);
        assert_eq!("", block_comment("/**/").unwrap().0);
        assert_eq!("\nb", block_comment("/* BOOM */\nb").unwrap().0);
        assert_eq!("*/", block_comment("/*/* hi */*/").unwrap().0); // nested block comments not supported

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
        assert_desc(msg).unwrap();
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
        assert_desc(msg).unwrap();
    }

    #[test]
    fn leading_comment() {
        let msg = r#"//foo
        message Bar {}"#;
        let desc = assert_desc(msg).unwrap();
        assert_eq!(1, desc.messages.len());
        assert_eq!("Bar", desc.messages[0].name);
        assert_eq!(0, desc.messages[0].fields.len());
        assert_desc(msg).unwrap();
    }

    #[test]
    fn test_package() {
        let msg = r#"
        package foo.bar;

    message ContainsImportedNested {
        optional ContainerForNested.NestedMessage m = 1;
        optional ContainerForNested.NestedEnum e = 2;
    }"#;
        let desc = file_descriptor(msg).unwrap().1;
        assert_eq!("foo.bar".to_string(), desc.package);
        assert_desc(msg).unwrap();
    }

    #[test]
    fn test_message_option() {
        let msg = r#"message A {
            option (extension_option) = "hello, world!";
            option non_extension_option = 12345;

            optional int32 field = 1;
        }"#;

        let desc = file_descriptor(msg).unwrap().1;
        assert_eq!(1, desc.messages.len());
    }

    #[test]
    fn test_enum_option() {
        let msg = r#"enum A {
            option (an_extension_option) = "value";
            option a_non_extension_option = 12345;
            TEST_FIELD = 0;
        }"#;
        let desc = file_descriptor(msg).unwrap().1;
        assert_eq!(1, desc.enums.len());
    }

    #[test]
    fn test_extend() {
        let msg = r#"message A {
            optional int32 a = 1;
            extensions 12000 to 12500;
        }
        extend A {
            optional int32 g = 12123;
            optional int32 h = 12123;
            optional int32 t = 12123;
        }
        "#;
        let desc = file_descriptor(msg).unwrap().1;
        assert_eq!(1, desc.messages.len());
        assert_eq!(1, desc.message_extends.len());
        let extend = &desc.message_extends[0];
        assert_eq!("A", &extend.name);
        assert_eq!(3, extend.fields.len());
        let g = &extend.fields[0];
        let h = &extend.fields[1];
        let t = &extend.fields[2];
        assert_eq!("g", &g.name);
        assert_eq!("h", &h.name);
        assert_eq!("t", &t.name);
    }

    #[test]
    fn test_extend_some_other() {
        let msg = r#"
        extend .foo.bar.Baz {
            optional int32 b = 12123;
        }
        "#;
        let desc = file_descriptor(msg).unwrap().1;
        assert_eq!(0, desc.messages.len());
        assert_eq!(1, desc.message_extends.len());
        let extend = &desc.message_extends[0];
        assert_eq!(".foo.bar.Baz", &extend.name);
        assert_eq!(1, extend.fields.len());
        let field = &extend.fields[0];
        assert_eq!("b", &field.name);
    }

    #[test]
    fn test_extensions() {
        let msg = r#"message A {
            optional int32 a = 1;

            extensions 1300 to max;
        }
        message B {
            optional int32 b = 1;

            extensions 10321 to 11000;
        }
        message c {
            optional int32 c = 1;
        }"#;

        let desc = file_descriptor(msg).unwrap().1;
        assert_eq!(3, desc.messages.len());
        let a = &desc.messages[0].extensions;
        let b = &desc.messages[1].extensions;
        let c = &desc.messages[2].extensions;
        assert!(a.is_some());
        assert!(b.is_some());
        assert!(c.is_none());
        let a = a.as_ref().unwrap();
        let b = b.as_ref().unwrap();
        assert_eq!(a.from, 1300);
        assert_eq!(a.to, Extensions::max());
        assert_eq!(b.from, 10321);
        assert_eq!(b.to, 11000);
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
        assert_eq!(1, desc.messages.len());
        assert_eq!(3, desc.messages[0].messages.len());
        assert_desc(msg).unwrap();
    }

    #[test]
    fn test_map() {
        test_syntaxes(move |syntax: Syntax| -> Result<(), &str> {
            let msg = r#"message A
            {
                optional map<string, int32> b = 1;
            }"#;

            let mess = message(syntax)(msg);
            if let ::nom::IResult::Ok((_, mess)) = mess {
                result_assert_eq(1, mess.fields.len(), None)?;
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
            assert_desc(msg)?;
            Ok(())
        })
    }

    #[test]
    fn test_oneof() {
        test_syntaxes(move |syntax: Syntax| -> Result<(), &str> {
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

            let mess = message(syntax)(msg);
            if let ::nom::IResult::Ok((_, mess)) = mess {
                result_assert_eq(1, mess.oneofs.len(), None)?;
                result_assert_eq(3, mess.oneofs[0].fields.len(), None)?;
            }
            assert_desc(msg)?;
            Ok(())
        });
    }

    #[test]
    fn test_reserved() {
        test_syntaxes(move |syntax: Syntax| -> Result<(), &str> {
            let msg = r#"message Sample {
                reserved 4, 15, 17 to 20, 30;
                reserved "foo", "bar";
                optional uint64 age =1;
                optional bytes name =2;
            }"#;

            let mess = message(syntax)(msg);
            dbg!(&mess);
            if let ::nom::IResult::Ok((_, mess)) = mess {
                result_assert_eq(
                    Some(vec![4, 15, 17, 18, 19, 20, 30]),
                    mess.reserved_nums,
                    None,
                )?;
                result_assert_eq(
                    Some(vec!["foo".to_string(), "bar".to_string()]),
                    mess.reserved_names,
                    None,
                )?;
                result_assert_eq(2, mess.fields.len(), None)?;
            } else {
                panic!("Could not parse reserved fields message");
            }
            assert_desc(msg)?;
            Ok(())
        });
    }

    #[test]
    fn enum_comments() {
        test_syntaxes(move |_| -> Result<(), &str> {
            let msg = r#"enum Turn {
                UP = 0;
                // for what?
                // for what, you ask?
                DOWN = 1;
            }"#;
            let en = assert_complete(enumerator(msg))?;
            result_assert_eq(2, en.fields.len(), None)?;
            assert_desc(msg)?;
            Ok(())
        });
    }

    #[test]
    fn enum_semicolon() {
        let msg = r#"message Foo { enum Bar { BAZ = 1; }; Bar boop = 1; }"#;
        let desc = assert_desc(msg).unwrap();
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
        assert_desc(msg2).unwrap();
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
        assert_desc(msg).unwrap();
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
        fn test_missing_tokens_message(syntax: Syntax) -> Result<(), &'static str> {
            /* Check that base case is actually correct before we start deleting tokens */
            result_assert(
                message(syntax)(
                    r#"message A {
                    optional int32 a = 1;
                }"#,
                )
                .is_ok(),
                None,
            )?;

            /* Empty message name */
            result_assert(
                message(syntax)(
                    r#"message {
                    optional int32 a = 1;
                }"#,
                )
                .is_err(),
                None,
            )?;

            /* Empty message field name */
            result_assert(
                message(syntax)(
                    r#"message A {
                    optional int32 = 1;
                }"#,
                )
                .is_err(),
                None,
            )?;

            /* Empty message field type */
            result_assert(
                message(syntax)(
                    r#"message A {
                    optional a = 1;
                }"#,
                )
                .is_err(),
                None,
            )?;

            /* Empty message field number */
            result_assert(
                message(syntax)(
                    r#"message A {
                    optional int32 a = ;
                }"#,
                )
                .is_err(),
                None,
            )?;

            Ok(())
        }

        fn test_missing_tokens_enum() -> Result<(), &'static str> {
            /* Check that base case is actually correct before we start deleting tokens */
            result_assert(
                enumerator(
                    r#"enum A {
                    FIELD_A = 0;
                }"#,
                )
                .is_ok(),
                None,
            )?;

            /* Empty enum name */
            result_assert(
                enumerator(
                    r#"enum {
                    FIELD_A = 0;
                }"#,
                )
                .is_err(),
                None,
            )?;

            /* Empty enum field name */
            result_assert(
                enumerator(
                    r#"enum A {
                    = 0;
                }"#,
                )
                .is_err(),
                None,
            )?;

            /* Empty enum field number */
            result_assert(
                enumerator(
                    r#"enum A {
                    FIELD_A = ;
                }"#,
                )
                .is_err(),
                None,
            )?;

            Ok(())
        }

        fn test_missing_tokens_one_of(syntax: Syntax) -> Result<(), &'static str> {
            /* Check that base case is actually correct before we start deleting tokens */
            result_assert(
                one_of(syntax)(
                    r#"oneof a {
                    int32 field_a = 0;
                }"#,
                )
                .is_ok(),
                None,
            )?;

            /* Empty oneof name */
            result_assert(
                one_of(syntax)(
                    r#"oneof {
                    int32 field_a = 0;
                }"#,
                )
                .is_err(),
                None,
            )?;

            /* Empty oneof field name */
            result_assert(
                one_of(syntax)(
                    r#"oneof a {
                    int32 = 0;
                }"#,
                )
                .is_err(),
                None,
            )?;
            /* Empty oneof field type */
            result_assert(
                one_of(syntax)(
                    r#"oneof a {
                    field_a = 0;
                }"#,
                )
                .is_err(),
                None,
            )?;
            /* Empty oneof field number */
            result_assert(
                one_of(syntax)(
                    r#"oneof a {
                    int32 field_a = ;
                }"#,
                )
                .is_err(),
                None,
            )?;

            Ok(())
        }

        // remember to actually call the test functions!
        test_syntaxes(move |syntax: Syntax| -> Result<(), &str> {
            test_missing_tokens_message(syntax)?;
            test_missing_tokens_enum()?;
            test_missing_tokens_one_of(syntax)?;

            Ok(())
        });
    }

    #[test]
    fn test_word_and_qualifiable_names() {
        fn assert_qualifiable_name_but_not_word(str_to_parse: &str, qualifiable_name_res: &str) {
            assert!(word(str_to_parse).is_err());
            assert_eq!(
                qualifiable_name_res,
                qualifiable_name(str_to_parse).unwrap().1
            );
        }

        fn assert_both_ok_same_res(str_to_parse: &str, res: &str) {
            assert_eq!(res, word(str_to_parse).unwrap().1);
            assert_eq!(res, qualifiable_name(str_to_parse).unwrap().1);
        }

        fn assert_both_ok_different_res(
            str_to_parse: &str,
            word_res: &str,
            qualifiable_name_res: &str,
        ) {
            assert_eq!(word_res, word(str_to_parse).unwrap().1);
            assert_eq!(
                qualifiable_name_res,
                qualifiable_name(str_to_parse).unwrap().1
            );
        }

        fn assert_both_err(s: &str) {
            assert!(word(s).is_err());
            assert!(qualifiable_name(s).is_err());
        }

        assert_both_ok_same_res("_a@", "_a");
        assert_both_ok_same_res("a_@", "a_");
        assert_both_ok_same_res("TestTypesRepeatedPacked", "TestTypesRepeatedPacked");
        assert_both_ok_same_res("_TestTypes_Repeated_Packed", "_TestTypes_Repeated_Packed");
        assert_both_ok_same_res(
            "____TestTypes__Repeated_Packed",
            "____TestTypes__Repeated_Packed",
        );
        assert_both_ok_same_res("TestTypesRepeatedPacked_", "TestTypesRepeatedPacked_");
        assert_both_ok_same_res("TestTypesRepeatedPacked____", "TestTypesRepeatedPacked____");
        assert_both_ok_same_res("______", "______");
        assert_both_ok_same_res("_1a.._2a@", "_1a");

        assert_both_err("");
        assert_both_err("@_a1");
        assert_both_err("123a");
        assert_both_err("1TestTypesRepeatedPacked");
        assert_both_err(".1TestTypesRepeatedPacked");
        assert_both_err(".1TestTypesRepeatedPacked_2hello");
        assert_both_err("1_1a._2a@");

        assert_qualifiable_name_but_not_word("._1a", "._1a");
        assert_qualifiable_name_but_not_word("._a@", "._a");
        assert_qualifiable_name_but_not_word("._1a.2_2a@", "._1a");
        assert_qualifiable_name_but_not_word(".Test1._Test2.3Test3.Test4@", ".Test1._Test2");
        assert_qualifiable_name_but_not_word("._1a._2a@", "._1a._2a");

        assert_both_ok_different_res("_1a._2a@", "_1a", "_1a._2a");
        assert_both_ok_different_res("Test1.Test2@", "Test1", "Test1.Test2");
        assert_both_ok_different_res("Test1._Test2.3Test3.Test4@", "Test1", "Test1._Test2");
    }

    #[test]
    fn test_which_names_can_be_qualified() {
        fn test_which_names_can_be_qualified_message(syntax: Syntax) -> Result<(), &'static str> {
            /* Check that base case is actually correct before we start qualifying names */
            result_assert(
                message(syntax)(
                    r#"message A {
                    optional int32 a = 1;
                }"#,
                )
                .is_ok(),
                None,
            )?;

            /* Message field types CAN be qualified */
            result_assert(
                message(syntax)(
                    r#"message A {
                    optional Qualified.Name a = 1;
                }"#,
                )
                .is_ok(),
                None,
            )?;

            /* Message names CANNOT be qualified */
            result_assert(
                message(syntax)(
                    r#"message Qualified.Name {
                    optional int32 a = 1;
                }"#,
                )
                .is_err(),
                None,
            )?;

            /* Message field names CANNOT be qualified */
            result_assert(
                message(syntax)(
                    r#"message A {
                    optional int32 qualified.name = 1;
                }"#,
                )
                .is_err(),
                None,
            )?;

            Ok(())
        }

        fn test_which_names_can_be_qualified_enum() -> Result<(), &'static str> {
            /* Check that base case is actually correct before we start qualifying names */
            result_assert(
                enumerator(
                    r#"enum A {
                    enum_field = 0;
                }"#,
                )
                .is_ok(),
                None,
            )?;

            /* Enum names CANNOT be qualified */
            result_assert(
                enumerator(
                    r#"enum Qualified.Name {
                    enum_field = 0;
                }"#,
                )
                .is_err(),
                None,
            )?;

            /* Enum field names CANNOT be qualified */
            result_assert(
                enumerator(
                    r#"enum A {
                    QUALIFIED.NAME = 0;
                }"#,
                )
                .is_err(),
                None,
            )?;

            Ok(())
        }

        fn test_which_names_can_be_qualified_oneof(syntax: Syntax) -> Result<(), &'static str> {
            /* Check that base case is actually correct before we start qualifying names */
            result_assert(
                one_of(syntax)(
                    r#"oneof a {
                    int32 field_name = 1;
                }"#,
                )
                .is_ok(),
                None,
            )?;

            /* Oneof field types CAN be qualified */
            result_assert(
                one_of(syntax)(
                    r#"oneof a {
                    Qualified.Name field_name = 1;
                }"#,
                )
                .is_ok(),
                None,
            )?;

            /* Oneof names CANNOT be qualified */
            result_assert(
                one_of(syntax)(
                    r#"oneof qualified.name {
                    int32 field_name = 1;
                }"#,
                )
                .is_err(),
                None,
            )?;

            /* Oneof field names CANNOT be qualified */
            result_assert(
                one_of(syntax)(
                    r#"oneof a {
                    int32 qualified.name = 1;
                }"#,
                )
                .is_err(),
                None,
            )?;

            Ok(())
        }

        // remember to actually call the test functions!
        test_syntaxes(move |syntax: Syntax| -> Result<(), &str> {
            test_which_names_can_be_qualified_message(syntax)?;
            test_which_names_can_be_qualified_enum()?;
            test_which_names_can_be_qualified_oneof(syntax)?;

            Ok(())
        });
    }

    #[test]
    fn enum_deprecated() {
        let e = enumerator(
            r#"enum Reason {
                HELLO                               = 0;

                SOME_FIELD                          = 1;

                SOME_OTHER_FIELD                    = 2;
                BLA_BLA_BLA                         = 3;
                THING                               = 9;    // comment
                OTHER_THING                         = 10;   // comment & comment
                ANOTHER_THING                       = 11;
                AGAIN_A_THING                       = 12;
                THINGY                              = 13;
                THINGYTHINGY                        = 14; // comment
                // comment
                THINGYTHINGYTHINGY                  = 15 [deprecated=true];    // comment
                THINGYTHINGYTHINGYTHINGY            = 16;
                THINGYTHINGYTHINGYTHINGYTHINGY      = 17;
                BLAB                                = 34;
                BLABBLAB                            = 35;   // comment
                BLABBLABBLAB                        = 36 [deprecated=true];   // comment
                BLABBLABBLABBLAB                    = 37;   // comment

                BLABBLABBLABBLABBLAB                = 100;
                HA                                  = 101;
                HAHA                                = 102 [deprecated=true];
                HAHAHA                              = 103;
                HAHAHAHA                            = 104 [deprecated=true];
                HAHAHAHAHA                          = 105 [deprecated=true];
            }"#,
        );
        assert!(e.is_ok());
    }
}
