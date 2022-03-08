index.html:
	(cd report/html && \
		npm install -g npx || :&& \
		npm install -d && \
		npx --yes parcel build --no-optimize index.html && \
		cp dist/index.html ../../)

.PHONY: build
build: index.html
	cargo build