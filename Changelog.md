> Legend:
  - feat: A new feature
  - fix: A bug fix
  - docs: Documentation only changes
  - style: White-space, formatting, missing semi-colons, etc
  - refactor: A code change that neither fixes a bug nor adds a feature
  - perf: A code change that improves performance
  - test: Adding missing tests
  - chore: Changes to the build process or auxiliary tools/libraries/documentation

## master
- docs: add examples, more details about message to struct conversion
- refactor: update dependencies
- fix: remove some warning on codegen fixed size types + non camel names
- feat: break codegen when reserved fields conflict
- feat: support imports in proto files
- feat: support packages by encapsulating into rust modules
- feat: support map
- refactor: major refatorings of codegen

## 0.2.0
- feat: do not allocate for bytes and string field types
- docs: improve documentation: readmes, examples in modules
- test: added new benches
- perf: much faster reader using `Cow` instead of `Vec<u8>` or `String`

## 0.1.0
- first release!
