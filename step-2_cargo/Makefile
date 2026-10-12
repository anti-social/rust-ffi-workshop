.default: run

.PHONY: run
run:
	LD_LIBRARY_PATH=. cargo run --release

.PHONY: build
build: lib
	cargo build --release

.PHONY: bin-deps
bin-deps: build
	readelf -d main |rg '^|libadd'

.PHONY: bin-syms
bin-syms: build
	readelf -s main |rg add

lib: add.c
	gcc -c -fPIC add.c -o add.o
	gcc add.o -shared -o libadd.so

.PHONY: lib-symbs
lib-syms: lib
	readelf -s libadd.so |rg '^|add'
