use axumstart_router::layer;
use tower::layer::util::Identity;

fn make_layer() -> Identity {
    Identity::new()
}

#[layer(make_layer)]
async fn orphan_layer() -> &'static str {
    "orphan"
}

fn main() {}
