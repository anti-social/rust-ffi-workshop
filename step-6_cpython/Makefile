.default: run

.PHONY: run
run: build lib
	LD_LIBRARY_PATH=target/release ./main

build: main.c
	gcc -L target/release -l step_5_cdylib main.c -o main

.PHONY: bin-deps
bin-deps: build
	objdump -x main |rg 'NEEDED'

.PHONY: bin-syms
bin-syms: build
	nm -gD main |rg '^|add'

.PHONY: lib
lib:
	cargo build --release

.PHONY: lib-syms
lib-syms: lib
	nm -gD target/release/libstep_5_cdylib.so |rg '^|add'
