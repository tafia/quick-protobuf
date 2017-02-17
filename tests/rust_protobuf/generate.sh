#!/bin/bash
#
# Test harness for generating files under tests/rust_protobuf/v[23]
# and expecting them either to succeed or fail for some known reason.
#

cd ../../codegen

# Checked in the end for non-empty value which serves as a boolean flag
have_failures=""

# Expected codegen failures are marked in the associative array `must_fail`
# with the relative path as the key and reason as value.
# When adding new, remember not to add any whitespace around `=`.
declare -A must_fail

must_fail["../tests/rust_protobuf/v2/test_group_pb.proto"]="expected failure (empty read)"

# Combined stdout and stderr for codegen of unexpectedly failed file.
declare -A outs

expecting_failure() {
	[ "${must_fail["$1"]+_}" ]
}

expecting_success() {
	! expecting_failure "$1"
}

success_msg() {
	if expecting_failure "$1"; then
		echo "${must_fail["$1"]}"
	else
		echo "ok"
	fi
}

for f in ../tests/rust_protobuf/v2/*.proto; do
	ret=0
	out="$(cargo run --quiet -- "$f" 2>&1)" || ret=$?

	if expecting_failure "$f" && [ "$ret" -eq 0 ]; then
		outs["$f"]="$out"
		have_failures="true"
		echo "$f: unexpected success"
	elif expecting_success "$f" && [ "$ret" -ne 0 ]; then
		have_failures="true"
		outs["$f"]="$out"
		echo "$f: unexpected failure $ret"
	else
		echo "$f: $(success_msg "$f")"
	fi
done

for f in ../tests/rust_protobuf/v3/*.proto; do
	ret=0
	out="$(cargo run --quiet -- "$f" 2>&1)" || ret=$?

	if expecting_failure "$f" && [ "$ret" -eq 0 ]; then
		have_failures="true"
		outs["$f"]="$out"
		echo "$f: unexpected success"
	elif expecting_success "$f" && [ "$ret" -ne 0 ]; then
		have_failures="true"
		outs["$f"]="$out"
		echo "$f: unexpected failure $ret"
	else
		echo "$f: $(success_msg "$f")"
	fi
done

echo

if [ "$have_failures" ]; then
	echo "There were code generation failures:"
	for f in "${!outs[@]}"; do
		echo
		echo "$f:"
		echo "${outs["$f"]}"
	done
	exit 1
else
	echo "All files generated as expected"
fi
