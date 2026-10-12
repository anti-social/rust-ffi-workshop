.default: run

.PHONY: run
run: lib
	LD_LIBRARY_PATH=. cargo run --release

.PHONY: build
build: lib
	cargo build --release

.PHONY: bin-deps
bin-deps: build
	readelf -d target/release/step-4_bindgen |rg '^|libadd'

.PHONY: bin-syms
bin-syms: build
	readelf -s target/release/step-4_bindgen |rg add

lib: add.c
	gcc -c -fPIC add.c -o add.o
	gcc add.o -shared -o libadd.so

.PHONY: lib-syms
lib-syms: lib
	readelf -s libadd.so |rg '^|add'
