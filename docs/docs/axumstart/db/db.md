---
sidebar_position: 3
---

# Db

Generates a repository implementation from a trait definition: annotate the trait, name your
methods following a small set of conventions, and the SQL (Postgres, MySQL, or SQLite — one
backend, chosen at compile time) gets written for you.

## Picking a backend

Exactly one of the `postgres` / `mysql` / `sqlite` Cargo features must be enabled. This is a
compile-time choice because SQL placeholder syntax and `RETURNING` availability are baked into
generated queries at macro-expansion time — enabling zero or more than one is a compile error.

```toml
axumstart = { version = "...", features = ["postgres"] }
```

## `#[repository(table = "...")]`

Annotate a trait to generate:
- a `Db{TraitName}` struct with a `pool: DbPool` field and `new(pool: DbPool) -> Self`,
- `impl {TraitName} for Db{TraitName}`, with method bodies generated from method *names*.

```rust
#[repository(table = "user_stats")]
pub trait UserStatsRepository: Repository {
    async fn first_by_user_id(&self, user_id: i32) -> sqlx::Result<Option<UserStatsRow>>;
    async fn insert(&self, values: UserStatsValues) -> sqlx::Result<UserStatsRow>;
}
```

Methods with a **default body** are emitted as-is (no codegen applied). A method name that
matches none of the patterns below is a **compile error** naming the method and listing the
valid patterns.

## Method name conventions

See [dsl](./dsl/dsl.md) for the full grammar (operators, connectors, ordering, pagination,
`#[transactional]`, and more) — this is the map:

| Function type | Pattern |
|---|---|
| [first_by](./dsl/first_by.md) | `first_by_{filter}` |
| [find_random_by](./dsl/find_random_by.md) | `find_random_by_{filter}` |
| [find_all](./dsl/find_all.md) | `find_all_by_{filter}`, `find_all`, `find_all_order_by_{col}`, `find_all_{bool_field}` |
| [count_by](./dsl/count_by.md) | `count_by_{filter}` |
| [exists_by](./dsl/exists_by.md) | `exists_by_{filter}` |
| [delete_by](./dsl/delete_by.md) | `delete_by_{filter}` |
| [set_by](./dsl/set_by.md) | `set_{field}_by_{filter}` |
| [insert](./dsl/insert.md) | `insert` |
| [insert_all](./dsl/insert_all.md) | `insert_all` |
| [insert_or_ignore](./dsl/insert_or_ignore.md) | `insert_or_ignore` |
| [upsert](./dsl/upsert.md) | `upsert` |
| [upsert_all](./dsl/upsert_all.md) | `upsert_all` |
| [update](./dsl/update.md) | `update` |
| [pool](./dsl/pool.md) | `pool` |

## Relations — `belongs_to` / `has_one` / `has_many` / `belongs_to_many`

Declare related tables so method names can reference their columns — see
[relations](./relations/relations.md).

## `#[derive(SqlxInsert)]` / `#[derive(SqlxUpdate)]`

The values-struct side of `insert`/`insert_all`/`insert_or_ignore`/`upsert`/`upsert_all`
([`#[derive(SqlxInsert)]`](./dsl/insert.md)) and `update`
([`#[derive(SqlxUpdate)]`](./dsl/update.md)) — see each method's page for the exact derive
mechanics and per-dialect SQL.

## Combining with `mockall`

Add `mock` to the macro to generate `#[cfg_attr(test, ::mockall::automock)]` on the trait:

```rust
#[repository(table = "onboarding_test", mock)]
pub trait OnboardingTestRepository: Repository { /* ... */ }
```
