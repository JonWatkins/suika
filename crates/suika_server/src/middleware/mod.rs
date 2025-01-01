mod combine_middleware;
mod cors_middleware;
mod favicon_middleware;
mod logger_middleware;
mod static_file_middleware;
mod next_middleware;

pub use combine_middleware::combine_middlewares;
pub use cors_middleware::cors_middleware;
pub use favicon_middleware::favicon_middleware;
pub use logger_middleware::logger_middleware;
pub use static_file_middleware::static_file_middleware;
pub use next_middleware::{NextMiddleware, Middleware, MiddlewareFn};
