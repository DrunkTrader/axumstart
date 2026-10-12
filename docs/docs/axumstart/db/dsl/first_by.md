# first_by

`first_by_{filter}` — fetches the first row matching the [filter grammar](./dsl.md#shared-filter-grammar).
The generated SQL always ends in `LIMIT 1`. If the filter isn't actually unique, "first" means
whichever row the database happens to pick — add [`#[order_by(...)]`](#ordering) to make that
deterministic (and, combined with `LIMIT 1`, let the database use an index on the ordered column
instead of sorting every matching row).

```rust
async fn first_by_username(&self, username: String) -> sqlx::Result<Option<UserRow>>;
// SELECT * FROM "user" WHERE username = $1 LIMIT 1
```

## Fetch mode — inferred from the return type

| Return type | sqlx call |
|---|---|
| `sqlx::Result<Option<T>>` | `fetch_optional` |
| anything else (e.g. `sqlx::Result<T>`) | `fetch_one` |

`-> sqlx::Result<Vec<T>>` is a **compile error** here — `first_by_*` always adds `LIMIT 1`, so a
`Vec<T>` would only ever hold zero or one element, which is a misleading type for a name that
implies "at most one." Use [find_all_by_*](./find_all.md) for a real list instead; it also gets
ordering suffixes, `Page` pagination, `_this_week`, and fan-out relation columns, none of which
`first_by_*` parses.

## Relations

A filter field may reference a joined table via `{relation_table}_{column}` (see
[relations](../relations/relations.md)) — except columns reached through a `has_many` or
`belongs_to_many` relation, which can match more than one row per base-table row and are
rejected at compile time on `first_by_*`. Use [find_all](./find_all.md) for those.

```rust
#[repository(table = "user")]
#[belongs_to(table = "team")]
pub trait UserRepository: Repository {
    // JOIN "team" ON "team".id = "user".team_id WHERE "team".name = $1 AND "user".username = $2 LIMIT 1
    async fn first_by_team_name_and_username(
        &self,
        team_name: String,
        username: String,
    ) -> sqlx::Result<Option<UserRow>>;
}
```

`team.name` alone wouldn't be a good fit here — many users can share a team, so filtering on it
by itself would just return an arbitrary member of that team. Pairing it with `username` keeps
the relation-filter demo while actually pinning down one row.

## Ordering

`#[order_by(...)]` is supported here (see [dsl](./dsl.md#order_bycol1-col2)):

```rust
#[order_by("elo")]
async fn first_by_team_id_ordered(&self, team_id: i32) -> sqlx::Result<Option<UserRow>>;
// SELECT * FROM "user" WHERE team_id = $1 ORDER BY elo LIMIT 1
```

## Errors caught at compile time

- Parameter count not matching the fields named in the method (arity mismatch).
- A filter column that doesn't exist on the return type's row struct (unless
  `#[unchecked_columns]` is set).
- A filter column reached through a `has_many`/`belongs_to_many` relation.
- A `Vec<T>` return type.
