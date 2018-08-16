extern crate bytes;
extern crate prost;
extern crate protobuf;
extern crate quick_protobuf;
extern crate rand;
extern crate time;
#[macro_use]
extern crate prost_derive;

use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::default::Default;
use std::fmt::Debug;
use std::fs::File;
use std::io::Read;

use bytes::{Buf, IntoBuf};

use perftest_data::PerftestData;
use perftest_data_quick::PerftestData as QuickPerftestData;

use prost::Message as ProstMessage;
use protobuf::Message;
use quick_protobuf::{BytesReader, MessageRead, MessageWrite, Reader, Writer};

mod perftest_data;
mod perftest_data_prost;
mod perftest_data_quick;

const SEED: [u8; 16] = [
    10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150, 160,
];

fn measure<R: Debug + PartialEq, F: FnMut() -> R>(iter: u64, mut f: F, check: Option<&R>) -> u64 {
    let start = time::precise_time_ns();
    let r = f();
    check.map(|c| assert_eq!(c, &r));
    (time::precise_time_ns() - start) / iter
}

struct TestRunner {
    data_size: u32,
    selected: Option<String>,
    any_matched: bool,
}

impl TestRunner {
    fn run_test<M: Clone + Message + Default + PartialEq>(&self, data: &[M]) -> [u64; 4] {
        let mut a = [0; 4];

        let mut rng = SmallRng::from_seed(SEED);
        let mut random_data = Vec::new();

        let mut total_size = 0;
        while total_size < self.data_size {
            let item = data[rng.gen_range(0, data.len())].clone();
            total_size += item.compute_size();
            random_data.push(item);
        }

        let mut buf = Vec::new();
        a[0] = measure(
            random_data.len() as u64,
            || {
                for m in &random_data {
                    m.write_length_delimited_to_vec(&mut buf).unwrap();
                }
            },
            None,
        );

        a[1] = measure(
            random_data.len() as u64,
            || {
                let mut r = Vec::new();
                let mut coded_input_stream = protobuf::CodedInputStream::from_bytes(&buf);
                while !coded_input_stream.eof().unwrap() {
                    r.push(
                        protobuf::parse_length_delimited_from::<M>(&mut coded_input_stream)
                            .unwrap(),
                    );
                }
                r
            },
            Some(&random_data),
        );

        a[2] = measure(
            random_data.len() as u64,
            || {
                let mut coded_input_stream = protobuf::CodedInputStream::from_bytes(&buf);
                while !coded_input_stream.eof().unwrap() {
                    protobuf::parse_length_delimited_from::<M>(&mut coded_input_stream).unwrap();
                }
            },
            None,
        );

        a[3] = measure(
            random_data.len() as u64,
            || {
                let mut coded_input_stream = protobuf::CodedInputStream::from_bytes(&buf);
                let mut msg: M = Default::default();
                let mut count = 0;
                while !coded_input_stream.eof().unwrap() {
                    msg.clear();
                    coded_input_stream.merge_message(&mut msg).unwrap();
                    count += 1;
                }
                count
            },
            None,
        );

        a
    }

    fn test<M: Message + Clone + Default + PartialEq>(&mut self, data: &[M]) -> [u64; 4] {
        let a = self.run_test(data);
        self.any_matched = true;
        a
    }

    fn quick_run_test<M>(&self, data: &[M]) -> [u64; 4]
    where
        M: for<'a> MessageRead<'a> + MessageWrite + Clone + PartialEq + ::std::fmt::Debug,
    {
        let mut b = [0; 4];

        let mut rng = SmallRng::from_seed(SEED);
        let mut random_data: Vec<M> = Vec::new();

        let mut total_size = 0;
        while total_size < self.data_size {
            let ref item = data[rng.gen_range(0, data.len())];
            random_data.push(item.clone());
            total_size += item.get_size() as u32;
        }

        let mut buf = Vec::new();
        b[0] = measure(
            random_data.len() as u64,
            || {
                let mut w = Writer::new(&mut buf);
                for m in &random_data {
                    w.write_message(m).unwrap();
                }
            },
            None,
        );

        b[1] = measure(
            random_data.len() as u64,
            || {
                let mut r = Vec::<M>::new();
                let mut reader = BytesReader::from_bytes(&buf);
                while !reader.is_eof() {
                    r.push(reader.read_message(&buf).unwrap());
                }
                r
            },
            Some(&random_data),
        );

        b[2] = measure(
            random_data.len() as u64,
            || {
                let mut reader = BytesReader::from_bytes(&buf);
                while !reader.is_eof() {
                    let _: M = reader.read_message(&buf).unwrap();
                }
            },
            None,
        );

        b
    }

