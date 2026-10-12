.default: run

.PHONY: run
run:
	LD_LIBRARY_PATH=. cargo run --release

.PHONY: build
build:
	cargo build --release

.PHONY: bin-deps
bin-deps: build
	readelf -d target/release/step-3_cc |rg '^|libadd'

.PHONY: bin-syms
bin-syms: build
	readelf -s target/release/step-3_cc |rg add
