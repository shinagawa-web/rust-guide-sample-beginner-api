mod handlers;
mod models;

use axum::{routing::get, Router};
use handlers::{get_user, get_users};
use models::User;

#[tokio::main]
async fn main() {
    let db = vec![
        User {
            id: 1,
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        },
        User {
            id: 2,
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
        },
    ];

    let app = Router::new()
        .route("/users", get(get_users))
        .route("/users/:id", get(get_user))
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
