.default: run

.PHONY: run
run: build
	LD_LIBRARY_PATH=. ./main

build: lib main.rs
	rustc -C opt-level=3 -C strip=symbols -L . -l add main.rs

.PHONY: bin-deps
bin-deps: build
	readelf -d main |rg '^|libadd'

.PHONY: bin-syms
bin-syms: build
	readelf -s main |rg add

lib: add.c
	gcc -c -fPIC add.c -o add.o
	gcc add.o -shared -o libadd.so

.PHONY: lib-symbols
lib-syms: lib
	readelf -s libadd.so |rg '^|add'
