//! Batteries-included Axum starter. Pick the pieces you need via Cargo features:
//!
//! - `components` — re-exports [`axumstart_components`], an async dependency-injection
//!   container with an `Inject<T>` Axum extractor.
//! - `router` — re-exports [`axumstart_router`] (implies `components`): `#[get("/path")]`/
//!   `#[post(...)]`/etc. auto-register routes, `#[layer(...)]` attaches per-route middleware,
//!   and [`serve`] loads `.env`, builds the DI container, and serves the auto-registered routes.
//! - `postgres` / `mysql` / `sqlite` — re-exports [`axumstart_db`] with the matching backend
//!   enabled. Pick exactly one.
//!
//! ```toml
//! axumstart = { version = "0.1", features = ["router", "postgres"] }
//! ```

#[cfg(feature = "components")]
pub use axumstart_components as components;

#[cfg(feature = "router")]
pub use axumstart_router as router;

#[cfg(any(feature = "postgres", feature = "mysql", feature = "sqlite"))]
pub use axumstart_db as db;

/// Loads a `.env` file from the current working directory (silently does nothing if there
/// isn't one — same as [`dotenvy::dotenv`]), then registers every `#[component]`/
/// `#[derive(Component)]` type via [`components::ComponentProvider::register_all`] and every
/// `#[router::get]`/`#[router::post]`/etc. route via [`router::register_routes`], builds the
/// resulting [`components::Components`] container, and serves the auto-registered routes on
/// `0.0.0.0:{port}`.
///
/// `.env` is loaded *before* components are built, so `#[derive(ComponentConfig)]`'s
/// `#[env_var(...)]` fields see values from it.
///
/// ```no_run
/// use axumstart::router::get;
///
/// #[get("/health")]
/// async fn health() -> &'static str {
///     "ok"
/// }
///
/// # async fn run() -> std::io::Result<()> {
/// axumstart::serve(3000).await
/// # }
/// ```
#[cfg(feature = "router")]
pub async fn serve(port: u16) -> std::io::Result<()> {
  dotenvy::dotenv().ok();

  let provider = components::ComponentProvider::default();
  provider.register_all();
  let state = provider.build().await;

  let auto_routes = router::register_routes(axum::Router::new(), &state);
  let app = auto_routes.with_state(state);

  let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await?;
  axum::serve(listener, app).await
}
