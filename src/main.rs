use msgpacker::prelude::*;
use rmp_serde::Serializer;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};

#[derive(MsgPacker, Debug, PartialEq, Deserialize, Serialize)]
pub struct Payload {
    uu08: u8,
    uu16: u16,
    uu32: u32,
    uu64: u64,
    ii08: i8,
    ii16: i16,
    ii32: i32,
    ii64: i64,
    vu08: Vec<u8>,
    vu16: Vec<u16>,
    vu32: Vec<u32>,
    vu64: Vec<u64>,
    vi08: Vec<i8>,
    vi16: Vec<i16>,
    vi32: Vec<i32>,
    vi64: Vec<i64>,
    vu08empty: Vec<u8>,
    ob: Option<u8>,
    obnone: Option<u8>,
    obvu8: Option<Vec<u8>>,
    obvi64: Option<Vec<i64>>,
    obvuu8struct: Option<Vec<Uu8Struct>>,
    #[msgpacker(binary)]
    #[serde(with = "serde_bytes")]
    bb: Vec<u8>,
    uu8struct: Uu8Struct,
    uu8tuplestruct: Uu8TupleStruct,
    unitstruct: UnitStruct,
    thinger: [u8; 5],
}

#[derive(MsgPacker, Debug, PartialEq, Deserialize, Serialize)]
pub struct Uu8Struct {
    first: u8,
}

#[derive(MsgPacker, Debug, PartialEq, Deserialize, Serialize)]
pub struct Uu8TupleStruct(u8);

#[derive(MsgPacker, Debug, PartialEq, Deserialize, Serialize)]
pub struct UnitStruct;

fn main() {
    let mut buf = vec![];
    let out = Payload {
        uu08: 0,
        uu16: 1,
        uu32: 2,
        uu64: 3,
        ii08: 4,
        ii16: 5,
        ii32: 6,
        ii64: 7,
        vu08: vec![8],
        vu16: vec![9],
        vu32: vec![10],
        vu64: vec![11],
        vi08: vec![12],
        vi16: vec![13],
        vi32: vec![14],
        vi64: vec![15],
        vu08empty: vec![],
        ob: Some(1),
        obnone: None,
        obvu8: Some(vec![2, 3]),
        obvi64: Some(vec![4, 5]),
        obvuu8struct: Some(vec![Uu8Struct { first: 6 }]),
        bb: vec![16, 17, 18, 19],
        uu8struct: Uu8Struct { first: 20 },
        uu8tuplestruct: Uu8TupleStruct(21),
        unitstruct: UnitStruct,
        thinger: [1, 2, 3, 4, 5],
    };

    out.pack(&mut buf);
    // out.serialize(&mut Serializer::new(&mut buf)).unwrap();

    // let stdout = io::stdout();
    // let mut handle = stdout.lock();

    // handle.write_all(&buf).unwrap();
    // handle.flush().unwrap();

    let (_, p) = Payload::unpack(&buf).unwrap();
    assert_eq!(out, p);
}
