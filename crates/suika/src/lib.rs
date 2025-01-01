//! # Suika
//!
//! `suika` is a web stack that re-exports features from various crates.
//! It provides a comprehensive set of tools for building web applications.

/// MIME type utilities.
pub mod mime {
  pub use suika_mime::*;
}

/// JSON handling utilities.
pub mod json {
  pub use suika_json::*;
}

/// Utility functions.
pub mod utils {
  pub use suika_utils::*;
}

/// Template rendering utilities.
pub mod templates {
  pub use suika_templates::*;
}

/// Server functionalities.
pub mod server {
  pub use suika_server::*;
}
