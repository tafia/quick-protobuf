> Legend:
  - feat: A new feature
  - fix: A bug fix
  - docs: Documentation only changes
  - style: White-space, formatting, missing semi-colons, etc
  - refactor: A code change that neither fixes a bug nor adds a feature
  - perf: A code change that improves performance
  - test: Adding missing tests
  - chore: Changes to the build process or auxiliary tools/libraries/documentation

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
