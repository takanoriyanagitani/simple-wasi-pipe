#!/bin/bash

run_wasmtime(){
	ls |
	  wasmtime \
	  run \
	  --env ENV_START=Cargo \
	  ./target/wasm32-wasi/release/string-filter.wasm
}

run(){
	run_wasmtime
}

run
