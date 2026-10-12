# upsert

`upsert(&self, values: T)` where `T` derives `#[derive(SqlxInsert)]` (see [insert](./insert.md))
— insert, or update in place on conflict. Requires `#[unique(col)]` on the method to name the
conflict column:

```rust
#[unique(username)]
async fn upsert(&self, values: UserValues) -> sqlx::Result<UserRow>;
```

Leaving off `#[unique(...)]` is a compile error. `col` is checked at compile time against `T`'s
fields, same as any other DSL column reference.

**Always row-returning** — unlike `insert`/`insert_all`/`upsert_all`, there's no `()`-returning
form of single-row `upsert`.

## Per-dialect SQL

**Postgres/SQLite:**
```sql
INSERT INTO "table" (cols...) VALUES ($1, ...)
ON CONFLICT (conflict_col) DO UPDATE SET other_col = EXCLUDED.other_col, ...
RETURNING *
```
Every column except the conflict column is re-set from `EXCLUDED`.

**MySQL has no `ON CONFLICT`/`RETURNING`.** Instead:
```sql
INSERT INTO `table` (cols...) VALUES (?, ...)
ON DUPLICATE KEY UPDATE col = VALUES(col), ...   -- every column, including the conflict column
```
followed by `` SELECT * FROM `table` WHERE {conflict_col} = ? ``. MySQL infers which unique key was
violated automatically (it isn't named the way Postgres/SQLite's `ON CONFLICT` target is), so the
generated code picks which field to bind into that follow-up `SELECT` based on the runtime value
of `conflict_col` — though since `#[unique(col)]` is itself compile-time checked against `T`'s
fields, this always resolves to a real field in practice.

See [insert](./insert.md#field-requirements) for the MySQL `Clone` requirement on every field.

## See also

[upsert_all](./upsert_all.md) — the batch form.
