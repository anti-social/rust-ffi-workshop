.default: run

LIB := libadd.so

.PHONY: run
run: main
	LD_LIBRARY_PATH=. ./main

.PHONY: build
build: main

main: main.rs $(LIB)
	rustc -C opt-level=3 -C strip=symbols -L . -l add main.rs

.PHONY: bin-deps
bin-deps: main
	readelf -d main |rg '^|libadd'

.PHONY: bin-syms
bin-syms: main
	readelf -s main |rg add

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
	rm -f main add.o $(LIB)
