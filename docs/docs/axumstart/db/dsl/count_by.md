# count_by

`count_by_{filter}` — same [filter grammar](./dsl.md#shared-filter-grammar) as
[first_by](./first_by.md), returning a row count:

```rust
async fn count_by_team_id(&self, team_id: i32) -> sqlx::Result<i64>;
// SELECT COUNT(*) FROM "user" WHERE team_id = $1
```

Fetched with `query_scalar`.

**Not supported here:** `#[order_by(...)]` (a compile error if present) — ordering a count is
meaningless.
