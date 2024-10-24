#![feature(negative_impls)]

pub mod app;
pub mod color;
pub mod universe_ref;
pub mod engine;
pub mod geometry;
pub mod gfx;
pub mod window;
pub mod class;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    app::run().await.unwrap();
}
