#![feature(negative_impls)]

pub mod app;
pub mod color;
pub mod geometry;
pub mod gfx;
pub mod window;
pub mod engine;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    app::run().await.unwrap();
}
