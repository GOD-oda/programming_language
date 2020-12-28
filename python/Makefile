.PHONY: build
build:
	docker build --tag=programming_language/python .

.PHONY: run
run:
	docker run -it -v $(PWD):/var/www/html programming_language/python python /var/www/html/index.py
