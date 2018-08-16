# perftest

The purpose of this package is to compare the performances of
- protobuf
- quick-protobuf
- prost

These tests are copied and adapted from [rust-protobuf](https://github.com/stepancheg/rust-protobuf/tree/master/src/perftest)

## Setup

Prost requires google `protoc` to be installed and available in your PATH.
For the 3 projects, all files are generated at build time.

## Results

```
         labels   rust-protobuf  quick-protobuf           prost      quick/rust      prost/rust
                        ns/iter         ns/iter         ns/iter               %               %

test1
          write              72              20              21            72.2            70.8
           read              71              31              53            56.3            25.4
    read no vec              44              20              46            54.5            -4.5
     read reuse              42                              NA              NA

test_repeated_bool
          write              80              33              28            58.8            65.0
           read             167             114             147            31.7            12.0
    read no vec             124              79             123            36.3             0.8
     read reuse              62                              NA              NA

test_repeated_packed_int32
          write             135              69              46            48.9            65.9
           read             241             186             216            22.8            10.4
    read no vec             182             152             180            16.5             1.1
     read reuse              71                              NA              NA

test_repeated_messages
          write             213             178             227            16.4            -6.6
           read             665             550             519            17.3            22.0
    read no vec             384             380             353             1.0             8.1
     read reuse             169                              NA              NA

test_optional_messages
          write             223             157             104            29.6            53.4
           read             515             316             313            38.6            39.2
    read no vec             354             241             249            31.9            29.7
     read reuse             191                              NA              NA

test_strings
          write             146              63              59            56.8            59.6
           read             345             185             247            46.4            28.4
    read no vec             234              87             178            62.8            23.9
     read reuse             124                              NA              NA

test_small_bytearrays
          write             479             463             318             3.3            33.6
           read             747              99             729            86.7             2.4
    read no vec             182              31             259            83.0           -42.3
     read reuse             113                              NA              NA

test_large_bytearrays
          write          109299           96753           89523            11.5            18.1
           read           47513            8598           66959            81.9           -40.9
    read no vec           11245              56            9641            99.5            14.3
     read reuse            8834                              NA              NA
```
