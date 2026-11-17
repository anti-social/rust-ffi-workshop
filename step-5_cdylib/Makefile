.default: run

LIB := target/release/libstep_5_cdylib.so

.PHONY: run
run: main
	LD_LIBRARY_PATH=target/release ./main

main: main.c $(LIB)
	gcc -L target/release -l step_5_cdylib main.c -o main

.PHONY: bin-deps
bin-deps: main
	objdump -x main |rg 'NEEDED'

.PHONY: bin-syms
bin-syms: main
	nm -gD main |rg '^|add'

.PHONY: lib
lib: $(LIB)

$(LIB): Cargo.toml Cargo.lock src/lib.rs
	cargo build --release

.PHONY: lib-syms
lib-syms: lib
	nm -gD $(LIB) |rg '^|add'

.PHONY: clean
clean:
	rm -rf target
	rm -f main
