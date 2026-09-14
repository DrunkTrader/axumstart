use axumstart_db::repository;

#[derive(sqlx::FromRow)]
#[allow(dead_code)]
struct PostRow {
    id: i32,
    title: String,
}

#[repository(table = "post")]
#[has_one(table = "post_meta")]
#[belongs_to_many(table = "tag", through = "post_tag")]
trait PostRepository: Send + Sync {
    // has_one column — always joins, but has_one never fans out, so find_by_* is fine
    async fn find_by_post_meta_summary(&self, summary: String) -> sqlx::Result<Option<PostRow>>;
    // belongs_to_many column — double JOIN through the pivot table; only legal on find_all_by_*
    async fn find_all_by_tag_name(&self, name: String) -> sqlx::Result<Vec<PostRow>>;
}

fn _assert_impl<T: PostRepository>() {}

fn main() {
    _assert_impl::<DbPostRepository>();
}
