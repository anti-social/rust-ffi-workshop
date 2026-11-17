.default: run

RELEASE_DIR := target/release
LIB := $(RELEASE_DIR)/libstep_6_cpython.so
EXT := $(RELEASE_DIR)/add$(shell python-config --extension-suffix)

.PHONY: run
run: $(EXT)
	PYTHONPATH=$(RELEASE_DIR) ./main.py

$(EXT): $(LIB)
	cp $< $@

$(LIB): Cargo.toml Cargo.lock build.rs $(wildcard src/*.rs)
	cargo build --release

.PHONY: lib
lib: $(LIB)

.PHONY: lib-syms
lib-syms: $(LIB)
	nm -gD $(LIB) |rg '^|add'
