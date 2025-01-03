//! # Suika
//!
//! `suika` is a web stack that re-exports features from various crates.
//! It provides a comprehensive set of tools for building web applications.

pub mod mime {
  pub use suika_mime::{get_mime_type, get_mime_type_from_path, MimeType};
}

pub mod json {
  pub use suika_json::{parse_json, JsonValue};
}

pub mod utils {
  pub use suika_utils::*;
}

pub mod templates {
  pub use suika_templates::{
      TemplateEngine, TemplateParser, TemplateToken, TemplateValue
  };
}

pub mod server {
  pub use suika_server::server::Server;
  pub use suika_server::router::Router;
  pub use suika_server::error::HttpError;
}

pub mod middleware {
  pub use suika_server::middleware::*;
  pub use suika_wasm::*;
}
