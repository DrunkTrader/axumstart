---
sidebar_position: 2
sidebar_label: Struct Components
---

# Struct Components

## `#[derive(Component)]`

The most common path. Each named field is resolved from the `ComponentProvider`:

- `Arc<T>` fields resolve via `ctx.get::<T>().await` (shared with every other consumer of `T`).
- Any other field resolves via `ctx.get_cloned::<T>().await` (requires `T: Clone`).
- A field marked `#[default]` is built with `Default::default()` instead, skipping the
  container entirely.

```rust
use axumstart_components::Component;
use std::sync::Arc;

#[derive(Component)]
struct Database {
    pool: Arc<DbPool>,
    #[default]
    query_count: AtomicU64,
}
```

This expands to a `ComponentBlueprint for Database` impl and an `inventory` registration, so it
participates in [`register_all()`](./component_provider.md#registering-everything-at-once-inventory)
automatically.

## Registering trait objects — `#[as_trait(Trait)]`

Add `#[as_trait(Trait)]` to also register the component as `Arc<dyn Trait>`:

```rust
trait Greeter: Send + Sync {
  fn greet(&self) -> &'static str;
}

#[derive(Component)]
#[as_trait(Greeter)]
pub struct EnglishGreeter;

impl Greeter for EnglishGreeter {
  fn greet(&self) -> &'static str { "hi" }
}

// later:
pub async fn welcome(greeter: Inject<dyn Greeter>) -> &'static str {
  greeter.greet()
}
```

Concrete and trait-object registrations resolve to the same underlying instance.

