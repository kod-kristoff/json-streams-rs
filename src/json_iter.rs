
use std::collections::HashMap;
use std::io;
use nom::{
    branch::alt,
    bytes::streaming::{tag},
    character::streaming::{char},
    combinator::{map, value},
    sequence::{preceded, terminated},
    IResult,
};
use nom_bufreader::bufreader::BufReader;
use nom_bufreader::{Parse};

use crate::JsonValue;
use crate::parsers::streaming::{
    json_value, sp_trn,
};


pub fn load_from_read<R: 'static>(read: R) -> Box<dyn Iterator<Item=JsonValue>>
where
    R: io::Read,
{
    log::trace!("called json_streams::load_from_read()");
    let mut reader = BufReader::new(read);
    // let res = reader.parse(root);
    match reader.parse(root) {
        Ok(IterValue::JsonVal(val)) => Box::new(std::iter::once(val)),
        Ok(IterValue::ArrayBegin) => Box::new(JsonIterator::new(reader)),
        _ => panic!("how to handle thid?")
    }
}

// #[derive(Debug)]
pub struct JsonIterator<R> {
    reader: BufReader<R>,
    seen_last: bool,

}

impl<R> JsonIterator<R> {
    pub fn new(reader: BufReader<R>) -> Self {
        Self { reader, seen_last: false }
    }
}

impl<R> Iterator for JsonIterator<R>
where
    R: io::Read,
{
    type Item = JsonValue;

    fn next(&mut self) -> Option<Self::Item> {
        log::trace!("called JsonIterator::next()");
        if self.seen_last {
            return None;
        }
        let res = self.reader.parse(array_item);
        log::debug!("Got: {:#?}", res);
        match res {
            Ok(ArrayItem::Item(val)) => Some(val),
            Ok(ArrayItem::LastItem(val)) => {
                self.seen_last = true;
                Some(val)
            },
            Ok(ArrayItem::End) => None,
            Err(err) => {
                log::error!("error: {:#?}", err);
                None
            },
        }
    }
}

#[derive(Debug)]
enum IterValue {
    JsonVal(JsonValue),
    ArrayBegin,
}

fn root(input: &[u8]) -> IResult<&[u8], IterValue, ()> {
    alt((
        map(array_start, |_| IterValue::ArrayBegin),
        map(json_value, IterValue::JsonVal),
    ))(input)
}

fn array_start(input: &[u8]) -> IResult<&[u8], (), ()> {
    value((), tag("["))(input)
}

fn array_end(input: &[u8]) -> IResult<&[u8], (), ()> {
    value((), tag("]"))(input)
}

#[derive(Debug, PartialEq)]
enum ArrayItem {
    Item(JsonValue),
    LastItem(JsonValue),
    End,
}

fn array_item(input: &[u8]) -> IResult<&[u8], ArrayItem, ()> {
    preceded(
        sp_trn,
        alt((
            map(
                terminated(json_value, preceded(sp_trn, char(','))),
                ArrayItem::Item
            ),
            map(
                terminated(json_value, preceded(sp_trn, char(']'))),
                ArrayItem::LastItem
            ),
            map(array_end, |_| ArrayItem::End),
        ))
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    mod array_item {
        use super::*;

        #[rstest]
        #[case(b"]", ArrayItem::End)]
        #[case(b" ]", ArrayItem::End)]
        #[case(b"\n]", ArrayItem::End)]
        #[case(b"\t]", ArrayItem::End)]
        #[case(b"null]", ArrayItem::LastItem(JsonValue::Null))]
        #[case(b" null\n]", ArrayItem::LastItem(JsonValue::Null))]
        #[case(b"\nnull\t]", ArrayItem::LastItem(JsonValue::Null))]
        #[case(b"\tnull ]", ArrayItem::LastItem(JsonValue::Null))]
        #[case(b"null,", ArrayItem::Item(JsonValue::Null))]
        #[case(b" null,", ArrayItem::Item(JsonValue::Null))]
        #[case(b"null\n,", ArrayItem::Item(JsonValue::Null))]
        #[case(b" null\n,", ArrayItem::Item(JsonValue::Null))]
        #[case(b"\nnull\t,", ArrayItem::Item(JsonValue::Null))]
        #[case(b"\tnull ,", ArrayItem::Item(JsonValue::Null))]
        fn valids(
            #[case] input: &[u8],
            #[case] expected: ArrayItem,
        ) {
            println!("input: {:?}", input);
            assert_eq!(
                array_item(input),
                Ok((&b""[..], expected))
            );
        }
    }
}
