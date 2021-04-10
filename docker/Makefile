REGISTRY     := docker.pkg.github.com/rust-math/intel-mkl-src
RUST_VERSION := 1.49.0

all: test ubuntu centos

build:
	docker build .                               \
		--build-arg "RUST_VERSION=$(RUST_VERSION)" \
		-t $(REGISTRY)/mkl-rust:$(RUST_VERSION)

push: build ubuntu centos
	docker push $(REGISTRY)/mkl-rust:$(RUST_VERSION)
	docker push $(REGISTRY)/mkl-centos:$(RUST_VERSION)
	docker push $(REGISTRY)/mkl-ubuntu:$(RUST_VERSION)

test: build
	docker run -it --rm $(REGISTRY)/mkl-rust:$(RUST_VERSION) pkg-config --libs mkl-dynamic-lp64-iomp
	docker run -it --rm $(REGISTRY)/mkl-rust:$(RUST_VERSION) ldconfig -p | grep mkl

ubuntu:
	docker build . -f $@.Dockerfile              \
		--build-arg "RUST_VERSION=$(RUST_VERSION)" \
		-t $(REGISTRY)/mkl-ubuntu:$(RUST_VERSION)

centos:
	docker build . -f $@.Dockerfile              \
		--build-arg "RUST_VERSION=$(RUST_VERSION)" \
		-t $(REGISTRY)/mkl-centos:$(RUST_VERSION)
