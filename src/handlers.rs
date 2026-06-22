use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use crate::models::User;

pub async fn get_users(State(db): State<Vec<User>>) -> Json<Vec<User>> {
    Json(db)
}

pub async fn get_user(State(db): State<Vec<User>>, Path(id): Path<u32>) -> Result<Json<User>, StatusCode> {
    for user in &db {
        if user.id == id {
            return Ok(Json(user.clone()));
        }
    }
    Err(StatusCode::NOT_FOUND)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{routing::get, Router};
    use axum_test::TestServer;
    use serde_json::json;

    fn app() -> Router {
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

        Router::new()
            .route("/users", get(get_users))
            .route("/users/:id", get(get_user))
            .with_state(db)
    }

    #[tokio::test]
    async fn test_get_users() {
        let server = TestServer::new(app()).unwrap();
        let response = server.get("/users").await;

        response.assert_status_ok();
        response.assert_json(&json!([
            { "id": 1, "name": "Alice", "email": "alice@example.com" },
            { "id": 2, "name": "Bob",   "email": "bob@example.com"   }
        ]));
    }

    #[tokio::test]
    async fn test_get_user() {
        let server = TestServer::new(app()).unwrap();
        let response = server.get("/users/1").await;

        response.assert_status_ok();
        response.assert_json(&json!(
            { "id": 1, "name": "Alice", "email": "alice@example.com" }
        ));
    }

    #[tokio::test]
    async fn test_get_user_not_found() {
        let server = TestServer::new(app()).unwrap();
        let response = server.get("/users/99").await;

        response.assert_status_not_found();
    }
}
