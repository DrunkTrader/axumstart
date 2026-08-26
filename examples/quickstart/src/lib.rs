//! Verifies that consumers depending only on `axumstart` (not the sub-crates directly)
//! can use the re-exported macros without hitting E0433 on `axumstart_components`/
//! `axumstart_db` — including the `#[repository(component)]` path, which needs both.

use axumstart::components::Component;
use axumstart::db::{repository, DbPool};

#[derive(Component)]
#[allow(dead_code)]
struct Settings {
  #[default]
  port: u16,
}

#[allow(dead_code)]
impl Settings {
  fn port(&self) -> u16 {
    self.port
  }
}

#[repository(table = "users", component)]
pub trait UserRepository: Send + Sync {
  async fn exists_by_id(&self, id: i32) -> sqlx::Result<bool>;
  fn pool(&self) -> &DbPool;
}

#[cfg(test)]
mod tests {
  use super::*;
  use axumstart::components::ComponentProvider;

  #[tokio::test]
  async fn components_resolve_through_the_axumstart_meta_crate() {
    let ctx = ComponentProvider::default();
    ctx.register::<Settings>();
    let components = ctx.build().await;
    let settings = components.get::<Settings>().expect("Settings should resolve");
    assert_eq!(settings.port(), 0);
  }
}
