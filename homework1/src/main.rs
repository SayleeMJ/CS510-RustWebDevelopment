mod questions_database;
mod request_handlers;
mod request_routes;

use request_routes::setup_routes;

#[tokio::main]
async fn main() {
    let routes = setup_routes();
    println!("Server Has Started!");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:1000").await.unwrap();
    axum::serve(listener, routes).await.unwrap();
}
