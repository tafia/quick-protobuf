extern crate quick_protobuf;
extern crate protobuf;
extern crate rand;
extern crate time;
extern crate prost;
#[macro_use]
extern crate prost_derive;

use std::default::Default;
use std::fs::File;
use std::path::Path;

use rand::Rng;
use rand::StdRng;
use rand::SeedableRng;

use protobuf::Message;
use protobuf::MessageStatic;
use perftest_data::PerftestData;

use quick_protobuf::{BytesReader, Reader, Writer, MessageWrite};
use quick_protobuf::Result as QuickResult;
use perftest_data_quick::mod_perftest_data_quick;
use perftest_data_quick::mod_perftest_data_quick::PerftestData as QuickPerftestData;

mod perftest_data;
mod perftest_data_quick;
mod perftest_data_prost;

fn measure<R, F: FnMut() -> R>(iter: u64, mut f: F) -> (u64, R) {
    let start = time::precise_time_ns();
    let r = f();
    ((time::precise_time_ns() - start) / iter, r)
}

struct TestRunner {
    data_size: u32,
    selected: Option<String>,
    any_matched: bool,
}

impl TestRunner {
    fn run_test<M : Message + MessageStatic>(&self, data: &[M]) -> [u64; 4] {

        let mut a = [0; 4];

        let mut rng: StdRng = SeedableRng::from_seed(&[10, 20, 30, 40][..]);
        let mut random_data: Vec<M> = Vec::new();

        let mut total_size = 0;
        while total_size < self.data_size {
            let ref item = data[rng.gen_range(0, data.len())];
            random_data.push(item.clone());
            total_size += item.compute_size();
        }

        let mut buf = Vec::new();
        a[0] = measure(random_data.len() as u64, || {
            for m in &random_data {
                m.write_length_delimited_to_vec(&mut buf).unwrap();
            }
        }).0;

        let (ns, read_data) = measure(random_data.len() as u64, || {
            let mut r = Vec::new();
            let mut coded_input_stream = protobuf::CodedInputStream::from_bytes(&buf);
            while !coded_input_stream.eof().unwrap() {
                r.push(protobuf::parse_length_delimited_from::<M>(&mut coded_input_stream).unwrap());
            }
            r
        });
        a[1] = ns;

        assert_eq!(random_data, &*read_data);

        a[2] = measure(random_data.len() as u64, || {
            let mut coded_input_stream = protobuf::CodedInputStream::from_bytes(&buf);
            while !coded_input_stream.eof().unwrap() {
                protobuf::parse_length_delimited_from::<M>(&mut coded_input_stream).unwrap();
            }
        }).0;

        let (ns, merged) = measure(random_data.len() as u64, || {
            let mut coded_input_stream = protobuf::CodedInputStream::from_bytes(&buf);
            let mut msg: M = Default::default();
            let mut count = 0;
            while !coded_input_stream.eof().unwrap() {
                msg.clear();
                coded_input_stream.merge_message(&mut msg).unwrap();
                count += 1;
            }
            count
        });

        a[3] = ns;

        assert_eq!(random_data.len(), merged);

        a
    }

    fn test<M : Message + MessageStatic>(&mut self, data: &[M]) -> [u64; 4] {
        let a = self.run_test(data);
        self.any_matched = true;
        a
    }

