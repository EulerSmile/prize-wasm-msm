# Submission for zprize-wasm-msm from EulerSmile

A Rust+C and WebAssembly project using [wasm-pack](https://github.com/rustwasm/wasm-pack).



## Approach

We use the [MIRACL Core](https://github.com/miracl/core) as the base elliptic curve lib.  However the Pippenger's algorithm for msm in  [MIRACL Core](https://github.com/miracl/core) is fixed 4-bit window. So we made it a flexible window, which resulted in a 50% increase in efficiency.

In addition, we use Rust to implement the interface to complete competition.



## Build with wasm-pack

[emcc](https://github.com/emscripten-core/emsdk) is used to build the c code from  [MIRACL Core](https://github.com/miracl/core) .

```
CC=emcc AR=llvm-ar wasm-pack build
```



## License

Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
