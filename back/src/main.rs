use server::Server;

mod controllers;
mod server;
mod services;
mod types;
mod utils;

#[tokio::main]
async fn main() {
    let server = Server::new("0.0.0.0:3000".to_string());
    server.start().await
}
