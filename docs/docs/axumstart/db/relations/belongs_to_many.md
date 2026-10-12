# belongs_to_many

Many-to-many through a pivot table. `through` is required and names the pivot table:

```rust
#[repository(table = "post")]
#[belongs_to_many(table = "tag", through = "post_tag")]
pub trait PostRepository: Repository {
    // tag_name → JOIN "post_tag" ON "post_tag".post_id = "post".id
    //            JOIN "tag" ON "tag".id = "post_tag".tag_id
    //            WHERE "tag".name = $1
    async fn find_all_by_tag_name(&self, name: String) -> sqlx::Result<Vec<PostRow>>;
}
```

`fk` (default `"{this_table}_id"`, e.g. `post_id`) is this table's FK column *in the pivot table*;
`other_fk` (default `"{table}_id"`, e.g. `tag_id`) is the target table's FK column in the pivot.
Override either explicitly if the pivot's column names don't follow that convention:

```rust
#[belongs_to_many(table = "tag", through = "post_tag", fk = "post_pk", other_fk = "tag_pk")]
```

**Only legal on [find_all_by_*](../dsl/find_all.md)** — same reasoning as
[has_many](./has_many.md): a matching pivot row can multiply the base row, see
[Fan-out](./relations.md#fan-out-has_manybelongs_to_many-columns-are-find_all_by_-only).

See [relations](./relations.md) for the full attribute table and defaults.
