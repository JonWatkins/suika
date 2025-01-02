# suika_json

Suika JSON is a JSON handling library for the Suika web stack.

**Note:** Suika is under active development and not intended for production use.
The API is subject to change and may lack comprehensive testing and
documentation.

## Features

- Parse JSON strings into `JsonValue` enums.
- Supports JSON objects, arrays, strings, numbers, booleans, and null values.
- Lightweight and easy to use.

## Usage

Here's an example of how to use `suika_json` to parse a JSON string:

```rust
use suika_json::{parse_json, JsonValue};

fn main() {
    let json = r#"{ "key": "value", "array": [1, 2, 3], "number": 123.45, "boolean": true, "null": null }"#;
    let value = parse_json(json).unwrap();

    assert_eq!(value, JsonValue::Object(vec![
        ("key".to_string(), JsonValue::String("value".to_string())),
        ("array".to_string(), JsonValue::Array(vec![
            JsonValue::Number(1.0),
            JsonValue::Number(2.0),
            JsonValue::Number(3.0)
        ])),
        ("number".to_string(), JsonValue::Number(123.45)),
        ("boolean".to_string(), JsonValue::Boolean(true)),
        ("null".to_string(), JsonValue::Null)
    ]));
}
```

## JsonParser Struct

The `JsonParser` struct is used to parse `JSON` strings into `JsonValue` enums.
Here's an example of how to use `JsonParser`:

```rust
use suika_json::{JsonParser, JsonValue};

fn main() {
    let mut parser = JsonParser::new(r#"{ "key": "value" }"#);
    let value = parser.parse().unwrap();

    assert_eq!(value, JsonValue::Object(vec![
        ("key".to_string(), JsonValue::String("value".to_string()))
    ]));
}
```

## JsonValue Enum

The `JsonValue` enum represents a JSON value and can be one of the following
variants:

- `Object(Vec<(String, JsonValue)>)`: A JSON object represented as a vector of
  key-value pairs.
- `Array(Vec<JsonValue>)`: A JSON array represented as a vector of `JsonValue`
  elements.
- `String(String)`: A JSON string.
- `Number(f64)`: A JSON number.
- `Boolean(bool)`: A JSON boolean.
- `Null`: A JSON null value.
