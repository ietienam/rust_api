use axum::{
    extract:: { Path, Query },
    body::Body,
    http::StatusCode,
    response::{ IntoResponse, Response },
    routing::{ get, post },
    Router, Json,
};
use serde::{ Serialize, Deserialize };

#[derive(Deserialize)]
struct Page {
    number: u32,
}

#[derive(Deserialize)]
struct Item {
    title: String,
}

#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

async fn show_item(Path(id): Path<u32>, Query(page): Query<Page>) -> String {
    format!("Item {} on page {}", id, page.number)
}

async fn add_item(Json(item): Json<Item>) -> String {
    format!("Added item: {}", item.title)
}

async fn create_user() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::from("User created successfully"))
        .unwrap()
}

async fn list_users() -> Json<Vec<User>> {
    let users = vec![
        User {
            id: 1,
            name: "Ini".to_string(),
            email: "ini@ini.com".to_string(),
        },
        User {
            id: 2,
            name: "Etienam".to_string(),
            email: "etienam@ini.com".to_string(),
        },
    ];
    Json(users)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, Rust!" }))
        .route("/users", post(create_user))
        .route("/items/:id", get(show_item))
        .route("/items", post(add_item))
        .route("/users", get(list_users));

    println!("Running on http://localhost:3009");
    let port: String = String::from("0.0.0.0:3009");
    axum::Server::bind(&port.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
