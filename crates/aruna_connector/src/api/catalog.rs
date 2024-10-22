use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use tracing::info;

//type SharedState = Arc<Mutex<HashMap<String, (Catalog)>>>;

pub async fn get_catalog() {
    info!("Get catalog called");
}

pub async fn get_dataset() -> impl IntoResponse{
    Json(json!({"test": "test"}))
}