# update

`update(&self, values: T)` where `T` derives `#[derive(SqlxUpdate)]` — updates one row by a key
field, setting every other field.

```rust
#[derive(SqlxUpdate)]
pub struct UserStatsUpdate {
    #[key]
    pub user_id: i32,
    pub self_acceptance: i32,
}

async fn update(&self, values: UserStatsUpdate) -> sqlx::Result<UserStatsRow>;
```

Exactly one field must be marked `#[key]` — it becomes the `WHERE` target and is excluded from
`SET`; every other field becomes a `SET` column. Zero or more than one `#[key]` field is a
compile error, and so is a struct with no non-key fields (nothing to `SET`).

**Always row-returning** — there's no `()`-returning form of `update`.

## Per-dialect SQL

**Postgres/SQLite:** `UPDATE "table" SET col = $1, ... WHERE key = $n RETURNING *`.

**MySQL has no `RETURNING`.** `` UPDATE `table` SET col = ?, ... WHERE key = ? `` followed by
`` SELECT * FROM `table` WHERE key = ? `` on the same executor — the same `Copy`-executor
requirement described in [dsl](./dsl.md#transactional), so `update` can't be combined with
`#[transactional]` under the `mysql` feature.

## What this is for vs. `set_by`

`update` takes a whole values struct and updates **exactly one row**, keyed by `#[key]`. Use
[set_by](./set_by.md) instead for updating a fixed set of columns across **many rows** matched by
an arbitrary filter, without a values struct.
