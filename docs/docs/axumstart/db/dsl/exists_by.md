# exists_by

`exists_by_{filter}` — same [filter grammar](./dsl.md#shared-filter-grammar) as
[first_by](./first_by.md), returning whether any row matches:

```rust
async fn exists_by_email(&self, email: String) -> sqlx::Result<bool>;
// SELECT EXISTS(SELECT 1 FROM "user" WHERE email = $1)
```

Fetched with `query_scalar`.

**Not supported here:** `#[order_by(...)]` (a compile error if present).
