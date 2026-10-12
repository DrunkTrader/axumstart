# belongs_to

Many-to-one — the foreign key lives on **this** (owning) table, pointing at the target table.

```rust
#[repository(table = "completed_challenge")]
#[belongs_to(table = "challenge")]
pub trait CompletedChallengeRepository: Repository {
    // challenge_uuid → JOIN "challenge" ON "challenge".id = "completed_challenge".challenge_id
    //                   WHERE "challenge".uuid = $1 AND user_id = $2
    async fn find_by_challenge_uuid_and_user_id(
        &self, challenge_uuid: Uuid, user_id: i32,
    ) -> sqlx::Result<Option<CompletedChallengeRow>>;

    // FK shortcut, no JOIN — challenge_id is the belongs_to FK column itself
    async fn find_by_challenge_id(&self, challenge_id: i32) -> sqlx::Result<Option<CompletedChallengeRow>>;
}
```

`{table}_id` (here, `challenge_id`) is the one column every relation kind has a no-JOIN shortcut
for — since `belongs_to`'s whole premise is that the FK already lives on this table, `{table}_id`
resolves straight to that FK column instead of triggering a JOIN. Any other `{table}_{col}`
(`challenge_uuid` above) does JOIN.

See [relations](./relations.md) for the full attribute table and defaults.
