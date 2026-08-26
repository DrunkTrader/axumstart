//! Batteries-included Axum starter. Pick the pieces you need via Cargo features:
//!
//! - `components` — re-exports [`axumstart_components`], an async dependency-injection
//!   container with an `Inject<T>` Axum extractor.
//! - `postgres` / `mysql` / `sqlite` — re-exports [`axumstart_db`] with the matching backend
//!   enabled. Pick exactly one.
//!
//! ```toml
//! axumstart = { version = "0.1", features = ["components", "postgres"] }
//! ```

#[cfg(feature = "components")]
pub use axumstart_components;

#[cfg(any(feature = "postgres", feature = "mysql", feature = "sqlite"))]
pub use axumstart_db;