    fn quick_test<M>(&mut self, data: &[M]) -> [u64; 4]
    where
        M: for<'a> MessageRead<'a> + MessageWrite + Clone + PartialEq + ::std::fmt::Debug,
    {
        let b = self.quick_run_test(data);
        self.any_matched = true;
        b
    }

    fn quick_run_test_strings(&self, data: &[perftest_data_quick::TestStrings]) -> [u64; 4] {
        let mut b = [0; 4];

        let mut rng = SmallRng::from_seed(SEED);
        let mut random_data = Vec::new();

        let mut total_size = 0;
        while total_size < self.data_size {
            let ref item = data[rng.gen_range(0, data.len())];
            random_data.push(item.clone());
            total_size += item.get_size() as u32;
        }

        let mut buf = Vec::new();
        b[0] = measure(
            random_data.len() as u64,
            || {
                let mut w = Writer::new(&mut buf);
                for m in &random_data {
                    w.write_message(m).unwrap();
                }
            },
            None,
        );

        b[1] = measure(
            random_data.len() as u64,
            || {
                let mut r = Vec::<perftest_data_quick::TestStrings>::new();
                let mut reader = BytesReader::from_bytes(&buf);
                while !reader.is_eof() {
                    r.push(reader.read_message(&buf).unwrap());
                }
                r
            },
            Some(&random_data),
        );

        b[2] = measure(
            random_data.len() as u64,
            || {
                let mut reader = BytesReader::from_bytes(&buf);
                while !reader.is_eof() {
                    let _: perftest_data_quick::TestStrings = reader.read_message(&buf).unwrap();
                }
            },
            None,
        );

        b
    }

    fn quick_run_test_bytes(&self, data: &[perftest_data_quick::TestBytes]) -> [u64; 4] {
        let mut b = [0; 4];

        let mut rng = SmallRng::from_seed(SEED);
        let mut random_data = Vec::new();

        let mut total_size = 0;
        while total_size < self.data_size {
            let ref item = data[rng.gen_range(0, data.len())];
            random_data.push(item.clone());
            total_size += item.get_size() as u32;
        }

        let mut buf = Vec::new();
        b[0] = measure(
            random_data.len() as u64,
            || {
                let mut w = Writer::new(&mut buf);
                for m in &random_data {
                    w.write_message(m).unwrap();
                }
            },
            None,
        );

        b[1] = measure(
            random_data.len() as u64,
            || {
                let mut r = Vec::<perftest_data_quick::TestBytes>::new();
                let mut reader = BytesReader::from_bytes(&buf);
                while !reader.is_eof() {
                    r.push(reader.read_message(&buf).unwrap());
                }
                r
            },
            Some(&random_data),
        );

        b[2] = measure(
            random_data.len() as u64,
            || {
                let mut reader = BytesReader::from_bytes(&buf);
                while !reader.is_eof() {
                    let _: perftest_data_quick::TestBytes = reader.read_message(&buf).unwrap();
                }
            },
            None,
        );

        b
    }

    fn prost_run_test<M: ProstMessage + Clone + Default + PartialEq>(
        &mut self,
        data: &[M],
    ) -> [u64; 4] {
        let mut c = [0; 4];

        let mut rng = SmallRng::from_seed(SEED);
        let mut random_data: Vec<M> = Vec::new();

        let mut total_size = 0;
        while total_size < self.data_size {
            let ref item = data[rng.gen_range(0, data.len())];
            random_data.push(item.clone());
            total_size += item.encoded_len() as u32;
        }

        let mut buf = Vec::new();
        c[0] = measure(
            random_data.len() as u64,
            || {
                for m in &random_data {
                    m.encode_length_delimited(&mut buf).unwrap();
                }
            },
            None,
        );

        let mut tmp_buf = buf.clone().into_buf();
        c[1] = measure(
            random_data.len() as u64,
            move || {
                let mut r = Vec::new();
                while tmp_buf.has_remaining() {
                    let m = M::decode_length_delimited(&mut tmp_buf).unwrap();
                    r.push(m);
                }
                r
            },
            Some(&random_data),
        );

        let mut tmp_buf = buf.clone().into_buf();
        c[2] = measure(
            random_data.len() as u64,
            move || {
                while tmp_buf.has_remaining() {
                    M::decode_length_delimited(&mut tmp_buf).unwrap();
                }
            },
            None,
        );

        c
    }

    fn prost_test<M: ProstMessage + Clone + Default + PartialEq>(
        &mut self,
        data: &[M],
    ) -> [u64; 4] {
        let c = self.prost_run_test(data);
        self.any_matched = true;
        c
    }

    fn check(&self) {
        if !self.any_matched {
            let name = self.selected.as_ref().map(|s| &s[..]).unwrap_or("bug");
            panic!("no tests found with name {}", name);
        }
    }
}

