REGISTRY     := rustmath/mkl-rust
RUST_VERSION := 1.43.0
MKL_VERSION  := 2020.1.217
IMAGE        := $(REGISTRY):$(RUST_VERSION)-$(MKL_VERSION)

all: test

build:
	docker build .                               \
		--build-arg "RUST_VERSION=$(RUST_VERSION)" \
		--build-arg "MKL_VERSION=$(MKL_VERSION)"   \
		-t $(IMAGE)

push: build
	docker push $(IMAGE)

test: build
	docker run -it --rm $(IMAGE) pkg-config --libs mkl-dynamic-lp64-iomp
