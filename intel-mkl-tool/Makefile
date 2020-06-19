HERE          := $(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))
RUST_VERSION  := 1.43.0
DOCKER_OPTION := --rm -u $(shell id -u):$(shell id -g) -v $(HERE):/src

test:
	docker run $(DOCKER_OPTION) --env CARGO_HOME=/src/.cargo-cache rustmath/mkl-rust:$(RUST_VERSION) cargo test with_mkl -- --ignored

seek:
	docker run $(DOCKER_OPTION) --env CARGO_HOME=/src/.cargo-cache rustmath/mkl-rust:$(RUST_VERSION) cargo run --release -- seek

package:
	docker run $(DOCKER_OPTION) --env CARGO_HOME=/src/.cargo-cache rustmath/mkl-rust:$(RUST_VERSION) cargo run --release -- package

seek-ubuntu:
	docker run $(DOCKER_OPTION) --env CARGO_HOME=/src/.cargo-ubuntu rustmath/mkl-ubuntu:$(RUST_VERSION) cargo run --release -- seek

seek-centos:
	docker run $(DOCKER_OPTION) --env CARGO_HOME=/src/.cargo-centos rustmath/mkl-centos:$(RUST_VERSION) cargo run --release -- seek
