use crate::JsonValue;
use nom::{
    branch::alt,
    bytes::streaming::{escaped, is_not, tag, take_while},
    character::streaming::{alphanumeric1, char, one_of},
    combinator::{cut, map, map_res, value, verify},
    error::context,
    multi::{fold_many0, separated_list0},
    number::streaming::double,
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult,
};
use std::collections::HashMap;
use std::str::from_utf8;

pub fn sp_trn(input: &[u8]) -> IResult<&[u8], &[u8], ()> {
    let chars = b" \t\r\n";

    take_while(move |c: u8| chars.contains(&c))(input)
}
pub fn null(input: &[u8]) -> IResult<&[u8], (), ()> {
    value((), tag("null"))(input)
}

pub fn boolean(input: &[u8]) -> IResult<&[u8], bool, ()> {
    alt((value(true, tag("true")), value(false, tag("false"))))(input)
}

pub fn parse_str(input: &[u8]) -> IResult<&[u8], &[u8], ()> {
    escaped(alphanumeric1, '\\', one_of("\"n\\"))(input)
}

pub fn parse_string(input: &[u8]) -> IResult<&[u8], &[u8], ()> {
    context(
        "parse_string",
        preceded(char('\"'), cut(terminated(parse_str, char('\"')))),
    )(input)
}

pub fn string_old(input: &[u8]) -> IResult<&[u8], &str, ()> {
    map_res(parse_string, |s| from_utf8(s))(input)
}

pub fn parse_literal(input: &[u8]) -> IResult<&[u8], &str, ()> {
    log::trace!("parse_literal: input={:?}", input);
    let not_quote_slash = is_not("\"\\");
    let res = verify(map_res(not_quote_slash, |s| from_utf8(s)), |s: &str| {
        !s.is_empty()
    })(input);
    // let res = map_res(parse_str, |s| from_utf8(s))(input);
    log::trace!("parse_literal: res={:?}", res);
    res
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum StringFragment<'a> {
    Literal(&'a str),
}

pub fn parse_fragment<'a>(input: &'a [u8]) -> IResult<&[u8], StringFragment<'a>, ()> {
    log::trace!("parse_fragment: input={:?}", input);
    let res = alt((map(parse_literal, StringFragment::Literal),))(input);
    log::trace!("parse_fragment: res={:?}", res);
    res
}
pub fn string(input: &[u8]) -> IResult<&[u8], String, ()> {
    log::trace!("string: input={:?}", input);
    let build_string = fold_many0(parse_fragment, String::new, |mut string, fragment| {
        log::trace!("string: fragment={:?}", fragment);
        match fragment {
            StringFragment::Literal(s) => string.push_str(s),
        }
        log::trace!("string: string={:?}", &string);
        string
    });

    let res = delimited(char('\"'), build_string, char('\"'))(input);
    log::trace!("string2: res={:?}", res);
    res
}

pub fn array(input: &[u8]) -> IResult<&[u8], Vec<JsonValue>, ()> {
    context(
        "array",
        preceded(
            char('['),
            cut(terminated(
                separated_list0(preceded(sp_trn, char(',')), preceded(sp_trn, json_value)),
                preceded(sp_trn, char(']')),
            )),
        ),
    )(input)
}

pub fn key_value(input: &[u8]) -> IResult<&[u8], (String, JsonValue), ()> {
    separated_pair(
        preceded(sp_trn, string),
        cut(preceded(sp_trn, char(':'))),
        preceded(sp_trn, json_value),
    )(input)
}

pub fn hash(input: &[u8]) -> IResult<&[u8], HashMap<String, JsonValue>, ()> {
    context(
        "map",
        preceded(
            char('{'),
            cut(terminated(
                map(
                    separated_list0(preceded(sp_trn, char(',')), key_value),
                    |tuple_vec| {
                        tuple_vec
                            .into_iter()
                            // .map(|(k, v)| (String::from(k), v))
                            .collect()
                    },
                ),
                preceded(sp_trn, char('}')),
            )),
        ),
    )(input)
}

