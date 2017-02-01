# perftest

These tests are copied and adapted from [rust-protobuf](https://github.com/stepancheg/rust-protobuf/tree/master/src/perftest)

## Setup

You can either run the tests as-is or manually regenerate the rust files following rust-protobuf guidelines or using pb-rs for quick-protobuf.

## Results

quick-protobuf is generally faster than rust-protobuf.

```
         labels   rust-protobuf  quick-protobuf
                        ns/iter         ns/iter        %

test1
          write              60              23     61.7
           read             124              28     77.4
    read no vec             106              20     81.1
     read reuse             106                       NA

test_repeated_bool
          write              60              31     48.3
           read             161              43     73.3
    read no vec             134              34     74.6
     read reuse             120                       NA

test_repeated_packed_int32
          write              88              52     40.9
           read             216             108     50.0
    read no vec             188              84     55.3
     read reuse             152                       NA

test_repeated_messages
          write             173             152     12.1
           read            1092             811     25.7
    read no vec             440             283     35.7
     read reuse             292                       NA

test_optional_messages
          write             179             159     11.2
           read             498             275     44.8
    read no vec             364             208     42.9
     read reuse             283                       NA

test_strings
          write             111              60     45.9
           read             319             121     62.1
    read no vec             206              82     60.2
     read reuse             156                       NA

test_small_bytearrays
          write             564             484     14.2
           read             498              27     94.6
    read no vec             212              18     91.5
     read reuse             121                       NA

test_large_bytearrays
          write           54619           76974    -40.9
           read           15462             152     99.0
    read no vec            6677              26     99.6
     read reuse            6195                       NA
```
