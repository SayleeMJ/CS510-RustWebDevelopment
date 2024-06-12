mod route_handlers;
mod routes;

#[tokio::main]
async fn main() {
    // Create all the routes for the application
    let all_routes = routes::create_routes();

    // Start the Warp server and run it on port 2000
    warp::serve(all_routes).run(([0, 0, 0, 0], 2000)).await;
}
