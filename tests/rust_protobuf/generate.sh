#!/bin/bash

cd ../../codegen

failures=0
have_failures=""

declare -A must_fail
must_fail["../tests/rust_protobuf/v2/test_group_pb.proto"]="expected failure (empty read)"

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

if [ "$have_failures" ]; then
	echo
	echo "There were code generation failures:" >&2
	for f in "${!outs[@]}"; do
		echo
		echo "$f:"
		echo "${outs["$f"]}"
	done
	exit 1
fi
