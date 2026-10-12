# insert_or_ignore

`insert_or_ignore(&self, values: T)` where `T` derives `#[derive(SqlxInsert)]` (see
[insert](./insert.md)) — inserts, silently doing nothing if it would conflict.

```rust
async fn insert_or_ignore(&self, values: UserValues) -> sqlx::Result<()>;
```

Always returns `sqlx::Result<()>` — there's no row-returning form (a conflicting insert has no
row to return, since nothing was written).

## Per-dialect SQL

- **Postgres:** `INSERT INTO "table" (...) VALUES (...) ON CONFLICT DO NOTHING`
- **SQLite:** same, `ON CONFLICT DO NOTHING` (bare, no conflict target needed)
- **MySQL:** `` INSERT IGNORE INTO `table` (...) VALUES (...) ``
