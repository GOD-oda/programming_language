.PHONY: build
build:
	docker build --tag=programming_language/rust .

.PHONY: run
run:
	docker run -it -v $(PWD):/var/www/html -w /var/www/html  programming_language/rust
