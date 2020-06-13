#!/bin/bash
set -exu

script_dir="$(cd $(dirname ${BASH_SOURCE:-$0}); pwd)"
image="rustmath/mkl-rust:1.43.0-2020.1.217"
option="--rm -u $(id -u):$(id -g) -v $script_dir:/src"

docker run $option $image cargo test with_mkl -- --ignored
docker run $option $image cargo run -- seek
