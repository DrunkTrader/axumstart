# has_many

One-to-many — same shape as [has_one](./has_one.md) (the foreign key lives on the target table,
pointing back at this one), but the target can have many matching rows per row here.

```rust
#[repository(table = "user")]
#[has_many(table = "order")]
pub trait UserRepository: Repository {
    // order_status → JOIN "order" ON "order".user_id = "user".id WHERE "order".status = $1
    async fn find_all_by_order_status(&self, status: String) -> sqlx::Result<Vec<UserRow>>;
}
```

**Only legal on [find_all_by_*](../dsl/find_all.md)** — a matching `order` row can multiply the
`user` row it JOINs against, which is only safe when the caller already expects a `Vec`. See
[Fan-out](./relations.md#fan-out-has_manybelongs_to_many-columns-are-find_all_by_-only) for why
this is a compile error everywhere else.

See [relations](./relations.md) for the full attribute table and defaults.
