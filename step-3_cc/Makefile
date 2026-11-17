.default: run

BIN := target/release/step-3_cc

.PHONY: run
run: $(BIN)
	LD_LIBRARY_PATH=. cargo run --release

.PHONY: build
build: $(BIN)

$(BIN): add.c Cargo.toml Cargo.lock build.rs $(wildcard src/*.rs)
	cargo build --release

.PHONY: bin-deps
bin-deps: $(BIN)
	readelf -d $(BIN) |rg '^|libadd'

.PHONY: bin-syms
bin-syms: $(BIN)
	readelf -s $(BIN) |rg add

.PHONY: clean
clean:
	rm -rf target
