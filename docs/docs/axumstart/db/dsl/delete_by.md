# delete_by

`delete_by_{filter}` — same [filter grammar](./dsl.md#shared-filter-grammar) as
[first_by](./first_by.md):

```rust
async fn delete_by_id(&self, id: i32) -> sqlx::Result<()>;
// DELETE FROM "user" WHERE id = $1
```

Always returns `sqlx::Result<()>` — the row count isn't surfaced.

**Cannot filter through a relation.** Unlike `first_by_*`, a `delete_by_*` filter can't reference
a joined table's columns at all (even `belongs_to`'s non-JOIN FK shortcut is fine, but anything
requiring an actual `JOIN` is rejected) — SQL's `DELETE` doesn't support joining against other
tables the way `SELECT`/`UPDATE` can here. This is a compile error:

```rust
#[repository(table = "user")]
#[belongs_to(table = "team")]
pub trait UserRepository: Repository {
    // ERROR: `delete_by_*` cannot filter through joined tables
    async fn delete_by_team_name(&self, name: String) -> sqlx::Result<()>;
    // fine — team_id is the belongs_to FK column itself, no JOIN needed
    async fn delete_by_team_id(&self, team_id: i32) -> sqlx::Result<()>;
}
```

For a delete that genuinely needs to filter on a joined column, write the method body by hand
(e.g. `DELETE FROM "user" WHERE id IN (SELECT ...)`).

**Not supported here:** `#[order_by(...)]` (a compile error if present).
