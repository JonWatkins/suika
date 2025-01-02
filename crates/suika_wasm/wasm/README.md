```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
wasm-pack --version
```

```bash
wasm-pack build --target web && mv pkg ../suika_wasm/wasm
```
