---
sidebar_position: 1
sidebar_label: Component Provider
---

# Component Provider

`ComponentProvider` is the registration-time container; `Components` is the immutable snapshot
you get from `.build()` — this page covers both, registering everything at once, and resolving
components. For how to *define* a component in the first place, see
[struct components](./struct_components.md) / [function components](./function_components.md).
For the `OnCreate` lifecycle hook and Axum integration, see [on create](./on_create.md) and
[axum extractor](./axum_extractor.md).

## Registering everything at once: `inventory`

Every `#[derive(Component)]`, `#[component]`, and `#[derive(ComponentConfig)]` submits itself to
a global `inventory` registry at compile time. Instead of calling `ctx.register::<T>()` for each
type by hand, call:

```rust
let ctx = ComponentProvider::default();
ctx.register_all();
let components = ctx.build().await;
```

This walks every collected registration and wires it up — the common case for an application
entry point. Manual `ComponentBlueprint` impls (not using the derive/attribute macros) are not
picked up by `register_all()` and must be registered explicitly with `ctx.register::<T>()`.

## Resolving components

- `ctx.get::<T>().await -> Arc<T>` — resolves (and caches) during the registration phase.
- `ctx.get_cloned::<T>().await -> T` — same, then clones out of the `Arc` (`T: Clone`).
- After `ctx.build().await` produces a `Components` snapshot: `components.get::<T>()`,
  `components.get_cloned::<T>()` — both return `Option`, no `.await` needed.

## Lifecycle: `OnCreate`

See [on_create](./on_create.md) for running logic exactly once, right after a component is first
constructed.

## Axum integration

See [axum_extractor](./axum_extractor.md) for using `Components` as Axum router state and
pulling dependencies out of handlers with `Inject<T>`.
