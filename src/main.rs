#![feature(negative_impls)]

pub mod app;
pub mod color;
pub mod engine;
pub mod geometry;
pub mod gfx;
pub mod node_class;
pub mod universe_ref;
pub mod window;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    app::run().await.unwrap();
}
