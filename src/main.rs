#![feature(negative_impls)]

pub mod app;
pub mod color;
pub mod gfx;
pub mod window;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    app::run().await.unwrap();
}
