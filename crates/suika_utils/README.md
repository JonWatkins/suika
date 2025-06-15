# Suika Utils: A Toy Utility Library

**Important: This is a personal toy project, developed as an experiment and learning exercise.**

**As a toy project, its future development is uncertain. It may or may not receive future updates, maintenance, or bug fixes. Please do not use it in production environments.**

Suika Utils is a utility library for the `Suika` web stack (also a toy project), providing essential tools and helpers primarily for learning and experimental purposes.

The API is subject to change. This project is not thoroughly tested or hardened for real-world applications, and documentation may be basic.

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