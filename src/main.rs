//! Run with
//!
//! ```not_rust
//! cd examples && cargo run -p example-readme
//! ```

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json,
    Router,
    extract::{Extension, ConnectInfo},
    // https://docs.rs/http/0.2.8/http/header/constant.ACCESS_CONTROL_ALLOW_METHODS.html
    //http::header::{ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN},
    //http::{HeaderMap, HeaderValue, Method},
    http::{HeaderValue, Method},
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

// used append to file
use std::fs::OpenOptions;
use std::io::prelude::*;

// used to share a file lock among tasks
use tokio::sync::Mutex;
use std::sync::Arc;

use tower_http::cors::CorsLayer;

struct State {
    file: Mutex<String>,
}

fn append_to_file(file: &str, txt: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(file)
        .unwrap();

    if let Err(e) = writeln!(file, "{}", txt.trim()) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let shared_state = 
        Arc::new(State { file: Mutex::new("log-file".to_string()), });

    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:8000".parse::<HeaderValue>().unwrap())
                .allow_headers([axum::http::header::CONTENT_TYPE])
                .allow_methods([Method::POST]),)
        .layer(Extension(shared_state.clone()));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> axum::response::Html<&'static str> {
    include_str!("index.html").into()
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Extension(state): Extension<Arc<State>>,
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {

// Add access-control header for browser usage
// Access-Control-Allow-Origin:  http://127.0.0.1:3000
// Access-Control-Allow-Methods: POST
// Access-Control-Allow-Headers: Content-Type, Authorization
//    Headers(vec![("X-Foo", "foo")])

    let locked_file = state.file.lock().await;
    append_to_file(&locked_file, &payload.username);

    let user = User {
        id: 7787,
        username: payload.username,
        addr: format!("Hello {}", addr),
    };

    /*
    let mut headers = HeaderMap::new();
    headers.insert(
        axum::http::header::ACCESS_CONTROL_ALLOW_ORIGIN, 
        HeaderValue::from_str(&format!("{}", addr)).unwrap());

    headers.insert(
        axum::http::header::ACCESS_CONTROL_ALLOW_ORIGIN, 
        HeaderValue::from_str(&format!("{}", "http://localhost:8000")).unwrap());

    headers.insert(
        axum::http::header::ACCESS_CONTROL_ALLOW_ORIGIN, 
        HeaderValue::from_str(&format!("{}", "*")).unwrap());

    headers.insert(
        axum::http::header::ACCESS_CONTROL_ALLOW_METHODS, 
        HeaderValue::from_str(&format!("{}", "POST")).unwrap());

    headers.insert(
        axum::http::header::ACCESS_CONTROL_ALLOW_HEADERS, 
        HeaderValue::from_str(&format!("{}", "X-Requested-With")).unwrap());
        */
    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (
        StatusCode::CREATED, 
        Json(user),
    )
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
    addr: String,
}
