use std::path::PathBuf;

use askama::Template;
use axum::{
    extract::{Path, Query},
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method,
    },
    response::IntoResponse,
    routing::get,
};

use route::create_router;
use serde::Deserialize;
use template::HtmlTemplate;
use tower_http::{cors::CorsLayer, services::ServeDir};

mod handler;
mod model;
mod response;
mod route;
mod template;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let public_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("public");

    let static_files_service = ServeDir::new(public_dir);

    let app = create_router()
        .layer(cors)
        .route("/greet/:name", get(greet))
        .route("/", get(index))
        .nest_service("/public", static_files_service);

    println!("🚀 Server started successfully");
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> impl IntoResponse {
    let template = IndexTemplate {};
    HtmlTemplate(template)
}

#[derive(Debug, Deserialize)]
struct Q {
    last_name: Option<String>,
}

async fn greet(Path(name): Path<String>, Query(q): Query<Q>) -> impl IntoResponse {
    let new_name = q.last_name.map_or_else(|| "".to_string(), |v| v);

    let mut name = name;

    name.push_str(&new_name);

    let template = HelloTemplate { name };
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate {
    name: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}
