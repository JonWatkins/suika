# suika_wasm

`suika_wasm` is a middleware for the Suika web stack to serve pre-compiled
WebAssembly (Wasm) files.

**Note:** Suika is under active development and not intended for production use.
The API is subject to change and may lack comprehensive testing and
documentation.

```rust
use suika::server::{Server, Router};
use suika::middleware::WasmFileMiddleware;
use std::sync::Arc;

pub fn main() {
    let mut server = Server::new("127.0.0.1:8080");
    let mut router = Router::new("/");

    router.add_route(Some("GET"), r"/?$", |_req, res| {
        Box::pin(async move {
            res.set_status(201).await;
            res.body("Hello World!".to_string()).await;
            Ok(())
        })
    });

    server.use_middleware(Arc::new(WasmFileMiddleware::new("/wasm", 86400)));
    server.use_middleware(Arc::new(router));

    server.run();
}
```
