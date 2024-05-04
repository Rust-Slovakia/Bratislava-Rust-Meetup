TARGET_DIR        ?= "$(PWD)/target/lambda/lambda-demo"

.PHONY: build-docker build-lambda

all: build-docker build-lambda

build-docker:
	cd ../ && \
	docker build -f Dockerfile.demo-lambda -t demo-lambda . && \
	cd -

build-lambda:
	mkdir -p $(TARGET_DIR)
	docker run -v $(TARGET_DIR):/dist --rm --entrypoint cp demo-lambda:latest "/usr/local/bin/bootstrap" /dist/bootstrap
	cd $(TARGET_DIR) && \
	zip boostrap.zip bootstrap && \
	cd -