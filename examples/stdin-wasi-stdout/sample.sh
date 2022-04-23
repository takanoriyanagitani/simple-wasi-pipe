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

linecount_wasmtime(){
	wasmtime run \
	  ../modules/wc/target/wasm32-wasi/release/wc.wasm
}

run_wasmtime_pipe(){
	ls_wasmtime |
	  filter_wasmtime |
	  linecount_wasmtime
}

run(){
	run_wasmtime_pipe
}

run
