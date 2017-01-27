# perftest

These tests are copied and adapted from [rust-protobuf](https://github.com/stepancheg/rust-protobuf/tree/master/src/perftest)

## Setup

You can either run the tests as-is or manually regenerate the rust files following rust-protobuf guidelines or using pb-rs for quick-xml.

## Results

quick-protobuf is generally faster (up to 80% faster) than rust-protobuf.
```
         labels   rust-protobuf  quick-protobuf         
                        ns/iter         ns/iter        %

test1
          write             160              42     73.8
           read             214              90     57.9
    read no vec             173              82     52.6
     read reuse             173                       NA

test_repeated_bool
          write             174              67     61.5
           read             379             225     40.6
    read no vec             287             161     43.9
     read reuse             228                       NA

test_repeated_packed_int32
          write             270             138     48.9
           read             505             340     32.7
    read no vec             435             266     38.9
     read reuse             322                       NA

test_repeated_messages
          write             523             461     11.9
           read            2215            1617     27.0
    read no vec            1060             829     21.8
     read reuse             630                       NA

test_optional_messages
          write             489             456      6.7
           read            1118             682     39.0
    read no vec             939             598     36.3
     read reuse             622                       NA

test_strings
          write             342             182     46.8
           read             901             536     40.5
    read no vec             570             394     30.9
     read reuse             424                       NA

test_small_bytearrays
          write            2279            1647     27.7
           read            1636            1520      7.1
    read no vec             679             605     10.9
     read reuse             526                       NA

test_large_bytearrays
          write          252108          197931     21.5
           read          112269           80121     28.6
    read no vec           38930           24839     36.2
     read reuse           20292                       NA
```
