---
sidebar_position: 1
---

# Relations

Declare related tables so method names in a [`#[repository]`](../db.md) trait can reference their
columns. Stack one relation attribute per relation **below** `#[repository(...)]` — order matters,
see [Attribute ordering](#attribute-ordering) below. A trait may declare several relations, of the
same or different kinds, as long as each targets a distinct table (a relation's `table` value
doubles as the DSL prefix matched in method names, e.g. `{table}_{col}` — so two relations
pointing at the same target table would collide and aren't supported).

| Kind | Params | Defaults | JOIN emitted | FK shortcut (`{table}_id`, no JOIN)? |
|---|---|---|---|---|
| [belongs_to](./belongs_to.md)`(table, fk?)` | `fk` = FK column on **this** (owning) table | `fk = "{table}_id"` | `JOIN "{table}" ON "{table}".id = "{this_table}".{fk}` | **Yes** — `{table}_id` resolves straight to the `fk` column on this table, no JOIN |
| [has_one](./has_one.md)`(table, fk?)` | `fk` = FK column on the **target** table, pointing back here | `fk = "{this_table}_id"` | `JOIN "{table}" ON "{table}".{fk} = "{this_table}".id` | No |
| [has_many](./has_many.md)`(table, fk?)` | same shape as `has_one` | same as `has_one` | same as `has_one` | No |
| [belongs_to_many](./belongs_to_many.md)`(table, through, fk?, other_fk?)` | `through` (**required**) = pivot table name; `fk` = this table's FK column in the pivot; `other_fk` = target table's FK column in the pivot | `fk = "{this_table}_id"`, `other_fk = "{table}_id"` | `JOIN "{through}" ON "{through}".{fk} = "{this_table}".id JOIN "{table}" ON "{table}".id = "{through}".{other_fk}` | No |

`{this_table}` is the `table = "..."` value passed to `#[repository(...)]` itself. There's no
separate relation-name/alias parameter — the `table` value is simultaneously the real SQL table
name and the DSL prefix matched in method names.

## Fan-out: `has_many`/`belongs_to_many` columns are `find_all_by_*`-only

`has_many` and `belongs_to_many` can each match *multiple* rows on the far side of the join —
unlike `belongs_to`/`has_one`, which are always many-to-one or one-to-one and can never multiply
rows. Concretely, filtering `user` by a `has_many` `order` column would generate:

```sql
SELECT "user".* FROM "user" JOIN "order" ON "order".user_id = "user".id WHERE "order".status = $1
```

If a user has 3 orders with that status, this returns **the same user row 3 times** — silently
wrong for a `first_by_*`/`fetch_optional`/`fetch_one` call site (expects at most one row), and
merely surprising for a `Vec` call site. To avoid this, referencing a `has_many`/`belongs_to_many`
column from anything other than [find_all](../dsl/find_all.md) (`first_by_*`, `find_random_by_*`,
`count_by_*`, `exists_by_*`, `delete_by_*`, `set_*_by_*`) is a **compile error**, not a runtime
surprise.

## Attribute ordering

`#[repository(...)]` must be the **first/outermost** attribute, with relation attributes listed
after it:

```rust
#[repository(table = "order")] // must come first
#[belongs_to(table = "user")]  // relation attributes come after
pub trait OrderRepository: Repository { /* ... */ }
```

This is because `has_many`/`has_one`/`belongs_to`/`belongs_to_many` aren't independently
registered proc-macro attributes — they're inert tokens that `#[repository(...)]` receives as
part of the trait's attribute list and strips out before re-emitting the trait. If a relation
attribute is listed *before* `#[repository(...)]`, rustc tries to resolve it on its own and fails
with "cannot find attribute `has_many` in this scope".
