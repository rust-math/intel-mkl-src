REGISTRY     := rustmath/mkl-rust
RUST_VERSION := 1.43.0
IMAGE        := $(REGISTRY):$(RUST_VERSION)

all: test

build:
	docker build . --build-arg "RUST_VERSION=$(RUST_VERSION)" -t $(IMAGE)

push: build
	docker push $(IMAGE)

test: build
	docker run -it --rm $(IMAGE) pkg-config --libs mkl-dynamic-lp64-iomp
