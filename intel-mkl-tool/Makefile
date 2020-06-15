HERE  := $(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))
IMAGE := rustmath/mkl-rust:1.43.0-2020.1.217

DOCKER_OPTION := --rm -u $(shell id -u):$(shell id -g) -v $(HERE):/src --env CARGO_HOME=/src/.cargo

test:
	docker run $(DOCKER_OPTION) $(IMAGE) cargo test with_mkl -- --ignored

seek:
	docker run $(DOCKER_OPTION) $(IMAGE) cargo run --release -- seek

package:
	docker run $(DOCKER_OPTION) $(IMAGE) cargo run --release -- package

download:
	docker run $(DOCKER_OPTION) $(IMAGE) cargo run --release -- download
