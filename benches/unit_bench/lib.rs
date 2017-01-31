#![feature(test)]

extern crate quick_protobuf;
extern crate test;

mod perftest_data;

use test::{Bencher, black_box};
use perftest_data::*;
use std::cmp::min;

use quick_protobuf::{Reader, Writer};
use quick_protobuf::message::{MessageRead, MessageWrite};

#[bench]
fn read_file(b: &mut Bencher) {
    b.iter(|| {
        let _ = black_box(::perftest_data::PerftestData::from_file("../rust-protobuf/perftest_data.pbbin").unwrap());
    })
}

macro_rules! perfbench {
    ($gen: ident, $write:ident, $read:ident) => {
#[bench]
fn $write(b: &mut Bencher) {
    let v = $gen();
    b.iter(|| {
        let mut buf = black_box(Vec::new());
        let mut w = Writer::new(&mut buf);
        for i in &v {
            i.write_message(&mut w).unwrap();
        }
    })
}

#[bench]
fn $read(b: &mut Bencher) {
    let v = $gen();
    let mut buf = Vec::new();
    {
        let mut w = Writer::new(&mut buf);
        for i in &v {
            i.write_message(&mut w).unwrap();
        }
    }
    let len = buf.len();
    b.iter(|| {
        let mut buf = &*buf;
        let mut r = Reader::from_reader(&mut buf, len);
        while !r.is_eof() {
            let _ = black_box(Test1::from_reader(&mut r).unwrap());
        }
    })
}
    }
}

fn generate_test1() -> Vec<Test1> {
    (0..500).map(|i| Test1 { value: Some(i) })
        .chain((0..100).map(|_| Test1 { value: None }))
        .collect()
}

perfbench!(generate_test1, write_test1, read_test1);

fn generate_repeated_bool() -> Vec<TestRepeatedBool> {
    (1..10).map(|j| TestRepeatedBool { values: (0..100).map(|i| i % j == 0).collect() })
        .collect()
}

perfbench!(generate_repeated_bool, write_repeated_bool, read_repeated_bool);

fn generate_repeated_packed_int32() -> Vec<TestRepeatedPackedInt32> {
    (1..400).map(|j| TestRepeatedPackedInt32 { values: (0..1000).map(|i| i * j).collect() })
        .collect()
}

perfbench!(generate_repeated_packed_int32, write_repeated_packed_int32, read_repeated_packed_int32);

fn generate_repeated_messages() -> Vec<TestRepeatedMessages> {
    let mut messages = Vec::new();
    messages.push(TestRepeatedMessages {
        messages1: vec![],
        messages2: vec![],
        messages3: vec![],
    });

    for _ in 0..100 {
        let i1 = min(messages.len() % 3, messages.len() - 1);
        let i2 = min(messages.len() % 6, messages.len() - 1);
        let i3 = min(messages.len() % 9, messages.len() - 1);
        let m1 = messages[i1].clone();
        let m2 = messages[i2].clone();
        let m3 = messages[i3].clone();
        messages.push(TestRepeatedMessages {
            messages1: vec![m1.clone()], 
            messages2: vec![m1.clone(), m2.clone()],
            messages3: vec![m1.clone(), m2.clone(), m3.clone()],
        });
    }
    messages
}

perfbench!(generate_repeated_messages, write_repeated_messages, read_repeated_messages);

fn generate_optional_messages() -> Vec<TestOptionalMessages> {
    let mut messages = Vec::new();
    messages.push(TestOptionalMessages {
        message1: None,
        message2: None,
        message3: None,
    });

    for _ in 0..60 {
        let i1 = min(messages.len() % 3, messages.len() - 1);
        let i2 = min(messages.len() % 6, messages.len() - 1);
        let i3 = min(messages.len() % 9, messages.len() - 1);
        let m1 = messages[i1].clone();
        let m2 = messages[i2].clone();
        let m3 = messages[i3].clone();
        messages.push(TestOptionalMessages {
            message1: Some(Box::new(m1.clone())),
            message2: Some(Box::new(m2.clone())),
            message3: Some(Box::new(m3.clone())),
        });
    }
    messages
}

perfbench!(generate_optional_messages, write_optional_messages, read_optional_messages);

fn generate_strings() -> Vec<TestStrings> {
    let mut s = "hello world from quick-protobuf!!!".split('_').cycle().map(|s| s.to_string());
    (1..100).map(|_| TestStrings { 
        s1: s.by_ref().next(),
        s2: s.by_ref().next(),
        s3: s.by_ref().next(),
    }).collect()
}

perfbench!(generate_strings, write_strings, read_strings);

fn generate_small_bytes() -> Vec<TestBytes> {
    let mut s = "hello world from quick-protobuf!!!".split('_').cycle()
        .map(|s| s.as_bytes().to_vec());
    (1..800).map(|_| TestBytes { b1: s.by_ref().next() })
        .collect()
}

perfbench!(generate_small_bytes, write_small_bytes, read_small_bytes);

fn generate_large_bytes() -> Vec<TestBytes> {
    let mut s = "hello world from quick-protobuf!!!".split('_').cycle().map(|s| s.as_bytes());
    (1..300).map(|_| TestBytes { 
        b1: Some(s.by_ref().take(500).fold(Vec::new(), |mut cur, nxt| {
                cur.extend_from_slice(nxt);
                cur 
            }))
    }).collect()
}

perfbench!(generate_large_bytes, write_large_bytes, read_large_bytes);

fn generate_all() -> Vec<PerftestData> {
    vec![PerftestData {
        test1: generate_test1(),
        test_repeated_bool: generate_repeated_bool(),
        test_repeated_messages: generate_repeated_messages(),
        test_optional_messages: generate_optional_messages(),
        test_strings: generate_strings(),
        test_repeated_packed_int32: generate_repeated_packed_int32(),
        test_small_bytearrays: generate_small_bytes(),
        test_large_bytearrays: generate_large_bytes(),
    }]
}

perfbench!(generate_all, write_all, read_all);
