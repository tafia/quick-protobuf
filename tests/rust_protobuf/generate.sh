#!/bin/bash

cd ../../codegen

failures=0
have_failures=""

declare -A must_fail
must_fail["../tests/rust_protobuf/v2/test_group_pb.rs"]=""

declare -A outs

for f in ../tests/rust_protobuf/v2/*.proto; do
	ret=0
	out="$(cargo run --quiet -- "$f" 2>&1)" || ret=$?

	if [ "${must_fail["$f"]+_}" ] && [ "$ret" -eq 0 ]; then
		outs["$f"]="$out"
		have_failures="true"
		echo "$f: unexpected success"
	elif [ "$ret" -ne 0 ]; then
		have_failures="true"
		outs["$f"]="$out"
		echo "$f: unexpected failure $?"
	else
		echo "$f: ok"
	fi
done

for f in ../tests/rust_protobuf/v3/*.proto; do
	ret=0
	out="$(cargo run --quiet -- "$f" 2>&1)" || ret=$?

	if [ "${must_fail["$f"]+_}" ] && [ "$ret" -eq 0 ]; then
		have_failures="true"
		outs["$f"]="$out"
		echo "$f: unexpected success"
	elif [ "$ret" -ne 0 ]; then
		have_failures="true"
		outs["$f"]="$out"
		echo "$f: unexpected failure $?"
	else
		echo "$f: ok"
	fi
done

if [ "$have_failures" ]; then
	echo
	echo "There were code generation failures:" >&2
	for f in "${!outs[@]}"; do
		echo "$f:"
		echo "${outs["$f"]}"
		echo
	done
	exit 1
fi
