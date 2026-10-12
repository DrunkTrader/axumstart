---
sidebar_position: 5
sidebar_label: On Create
---

# On Create

`OnCreate` runs logic exactly once, right after a component is first constructed — useful for
spawning background tasks or kicking off warm-up work:

```rust
use axumstart_components::{async_trait, ComponentProvider, OnCreate};

#[async_trait]
impl OnCreate for Worker {
    async fn on_create(&self, ctx: &ComponentProvider) {
        // runs once, before any caller observes the resolved instance
    }
}
```

`#[derive(Component)]`/`#[component]` detect a hand-written `OnCreate` impl automatically and route to the right
`register_*` call when [`register_all()`](./component_provider.md#registering-everything-at-once-inventory)
runs.
