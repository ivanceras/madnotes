#![deny(warnings)]
use tokio::sync::oneshot;

mod serve_files;
mod server;

#[tokio::main]
async fn main() {
    let settings = client::Settings::default();

    let (socket_tx, _socket_rx) = oneshot::channel();
    let (_shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

    server::serve(settings, None, Some(3030), socket_tx, shutdown_rx).await;
}
