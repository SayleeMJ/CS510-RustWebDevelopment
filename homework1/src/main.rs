/// Handles the database interactions for questions.
mod questions_database;

/// Defines request handlers that process incoming Http requests
mod request_handlers;

/// Sets up the routing for HTTP requests
mod request_routes;

use crate::questions_database::initialize_questions_database;
use dotenv::dotenv;
/// Imports routes from the `request_routes` module
use request_routes::setup_routes;
use std::sync::Arc;

// Entry point for the application
// Sets up TCP listener for the server, binds it to a specific address,
// and runs the server using Axum
#[tokio::main]
async fn main() {
    // Open a.env file to load the environment variables.
    dotenv().ok();

    // Set up the pool of database connections.
    let database_pool = Arc::new(initialize_questions_database().await);

    /* Set up the routes for the server */
    let routes = setup_routes(database_pool.clone());
    println!("Server Has Started!");

    /* Creates a TCP listener bound to address `0.0.0.0:1000  */
    let tcp_listener = tokio::net::TcpListener::bind("0.0.0.0:1000").await.unwrap();

    /* Serves incoming request using configured routes */
    axum::serve(tcp_listener, routes).await.unwrap();
}
