pub mod cors;
pub mod favicon;
pub mod logger;
pub mod static_file;
pub mod traits;

pub use cors::CorsMiddleware;
pub use favicon::FaviconMiddleware;
pub use logger::LoggerMiddleware;
pub use static_file::StaticFileMiddleware;
pub use traits::{Middleware, MiddlewareFuture, Next};
