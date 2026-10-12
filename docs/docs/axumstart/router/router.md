---
sidebar_position: 4
---

# Router

Auto-registers Axum routes from annotated handler functions — no hand-built `Router` required.
`#[get("/path")]`/`#[post(...)]`/etc. submit a route registration via `inventory`, the same
mechanism [components](../components/components.md) uses to auto-register DI components;
[`serve`](./serve.md) folds every registered route onto an `axum::Router` at startup.

`router` is a **default feature** (it implies `components`, since handlers extract DI components
via [`Inject<T>`](../components/components.md)) — most consumers don't need to touch Cargo.toml
at all:

```toml
axumstart = { version = "..." }
```

```rust
use axumstart::router::get;

#[get("/health")]
async fn health() -> &'static str {
    "ok"
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    axumstart::serve(3000).await
}
```

## Pieces

| Piece | Purpose |
|---|---|
| [HTTP methods](./http_methods.md) | `#[get]`/`#[post]`/`#[put]`/`#[delete]`/`#[patch]`/`#[head]`/`#[options]`/`#[trace]`/`#[connect]` — one per handler, path as the argument |
| [`#[layer(...)]`](./layer.md) | Attaches a `tower::Layer` to one route, optionally built from a registered [`Components`](../components/components.md) value |
| [`serve`](./serve.md) | Loads `.env`, builds the DI container, folds in every auto-registered route, binds and serves |

Handlers are ordinary Axum handlers — extractors like
[`Inject<T>`](../components/components.md) work exactly as they do in a hand-built `Router`,
since every auto-registered route shares the same `Components` state.
