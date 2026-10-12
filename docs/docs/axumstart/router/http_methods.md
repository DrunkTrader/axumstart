---
sidebar_position: 1
sidebar_label: HTTP methods
---

# HTTP methods

One attribute per HTTP method, taking the route path as its only argument:
`#[get]`, `#[post]`, `#[put]`, `#[delete]`, `#[patch]`, `#[head]`, `#[options]`, `#[trace]`,
`#[connect]`.

```rust
use axumstart::router::get;

#[get("/users/me")]
async fn current_user() -> &'static str {
    "me"
}
```

This submits a route registration via `inventory` — [`serve`](./serve.md) (or
`axumstart_router::register_routes` directly) folds every registered route onto the `Router`
at startup. There's no manual `.route("/users/me", get(current_user))` call to write or forget.

## Exactly one method attribute per handler

A handler needs **exactly one** of these attributes. Zero is just a plain function (not
registered as a route); more than one is a compile error:

```rust
#[get("/a")]
#[post("/a")]   // error: only one HTTP method attribute is allowed per handler
async fn conflicting() -> &'static str { "no" }
```

If you need the same logic reachable from two methods, write two small handlers that both call
a shared function, or route both to one handler manually via a hand-built `Router` merged in
elsewhere (see [`serve`](./serve.md) for how auto-registered and manual routes combine).

## Extractors work normally

The router state is always [`Components`](../components/components.md), so any extractor that
works against it — [`Inject<T>`](../components/components.md), `State<Components>`, or any
ordinary Axum extractor (`Json<T>`, `Query<T>`, path params, etc.) — works exactly as it would in
a hand-built `Router`:

```rust
use axum::extract::Path;
use axumstart::components::component;
use axumstart::router::{Inject, get};

struct Greeting(&'static str);

#[component] // registered the same way as any other component — see the components docs
fn build_greeting() -> Greeting {
    Greeting("hello")
}

#[get("/greet/{name}")]
async fn greet(Path(name): Path<String>, Inject(greeting): Inject<Greeting>) -> String {
    format!("{}, {name}", greeting.0)
}
```

## Combining with `#[layer(...)]`

See [`#[layer(...)]`](./layer.md) for attaching per-route middleware — it stacks with any of the
method attributes above, in either order.
