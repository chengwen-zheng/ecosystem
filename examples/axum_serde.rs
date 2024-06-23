use std::sync::{Arc, Mutex};

use axum::{
    extract::State,
    routing::{get, patch},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument, level_filters::LevelFilter};
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt as _,
    util::SubscriberInitExt as _,
    Layer,
};

#[derive(Debug, Clone, PartialEq, Serialize)]
struct User {
    name: String,
    age: u32,
    skills: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct UserUpdate {
    name: Option<String>,
    age: Option<u32>,
    skills: Option<Vec<String>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let console = fmt::Layer::new()
        .with_span_events(FmtSpan::CLOSE)
        .pretty()
        .with_filter(LevelFilter::DEBUG);

    tracing_subscriber::registry().with(console).init();

    let user = User {
        name: "Alice".to_string(),
        age: 30,
        skills: vec!["Rust".to_string(), "Python".to_string()],
    };
    let user = Arc::new(Mutex::new(user));

    let addr = "0.0.0.0:8080";
    let listener = tokio::net::TcpListener::bind(addr).await?;

    info!("Listening on: {}", addr);

    let app = Router::new()
        .route("/user", get(get_user))
        .route("/user", patch(update_user))
        .with_state(user);

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

#[instrument]
async fn get_user(State(user): State<Arc<Mutex<User>>>) -> Json<User> {
    (*user.lock().unwrap()).clone().into()
}

#[instrument]
async fn update_user(
    State(user): State<Arc<Mutex<User>>>,
    Json(update): Json<UserUpdate>,
) -> Json<User> {
    let mut user = user.lock().unwrap();
    if let Some(name) = update.name {
        user.name = name;
    }
    if let Some(age) = update.age {
        user.age = age;
    }
    if let Some(skills) = update.skills {
        user.skills = skills;
    }
    (*user).clone().into()
}
