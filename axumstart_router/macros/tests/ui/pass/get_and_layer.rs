use axumstart_router::{Components, get, layer, post};
use tower::layer::util::Identity;

#[get("/plain")]
async fn plain() -> &'static str {
    "plain"
}

fn make_layer() -> Identity {
    Identity::new()
}

// bare function name, zero args — layer above get
#[layer(make_layer)]
#[get("/zero-arg")]
async fn with_zero_arg_layer() -> &'static str {
    "zero-arg"
}

fn ctx_layer(_ctx: &Components) -> Identity {
    Identity::new()
}

// bare function name, takes &Components — get above layer (reverse order)
#[post("/ctx-arg")]
#[layer(ctx_layer)]
async fn with_ctx_layer() -> &'static str {
    "ctx-arg"
}

// ready expression, not a bare path
#[layer(Identity::new())]
#[get("/expr")]
async fn with_expr_layer() -> &'static str {
    "expr"
}

fn main() {}