pub fn json_value(input: &[u8]) -> IResult<&[u8], JsonValue, ()> {
    preceded(
        sp_trn,
        alt((
            map(null, |_| JsonValue::Null),
            map(boolean, JsonValue::Bool),
            map(double, JsonValue::Num),
            map(string, |s| JsonValue::Str(String::from(s))),
            map(array, JsonValue::Array),
            map(hash, JsonValue::Object),
        )),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    mod array {
        use super::*;

        #[rstest]
        #[case(b"[]", [].to_vec())]
        #[case(b"[null]", vec![JsonValue::Null])]
        #[case(
            b"[null,\nnull\n]",
            vec![JsonValue::Null, JsonValue::Null]
        )]
        fn valids(#[case] input: &[u8], #[case] expected: Vec<JsonValue>) {
            println!("input: '{:?}", input);
            assert_eq!(array(input), Ok((&b""[..], expected)));
        }

        #[test]
        fn valid_empty() {
            let inputs = [&b"[]"[..], &b"[  ]"[..]];
            let empty_vec: Vec<JsonValue> = vec![];
            let expected_result = Ok((&b""[..], empty_vec));
            for input in &inputs {
                println!("input: '{:?}'", input);

                assert_eq!(&array(input), &expected_result);
            }
        }

        #[test]
        fn valid_null() {
            let inputs = [
                &b"[null]"[..],
                &b"[ null]"[..],
                &b"[null ]"[..],
                &b"[ null ]"[..],
            ];
            let expected_result = Ok((&b""[..], vec![JsonValue::Null]));
            for input in &inputs {
                println!("input: '{:?}'", input);
                assert_eq!(&array(input), &expected_result);
            }
        }

        #[test]
        fn valid_nulls() {
            let inputs = [
                &b"[null,null]"[..],
                &b"[null, null]"[..],
                &b"[null, null ]"[..],
                &b"[ null,null ]"[..],
                &b"[ null\t,\nnull ]"[..],
            ];
            let expected_result = Ok((&b""[..], vec![JsonValue::Null, JsonValue::Null]));
            for input in &inputs {
                println!("input: '{:?}'", input);
                assert_eq!(&array(input), &expected_result);
            }
        }

        #[test]
        fn valid_arrays() {
            let expected_vec: Vec<JsonValue> =
                vec![JsonValue::Array(vec![JsonValue::Null]), JsonValue::Null];
            assert_eq!(array(&b"[[null],null]"[..]), Ok((&b""[..], expected_vec)));
        }
    }

    mod hash {
        use super::*;

        #[test]
        fn valid_empty() {
            let inputs = [&b"{}"[..], &b"{  }"[..], &b"{\n}"[..], &b"{\t}"[..]];
            let expected_result = Ok((&b""[..], HashMap::new()));
            for input in &inputs {
                println!("input: '{:?}'", input);
                assert_eq!(&hash(input), &expected_result);
            }
        }

        #[test]
        fn valid_null() {
            let inputs = [
                &b"{\"a\": null}"[..],
                &b"{ \"a\": null}"[..],
                &b"{\"a\":\n null}"[..],
                &b"{\"a\" : null}"[..],
                &b"{\"a\": null\n}"[..],
                &b"{\"a\": null}"[..],
                &b"{ \"a\" : null }"[..],
            ];
            let expected_result = Ok((
                &b""[..],
                [(String::from("a"), JsonValue::Null)].into_iter().collect(),
            ));
            for input in &inputs {
                println!("input: '{:?}'", input);
                assert_eq!(&hash(input), &expected_result);
            }
        }
    }
    mod null {
        use super::*;

        #[test]
        fn valid_null() {
            assert_eq!(null(&b"null"[..]), Ok((&b""[..], ())));
        }
    }

    mod string {
        use super::*;

        #[rstest]
        #[case(b"\"\"", "")]
        #[case(b"\"abc\"", "abc")]
        #[case(b"\"h8\"", "h8")]
        #[case(b"\"\xE2\x82\xAC67\"", "€67")]
        #[case(b"\"hello world\"", "hello world")]
        #[case(b"\"+46 1234567\"", "+46 1234567")]
        fn valids(#[case] input: &[u8], #[case] expected: String) {
            println!("input: {:?}", input);
            assert_eq!(string(input), Ok((&b""[..], expected)));
        }
    }
}
