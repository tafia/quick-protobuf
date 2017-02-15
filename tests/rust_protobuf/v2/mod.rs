//! A module to import many generated files plus some custom tests
//!
//! For most generated files, the 'test' is just making sure it compiles!

mod test_basic_pb;
mod test_default_values_pb;
mod test_enum_values_pb;
mod test_group_pb;
mod test_ident_pb;
mod test_import_nested_pb;
mod test_import_nested_imported_pb;
// mod test_import_pkg_nested_pb;
mod test_import_pkg_nested_imported_pb;
mod test_import_root_pb;
mod test_import_root_imported_pb;
mod test_import_nonunique_pb;
mod test_import_nonunique_1_pb;
mod test_import_nonunique_2_pb;
mod test_lite_runtime_pb;
mod test_nonunique_enum_pb;
mod test_required_pb;
// mod test_root_pb;
// mod test_sanitize_file_name_pb;
mod test_oneof_pb;
// mod test_special_characters_file_name__pb;
mod test_enum_alias_pb;

mod test_basic;
mod test_required;
mod test_default_values;
mod test_lite_runtime;

mod test_text_format_pb;
mod test_text_format;

mod test_oneof;
// mod test_enum_values;
// mod test_enum_alias;
mod test_sync;

mod struct_pb;
