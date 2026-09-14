# Feature plan: replace `join(...)` with `has_many` / `has_one` / `belongs_to` / `belongs_to_many`

> Standalone execution doc. A fresh Claude Code session in the `axumstart` repo root should be
> able to implement this end-to-end from this file alone, with no prior conversation context.
> All file paths below are relative to the repo root (`/Users/dawid/Projects/axumstart`).

## Context / why

`axumstart_db` is a proc-macro crate that turns a Rust trait into a full repository
implementation (SQL generated from method *names*, one backend of Postgres/MySQL/SQLite chosen
at compile time via Cargo feature). See `docs/docs/axumstart/db.md` for the full user-facing
reference of the existing DSL (method name conventions, `Page`, `#[transactional]`, etc.) —
read that file first for general orientation before touching relation code specifically.

Today, cross-table filtering is declared with a single flat macro argument:

```rust
#[repository(table = "completed_challenge", join(challenge))]
pub trait CompletedChallengeRepository: Repository {
    // challenge_uuid → JOIN "challenge" ON "challenge".id = "completed_challenge".challenge_id
    //                   WHERE "challenge".uuid = $1
    async fn find_by_challenge_uuid(&self, challenge_uuid: Uuid) -> sqlx::Result<Option<Row>>;
}
```

`join(name)` assumes exactly one relation shape: `{table}` has a `{name}_id` FK column pointing
at table `{name}` (many-to-one / "belongs to"). It's parsed in
`axumstart_db/macros/src/repository.rs` (`TableAttr`/`TableAttr::parse`, roughly lines 13-61)
and consumed by `axumstart_db/macros/src/dsl.rs` (`resolve_target` and `join_clauses`, roughly
lines 206-235) to rewrite `{name}_{col}` tokens found in method names like `find_by_*` /
`find_all_by_*` into a qualified column + a `JOIN` clause. `{name}_id` alone is special-cased as
the raw FK column with no JOIN needed.

**Goal:** replace this one primitive with four explicit, direction-aware relation attributes —
`has_many`, `has_one`, `belongs_to`, `belongs_to_many` — declared directly on the trait (not as
`#[repository(...)]` arguments), each with **named parameters only** (no positional relation
identifier — the target `table` value doubles as the DSL prefix used in method names). This is a
**full replacement**: delete `join(...)` parsing entirely, don't keep it as a deprecated alias.

### The fan-out problem this design has to handle

`has_many`/`belongs_to_many` can each match *multiple* rows on the far side of the join, which
`belongs_to`/`has_one`/today's `join()` never could (those are always many-to-one or one-to-one,
so joining never multiplies rows). Concretely: if `user` `has_many` `order` (`order.user_id →
user.id`), then

```rust
async fn find_by_order_status(&self, status: String) -> sqlx::Result<Option<UserRow>>;
```

generates `SELECT "user".* FROM "user" JOIN "order" ON "order".user_id = "user".id WHERE
"order".status = $1` — if a user has 3 orders with that status, this returns **the same user row
3 times**, which silently breaks a `fetch_optional`/`fetch_one` call site (expects ≤1 row) far
worse than a `Vec` call site (just has duplicate entries, at least visible).

**Decision (confirmed with the user, not open for re-litigation unless they say otherwise):**
handle this at compile time, not by changing SQL. A column reached through a `has_many` or
`belongs_to_many` relation is **only** allowed in `find_all_by_*` method names. Using one in
`find_by_*`, `find_random_by_*`, `count_by_*`, `exists_by_*`, or `delete_by_*` must be a
`syn::Error` compile error (not a runtime panic), with a message explaining why. No `DISTINCT`,
no subqueries — the JOIN SQL itself is unchanged from what `join()` already emits today; only
which method-kinds may reference such a column changes.

## Attribute syntax (final — named args only, confirmed with user)

Stack the relation attribute(s) *below* `#[repository(...)]`:

