# Suika Utils

Suika Utils is a utility library for the Suika web stack, providing essential
tools and helpers.

**Note:** Suika is under active development and not intended for production use.
The API is subject to change and may lack comprehensive testing and
documentation.

## Usage

### Parsing Query Strings

Use the `parse_query_string` function to parse a query string into a `HashMap`:

```rust
use suika_utils::parse_query_string;

let query = "name=John&age=30";
let params = parse_query_string(query);
assert_eq!(params.get("name"), Some(&"John".to_string()));
assert_eq!(params.get("age"), Some(&"30".to_string()));
```

### Building URLs

Use the `build_url` function to build a URL from a base and query parameters:

```rust
use suika_utils::build_url;

let base = "https://example.com";
let mut params = std::collections::HashMap::new();
params.insert("name", "John");
params.insert("age", "30");
let url = build_url(base, &params);
assert_eq!(url, "https://example.com?age=30&name=John");
```

### Parsing URLs

Use the `parse_url` function to parse a URL into its components:

```rust
use suika_utils::parse_url;

let url = "https://example.com/path?name=John&age=30";
let components = parse_url(url).unwrap();
assert_eq!(components.0, "https");
assert_eq!(components.1, "example.com");
assert_eq!(components.2, "/path");
assert_eq!(components.3.get("name"), Some(&"John".to_string()));
assert_eq!(components.3.get("age"), Some(&"30".to_string()));
```

### Creating No-Op Wakers

Use the `noop_waker` function to create a no-op waker for use in tests:

```rust
use suika_utils::noop_waker;
use std::task::{Context, Waker};
use std::future::Future;
use std::pin::Pin;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

let waker = noop_waker();
let mut cx = Context::from_waker(&waker);

let ready = Arc::new(AtomicBool::new(false));
let ready_clone = Arc::clone(&ready);

let mut future = Box::pin(async move {
    ready_clone.store(true, Ordering::SeqCst);
});

assert!(!ready.load(Ordering::SeqCst));
let _ = future.as_mut().poll(&mut cx);
assert!(ready.load(Ordering::SeqCst));
```
