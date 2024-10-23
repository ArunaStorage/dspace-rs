use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use tracing::{debug, error, info};
use dsp_api::catalog::Catalog;

type SharedState = Arc<Mutex<Catalog>>;

pub async fn get_catalog(State(state): State<SharedState>) -> impl IntoResponse {
    info!("Get catalog called");

    let state = state.lock().unwrap();
    let catalog = state.clone();

    //TODO: Add query filtering

    (StatusCode::OK, Json(catalog.clone())).into_response()
}

pub async fn get_dataset(State(state): State<SharedState>, Path(id): Path<String>,) -> impl IntoResponse{
    info!("Get Dataset called");
    debug!("Received Dataset request for id {:#?}", id.clone());

    let state = state.lock().unwrap();

    let datasets = state.clone().datasets.unwrap();
    for dataset in datasets {
        let dataset_id = dataset.clone().abstract_dataset.resource.id.unwrap_or(String::new());
        if dataset_id == id {
            return (StatusCode::OK, Json(dataset.clone())).into_response();
        }
    }

    let err_msg = format!("A Dataset with the given id {:#?} does not exist.", id.clone());

    error!("{}", err_msg);

    (StatusCode::NOT_FOUND, Json(err_msg)).into_response()
}