#!/bin/bash

ls_wasmtime(){
	wasmtime run \
	  --env ENV_DIR=./ \
	  --mapdir "./::$PWD" \
	  ../modules/ls/target/wasm32-wasi/release/ls.wasm
}

filter_wasmtime(){
	wasmtime run \
	  --env ENV_START=./Cargo \
	  ../modules/string-filter/target/wasm32-wasi/release/string-filter.wasm
}

run_wasmtime_pipe(){
	ls_wasmtime |
	  filter_wasmtime
}

run(){
	run_wasmtime_pipe
}

run
