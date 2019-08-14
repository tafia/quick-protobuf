> Legend:
  - feat: A new feature
  - fix: A bug fix
  - docs: Documentation only changes
  - style: White-space, formatting, missing semi-colons, etc
  - refactor: A code change that neither fixes a bug nor adds a feature
  - perf: A code change that improves performance
  - test: Adding missing tests
  - chore: Changes to the build process or auxiliary tools/libraries/documentation

## pb-rs 0.8.2
- feat: add optional Owned variant when generating Messages (relies on `Pin`)
- fix: propagate `dont_use_cow` to all messages

## pb-rs 0.8.1
- feat: avoid Cow::Borrow when comparing `&str`

## pb-rs 0.8.0

- feat: Added the ability to use custom RPC generation code
- feat: Added the ability to modify custom derives such as derive-new
- feat: (Large change) Added the `dont_use_cow` method [v2 and v3 tested] which replaces lifetimes 
and Cow with regular strings and vectors. This will decrease performance - and will 
eventually be replaced with a whitelist for specific structures instead of a boolean.
This is meant for the structure to fully own the data it has.

## pb-rs 0.7.0
- feat: add a ConfigBuilder

## pb-rs 0.5.0
- fix: nested messages issue (missing OneOf, lifetime)
- feat: break cycles on optional messages only, and required one if user chooses to

## quick-protobuf 0.6.3
- feat: add `serialize_into_vec` and `deserialize_from_slice` helper functions

## pb-rs 0.4.1
- fix: fix some nested messages not recognised (#90)
- test: add new mesos.proto test
- refactor: stop using `find_enum` and `find_message`, have MessageIndex and EnumIndex instead

## pb-rs 0.4.0
- fix: ignore packed for non primitive fields
- fix: force message with default values to be enum
- feat: check if enum variant exists in default fields

## 0.6.2
- feat: update dependencies
- feat: have a public `read_message_by_len` to allow non varint message parsing

## 0.6.1
- feat: make byte-specific utilities available in BytesReader

## pb-rs 0.3.2
- docs: mention usage pf pb-rs with build.rs script in readme

## pb-rs 0.3.1
- feat: codegen, add `From<&str>` for enums
- feat: dependable codegen

##0.6.0
- feat: add MessageRead trait (thanks @daboross !)
- fix: nested enums
- fix: fix reserved fields management
- perf: avoid varint decoding for bools
- chore: have travis run rustfmt

## 0.5.0
- fix: removed one unwanting println
- feat: add automatic `std::io::Error` conversion
- refactor:  add clap-rs to pb-rs

## 0.3.0 - 0.4.0
- docs: add examples, more details about message to struct conversion
- refactor: update dependencies
- fix: remove some warning on codegen fixed size types + non camel names
- feat: break codegen when reserved fields conflict
- feat: support imports in proto files
- feat: support packages by encapsulating into rust modules
- feat: support map
- refactor: major refatorings of codegen
- feat: rename rust keywords
- feat: normalize file names
- feat: packed fixed size fields are now `Cow`
- test: migrated most rust-protobuf tests

## 0.2.0
- feat: do not allocate for bytes and string field types
- docs: improve documentation: readmes, examples in modules
- test: added new benches
- perf: much faster reader using `Cow` instead of `Vec<u8>` or `String`

## 0.1.0
- first release!
