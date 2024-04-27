/// Handles the database interactions for questions.
mod questions_database;

/// Defines request handlers that process incoming Http requests
mod request_handlers;

/// Sets up the routing for HTTP requests
mod request_routes;

/// Imports routes from the `request_routes` module
use request_routes::setup_routes;

//! Entry point for the application
//! Sets up TCP listener for the server, binds it to a specific address,
//! and runs the server using Axum
#[tokio::main]
async fn main() {
    /* Set up the routes for the server */
    let routes = setup_routes();
    println!("Server Has Started!");

    /* Creates a TCP listener bound to address `0.0.0.0:1000  */
    let listener = tokio::net::TcpListener::bind("0.0.0.0:1000").await.unwrap();

    /* Serves incoming request using configured routes */
    axum::serve(listener, routes).await.unwrap();
}
