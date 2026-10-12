# find_by

`find_by_{filter}` — fetches a single row matching the [filter grammar](./dsl.md#shared-filter-grammar).

```rust
async fn find_by_username(&self, username: String) -> sqlx::Result<Option<UserRow>>;
// SELECT * FROM "user" WHERE username = $1
```

## Fetch mode — inferred from the return type

| Return type | sqlx call |
|---|---|
| `sqlx::Result<Option<T>>` | `fetch_optional` |
| anything else (e.g. `sqlx::Result<T>`) | `fetch_one` |

`-> sqlx::Result<Vec<T>>` is a **compile error** here — `find_by_*` never adds a `LIMIT`, so
returning `Vec<T>` would silently fetch every matching row under a name that implies "at most
one". Use [find_all_by_*](./find_all.md) for a list instead; it also gets ordering suffixes,
`Page` pagination, `_this_week`, and fan-out relation columns, none of which `find_by_*` parses.

## Relations

A filter field may reference a joined table via `{relation_table}_{column}` (see
[relations](../relations/relations.md)) — except columns reached through a `has_many` or
`belongs_to_many` relation, which can match more than one row per base-table row and are
rejected at compile time on `find_by_*`. Use [find_all](./find_all.md) for those.

```rust
#[belongs_to(table = "team")]
pub trait UserRepository: Repository {
    // JOIN "team" ON "team".id = "user".team_id WHERE "team".name = $1
    async fn find_by_team_name(&self, name: String) -> sqlx::Result<Option<UserRow>>;
}
```

## Ordering

`#[order_by(...)]` is supported here (see [dsl](./dsl.md#order_bycol1-col2)):

```rust
#[order_by("elo")]
async fn find_by_team_id_ordered(&self, team_id: i32) -> sqlx::Result<Option<UserRow>>;
```

## Errors caught at compile time

- Parameter count not matching the fields named in the method (arity mismatch).
- A filter column that doesn't exist on the return type's row struct (unless
  `#[unchecked_columns]` is set).
- A filter column reached through a `has_many`/`belongs_to_many` relation.
- A `Vec<T>` return type.
