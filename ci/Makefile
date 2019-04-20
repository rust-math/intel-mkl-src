REGISTRY := rustmath/mkl

all: image

image:
	docker build . -t $(REGISTRY)

push: image
	docker push $(REGISTRY)

run: image
	docker run -it --rm $(REGISTRY)
