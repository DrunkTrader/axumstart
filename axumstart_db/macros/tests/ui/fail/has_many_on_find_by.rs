use axumstart_db::repository;

#[derive(sqlx::FromRow)]
#[allow(dead_code)]
struct UserRow {
    id: i32,
    email: String,
}

#[repository(table = "user")]
#[has_many(table = "order")]
trait UserRepository: Send + Sync {
    async fn find_by_order_status(&self, status: String) -> sqlx::Result<Option<UserRow>>;
}

fn main() {}
