.PHONY: build
build:
	docker build -t programming_language/go .

.PHONY: run
run:
	docker run -it -v $(PWD)/src:/go/src -w /go/src programming_language/go go run main.go
