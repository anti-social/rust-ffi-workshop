.default: run

LIB := libadd.so
BIN := target/release/step-2_cargo

.PHONY: run
run: $(BIN)
	LD_LIBRARY_PATH=. cargo run --release

.PHONY: build
build: $(BIN)

$(BIN): $(LIB) Cargo.toml Cargo.lock build.rs $(wildcard src/*.rs)
	cargo build --release

.PHONY: bin-deps
bin-deps: $(BIN)
	readelf -d $(BIN) |rg '^|libadd'

.PHONY: bin-syms
bin-syms: $(BIN)
	readelf -s $(BIN) |rg add

.PHONY: lib
lib: $(LIB)

add.o: add.c
	gcc -c -fPIC add.c -o add.o

$(LIB): add.o
	gcc add.o -shared -o $(LIB)

.PHONY: lib-syms
lib-syms: $(LIB)
	readelf -s $(LIB) |rg '^|add'

.PHONY: clean
clean:
	rm -rf target
	rm -f add.o $(LIB)
