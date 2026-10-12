# pool

`pool(&self) -> &DbPool` — an exact method named `pool` returns the repository's underlying
connection pool directly, with no query generated:

```rust
async fn pool(&self) -> &DbPool;
```

Useful for dropping down to a raw `sqlx::query(...)` call for something the DSL doesn't cover,
without leaving the trait.

```rust
#[repository(table = "message")]
pub trait MessageRepository: Repository {
  async fn first_by_id_raw(&self, id: i32) -> sqlx::Result<Option<Message>> {
    sqlx::query_as("SELECT * FROM message m WHERE m.id = $1")
      .bind(id)
      .fetch_optional(self.pool())
      .await
  }
}
```
