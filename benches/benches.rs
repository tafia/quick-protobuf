#![feature(test)]

extern crate quick_protobuf;
extern crate test;

use test::{Bencher, black_box};

use quick_protobuf::{Reader, Writer};

#[bench]
fn read_varint(b: &mut Bencher) {
    let mut buf = Vec::new();
    {
        let mut writer = Writer::new(&mut buf);
        for i in 0..10_000 {
            writer.write_int32(i).unwrap();
        }
    }
    b.iter(|| {
        let len = buf.len();
        let mut buf = &*buf;
        let mut reader = Reader::from_reader(&mut buf, len);
        while !reader.is_eof() {
            let _ = black_box(reader.read_varint().unwrap());
        }
    })
}

