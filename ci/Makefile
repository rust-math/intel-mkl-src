REGISTRY     := rustmath/mkl
RUST_VERSION := 1.43.0
DISTRIBUTIONS := debian ubuntu centos

all: $(DISTRIBUTIONS)

define docker-build
$(1):
	docker build . -f $$@.Dockerfile --build-arg "RUST_VERSION=$(RUST_VERSION)" -t $(REGISTRY)-$$@:$(RUST_VERSION)
endef
$(foreach DIST,$(DISTRIBUTIONS), $(eval $(call docker-build,$(DIST))))

push: $(DISTRIBUTIONS)
	$(foreach DIST,$(DISTRIBUTIONS), docker push $(REGISTRY)-$(DIST):$(RUST_VERSION); )

test: $(DISTRIBUTIONS)
	$(foreach DIST,$(DISTRIBUTIONS), docker run -it --rm $(REGISTRY)-$(DIST):$(RUST_VERSION) pkg-config --libs mkl-dynamic-lp64-seq; )
