default: build/example

RUSTC_OPTIONS = -Z codegen-backend=target/debug/librustc_codegen_c.so --out-dir build -Lall=build \
	-C panic=abort

build/example: backend example/example.rs build/mini_core.lib 
	rustc example/example.rs --crate-type bin $(RUSTC_OPTIONS)

build/%: backend build/mini_core.lib example/tests/%.rs
	rustc example/tests/$*.rs --crate-type bin $(RUSTC_OPTIONS)

build/mini_core.lib: backend example/mini_core.rs 
	rustc example/mini_core.rs --crate-type lib $(RUSTC_OPTIONS)

.PHONY: clean backend

backend:
	cargo build

clean:
	rm -f rustc-ice-*
	rm -rf build
