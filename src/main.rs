#![feature(negative_impls)]

pub mod app;
pub mod window;
pub mod color;
pub mod gfx;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    app::run().await.unwrap();
}
