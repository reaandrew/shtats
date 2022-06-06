index.html:
	(cd report/html && \
		npm install -d && \
		parcel build index.html && \
		cp dist/index.html ../../)

.PHONY: build
build: index.html
	cargo build

.PHONY: test
test: index.html
	cargo test

