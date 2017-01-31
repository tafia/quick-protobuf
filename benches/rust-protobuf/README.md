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
          write              51              19     62.7
           read             108              35     67.6
    read no vec              88              30     65.9
     read reuse              88                       NA

test_repeated_bool
          write              58              32     44.8
           read             161              71     55.9
    read no vec             137              63     54.0
     read reuse             123                       NA

test_repeated_packed_int32
          write              88              56     36.4
           read             229             126     45.0
    read no vec             194              98     49.5
     read reuse             158                       NA

test_repeated_messages
          write             180             164      8.9
           read            1112             742     33.3
    read no vec             417             291     30.2
     read reuse             296                       NA

test_optional_messages
          write             189             151     20.1
           read             504             279     44.6
    read no vec             377             219     41.9
     read reuse             293                       NA

test_strings
          write              99              62     37.4
           read             335             200     40.3
    read no vec             204             135     33.8
     read reuse             151                       NA

test_small_bytearrays
          write             788             602     23.6
           read             543             454     16.4
    read no vec             157             120     23.6
     read reuse             120                       NA

test_large_bytearrays
          write           93661           86088      8.1
           read           38295           30259     21.0
    read no vec            7324            6958      5.0
     read reuse            6102                       NA
```
