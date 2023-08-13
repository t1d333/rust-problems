#!/bin/bash

clippy_cmd="cargo clippy"

for module in */; do
	if [[ "scripts/" == "$module" ]]; then
		continue
	fi
	cd $module 
		for task in */; do
			echo "====================================="
			echo "Run clippy for $module$task"
			echo "====================================="
			cargo test --release  --manifest-path "./$task/Cargo.toml"
		done
	cd ..
done
