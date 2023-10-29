#!/bin/sh

cargo \
	build \
	--target wasm32-wasi \
	--profile release-wasi
