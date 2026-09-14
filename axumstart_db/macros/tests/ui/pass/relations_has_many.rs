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
    // has_many column — only legal on find_all_by_* (fan-out gate)
    async fn find_all_by_order_status(&self, status: String) -> sqlx::Result<Vec<UserRow>>;
}

fn _assert_impl<T: UserRepository>() {}

fn main() {
    _assert_impl::<DbUserRepository>();
}
