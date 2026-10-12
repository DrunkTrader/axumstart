---
sidebar_position: 1
slug: /
---

# Quick start

axumstart is a crate built around [Axum](https://github.com/tokio-rs/axum) that
provides libraries to help you get started building rust web apps

## Installation

```toml
[dependencies]
axumstart = { version = "..." }
```

## Example

A config component, a struct component that depends on it, and a route — the three pieces most
apps start with:

```rust
use axumstart::components::{Component, ComponentConfig};
use axumstart::router::{Inject, get};
use axum::extract::Path;
use std::sync::Arc;

// Config component — reads GREETING from the environment (or .env), falls back to "Hello".
#[derive(ComponentConfig)]
struct GreetingConfig {
    #[env_var("GREETING", "Hello".to_string())]
    greeting: String,
}

// Struct component — depends on GreetingConfig, resolved automatically.
#[derive(Component)]
struct Greeter {
    config: Arc<GreetingConfig>,
}

// Route — auto-registered, no Router to build by hand.
#[get("/greet/{name}")]
async fn greet(Path(name): Path<String>, Inject(greeter): Inject<Greeter>) -> String {
    format!("{}, {name}!", greeter.config.greeting)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    axumstart::serve(3000).await
}
```

`GET /greet/world` returns `Hello, world!` (or whatever `GREETING` is set to). See
[Components](./components/components.md) and [Router](./router/router.md) for everything else
each piece supports.

