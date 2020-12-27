.PHONY: build
build:
	docker build --tag=programming_language/php .
	
.PHONY: run
run:
	docker run -it -v $(PWD):/var/www/html  programming_language/php php /var/www/html/index.php
