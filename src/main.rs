mod app;
mod http;
mod log;
mod util;
mod controller;

#[tokio::main]
async fn main() {
    controller::run().await
}