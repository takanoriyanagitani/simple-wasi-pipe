#!/bin/bash

run_wasmtime(){
	wasmtime \
	  run \
	  --env ENV_DIR=/guest.d/dir.d \
	  --mapdir "/guest.d/dir.d::${PWD}" \
	  ./target/wasm32-wasi/release/ls.wasm
}

run(){
	run_wasmtime
}

run
