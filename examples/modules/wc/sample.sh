#!/bin/bash

run_wasmtime(){
	wasmtime run \
	  ./target/wasm32-wasi/release/wc.wasm
}

run_wc(){
	wc --lines
}

wasmtime_loc=$( which wasmtime )

run=$( test -f "${wasmtime_loc}" && echo run_wasmtime || echo run_wc )

ls | $run
