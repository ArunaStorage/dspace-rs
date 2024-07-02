/*
 * management-api
 *
 * REST API documentation for the Eclipse EDC management-api.
 * https://app.swaggerhub.com/apis/eclipse-edc-bot/management-api/
 * Version: 0.7.0
 *
 */


use reqwest;

use crate::ResponseContent;
use crate::utils::remove_prefixes_from_value;
use super::{Error, configuration};


/// struct for typed errors of method [`add_entry`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddEntryError {
    Status400(Vec<crate::ApiErrorDetail>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`find`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FindError {
    Status400(Vec<crate::ApiErrorDetail>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_all`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAllError {
    Status400(Vec<crate::ApiErrorDetail>),
    UnknownValue(serde_json::Value),
}


/// Adds one dataplane instance to the internal database of the selector. DEPRECATED: dataplanes should register themselves through control-api
#[deprecated(note="Deprecated since management api version 0.6.5-SNAPSHOT; Dataplanes should register themselves through control-api")]
pub async fn add_entry(configuration: &configuration::Configuration, data_plane_instance_schema: Option<crate::DataPlaneInstanceSchema>) -> Result<(), Error<AddEntryError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/dataplanes", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&data_plane_instance_schema);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<AddEntryError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Finds the best fitting data plane instance for a particular query
#[deprecated(note="Deprecated since management api version 0.6.5-SNAPSHOT")]
pub async fn find(configuration: &configuration::Configuration, selection_request_schema: Option<crate::SelectionRequestSchema>) -> Result<crate::DataPlaneInstanceSchema, Error<FindError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/dataplanes/select", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&selection_request_schema);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let mut val = serde_json::from_str(&local_var_content).map_err(Error::from)?;
        val = remove_prefixes_from_value(val);
        serde_json::from_value(val).map_err(Error::from)
    } else {
        let local_var_entity: Option<FindError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a list of all currently registered data plane instances
pub async fn get_all(configuration: &configuration::Configuration) -> Result<(), Error<GetAllError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/dataplanes", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<GetAllError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}