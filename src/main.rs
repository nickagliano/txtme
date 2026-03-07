use std::sync::Arc;

use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::Json,
    routing::{get, post},
    Router,
};
use tower_http::trace::TraceLayer;
use serde::Deserialize;
use serde_json::{json, Value};

mod providers;

// PORT: SMS_PROVIDER
// Swap this import (and the line in main()) to change how notifications are delivered:
//   use providers::log::LogProvider;   ← dev mode, no SMS sent
use providers::email_sms::EmailSmsProvider;

struct AppState {
    provider: Arc<dyn providers::SmsProvider>,
    api_key:  String,
}

#[derive(Deserialize)]
struct NotifyBody {
    message: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    let provider: Arc<dyn providers::SmsProvider> = Arc::new(
        EmailSmsProvider::from_env().expect("missing SMTP_USER, SMTP_PASS, or SMS_GATEWAY in .env"),
    );

    let api_key = std::env::var("TXTME_API_KEY").unwrap_or_default();
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(5543);

    let state = Arc::new(AppState { provider, api_key });

    let app = Router::new()
        .route("/health", get(health))
        .route("/notify", post(notify))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".into());
    let listener = tokio::net::TcpListener::bind(format!("{host}:{port}"))
        .await
        .unwrap();

    println!("[txtme] listening on :{port}");
    axum::serve(listener, app).await.unwrap();
}

async fn health() -> Json<Value> {
    Json(json!({"status": "ok"}))
}

async fn notify(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(body): Json<NotifyBody>,
) -> (StatusCode, Json<Value>) {
    if !state.api_key.is_empty() {
        let key = headers
            .get("x-api-key")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");
        if key != state.api_key {
            return (StatusCode::UNAUTHORIZED, Json(json!({"error": "unauthorized"})));
        }
    }

    let provider = state.provider.clone();
    let message  = body.message.clone();

    match tokio::task::spawn_blocking(move || provider.send(&message)).await {
        Ok(Ok(())) => (StatusCode::OK, Json(json!({"status": "sent"}))),
        Ok(Err(e)) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))),
        Err(e)     => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))),
    }
}
