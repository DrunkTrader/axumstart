# upsert

`upsert(&self, values: T)` where `T` derives `#[derive(SqlxInsert)]` (see [insert](./insert.md))
— insert, or update in place on conflict. Requires `#[unique(col)]` on the method to name the
conflict column:

```rust
#[unique(username)]
async fn upsert(&self, values: UserValues) -> sqlx::Result<UserRow>;
```

Leaving off `#[unique(...)]` is a compile error. `col` is checked at compile time against `T`'s
fields, same as any other DSL column reference — but that only confirms the field *exists*, not
that the database treats it as unique. **You still have to add the actual constraint** (a `UNIQUE`
column/index or `PRIMARY KEY`) in your migration/schema; the macro has no way to see that it's
missing.

- **Postgres/SQLite:** `ON CONFLICT (col)` names a specific conflict target. If `col` has no
  matching unique or exclusion constraint, every `upsert` call fails at runtime with something
  like `there is no unique or exclusion constraint matching the ON CONFLICT specification` —
  the first time you actually hit a conflict, not at compile time.
- **MySQL:** `ON DUPLICATE KEY UPDATE` doesn't name a column at all — it fires on *any* unique-key
  violation on the table. If `col` isn't actually backed by a unique key, there's no error; the
  statement just inserts a fresh duplicate row instead of updating, which is easy to miss since
  nothing fails.

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
