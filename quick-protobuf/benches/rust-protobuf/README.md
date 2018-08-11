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
          write              49              15              16            69.4            67.3
           read              56              20              33            64.3            41.1
    read no vec              43              16              30            62.8            30.2
     read reuse              46                              NA              NA

test_repeated_bool
          write              53              24              21            54.7            60.4
           read             100              37              75            63.0            25.0
    read no vec              85              26              60            69.4            29.4
     read reuse              69                              NA              NA

test_repeated_packed_int32
          write              69              42              35            39.1            49.3
           read             122              69              87            43.4            28.7
    read no vec              98              68              77            30.6            21.4
     read reuse              70                              NA              NA

test_repeated_messages
          write             184             139             115            24.5            37.5
           read             473             520             580            -9.9           -22.6
    read no vec             330             265             275            19.7            16.7
     read reuse             185                              NA              NA

test_optional_messages
          write             135             112             149            17.0           -10.4
           read             334             192             229            42.5            31.4
    read no vec             252             149             180            40.9            28.6
     read reuse             168                              NA              NA

test_strings
          write              72              33              39            54.2            45.8
           read             210              78             152            62.9            27.6
    read no vec             150              43             117            71.3            22.0
     read reuse             106                              NA              NA

test_small_bytearrays
          write             289             267             182             7.6            37.0
           read             315              25             247            92.1            21.6
    read no vec             107              13             143            87.9           -33.6
     read reuse              82                              NA              NA

test_large_bytearrays
          write           39585           48276           21131           -22.0            46.6
           read           15651             116           22947            99.3           -46.6
    read no vec            6844              27           11288            99.6           -64.9
     read reuse            5439                              NA              NA
```
