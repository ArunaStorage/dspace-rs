use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    Json,
    response::IntoResponse,
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use axum::extract::Path;
use chrono::Utc;
use tracing::{error, info};
use edc_api::{ContractDefinitionInput, ContractDefinitionOutput, ContractNegotiation, ContractRequest, IdResponse, QuerySpec, TerminateNegotiationSchema};
use edc_api::contract_negotiation::EnumType;
use edc_api::ContractNegotiationState;
use edc_api::query_spec::SortOrder;
use odrl::functions::state_machine::{ConsumerStateMachine, ProviderStateMachine, ConsumerState, ProviderState};

// Shared state to store contract negotiations and their corresponding consumer and provider state machines
type SharedState = Arc<Mutex<HashMap<String, (ContractNegotiation, ConsumerStateMachine<ConsumerState>, ProviderStateMachine<ProviderState>)>>>;

// TODO: State Machine state handling

pub(crate) async fn initiate_contract_negotiation(headers: HeaderMap, State(state): State<SharedState>, Json(input): Json<ContractRequest>,) -> impl IntoResponse {

    /// Initiates a contract negotiation for a given offer and with the given counterpart.
    /// Please note that successfully invoking this endpoint only means that the negotiation was initiated.
    /// Clients must poll the /{id}/state endpoint to track the state
    ///
    /// # Example
    ///
    /// Request Body:
    /// {
    ///   "@context": {
    ///     "@vocab": "https://w3id.org/edc/v0.0.1/ns/"
    ///   },
    ///   "@type": "https://w3id.org/edc/v0.0.1/ns/ContractRequest",
    ///   "counterPartyAddress": "http://provider-address",
    ///   "protocol": "dataspace-protocol-http",
    ///   "policy": {
    ///     "@context": "http://www.w3.org/ns/odrl.jsonld",
    ///     "@type": "odrl:Offer",
    ///     "@id": "offer-id",
    ///     "assigner": "providerId",
    ///     "permission": [],
    ///     "prohibition": [],
    ///     "obligation": [],
    ///     "target": "assetId"
    ///   },
    ///   "callbackAddresses": [
    ///     {
    ///       "transactional": false,
    ///      "uri": "http://callback/url",
    ///       "events": [
    ///         "contract.negotiation",
    ///         "transfer.process"
    ///       ],
    ///       "authKey": "auth-key",
    ///       "authCodeId": "auth-code-id"
    ///     }
    ///   ]
    /// }
    ///
    /// Responses:
    /// 200 - The negotiation was successfully initiated. Returns the contract negotiation ID and created timestamp
    ///       {
    ///            "@context": {
    ///                "@vocab": "https://w3id.org/edc/v0.0.1/ns/"
    ///            },
    ///            "@id": "id-value",
    ///            "createdAt": 1688465655
    ///        }
    /// 400 - Request was malformed, e.g. id was null

    info!("Received initiate contract negotiation <> POST /v2/contractnegotiations:\n{:#?}\n", input);

    let mut state = state.lock().unwrap();
    let id = uuid::Uuid::new_v4().to_string();
    let created_at = Utc::now().timestamp();

    let negotiation = ContractNegotiation {
        context: input.context,
        at_id: Some(id.clone()),
        at_type: Some(input.at_type.clone().unwrap_or_else(|| "ContractNegotiation".to_string())),
        callback_addresses: input.callback_addresses.clone(),
        contract_agreement_id: None,    // Set to None as this is a new negotiation; will be set when agreement is reached and negotiation is in state finalized
        counter_party_address: Some(input.counter_party_address.clone()),
        counter_party_id: Some(input.provider_id.clone().unwrap_or_else(|| input.policy.clone().unwrap().assigner.clone())),
        error_detail: None,            // Set to None as this is a new negotiation; will be set when negotiation is in state failed
        protocol: Some(input.protocol.clone()),
        state: ContractNegotiationState::Initial,
        r#type: Some(EnumType::Consumer),   // Only consumer initiates the negotiation
                                            // (according to https://github.com/eclipse-edc/Connector/blob/main/core/control-plane/control-plane-contract/src/main/java/org/eclipse/edc/connector/controlplane/contract/negotiation/ConsumerContractNegotiationManagerImpl.java )
        created_at: Some(created_at),
    };

    let consumer_state_machine = ConsumerStateMachine::new(headers.get("host").unwrap().to_str().unwrap(), input.counter_party_address.clone().as_str());
    let provider_state_machine = ProviderStateMachine::new(input.counter_party_address.clone().as_str(), headers.get("host").unwrap().to_str().unwrap());

    info!("Negotiation state machine initialized for consumer: {:#?}\n", consumer_state_machine);
    info!("Negotiation state machine initialized for provider: {:#?}\n", provider_state_machine);

    state.insert(id.clone(), (negotiation.clone(), consumer_state_machine, provider_state_machine));

    let id_response = IdResponse {
        at_id: Some(id.clone()),
        created_at: Some(created_at.clone()),
    };

    (StatusCode::OK, Json(id_response)).into_response()

}

