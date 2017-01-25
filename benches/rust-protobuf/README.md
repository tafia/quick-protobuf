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
          write              86              12     86.0
           read             111              31     72.1
    read no vec              97              27     72.2
     read reuse              91                       NA

test_repeated_bool
          write              87              20     77.0
           read             177              70     60.5
    read no vec             152              62     59.2
     read reuse             125                       NA

test_repeated_packed_int32
          write             128              42     67.2
           read             222             112     49.5
    read no vec             202              95     53.0
     read reuse             151                       NA

test_repeated_messages
          write             229             152     33.6
           read             822             711     13.5
    read no vec             484             284     41.3
     read reuse             343                       NA

test_optional_messages
          write             229             148     35.4
           read             581             290     50.1
    read no vec             454             228     49.8
     read reuse             346                       NA

test_strings
          write             154              38     75.3
           read             332             195     41.3
    read no vec             236             137     41.9
     read reuse             188                       NA

test_small_bytearrays
          write             807             596     26.1
           read             610             423     30.7
    read no vec             185             119     35.7
     read reuse             158                       NA

test_large_bytearrays
          write           85872           91591     -6.7
           read           33733           27149     19.5
    read no vec            6915            6762      2.2
     read reuse            6184                       NA
```
