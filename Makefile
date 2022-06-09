index.html:
	(cd report/html && \
		npm install -d && \
		parcel build index.html && \
		cp dist/index.html ../../)

grcov:
	curl -LO https://github.com/mozilla/grcov/releases/download/v0.8.7/grcov-x86_64-unknown-linux-gnu.tar.bz2
	tar -xf grcov-x86_64-unknown-linux-gnu.tar.bz2
	rm grcov-x86_64-unknown-linux-gnu.tar.bz2

.PHONY: build
build: index.html
	cargo build

.PHONY: test
test: export CARGO_INCREMENTAL=0
test: export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort"
test: export RUSTDOCFLAGS="-Cpanic=abort"
test: index.html grcov
	cargo build
	cargo test
	grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/

