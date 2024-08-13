use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use tracing::{error, info};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use axum::response::IntoResponse;
use axum_macros::debug_handler;
use chrono::{DateTime, Local, TimeZone, Utc};
use edc_api::{ContractDefinitionInput, ContractDefinitionOutput, Criterion, IdResponse, QuerySpec};
use uuid::Uuid;
use edc_api::query_spec::SortOrder;

// Shared state to store contract definitions
type SharedState = Arc<Mutex<HashMap<String, ContractDefinitionOutput>>>;

pub(crate) async fn update_contract_definition(State(state): State<SharedState>, Json(input): Json<ContractDefinitionInput>,) -> impl IntoResponse {

    /// Updated a contract definition with the given ID. The supplied JSON structure must be a valid JSON-LD object
    ///
    /// # Example
    ///
    /// Request Body:
    /// {
    ///   "@context": {
    ///     "@vocab": "https://w3id.org/edc/v0.0.1/ns/"
    ///   },
    ///   "@id": "definition-id",
    ///   "accessPolicyId": "asset-policy-id",
    ///   "contractPolicyId": "contract-policy-id",
    ///   "assetsSelector": []
    /// }
    ///
    /// Responses:
    /// 204 - Contract definition was updated successfully
    /// 400 - Request was malformed, e.g. id was null
    /// 404 - A contract definition with the given ID does not exist

    if input.at_id.is_none() {
        return (StatusCode::BAD_REQUEST, Json(error!("Request was malformed, id was null"))).into_response();
    }

    let id = input.at_id.clone().unwrap();

    info!("Received contract definition update for id {:#?} <> PUT /v2/contractdefinitions/:\n{:#?}\n", id.clone(), input);

    let mut state = state.lock().unwrap();

    if state.contains_key(&id) {
        let created_at = Utc::now().timestamp();

        let output = ContractDefinitionOutput {
            context: input.context,
            at_id: Some(id.clone()),
            at_type: input.at_type.clone(),
            access_policy_id: Some(input.access_policy_id.clone()),
            assets_selector: Some(input.assets_selector.clone()),
            contract_policy_id: Some(input.contract_policy_id.clone()),
            created_at: Some(created_at),
        };

        state.insert(id.clone(), output);
        StatusCode::NO_CONTENT.into_response()
    } else {
        (StatusCode::NOT_FOUND, Json(error!("A contract definition with the given ID does not exist"))).into_response()
    }
}

pub(crate) async fn create_contract_definition(State(state): State<SharedState>, Json(input): Json<ContractDefinitionInput>,) -> impl IntoResponse {

    /// Creates a new contract definition
    /// If no @id is provided, a new random UUID will be generated
    ///
    /// # Example
    ///
    /// Request Body:
    /// {
    ///   "@context": {
    ///     "@vocab": "https://w3id.org/edc/v0.0.1/ns/"
    ///   },
    ///   "@id": "definition-id",
    ///   "accessPolicyId": "asset-policy-id",
    ///   "contractPolicyId": "contract-policy-id",
    ///   "assetsSelector": []
    /// }
    ///
    /// Responses:
    /// 200 -  contract definition was created successfully. Returns the Contract Definition Id and created timestamp
    ///        {
    ///             "@context": {
    ///                 "@vocab": "https://w3id.org/edc/v0.0.1/ns/"
    ///             },
    ///             "@id": "definition-id",
    ///             "createdAt": 1688465655
    ///         }
    /// 400 - Request was malformed
    /// 404 - Could not create contract definition, because a contract definition with that ID already exists

    info!("Received contract definition <> POST /v2/contractdefinitions:\n{:#?}\n", input);

    let mut state = state.lock().unwrap();

    if input.at_id.is_some() {

        let id = input.at_id.clone().unwrap();

        // Response with 409 Conflict
        // Could not create contract definition, because a contract definition with that ID already exists
        if state.contains_key(&id) {
            return (StatusCode::CONFLICT, Json(error!("Could not create contract definition, because a contract definition with that ID already exists"))).into_response();
        }

        let created_at = Utc::now().timestamp();

        let output = ContractDefinitionOutput {
            context: input.context,
            at_id: Some(id.clone()),
            at_type: input.at_type.clone(),
            access_policy_id: Some(input.access_policy_id.clone()),
            assets_selector: Some(input.assets_selector.clone()),
            contract_policy_id: Some(input.contract_policy_id.clone()),
            created_at: Some(created_at),
        };

        state.insert(id.clone(), output.clone());

        let id_response = IdResponse {
            at_id: Some(id.clone()),
            created_at: Some(created_at.clone()),
        };

        return (StatusCode::OK, Json(id_response)).into_response();
    }

    let id = Uuid::new_v4().to_string();
    let created_at = Utc::now().timestamp();

    let output = ContractDefinitionOutput {
        context: input.context,
        at_id: Some(id.clone()),
        at_type: input.at_type.clone(),
        access_policy_id: Some(input.access_policy_id.clone()),
        assets_selector: Some(input.assets_selector.clone()),
        contract_policy_id: Some(input.contract_policy_id.clone()),
        created_at: Some(created_at),
    };

    state.insert(id.clone(), output.clone());

    let id_response = IdResponse {
        at_id: Some(id.clone()),
        created_at: Some(created_at.clone()),
    };

    (StatusCode::OK, Json(id_response)).into_response()

}

