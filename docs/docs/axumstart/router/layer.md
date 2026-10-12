---
sidebar_position: 2
sidebar_label: "#[layer(...)]"
---

# `#[layer(...)]`

Attaches a `tower::Layer` to **one route** — via `MethodRouter::layer`, not `Router::layer` — so
it's completely independent of every other auto-registered route. Must be paired with exactly
one [HTTP method attribute](./http_methods.md) on the same function; the two can be written in
either order.

```rust
use axumstart::router::{get, layer};
use tower_http::trace::TraceLayer;

#[layer(TraceLayer::new_for_http())]
#[get("/traced")]
async fn traced() -> &'static str {
    "traced"
}
```

## Two forms for the argument

- **A ready expression** — anything that isn't a bare function name, spliced in directly as
  `.layer(expr)`. `TraceLayer::new_for_http()` above is this form.
- **A bare function name** — called *for you*. The function may take no arguments, or a single
  `&Components` if it needs to build the layer from a registered component (e.g. a secret or
  config value):

```rust
use axumstart::router::{Components, get, layer};
use tower_http::set_header::SetResponseHeaderLayer;
use axum::http::{HeaderName, HeaderValue};

fn request_id_layer() -> SetResponseHeaderLayer<HeaderValue> {
    SetResponseHeaderLayer::overriding(
        HeaderName::from_static("x-request-id"),
        HeaderValue::from_static("fixed-for-example"),
    )
}

#[layer(request_id_layer)]
#[get("/a")]
async fn a() -> &'static str { "a" }

fn cors_from_config(ctx: &Components) -> SetResponseHeaderLayer<HeaderValue> {
    let config = ctx.get_cloned::<CorsConfig>().unwrap();
    SetResponseHeaderLayer::overriding(
        HeaderName::from_static("access-control-allow-origin"),
        HeaderValue::from_str(&config.allowed_origin.0).unwrap(),
    )
}

#[derive(Clone, ComponentConfig)] 
struct CorsConfig {
  #[env_var("ALLOWED_ORIGIN", "http://localhost:8080".to_string())]
  pub allowed_origin: String
}

#[get("/b")]
#[layer(cors_from_config)]
async fn b() -> &'static str { "b" }
```

Which of the two zero/one-argument forms applies is picked automatically (the same autoref
trick `axumstart_components`'s `#[component]` uses to detect `OnCreate`/`as_trait`) — you don't
annotate which one you meant.

## Stacking multiple layers

Repeat `#[layer(...)]`; they apply in the order written:

```rust
#[layer(first_layer)]
#[layer(second_layer)]
#[get("/stacked")]
async fn stacked() -> &'static str { "stacked" }

fn first_layer() -> tower::layer::util::Identity { tower::layer::util::Identity::new() }
fn second_layer() -> tower::layer::util::Identity { tower::layer::util::Identity::new() }
```

## Without a method attribute

`#[layer(...)]` on its own, with no `#[get]`/`#[post]`/etc. on the same function, is a compile
error — there's no route for it to attach to:

```text
error: #[layer(...)] requires exactly one #[get]/#[post]/#[put]/#[delete]/#[patch]/#[head]/
#[options]/#[trace]/#[connect] attribute on the same function
```

## Global middleware

`#[layer(...)]` is for *this one route only*. For middleware that should apply to the whole app
(tracing, CORS, compression, a global timeout) regardless of how routes got registered, that's a
property of the finished `Router`, not of an individual handler — [`serve`](./serve.md) doesn't
expose a hook for that today, so build your own `main` with `axumstart_router::register_routes`
directly and call `.layer(...)` on the result before serving it yourself.
