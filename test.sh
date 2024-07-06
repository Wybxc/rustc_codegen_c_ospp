#!/bin/bash

rm -f rustc-ice-*
cargo build
rustc -Z codegen-backend=target/debug/librustc_codegen_c.so example/mini_core.rs --out-dir build -Lall=example
