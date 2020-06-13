#!/bin/bash
set -exu

docker run -it --rm                   \
  -u $(id -u):$(id -g)                \
  -v $PWD:/src                        \
  rustmath/mkl-rust:1.43.0-2020.1.217 \
  cargo test with_mkl -- --ignored
