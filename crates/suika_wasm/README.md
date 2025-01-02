# suika_wasm

`suika_wasm` is a middleware for the Suika web stack to serve pre-compiled
WebAssembly (Wasm) files.

**Note:** Suika is under active development and not intended for production use.
The API is subject to change and may lack comprehensive testing and
documentation.

```rust
use suika::server::{
    middleware::wasm_file_middleware,
    router::Router,
    Server,
};

use std::sync::Arc;

pub fn main() {
    let server = Server::new();
    let mut router = Router::new();

    router.get("/", |_req, res, _next| async move {
        res.set_status(200);
        res.body("Hello World".to_string());
        Ok(())
    });

    let router = Arc::new(router);

    let combined_middleware = combine_middlewares(vec![
        Arc::new(wasm_file_middleware("/wasm", 3600)),
        Arc::new(move |req, res, next| {
            let router = Arc::clone(&router);
            Box::pin(async move { router.handle(req, res, next).await })
        }),
    ]);

    server.use_middleware(move |req, res, next| combined_middleware(req, res, next));
    server.listen("127.0.0.1:7878");
}
```