    fn quick_run_test<M, F>(&self, data: &[M], read: F) -> [u64; 4]
        where M: MessageWrite + Clone + PartialEq + ::std::fmt::Debug,
              F: Copy + FnMut(&mut BytesReader, &[u8]) -> QuickResult<M>,
    {

        let mut b = [0; 4];

        let mut rng: StdRng = SeedableRng::from_seed(&[10, 20, 30, 40][..]);
        let mut random_data: Vec<M> = Vec::new();

        let mut total_size = 0;
        while total_size < self.data_size {
            let ref item = data[rng.gen_range(0, data.len())];
            random_data.push(item.clone());
            total_size += item.get_size() as u32;
        }

        let mut buf = Vec::new();
        b[0] = measure(random_data.len() as u64, || {
            let mut w = Writer::new(&mut buf);
            for m in &random_data {
                w.write_message(m).unwrap();
            }
        }).0;

        let (ns, read_data) = measure(random_data.len() as u64, || {
            let mut r = Vec::new();
            let mut reader = BytesReader::from_bytes(&buf);
            while !reader.is_eof() {
                r.push(reader.read_message(&buf, read).unwrap());
            }
            r
        });
        b[1] = ns;

        assert_eq!(random_data, &*read_data);

        b[2] = measure(random_data.len() as u64, || {
            let mut reader = BytesReader::from_bytes(&buf);
            while !reader.is_eof() {
                let _ = reader.read_message(&buf, read).unwrap();
            }
        }).0;

        b
    }

    fn quick_test<M, F>(&mut self, data: &[M], read: F) -> [u64; 4]
        where M: MessageWrite + Clone + PartialEq + ::std::fmt::Debug,
              F: Copy + FnMut(&mut BytesReader, &[u8]) -> QuickResult<M>,
    {
        let b = self.quick_run_test(data, read);
        self.any_matched = true;
        b
    }

    fn quick_run_test_strings(&self, data: &[mod_perftest_data_quick::TestStrings]) -> [u64; 4]
    {

        let mut b = [0; 4];

        let mut rng: StdRng = SeedableRng::from_seed(&[10, 20, 30, 40][..]);
        let mut random_data = Vec::new();

        let mut total_size = 0;
        while total_size < self.data_size {
            let ref item = data[rng.gen_range(0, data.len())];
            random_data.push(item.clone());
            total_size += item.get_size() as u32;
        }

        let mut buf = Vec::new();
        b[0] = measure(random_data.len() as u64, || {
            let mut w = Writer::new(&mut buf);
            for m in &random_data {
                w.write_message(m).unwrap();
            }
        }).0;

        let (ns, read_data) = measure(random_data.len() as u64, || {
            let mut r = Vec::new();
            let mut reader = BytesReader::from_bytes(&buf);
            while !reader.is_eof() {
                r.push(reader.read_message(&buf, mod_perftest_data_quick::TestStrings::from_reader).unwrap());
            }
            r
        });
        b[1] = ns;

        assert_eq!(random_data, &*read_data);

        b[2] = measure(random_data.len() as u64, || {
            let mut reader = BytesReader::from_bytes(&buf);
            while !reader.is_eof() {
                let _ = reader.read_message(&buf, mod_perftest_data_quick::TestStrings::from_reader).unwrap();
            }
        }).0;

        b
    }

    fn quick_run_test_bytes(&self, data: &[mod_perftest_data_quick::TestBytes]) -> [u64; 4]
    {

        let mut b = [0; 4];

        let mut rng: StdRng = SeedableRng::from_seed(&[10, 20, 30, 40][..]);
        let mut random_data = Vec::new();

        let mut total_size = 0;
        while total_size < self.data_size {
            let ref item = data[rng.gen_range(0, data.len())];
            random_data.push(item.clone());
            total_size += item.get_size() as u32;
        }

        let mut buf = Vec::new();
        b[0] = measure(random_data.len() as u64, || {
            let mut w = Writer::new(&mut buf);
            for m in &random_data {
                w.write_message(m).unwrap();
            }
        }).0;

        let (ns, read_data) = measure(random_data.len() as u64, || {
            let mut r = Vec::new();
            let mut reader = BytesReader::from_bytes(&buf);
            while !reader.is_eof() {
                r.push(reader.read_message(&buf, mod_perftest_data_quick::TestBytes::from_reader).unwrap());
            }
            r
        });
        b[1] = ns;

        assert_eq!(random_data, &*read_data);

        b[2] = measure(random_data.len() as u64, || {
            let mut reader = BytesReader::from_bytes(&buf);
            while !reader.is_eof() {
                let _ = reader.read_message(&buf, mod_perftest_data_quick::TestBytes::from_reader).unwrap();
            }
        }).0;

        b
    }

