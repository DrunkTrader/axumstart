# pool

`pool(&self) -> &DbPool` — an exact method named `pool` returns the repository's underlying
connection pool directly, with no query generated:

```rust
async fn pool(&self) -> &DbPool;
```

Useful for dropping down to a raw `sqlx::query(...)` call for something the DSL doesn't cover,
without leaving the trait. There's no `db` alias — only the exact name `pool` is recognized.
