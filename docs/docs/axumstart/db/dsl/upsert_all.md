# upsert_all

`upsert_all(&self, values: Vec<T>)` where `T` derives `#[derive(SqlxInsert)]` (see
[insert](./insert.md)) — the batch form of [upsert](./upsert.md), one round-trip for the whole
batch. Also requires `#[unique(col)]`:

```rust
#[unique(username)]
async fn upsert_all(&self, values: Vec<UserValues>) -> sqlx::Result<Vec<UserRow>>;
```

Unlike single-row `upsert`, this **does** support a `()`-returning form — declare `->
sqlx::Result<()>` to skip fetching the written rows back. An empty `Vec` short-circuits without
touching the database.

## Per-dialect SQL

**Postgres/SQLite:** built with `sqlx::QueryBuilder::push_values`, same as
[insert_all](./insert_all.md#per-dialect-sql):
```sql
INSERT INTO "table" (cols...) VALUES ($1,$2),($3,$4),...
ON CONFLICT (conflict_col) DO UPDATE SET other_col = EXCLUDED.other_col, ...
RETURNING *
```
Rebuilt per call (row count varies), so — unlike single-row `upsert` — there's no cached SQL
string here.

**MySQL** batch-inserts with `ON DUPLICATE KEY UPDATE`, then selects the written rows back by
`WHERE {conflict_col} IN (v1, v2, ...)`:
```sql
INSERT INTO `table` (cols...) VALUES (?, ?),(?, ?),...
ON DUPLICATE KEY UPDATE col = VALUES(col), ...
-- then:
SELECT * FROM `table` WHERE {conflict_col} IN (?, ?, ...)
```

This differs from [insert_all](./insert_all.md#per-dialect-sql)'s MySQL strategy: `insert_all`
selects back by a contiguous auto-increment ID range, which only works because every row is a
fresh insert. A batch upsert mixes fresh inserts with in-place updates, so there's no such ID
range — but the conflict column's values are already known *before* the write (they're struct
fields on `T`, not DB-assigned), so selecting back by `IN (...)` on that column works regardless
of which rows were inserted vs. updated.
