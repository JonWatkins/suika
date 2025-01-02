# suika_ui

This project is a WebAssembly (Wasm) module built with Rust. It includes
functionality to bind events to HTML elements using attributes and logs messages
to the console.

## Prerequisites

Before you begin, ensure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

## Installation

To install `wasm-pack`, run the following command:

```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
wasm-pack --version
```

## Building the Project

To build the project and move the generated package to the desired location, run
the following command:

```bash
wasm-pack build --target web && mv pkg ../suika_wasm/wasm
```
