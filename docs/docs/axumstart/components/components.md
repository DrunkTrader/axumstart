---
sidebar_position: 2
sidebar_label: Components
---

# Components

An async dependency-injection container. Components are registered as *blueprints* (a recipe
for constructing `T`) and can be pulled out of Axum
handlers with the [Inject](./axum_extractor.md) extractor.

## Core types

| Type | Purpose                                                                                     |
|---|---------------------------------------------------------------------------------------------|
| `ComponentProvider` | Registration-time container. Register blueprints, then call `.build()`.                     |
| `Components` | Immutable, `Clone`-cheap snapshot of every built component — this should be axum app state. |
| `Inject<T>` | `FromRequestParts<Components>` extractor; derefs to `&T`.                                   |

```rust
use axumstart_components::{ComponentProvider, Component};

#[derive(Component)] // automatically registers blueprint
struct Settings {
    #[default]
    port: u16,
}

let provider = ComponentProvider::default();
provider.register::<Settings>();

let components = provider.build().await; // resolves every registered blueprint
let settings = components.get::<Settings>().unwrap();
```

## Registering a component

Three ways to define one:

- [Struct components](./struct_components.md) — `#[derive(Component)]` (most common) and
  trait-object registration via `#[as_trait(Trait)]`.
- [Function components](./function_components.md) — `#[component]` on a free function, for
  construction logic that doesn't fit a plain struct literal.
- [Config components](./config_components.md) — `#[derive(ComponentConfig)]` for env var configuration

