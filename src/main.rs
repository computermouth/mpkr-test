use msgpacker::prelude::*;
use std::io::{self, Write};
use serde::{Deserialize, Serialize};
use rmp_serde::Serializer;

#[derive(MsgPacker, Debug, PartialEq)]
#[derive(Deserialize, Serialize)]
pub struct Payload {
  uu08: u8,
  uu16: u16,
  uu32: u32,
  uu64: u64,
  ii08: i8,
  ii16: i16,
  ii32: i32,
  ii64: i64,
  vu08: Vec<u8 >,
  vu16: Vec<u16>,
  vu32: Vec<u32>,
  vu64: Vec<u64>,
  vi08: Vec<i8 >,
  vi16: Vec<i16>,
  vi32: Vec<i32>,
  vi64: Vec<i64>,
  #[msgpacker(binary)]
  #[serde(with = "serde_bytes")]
  bb: Vec<u8>,
  uu8struct: Uu8Struct,
  uu8tuplestruct: Uu8TupleStruct,
  unitstruct: UnitStruct,
  vempty: Vec<u8>,
}

#[derive(MsgPacker)]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Uu8Struct {
  first: u8,
}

#[derive(MsgPacker)]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Uu8TupleStruct(u8);

#[derive(MsgPacker)]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
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
      bb: vec![16,17,18,19],
      uu8struct: Uu8Struct { first: 20 },
      uu8tuplestruct: Uu8TupleStruct(21),
      unitstruct: UnitStruct,
      vempty: vec![],
    };

    out.pack(&mut buf);
    // out.serialize(&mut Serializer::new(&mut buf)).unwrap();

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    handle.write_all(&buf).unwrap();
    handle.flush().unwrap();

    let (_, p) = Payload::unpack(&buf).unwrap();

    assert_eq!(out, p);

}