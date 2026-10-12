# find_all

Four related patterns, all `fetch_all`, all returning `Vec<T>` (or `sqlx::Result<()>` isn't
meaningful here — these always return rows):

| Method name | SQL |
|---|---|
| `find_all_by_{filter}` | `SELECT * FROM "table" WHERE {filter}` |
| `find_all` (bare) | `SELECT * FROM "table"` |
| `find_all_order_by_{col}[_asc\|_desc]` | `SELECT * FROM "table" ORDER BY {col} [ASC\|DESC]` |
| `find_all_{bool_field}` (no `_by_`) | `SELECT * FROM "table" WHERE {bool_field} = TRUE` |

```rust
async fn find_all_by_team_id(&self, team_id: i32) -> sqlx::Result<Vec<UserRow>>;
async fn find_all(&self) -> sqlx::Result<Vec<UserRow>>;
async fn find_all_order_by_created_at_desc(&self) -> sqlx::Result<Vec<UserRow>>;
async fn find_all_verified(&self) -> sqlx::Result<Vec<UserRow>>; // WHERE verified = TRUE
```

`find_all_{bool_field}` takes **no bound parameter** — the field is a fixed boolean flag, not a
filter value. It also gets an extra compile-time check beyond ordinary column probing: the field
must actually be `bool`-typed on the row struct, not merely present (a non-bool column would
otherwise only fail at query time).

## Relations — the one place fan-out is allowed

Unlike every other filtered method, `find_all_by_*` **is** allowed to filter through a `has_many`
or `belongs_to_many` relation (see [relations](../relations/relations.md#fan-out-has_manybelongs_to_many-columns-are-find_all_by_-only)) —
a matching row on the far side can legitimately multiply the result set here, since the return
type is already `Vec`.

```rust
#[has_many(table = "order")]
pub trait UserRepository: Repository {
    async fn find_all_by_order_status(&self, status: String) -> sqlx::Result<Vec<UserRow>>;
}
```

## Ordering — two ways, not both

**Embedded in the name**, appended after the filter portion: `_order_by_{col}`, optionally
suffixed `_asc`/`_desc` (defaults to no explicit direction):

```rust
async fn find_all_by_team_id_order_by_elo_desc(&self, team_id: i32) -> sqlx::Result<Vec<UserRow>>;
// SELECT * FROM "user" WHERE team_id = $1 ORDER BY elo DESC
```

**`#[order_by("col1", "col2")]`**, for a fixed order not derived from the name, or multiple
columns (see [dsl](./dsl.md#order_bycol1-col2)) — the method name must end `_ordered`:

```rust
#[order_by("elo", "created_at")]
async fn find_all_by_team_id_ordered(&self, team_id: i32) -> sqlx::Result<Vec<UserRow>>;
```

Combining both on the same method is a compile error — pick one.

## `_this_week`

`find_all_by_*_this_week` filters to the current ISO week (Monday start, via the backend-specific
`axumstart_db::WEEK_START_SQL` boundary expression), appended after any filter:

```rust
async fn find_all_by_team_id_this_week(&self, team_id: i32) -> sqlx::Result<Vec<UserRow>>;
// SELECT * FROM "user" WHERE team_id = $1 AND created_at >= <week start>
```

Filters on a column other than `created_at` with `#[created_at(col)]`:

```rust
#[created_at(submitted_at)]
async fn find_all_by_team_id_this_week(&self, team_id: i32) -> sqlx::Result<Vec<UserRow>>;
```

**Cross-dialect caveat:**
- **Postgres/MySQL:** works against a native `TIMESTAMP`/`DATETIME` column.
- **SQLite:** has no native timestamp type. `WEEK_START_SQL` produces an ISO8601 date string,
  which only compares correctly against a column storing ISO8601 **TEXT** (e.g. `created_at TEXT
  DEFAULT CURRENT_TIMESTAMP`). If your SQLite schema stores `created_at` as an INTEGER unix
  timestamp instead, the comparison silently returns no rows — SQLite compares INTEGER-affinity
  columns and TEXT literals by type, not value. Use a method body instead in that case.

Exact edge-boundary behavior (timezone handling, week-start inclusivity) isn't guaranteed to be
bit-identical across the three backends — pick one backend per deployment and rely on that.

## `Page` — pagination

A `Page`-typed parameter appends `LIMIT $n OFFSET $m` (see [dsl](./dsl.md#page--pagination)), on
any of the four variants above:

```rust
async fn find_all_by_team_id(&self, team_id: i32, page: Page) -> sqlx::Result<Vec<UserRow>>;
```
