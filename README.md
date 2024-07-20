# Rustc Codegen C

[![CI](https://github.com/Wybxc/rustc_codegen_c/actions/workflows/ci.yml/badge.svg)](https://github.com/Wybxc/rustc_codegen_c/actions/workflows/ci.yml)

This a C codegen backend for rustc, which lowers Rust MIR to C code and compiles
by C compiler.

The project is at very early stage. Most of the code is not implemented yet.

## Try it

```bash
./y.sh rustc example/example.rs
./build/example
```

The usage of `y.sh` can be viewed from `./y.sh help`.

## License

This project is licensed under a dual license: MIT or Apache 2.0.
