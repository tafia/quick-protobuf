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
          write              44              13              18            70.5            59.1
           read              49              18              44            63.3            10.2
    read no vec              39              15              41            61.5            -5.1
     read reuse              39                              NA              NA

test_repeated_bool
          write              48              20              23            58.3            52.1
           read             101              33              86            67.3            14.9
    read no vec              77              24              74            68.8             3.9
     read reuse              65                              NA              NA

test_repeated_packed_int32
          write              70              40              40            42.9            42.9
           read             121              68             116            43.8             4.1
    read no vec             101              60             108            40.6            -6.9
     read reuse              66                              NA              NA

test_repeated_messages
          write             129              94             126            27.1             2.3
           read             400             420             537            -5.0           -34.2
    read no vec             263             202             264            23.2            -0.4
     read reuse             161                              NA              NA

test_optional_messages
          write             132             105             147            20.5           -11.4
           read             306             179             257            41.5            16.0
    read no vec             230             142             219            38.3             4.8
     read reuse             163                              NA              NA

test_strings
          write              78              34              50            56.4            35.9
           read             203              78             184            61.6             9.4
    read no vec             160              41             143            74.4            10.6
     read reuse             105                              NA              NA

test_small_bytearrays
          write             319             327             215            -2.5            32.6
           read             373              30             308            92.0            17.4
    read no vec             130              25             203            80.8           -56.2
     read reuse             103                              NA              NA

test_large_bytearrays
          write           42600           60950           27889           -43.1            34.5
           read           18202             141           24362            99.2           -33.8
    read no vec            8120              45           12413            99.4           -52.9
     read reuse            5183                              NA              NA
```
