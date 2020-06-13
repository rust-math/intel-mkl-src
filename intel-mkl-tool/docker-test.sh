#!/bin/bash
set -exu
docker run -it -u $(id -u):$(id -g) --rm -v $PWD:/src rustmath/mkl-rust:1.43.0-2020.1.217 cargo run -- seek
