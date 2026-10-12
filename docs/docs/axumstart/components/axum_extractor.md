---
sidebar_position: 6
sidebar_label: Axum Extractor
---

# Axum Extractor

`Components` implements the shape Axum's `FromRequestParts` needs, so use it directly as router
state, and pull dependencies out of handlers with `Inject<T>`:

```rust
use axum::{Router, routing::get};
use axumstart_components::{ComponentProvider, axum::Inject};
use std::sync::Arc;

async fn handler(db: Inject<Database>) -> String {
    db.query_count.load(Ordering::Relaxed).to_string()
}

async fn build_app() -> Router {
    let ctx = ComponentProvider::default();
    ctx.register_all();
    let components = ctx.build().await;

    Router::new()
        .route("/", get(handler))
        .with_state(components)
}
```

`Inject<T>` rejects with a `String` (used as the Axum rejection body) naming the missing type if
`T` was never registered — this only happens if the component is missing from the container, not
per-request.