fn print_results(name: &str, a: &[u64], b: &[u64], c: &[u64], print_header: bool) {
    let labels = ["write", "read", "read no vec", "read reuse"];

    if print_header {
        println!(
            "{:>15} {:>15} {:>15} {:>15} {:>15} {:>15}",
            "labels", "rust-protobuf", "quick-protobuf", "prost", "quick/rust", "prost/rust"
        );
        println!(
            "{:>15} {:>15} {:>15} {:>15} {:>15} {:>15}",
            "", "ns/iter", "ns/iter", "ns/iter", "%", "%"
        );
    }
    println!("");
    println!("{}", name);
    for i in 0..3 {
        println!(
            "{:>15} {:>15} {:>15} {:>15} {:>15.1} {:>15.1}",
            labels[i],
            a[i],
            b[i],
            c[i],
            100. - b[i] as f32 / a[i] as f32 * 100.,
            100. - c[i] as f32 / a[i] as f32 * 100.
        );
    }
    let i = 3;
    println!(
        "{:>15} {:>15} {:>15} {:>15} {:>15}",
        labels[i], a[i], "", "NA", "NA"
    );
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() > 3 {
        panic!("usage: {} [data_size] [test]", args[0])
    }
    let data_size = args
        .iter()
        .nth(1)
        .map(|x| x.parse().unwrap())
        .unwrap_or(1000000);
    let selected = args.iter().nth(2).cloned();

    let mut runner = TestRunner {
        selected: selected,
        any_matched: false,
        data_size: data_size,
    };

    let data = {
        let mut is = File::open(&format!(
            "{}/{}",
            env!("CARGO_MANIFEST_DIR"),
            "perftest_data.pbbin"
        )).unwrap();
        let mut data = Vec::new();
        is.read_to_end(&mut data).unwrap();
        data
    };

    let test_data = protobuf::parse_from_reader::<PerftestData>(&mut &*data).unwrap();
    let mut reader = Reader::from_reader(&mut &*data, data.len()).unwrap();
    let test_data_quick = reader.read(QuickPerftestData::from_reader).unwrap();

    let test_data_prost = perftest_data_prost::PerftestData::decode(&data).unwrap();

    let a = runner.test(test_data.get_test1());
    let b = runner.quick_test::<perftest_data_quick::Test1>(&test_data_quick.test1);
    let c = runner.prost_test(&test_data_prost.test1);
    print_results("test1", &a, &b, &c, true);

    let a = runner.test(test_data.get_test_repeated_bool());
    let b = runner
        .quick_test::<perftest_data_quick::TestRepeatedBool>(&test_data_quick.test_repeated_bool);
    let c = runner.prost_test(&test_data_prost.test_repeated_bool);
    print_results("test_repeated_bool", &a, &b, &c, false);

    let a = runner.test(test_data.get_test_repeated_packed_int32());
    let b = runner.quick_test::<perftest_data_quick::TestRepeatedPackedInt32>(
        &test_data_quick.test_repeated_packed_int32,
    );
    let c = runner.prost_test(&test_data_prost.test_repeated_packed_int32);
    print_results("test_repeated_packed_int32", &a, &b, &c, false);

    let a = runner.test(test_data.get_test_repeated_messages());
    let b = runner.quick_test::<perftest_data_quick::TestRepeatedMessages>(
        &test_data_quick.test_repeated_messages,
    );
    let c = runner.prost_test(&test_data_prost.test_repeated_messages);
    print_results("test_repeated_messages", &a, &b, &c, false);

    let a = runner.test(test_data.get_test_optional_messages());
    let b = runner.quick_test::<perftest_data_quick::TestOptionalMessages>(
        &test_data_quick.test_optional_messages,
    );
    let c = runner.prost_test(&test_data_prost.test_optional_messages);
    print_results("test_optional_messages", &a, &b, &c, false);

    let a = runner.test(test_data.get_test_strings());
    let b = runner.quick_run_test_strings(&test_data_quick.test_strings);
    let c = runner.prost_test(&test_data_prost.test_strings);
    print_results("test_strings", &a, &b, &c, false);

    let a = runner.test(test_data.get_test_small_bytearrays());
    let b = runner.quick_run_test_bytes(&test_data_quick.test_small_bytearrays);
    let c = runner.prost_test(&test_data_prost.test_small_bytearrays);
    print_results("test_small_bytearrays", &a, &b, &c, false);

    let a = runner.test(test_data.get_test_large_bytearrays());
    let b = runner.quick_run_test_bytes(&test_data_quick.test_large_bytearrays);
    let c = runner.prost_test(&test_data_prost.test_large_bytearrays);
    print_results("test_large_bytearrays", &a, &b, &c, false);

    runner.check();
}
