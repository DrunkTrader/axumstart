use axumstart_db::repository;

#[derive(sqlx::FromRow)]
#[allow(dead_code)]
struct UserRow {
    id: i32,
    team_id: i32,
}

#[repository(table = "user")]
trait UserRepository: Send + Sync {
    async fn first_by_team_id(&self, team_id: i32) -> sqlx::Result<Vec<UserRow>>;
    async fn find_random_by_team_id(&self, team_id: i32) -> sqlx::Result<Vec<UserRow>>;
}

fn main() {}
