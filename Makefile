default: build/example

RUSTC_OPTIONS = -Z codegen-backend=crates/target/debug/librustc_codegen_c.so -C panic=abort \
	-Lall=build 

build/example: example/example.rs backend build/mini_core.lib 
	rustc example/example.rs --crate-type bin $(RUSTC_OPTIONS) --out-dir build 

TESTCASES = $(wildcard tests/**/*.rs)
TESTCASE_OUTPUT = $(patsubst tests/%.rs, build/tests/%, $(TESTCASES))

build/tests/%: tests/%.rs backend build/mini_core.lib
	rustc tests/$*.rs --crate-type bin $(RUSTC_OPTIONS) -o build/tests/$*

build/mini_core.lib: example/mini_core.rs backend
	rustc example/mini_core.rs --crate-type lib $(RUSTC_OPTIONS) --out-dir build

.PHONY: clean backend

backend:
	cd crates && cargo build

clean:
	rm -f rustc-ice-*
	rm -rf build
