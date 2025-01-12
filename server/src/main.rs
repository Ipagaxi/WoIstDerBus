use axum::{
    routing::get,
    Router,
};

use std::time::Duration;
use tokio::task;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(root))
        .route("/register-bus", get(register_bus))
        .route("/get-bus-pos", get(get_bus_pos));

    // run our app with hyper, listening globally on port 3000
    let url = "0.0.0.0";
    let port = "3000";
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", url, port)).await.unwrap();
    println!("Server is running on http://{}:{}", url, port);
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, world!"
}

async fn register_bus() -> &'static str {
    // Spawn a background task
    task::spawn(async {
        for i in 1..=10 {
            println!("Background task running: iteration {}", i);
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
        println!("Background task completed!");
    });

    "Register Bus... Background task started!"
}

async fn get_bus_pos() -> &'static str {
    "Bus position..."
}