    fn check(&self) {
        if !self.any_matched {
            let name = self.selected.as_ref().map(|s| &s[..]).unwrap_or("bug");
            panic!("no tests found with name {}", name);
        }
    }
}

fn print_results(name: &str, a: &[u64], b: &[u64], print_header: bool) {
    let labels = ["write", "read", "read no vec", "read reuse"];

    if print_header {
        println!("{:>15} {:>15} {:>15} {:>8}", "labels", "rust-protobuf", "quick-protobuf", "");
        println!("{:>15} {:>15} {:>15} {:>8}", "", "ns/iter", "ns/iter", "%");
    }
    println!("");
    println!("{}", name);
    for i in 0..3 {
        println!("{:>15} {:>15} {:>15} {:>8.1}", labels[i], a[i], b[i], 100. - b[i] as f32 / a[i] as f32 * 100.);
    }
    let i = 3;
    println!("{:>15} {:>15} {:>15} {:>8}", labels[i], a[i], "", "NA");
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() > 3 { panic!("usage: {} [data_size] [test]", args[0]) }
    let data_size = args.iter().nth(1).map(|x| x.parse().unwrap()).unwrap_or(1000000);
    let selected = args.iter().nth(2).cloned();

    let mut runner = TestRunner {
        selected: selected,
        any_matched: false,
        data_size: data_size,
    };

    let mut is = File::open(&Path::new("perftest_data.pbbin")).unwrap();
    let test_data = protobuf::parse_from_reader::<PerftestData>(&mut is).unwrap();

    let mut reader = Reader::from_file("perftest_data.pbbin").unwrap();
    let test_data_quick = reader.read(QuickPerftestData::from_reader).unwrap();

    let a = runner.test(test_data.get_test1());
    let b = runner.quick_test(&test_data_quick.test1, mod_perftest_data_quick::Test1::from_reader);
    print_results("test1", &a, &b, true);

    let a = runner.test(test_data.get_test_repeated_bool());
    let b = runner.quick_test(&test_data_quick.test_repeated_bool, mod_perftest_data_quick::TestRepeatedBool::from_reader);
    print_results("test_repeated_bool", &a, &b, false);

    let a = runner.test(test_data.get_test_repeated_packed_int32());
    let b = runner.quick_test(&test_data_quick.test_repeated_packed_int32, mod_perftest_data_quick::TestRepeatedPackedInt32::from_reader);
    print_results("test_repeated_packed_int32", &a, &b, false);

    let a = runner.test(test_data.get_test_repeated_messages());
    let b = runner.quick_test(&test_data_quick.test_repeated_messages, mod_perftest_data_quick::TestRepeatedMessages::from_reader);
    print_results("test_repeated_messages", &a, &b, false);

    let a = runner.test(test_data.get_test_optional_messages());
    let b = runner.quick_test(&test_data_quick.test_optional_messages, mod_perftest_data_quick::TestOptionalMessages::from_reader);
    print_results("test_optional_messages", &a, &b, false);

    let a = runner.test(test_data.get_test_strings());
    let b = runner.quick_run_test_strings(&test_data_quick.test_strings);
    print_results("test_strings", &a, &b, false);

    let a = runner.test(test_data.get_test_small_bytearrays());
    let b = runner.quick_run_test_bytes(&test_data_quick.test_small_bytearrays);
    print_results("test_small_bytearrays", &a, &b, false);

    let a = runner.test(test_data.get_test_large_bytearrays());
    let b = runner.quick_run_test_bytes(&test_data_quick.test_large_bytearrays);
    print_results("test_large_bytearrays", &a, &b, false);

    runner.check();
}
