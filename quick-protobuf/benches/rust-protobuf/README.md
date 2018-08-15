# perftest

These tests are copied and adapted from [rust-protobuf](https://github.com/stepancheg/rust-protobuf/tree/master/src/perftest)

## Setup

You can either run the tests as-is or manually regenerate the rust files following rust-protobuf guidelines or using pb-rs for quick-protobuf.

## Results

quick-protobuf is generally faster than rust-protobuf.

```
         labels   rust-protobuf  quick-protobuf           prost      quick/rust      prost/rust
                        ns/iter         ns/iter         ns/iter               %               %

test1
          write              70              21              21            70.0            70.0
           read              75              30              57            60.0            24.0
    read no vec              43              20              46            53.5            -7.0
     read reuse              38                              NA              NA

test_repeated_bool
          write              75              33              29            56.0            61.3
           read             169             113             149            33.1            11.8
    read no vec             118              79             131            33.1           -11.0
     read reuse              60                              NA              NA

test_repeated_packed_int32
          write             132              79              44            40.2            66.7
           read             210             184             217            12.4            -3.3
    read no vec             189             160             212            15.3           -12.2
     read reuse              70                              NA              NA

test_repeated_messages
          write             214             175             237            18.2           -10.7
           read             669             560             529            16.3            20.9
    read no vec             395             385             354             2.5            10.4
     read reuse             178                              NA              NA

test_optional_messages
          write             212             153             104            27.8            50.9
           read             476             329             318            30.9            33.2
    read no vec             305             243             266            20.3            12.8
     read reuse             158                              NA              NA

test_strings
          write             177              60              61            66.1            65.5
           read             314             184             319            41.4            -1.6
    read no vec             227              86             179            62.1            21.1
     read reuse             137                              NA              NA

test_small_bytearrays
          write             503             455             321             9.5            36.2
           read             684             103             617            84.9             9.8
    read no vec             190              33             158            82.6            16.8
     read reuse             100                              NA              NA

test_large_bytearrays
          write           83282           95547           63232           -14.7            24.1
           read           36913            8503           67487            77.0           -82.8
    read no vec            9047              51           18109            99.4          -100.2
     read reuse            6694                              NA              NA
```
