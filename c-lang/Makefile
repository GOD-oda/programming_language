.PHONY: build
build:
	docker build -t programming_language/c-lang .

.PHONY: run
run:
	docker run -it -v $(PWD):/var/www/html -w /var/www/html programming_language/c-lang
