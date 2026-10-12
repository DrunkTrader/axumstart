---
sidebar_position: 3
sidebar_label: Function Components
---

# Function Components

## Factory functions — `#[component]`

For construction logic that doesn't fit a plain struct literal (or types you don't own), write
a free function instead:

```rust
use axumstart_components::{component, ComponentProvider};

#[component]
fn build_http_client() -> reqwest::Client {
    reqwest::Client::new()
}

#[component]
async fn build_thing(ctx: &ComponentProvider) -> Thing {
    let db: Arc<Database> = ctx.get().await;
    Thing::new(db)
}
```

Rules: at most one parameter, and it must be `&ComponentProvider`; the function must return the
constructed type (async or sync, both work).

Like [`#[derive(Component)]`](./struct_components.md#derivecomponent), this expands to a
`ComponentBlueprint` impl plus an `inventory` registration, so it participates in
[`register_all()`](./component_provider.md#registering-everything-at-once-inventory)
automatically.
