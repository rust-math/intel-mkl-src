#!/bin/bash
set -exu

script_dir="$(cd $(dirname ${BASH_SOURCE:-$0}); pwd)"

docker run --rm                       \
  -u $(id -u):$(id -g)                \
  -v $script_dir:/src                 \
  rustmath/mkl-rust:1.43.0-2020.1.217 \
  cargo test with_mkl -- --ignored
