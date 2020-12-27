.PHONY: build
build:
	docker build --tag=programming_language/ruby .

.PHONY: run
run:
	docker run -it -v $(PWD):/var/www/html programming_language/ruby ruby /var/www/html/index.rb
