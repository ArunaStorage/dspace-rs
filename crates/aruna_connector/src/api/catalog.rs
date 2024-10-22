use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::info;

//type SharedState = Arc<Mutex<HashMap<String, (Catalog)>>>;

pub async fn get_catalog() {
    info!("Get catalog called");
}

pub async fn get_dataset() {

}