pub(crate) async fn request_contract_negotiation(State(state): State<SharedState>, Json(query): Json<QuerySpec>,) -> impl IntoResponse {

    /// Returns all contract negotiations according to a query
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
    /// 200 - The contract negotiations that match the query
    /// 400 - Request was malformed

    info!("Received contract negotiation request <> POST /v2/contractnegotiations/request for query:\n{:#?}\n", query);

    let state = state.lock().unwrap();

    // Collect all contract negotiations into a vector
    let mut output: Vec<ContractNegotiation> = state.values().map(|(contract_negotiation, _, _)| contract_negotiation.clone()).collect();

    let offset = query.offset.unwrap_or(0);
    let limit = query.limit.unwrap_or(i32::MAX);
    let sort_order = query.sort_order.unwrap_or(SortOrder::Asc);
    let sort_field = query.sort_field;
    let filter_expression = query.filter_expression;

    // Sort state hashmap by value for the given key (sort_field) and order (sort_order) if provided in the query and save the result in output
    if sort_field.is_some() {
        let sort_field = sort_field.unwrap();

        output.sort_by(|a: &ContractNegotiation, b: &ContractNegotiation| {
            let a_contract = &a;
            let b_contract = &b;

            let ordering = match sort_field.as_str() {
                "@id" => a_contract.at_id.cmp(&b_contract.at_id),
                "@type" => a_contract.at_type.cmp(&b_contract.at_type),
                "contractAgreementId" => a_contract.contract_agreement_id.cmp(&b_contract.contract_agreement_id),
                "counterPartyAddress" => a_contract.counter_party_address.cmp(&b_contract.counter_party_address),
                "counterPartyId" => a_contract.counter_party_id.cmp(&b_contract.counter_party_id),
                "protocol" => a_contract.protocol.cmp(&b_contract.protocol),
                "state" => a_contract.state.cmp(&b_contract.state),
                "type" => a_contract.r#type.cmp(&b_contract.r#type),
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

fn evaluate_condition(contract: &ContractNegotiation, operand_left: &serde_json::Value, operator: &str, operand_right: &serde_json::Value,) -> bool {
    let field_name = operand_left.as_str().unwrap_or("");

    match field_name {
        "@id" => compare_values(contract.at_id.as_deref(), operator, operand_right.as_str()),
        "@type" => compare_values(contract.at_type.as_deref(), operator, operand_right.as_str()),
        "contractAgreementId" => compare_values(contract.contract_agreement_id.as_deref(), operator, operand_right.as_str()),
        "counterPartyAddress" => compare_values(contract.counter_party_address.as_deref(), operator, operand_right.as_str()),
        "counterPartyId" => compare_values(contract.counter_party_id.as_deref(), operator, operand_right.as_str()),
        "protocol" => compare_values(contract.protocol.as_deref(), operator, operand_right.as_str()),
        "state" => compare_values(Some(contract.state.clone()), operator, Some(serde_json::from_value(operand_right.clone()).unwrap())),
        "type" => compare_values(contract.r#type, operator, Some(serde_json::from_value::<EnumType>(operand_right.clone()).unwrap())),
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

pub(crate) async fn get_contract_negotiation(State(state): State<SharedState>, Path(id): Path<String>,) -> impl IntoResponse {

    /// Gets a contract negotiation with the given ID
    ///
    /// # Example
    ///
    /// Request:
    /// GET /v2/contractnegotiations/{id}
    ///
    /// Parameter:
    /// id: String (required)  - The ID of the contract
    ///
    /// Responses:
    /// 200 - The contract negotiation
    /// 400 - Request was malformed, e.g. id was null
    /// 404 - An contract negotiation with the given ID does not exist

    info!("Received contract negotiation request <> GET /v2/contractnegotiations/{} for id:\n{:#?}\n", id.clone(), id.clone());

    let state = state.lock().unwrap();
    match state.get(&id) {
        Some(output) => (StatusCode::OK, Json(output.0.clone())).into_response(),
        None => (StatusCode::NOT_FOUND, Json(error!("A contract negotiation with the given ID does not exist"))).into_response(),
    }
}

pub(crate) async fn get_agreement_by_negotiation_id() {
    // TODO
}

pub(crate) async fn get_negotiation_state(State(state): State<SharedState>, Path(id): Path<String>,) -> impl IntoResponse {

    /// Gets the state of a contract negotiation with the given ID
    ///
    /// # Example
    ///
    /// Request:
    /// GET /v2/contractnegotiations/{id}/state
    ///
    /// Parameter:
    /// id: String (required)  - The ID of the contract negotiation
    ///
    /// Responses:
    /// 200 - The contract negotiation's state
    /// 400 - Request was malformed, e.g. id was null
    /// 404 - An contract negotiation with the given ID does not exist

    info!("Received contract negotiation state request <> GET /v2/contractnegotiations/{}/state for id:\n{:#?}\n", id.clone(), id.clone());

    let state = state.lock().unwrap();
    match state.get(&id) {
        Some(output) => (StatusCode::OK, Json(output.0.state.clone())).into_response(),
        None => (StatusCode::NOT_FOUND, Json(error!("A contract negotiation with the given ID does not exist"))).into_response(),
    }

}

pub(crate) async fn terminate_contract_negotiation(State(state): State<SharedState>, Path(id): Path<String>, Json(termination_request): Json<TerminateNegotiationSchema>) -> impl IntoResponse {

    /// Terminates the contract negotiation.
    ///
    /// # Example
    ///
    /// Request:
    /// POST /v2/contractnegotiations/{id}/terminate
    ///
    /// Parameter:
    /// id: String (required)  - The ID of the contract negotiation
    ///
    /// Request Body:
    /// {
    ///   "@context": {
    ///     "@vocab": "https://w3id.org/edc/v0.0.1/ns/"
    ///   },
    ///   "@type": "https://w3id.org/edc/v0.0.1/ns/TerminateNegotiation",
    ///   "@id": "negotiation-id",
    ///   "reason": "a reason to terminate"
    /// }
    ///
    /// Responses:
    /// 200 - ContractNegotiation is terminating
    /// 400 - Request was malformed
    /// 404 - An contract negotiation with the given ID does not exist

    let reason = termination_request.reason.clone().unwrap_or_else(|| "No reason provided".to_string());

    info!("Received contract negotiation termination request <> POST /v2/contractnegotiations/{}/terminate for id:\n{:#?}\nwith reason:{:#?}\n", id.clone(), id.clone(), reason.clone());

    let mut state = state.lock().unwrap();

    if state.contains_key(&id) {
        let (negotiation, csm, psm) = state.get(&id).unwrap();
        let mut terminating_negotiation = negotiation.clone();
        let mut terminating_csm = csm.clone();
        let mut terminating_psm = psm.clone();
        terminating_negotiation.error_detail = Some(reason.clone());
        terminating_negotiation.state = ContractNegotiationState::Terminating;
        terminating_csm.transition_to_terminating(reason.clone().as_str());
        terminating_psm.transition_to_terminating(reason.clone().as_str());
        state.insert(id.clone(), (terminating_negotiation.clone(), terminating_csm.clone(), terminating_psm.clone()));
        StatusCode::OK.into_response()
    } else {
        (StatusCode::NOT_FOUND, Json(error!("A contract negotiation with the given ID does not exist"))).into_response()
    }

}