pub(crate) async fn request_contract_definition(State(state): State<SharedState>, Json(query): Json<QuerySpec>,) -> impl IntoResponse {

    /// Returns all contract definitions according to a query
    ///
    /// # Example
    ///
    /// Request Body:
    /// {
    ///   "@context": {
    ///     "@vocab": "https://w3id.org/edc/v0.0.1/ns/"
    ///   },
    ///   "@type": "QuerySpec",
    ///   "offset": 5,
    ///   "limit": 10,
    ///   "sortOrder": "DESC",
    ///   "sortField": "fieldName",
    ///   "filterExpression": []
    /// }
    ///
    /// Responses:
    /// 200 - The contract definitions matching the query
    /// 400 - Request was malformed

    info!("Received contract definition request <> POST /v2/contractdefinitions/request for query:\n{:#?}\n", query);

    let state = state.lock().unwrap();

    // Collect all contract definitions into a vector
    let mut output: Vec<ContractDefinitionOutput> = state.values().cloned().collect();

    let offset = query.offset.unwrap_or(0);
    let limit = query.limit.unwrap_or(i32::MAX);
    let sort_order = query.sort_order.unwrap_or(SortOrder::Asc);
    let sort_field = query.sort_field;
    let filter_expression = query.filter_expression;

    // Sort state hashmap by value for the given key (sort_field) and order (sort_order) if provided in the query and save the result in output
    if sort_field.is_some() {
        let sort_field = sort_field.unwrap();

        output.sort_by(|a: &ContractDefinitionOutput, b: &ContractDefinitionOutput| {
            let a_contract = &a;
            let b_contract = &b;

            let ordering = match sort_field.as_str() {
                "@id" => a_contract.at_id.cmp(&b_contract.at_id),
                "@type" => a_contract.at_type.cmp(&b_contract.at_type),
                "accessPolicyId" => a_contract.access_policy_id.cmp(&b_contract.access_policy_id),
                "contractPolicyId" => a_contract.contract_policy_id.cmp(&b_contract.contract_policy_id),
                "createdAt" => a_contract.created_at.cmp(&b_contract.created_at),
                _ => std::cmp::Ordering::Equal,
            };

            if sort_order == SortOrder::Asc {
                ordering
            } else {
                ordering.reverse()
            }
        });
    }

    // Filter the output based on the filter_expression
    if !filter_expression.is_empty() {
        output = output.into_iter().filter(|(v)| {
            filter_expression.iter().all(|criterion| {
                evaluate_condition(v, &criterion.operand_left, &criterion.operator, &criterion.operand_right)
            })
        }).collect();
    }

    // Return only the requested range of results (based on offset and limit)
    output = if offset > output.len() as i32 {
        Vec::new()
    } else {
        output.into_iter()
            .skip(offset as usize)
            .take(limit as usize)
            .collect()
    };

    (StatusCode::OK, Json(output)).into_response()

}

fn evaluate_condition(contract: &ContractDefinitionOutput, operand_left: &serde_json::Value, operator: &str, operand_right: &serde_json::Value,) -> bool {
    let field_name = operand_left.as_str().unwrap_or("");

    match field_name {
        "@id" => compare_values(contract.at_id.as_deref(), operator, operand_right.as_str()),
        "@type" => compare_values(contract.at_type.as_deref(), operator, operand_right.as_str()),
        "accessPolicyId" => compare_values(contract.access_policy_id.as_deref(), operator, operand_right.as_str()),
        "contractPolicyId" => compare_values(contract.contract_policy_id.as_deref(), operator, operand_right.as_str()),
        "createdAt" => {
            if let Some(parsed_value) = operand_right.as_i64() {
                compare_values(contract.created_at, operator, Some(parsed_value))
            } else {
                false
            }
        }
        _ => false, // Unknown field
    }
}

fn compare_values<T: PartialOrd>(field_value: Option<T>, operator: &str, operand_right: Option<T>) -> bool {
    match (field_value, operand_right) {
        (Some(field_value), Some(operand_right)) => match operator {
            "=" => field_value == operand_right,
            "!=" => field_value != operand_right,
            ">" => field_value > operand_right,
            ">=" => field_value >= operand_right,
            "<" => field_value < operand_right,
            "<=" => field_value <= operand_right,
            _ => false,
        },
        _ => false,
    }
}

pub(crate) async fn get_contract_definition(State(state): State<SharedState>, Path(id): Path<String>,) -> impl IntoResponse {

    /// Gets a contract definition with the given ID
    ///
    /// # Example
    ///
    /// GET /v2/contractdefinitions/{id}
    ///
    /// Parameter:
    /// id: String (required)  - The ID of the contract definition
    ///
    /// Responses:
    /// 200 - The contract definition
    /// 400 - Request was malformed, e.g. id was null
    /// 404 - A contract definition with the given ID does not exist

    info!("Received contract request <> GET /v2/contractdefinitions/{} for id:\n{:#?}\n", id.clone(), id.clone());

    let state = state.lock().unwrap();
    match state.get(&id) {
        Some(output) => (StatusCode::OK, Json(output.clone())).into_response(),
        None => (StatusCode::NOT_FOUND, Json(error!("A contract definition with the given ID does not exist"))).into_response(),
    }
}

pub(crate) async fn delete_contract_definition(State(state): State<SharedState>, Path(id): Path<String>,) -> impl IntoResponse {

    /// Gets a contract definition with the given ID
    ///
    /// # Example
    ///
    /// DELETE /v2/contractdefinitions/{id}
    ///
    /// Parameter:
    /// id: String (required)  - The ID of the contract definition
    ///
    /// Responses:
    /// 204 - The contract definition was deleted successfully
    /// 400 - Request was malformed, e.g. id was null
    /// 404 - A contract definition with the given ID does not exist

    info!("Received contract deletion request <> DELETE /v2/contractdefinitions/{} for id:\n{:#?}\n", id.clone(), id.clone());

    let mut state = state.lock().unwrap();
    if state.remove(&id).is_some() {
        StatusCode::NO_CONTENT.into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}