```rust
#[repository(table = "order")]
#[belongs_to(table = "user")]
#[has_many(table = "item")]
pub trait OrderRepository: Repository {
    // user_email → JOIN "user" ON "user".id = "order".user_id, WHERE "user".email = $1
    async fn find_by_user_email(&self, email: String) -> sqlx::Result<Option<OrderRow>>;
    // FK shortcut, no JOIN — user_id is the belongs_to FK column itself, on "order"
    async fn find_by_user_id(&self, user_id: i32) -> sqlx::Result<Option<OrderRow>>;
    // has_many column — only legal on find_all_by_* (fan-out gate)
    async fn find_all_by_item_sku(&self, sku: String) -> sqlx::Result<Vec<OrderRow>>;
}
```

**Why this compiles at all** (important for whoever implements this — it's not obvious): a
proc-macro attribute macro only actually gets *invoked* by rustc if it's registered
(`#[proc_macro_attribute]`). `has_many`/`has_one`/`belongs_to`/`belongs_to_many` are **not**
separately registered macros — they're inert tokens that `#[repository(...)]` (the outermost,
first-listed attribute) receives as part of `ItemTrait.attrs` in its own macro invocation, and
must strip out of `trait_def.attrs` before re-emitting the trait. This is exactly how the
existing method-level attributes `#[unique(...)]`, `#[order_by(...)]`, `#[created_at(...)]`,
`#[unchecked_columns]`, `#[transactional]` already work today (see `collect_attr_map`,
`collect_order_by_map`, `collect_flag_attr` in `repository.rs`) — same trick, just applied to
the trait's own attribute list instead of each method's. **Consequence: `#[repository(...)]`
must stay the first/outermost attribute**, with the relation attributes listed after it — if
rustc hits `has_many` first (i.e. it's listed above `#[repository]`), it fails with "cannot find
attribute `has_many` in this scope" because nothing ever registered it. Document this ordering
requirement in the docs update (Step 4 below).

### Named parameters per attribute

`fk` always defaults to `"{some_table}_id"` — which table depends on relation direction, per the
rule the user gave: *"fk should default to table name + _id"*, applied to whichever table
actually owns that FK column in each relation shape.

| Attribute | Params | Defaults | JOIN emitted | FK shortcut (`{table}_id`, no JOIN)? |
|---|---|---|---|---|
| `belongs_to(table, fk?)` | `fk` = FK column on **this** (owning) table | `fk = "{table}_id"` | `JOIN "{table}" ON "{table}".id = "{this_table}".{fk}` | **Yes** — `{table}_id` resolves straight to the `fk` column on this table, no JOIN |
| `has_one(table, fk?)` | `fk` = FK column on the **target** table, pointing back here | `fk = "{this_table}_id"` | `JOIN "{table}" ON "{table}".{fk} = "{this_table}".id` | No |
| `has_many(table, fk?)` | same shape as `has_one` | same as `has_one` | same as `has_one` | No |
| `belongs_to_many(table, through, fk?, other_fk?)` | `through` (**required**) = pivot table name; `fk` = this table's FK column in the pivot; `other_fk` = target table's FK column in the pivot | `fk = "{this_table}_id"`, `other_fk = "{table}_id"` | `JOIN "{through}" ON "{through}".{fk} = "{this_table}".id JOIN "{table}" ON "{table}".id = "{through}".{other_fk}` | No |

`{this_table}` = the `table = "..."` value passed to `#[repository(...)]` itself (already
available as `ctx.table` in the current code).

There is **no separate relation-name/alias parameter** — the `table` value is simultaneously the
real SQL table name and the DSL prefix matched in method names (`{table}_{col}`). This matches
today's `join()` behavior, where the identifier already had to equal the table name. (If two
relations to the same table under different roles are ever needed — e.g. `author`/`reviewer`
both pointing at `user` — that's an explicit non-goal for this pass; flag it to the user if it
comes up, don't silently design around it.)

## Implementation steps

### 1. `axumstart_db/macros/src/dsl.rs` — relation model

This module is deliberately pure string/data logic with **no `syn` types** (see its top doc
comment: "Pure string parsing of the method-name DSL. No syn types — unit-testable.") — keep it
that way; all `syn`-based attribute parsing belongs in `repository.rs`.

Add:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelationKind { BelongsTo, HasOne, HasMany, BelongsToMany }

#[derive(Debug, Clone)]
pub struct Relation {
    /// Target table name AND the DSL prefix matched in method names.
    pub table: String,
    pub kind: RelationKind,
    pub fk: String,
    /// BelongsToMany only.
    pub other_fk: Option<String>,
    /// BelongsToMany only.
    pub through: Option<String>,
}
```

Change every function currently taking `joins: &[String]` to take `relations: &[Relation]`
instead:

- **`resolve_target`** (currently ~`dsl.rs:206-227`): today it does
  `let prefix = format!("{j}_"); column.strip_prefix(prefix)` against each bare join name, and
  special-cases `col == "id"` as the FK shortcut. Change to iterate `relations`, match by
  `"{relation.table}_"` prefix, then branch on `relation.kind`:
  - `BelongsTo`: if the remainder is `"id"`, return the raw `relation.fk` column name (no JOIN,
    push nothing to `joins_needed`) — this is the existing shortcut, generalized to use
    `relation.fk` instead of assuming the FK is always literally `"{name}_id"` under the hood
    (it still defaults there, but a custom `fk` must also work).
  - `HasOne` / `HasMany`: always needs the JOIN (push `relation.table` into `joins_needed` if not
    already there), return `"{relation.table}".{col}` — including when `col == "id"`, since
    there's no FK on this table to shortcut to.
  - `BelongsToMany`: same as `HasOne`/`HasMany` (always joins), but the relation carries two
    tables to join (`through` + `table`) — `join_clauses` (below) handles rendering both; here,
    just push `relation.table` into `joins_needed` as the marker (so a set of `HashSet`/`Vec`
    keyed on target table names, same shape as today, still works — `join_clauses` will look up
    the full `Relation` again, not just the table string, to know it needs to also render the
    pivot join. Practically this likely means `joins_needed` should become `Vec<&Relation>` or
    `Vec<usize>` indices rather than `Vec<String>`, since `join_clauses` needs the *whole*
    relation, not just its table name, to render `BelongsToMany`'s pivot join and to distinguish
    `HasMany`/`BelongsToMany` for the fan-out flag below. Pick whichever is more ergonomic in
    context — `Vec<String>` plus a second lookup back into `relations: &[Relation]` by table name
    inside `join_clauses` is also fine and keeps `WhereClause` unchanged shape-wise.)
  - Add a new `WhereClause` field: `pub fan_out: bool` — set to `true` if *any* resolved column
    in this WHERE clause came from a `HasMany` or `BelongsToMany` relation. `repository.rs` reads
    this to enforce the compile-time restriction (Step 2).
- **`join_clauses`** (currently ~`dsl.rs:229-235`): currently always renders
  `JOIN "{j}" ON "{j}".id = "{table}".{j}_id"` for every needed join. Change to look up each
  needed relation and switch on `kind`:
  - `BelongsTo`: `JOIN "{table}" ON "{table}".id = "{this_table}".{fk}"`
  - `HasOne`/`HasMany`: `JOIN "{table}" ON "{table}".{fk} = "{this_table}".id"`
  - `BelongsToMany`: two joins — `JOIN "{through}" ON "{through}".{fk} = "{this_table}".id JOIN
    "{table}" ON "{table}".id = "{through}".{other_fk}"`
- **`build_where`**: signature's `joins: &[String]` parameter becomes `relations: &[Relation]`.
  Update the three call sites in `repository.rs` (`gen_filtered`, `gen_select_all`, `gen_set` —
  currently around lines 717, 890, 1102).
- **Unit tests** (bottom of `dsl.rs`, `#[cfg(test)]` module): update `join_resolution` (currently
  builds `vec!["user".to_string()]`) to build a `Relation { table: "user".into(), kind:
  RelationKind::BelongsTo, fk: "user_id".into(), other_fk: None, through: None }` instead. Add
  new cases: `has_one`/`has_many` always join (even for the `_id` column), `belongs_to_many`
  renders both joins in `join_clauses`, and a case asserting `fan_out == true` only for
  `HasMany`/`BelongsToMany` relations (not for `BelongsTo`/`HasOne`).

### 2. `axumstart_db/macros/src/repository.rs` — attribute parsing + codegen wiring

- **Remove `join` entirely** from `TableAttr` / `TableAttr::parse` (currently lines 13-61):
  delete the `joins: Vec<String>` field and the `else if flag == "join" { ... }` branch. The
  macro's own arguments go back to just `table = "..."`, `mock`, `component`.
- **Add relation-attribute parsing.** New function, placed alongside `collect_attr_map` /
  `collect_order_by_map` / `collect_flag_attr` (same file):
  ```rust
  fn collect_relations(
      trait_def: &mut ItemTrait,
      this_table: &str,
      errors: &mut Vec<TokenStream2>,
  ) -> Vec<dsl::Relation>
  ```
  For each of the four attribute names, scan `trait_def.attrs` (the **trait's own** attribute
  list — this is different from `collect_attr_map` et al., which scan each method's attrs; here
  there's one flat list on the `ItemTrait` itself, and there can be multiple matches since a
  trait can declare several relations of the same or different kinds), remove each match, and
  parse its argument list as comma-separated `ident = "litstr"` pairs (reuse the same parsing
  shape as `TableAttr::parse`'s existing flag loop for consistency — `Ident`, `Token![=]`,
  `LitStr`, looped on `Token![,]`). Required keys: `table` for all four, plus `through` for
  `belongs_to_many`. Compute `fk`/`other_fk` defaults per the table in the syntax section above
  when not explicitly given. An unknown key, a missing `table`, or a missing `through` on
  `belongs_to_many` should push a `syn::Error::new(span, "...").to_compile_error()` into `errors`
  (same pattern as `validate_attr_targets`) rather than panicking the proc-macro.
- **`Ctx`** (currently lines 64-67): change `joins: &'a [String]` to `relations: &'a
  [dsl::Relation]`.
- **`expand()`** (currently starts ~line 69): call `collect_relations(&mut trait_def, &table,
  &mut errors)` right after `parse_macro_input!(item as ItemTrait)` and before `let trait_attrs =
  &trait_def.attrs;` is captured — ordering matters, since stripping must happen before that
  line or the removed attributes will still be re-emitted (rustc would then fail trying to
  resolve them, per the "why this compiles" note above).
- **Fan-out compile-time gate**: `gen_filtered` and `gen_select_all` (the functions that consume
  `WhereClause` for `find_by_*`/`find_random_by_*`/`count_by_*`/`exists_by_*`/`delete_by_*` and
  `find_all_by_*` respectively) need to check `wc.fan_out`. `gen_select_all` backs
  `find_all_by_*`, which is allowed, so no gate there. `gen_filtered` backs everything else
  (`Kind::One`/`Random`/`Count`/`Exists`/`Delete` — see the `Kind` enum and `gen_filtered`
  function, currently ~lines 692-869) — if `wc.fan_out` is true, return a `syn::Error` there,
  mirroring the existing `Kind::Delete` + non-empty `joins_needed` check at ~lines 834-839:
  ```rust
  if wc.fan_out {
      return Err(syn::Error::new(
          sig.ident.span(),
          "this filter is reached through a has_many/belongs_to_many relation and may match \
           multiple rows; only find_all_by_* may filter through it",
      ));
  }
  ```
  Also double check `gen_set`'s `WHERE`-side `build_where` call (~line 1102, backs `set_*_by_*`)
  — same fan-out risk applies there (an `UPDATE ... WHERE` fanning out through a `has_many` join
  would update/target ambiguous rows), so gate it the same way.

### 3. Tests

- `axumstart_db/macros/tests/ui/pass/joins_tx.rs`: update the trait attribute from
  `#[repository(table = "order", join(user))]` to:
  ```rust
  #[repository(table = "order")]
  #[belongs_to(table = "user")]
  trait OrderRepository: Send + Sync { ... }
  ```
  (rest of the file — the `find_all_by_user_email`/`find_all_by_user_id` methods — stays as-is;
  they should still compile and behave identically).
- Add a new `ui/pass` test, e.g. `tests/ui/pass/relations_has_many.rs`, declaring a trait with
  `#[has_many(table = "...")]` and a `find_all_by_*` method filtering through it — should compile
  cleanly.
- Add a new `ui/fail` test, e.g. `tests/ui/fail/has_many_on_find_by.rs`, declaring the same
  `has_many` relation but a `find_by_*` (not `find_all_by_*`) method filtering through it —
  should fail to compile with the new fan-out error message. Follow the existing
  pattern in `tests/ui/fail/*.rs` plus whatever `.stderr` snapshot mechanism `tests/codegen.rs`
  uses for the other `ui/fail` cases (check how e.g. `unknown_method.rs`/`arity_mismatch.rs` are
  wired up before adding this one, to match the harness's expectations exactly).
- Consider a `ui/pass` test for `belongs_to_many` (double join) and `has_one` too, since neither
  has any coverage today under the old `join()` scheme.

### 4. Docs — `docs/docs/axumstart/db.md`

Replace the existing `### `join(table_name)` — cross-table field resolution` section (currently
lines 68-85) with one subsection per new attribute. Keep the file's existing style (short prose +
a fenced Rust example + a note on conventions, matching how the rest of the file is written).
Cover, at minimum:
- The full parameter table from the syntax section above (all four attributes, defaults).
- A worked example per attribute (the existing `completed_challenge`/`challenge` example can be
  ported almost directly for `belongs_to`; add fresh, small examples for `has_one`, `has_many`,
  `belongs_to_many` — doesn't need to be a real domain, just illustrative).
- The attribute-ordering requirement (`#[repository(...)]` must come first, relation attributes
  after — see "why this compiles" above).
- The `find_all_by_*`-only restriction on `has_many`/`belongs_to_many` columns, with the
  duplicate-row rationale (can reuse the `user`/`order` example from this doc's Context section).

## Verification

Run from repo root unless noted:

1. `cargo test -p axumstart_db_macros` — runs `dsl.rs`'s `#[cfg(test)]` unit tests plus the
   trybuild UI suite driven by `tests/codegen.rs` (which executes everything under
   `tests/ui/pass/` and `tests/ui/fail/`). This is the primary correctness check for this
   feature — make sure the new pass/fail cases from Step 3 are picked up and green/red as
   expected.
2. `cargo build -p axumstart_db --no-default-features --features postgres` (repeat with `mysql`,
   then `sqlite`) — confirms the relation-based JOIN SQL still compiles under each backend
   feature flag. Dialect/placeholder code is untouched by this change, but it's a cheap sanity
   check since `#[repository(...)]`-generated code is feature-gated.
3. `cd docs && npm run build` — confirms the Step 4 docs edit doesn't break the Docusaurus build
   (this repo's CI already fails the build on broken internal links/anchors — see the recent fix
   in `docs/docusaurus.config.js` / `docs/src/pages/index.js` for context on how strict this
   check is).
4. Read back the final `db.md` relations section once written and sanity-check the rendered
   SQL in each example against the JOIN-emission rules in the parameter table — it's easy to
   transpose `fk`/`other_fk` direction by mistake when writing prose examples by hand.
