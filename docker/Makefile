TARGETS := rust ubuntu

all: build
build: $(foreach TARGET,$(TARGETS),build/$(TARGET))
push:  $(foreach TARGET,$(TARGETS),push/$(TARGET))

build/rust:
	$(MAKE) -f rust.mk build

build/ubuntu:
	$(MAKE) -f ubuntu.mk build

push/rust: build/rust
	$(MAKE) -f rust.mk push

push/ubuntu: build/ubuntu
	$(MAKE) -f ubuntu.mk push
