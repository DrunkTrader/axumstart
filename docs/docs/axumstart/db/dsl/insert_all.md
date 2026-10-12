# insert_all

`insert_all(&self, values: Vec<T>)` where `T` derives `#[derive(SqlxInsert)]` (see
[insert](./insert.md)) — a single multi-row `INSERT` for the whole batch, one round-trip.

```rust
async fn insert_all(&self, values: Vec<UserValues>) -> sqlx::Result<Vec<UserRow>>;
```

Void vs. row-returning works the same as [insert](./insert.md#void-vs-row-returning) — declare
`-> sqlx::Result<()>` to skip fetching the written rows back. An empty `Vec` short-circuits to
`Ok(vec![])` / `Ok(())` without touching the database.

## Per-dialect SQL

**Postgres/SQLite:** built with `sqlx::QueryBuilder::push_values` — `INSERT INTO "table" (cols...)
VALUES ($1,$2),($3,$4),... RETURNING *`.

**MySQL has no `RETURNING`.** The row-returning form does the batch `INSERT`, then re-`SELECT`s
by primary-key range: `SELECT * WHERE {key} >= first_id AND {key} < first_id + rows.len() ORDER
BY {key}`, relying on MySQL's **contiguous auto-increment allocation** for a single multi-row
insert statement (true under the default `innodb_autoinc_lock_mode`). If the table's `{key}`
isn't an auto-increment column, this will not return the right rows — use the `()`-returning form
or a method body instead in that case.
