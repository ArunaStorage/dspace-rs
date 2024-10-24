use axum::{routing::{get, post}, Json, Router, ServiceExt};
use serde::{Deserialize, Serialize};
use tracing::{Level};
use tracing_subscriber;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use axum::extract::Path;
use axum::http::HeaderMap;
use tracing::{error, info};
use dsp_api::catalog::{AbstractDataset, Catalog, DataService, Dataset, Distribution, MultiLanguage, Resource};
use dsp_api::contract_negotiation::{AbstractPolicyRule, Action, Constraint, Duty, LeftOperand, MessageOffer, Offer, Operator, Permission, PolicyClass, RightOperand, Target};

pub mod api {
    pub mod catalog;
    pub mod contract_negotiation_provider;
}

use crate::api::catalog;
use crate::api::contract_negotiation_provider as negotiation_provider;

async fn debug_route(headers: HeaderMap, Path(path): Path<String>, value: Option<Json<serde_json::Value>>) {
    info!("Debug route called path {:#?} with value: {:#?}\nHeader: {:#?}", path, value, headers);
}

async fn initiate_catalog() -> Catalog {
    
    let context: HashMap<String, serde_json::Value> = HashMap::from([("dspace".to_string(), serde_json::Value::String("https://w3id.org/dspace/2024/1/context.json".to_string()))]);

    let first_dataset = Dataset::new(AbstractDataset {
        resource: Resource {
            id: Some("Test".to_string()),
            keywords: None,
            themes: None,
            conforms_to: None,
            creator: None,
            descriptions: None,
            identifier: None,
            issued: None,
            modified: None,
            title: None,
        },
        policies: Some(vec![Offer { message_offer: MessageOffer { policy_class: PolicyClass {
            abstract_policy_rule: AbstractPolicyRule { assigner: Some("aruna-connector".to_string()), assignee: None },
            id: "test-policy".to_string(),
            profile: vec![],
            permission: vec![Permission {
                abstract_policy_rule: AbstractPolicyRule { assigner: Some("aruna-connector".to_string()), assignee: None },
                action: Action::Use,
                constraint: vec![],
                duty: None,
            }],
            obligation: vec![],
            target: Target { id: "Test".to_string() },
        }, odrl_type: "odrl:Offer".to_string() } }]),
        distributions: None,
    });
    let second_dataset = Dataset::new(AbstractDataset {
        resource: Resource {
            id: Some("3dd1add8-4d2d-569e-d634-8394a8836a88".to_string()),
            keywords: Some(vec!["traffic".to_string()]),
            themes: None,
            conforms_to: None,
            creator: None,
            descriptions: Some(vec![MultiLanguage { value: "Traffic data sample extract".to_string(), language: "en".to_string() }]),
            identifier: Some("3dd1add8-4d2d-569e-d634-8394a8836a88".to_string()),
            issued: None,
            modified: None,
            title: Some("Traffic Data".to_string()),
        },
        policies: Some(vec![Offer { message_offer: MessageOffer { policy_class: PolicyClass {
            abstract_policy_rule: AbstractPolicyRule { assigner: Some("http://example.com/Provider".to_string()), assignee: None },
            id: "3dd1add8-4d2d-569e-d634-8394a8836a88".to_string(),
            profile: vec![],
            permission: vec![Permission {
                abstract_policy_rule: AbstractPolicyRule { assigner: Some("http://example.com/Provider".to_string()), assignee: None },
                action: Action::Use,
                constraint: vec![Constraint {
                    right_operand: Some(RightOperand::String("http://example.org/EU".to_string())),
                    right_operand_reference: None,
                    left_operand: LeftOperand::AbsolutePosition,
                    operator: Operator::Eq,
                }],
                duty: Some(Duty {
                    abstract_policy_rule: AbstractPolicyRule { assigner: None, assignee: None },
                    id: None,
                    action: Action::Attribution,
                    constraint: vec![],
                }),
            }],
            obligation: vec![],
            target: Target { id: "3dd1add8-4d2d-569e-d634-8394a8836a88".to_string() },
        }, odrl_type: "odrl:Offer".to_string() } }]),
        distributions: Some(vec![Distribution {
            title: None,
            descriptions: vec![],
            issued: None,
            modified: None,
            policy: vec![],
            access_services: vec![DataService {
                resource: Resource {
                    id: None,
                    keywords: None,
                    themes: None,
                    conforms_to: None,
                    creator: None,
                    descriptions: None,
                    identifier: None,
                    issued: None,
                    modified: None,
                    title: None,
                },
                endpoint_description: None,
                endpoint_url: Some("https://provider-a.com/connector".to_string()),
                serves_datasets: None,
            }],
        }]),
    });
    let datasets = Vec::from([first_dataset, second_dataset]);

    let data_service = vec![DataService {
        resource: Resource {
            id: None,
            keywords: None,
            themes: None,
            conforms_to: None,
            creator: None,
            descriptions: None,
            identifier: None,
            issued: None,
            modified: None,
            title: None,
        },
        endpoint_description: None,
        endpoint_url: Some("https://aruna-connector/public".to_string()),
        serves_datasets: None,
    }];

    let catalog = Catalog::new(AbstractDataset {
        resource: Resource {
            id: None,
            keywords: None,
            themes: None,
            conforms_to: None,
            creator: None,
            descriptions: None,
            identifier: None,
            issued: None,
            modified: None,
            title: None,
        },
        policies: None,
        distributions: None,
    }, context, "dcat:Catalog".to_string(), Some(datasets), data_service, None, None);

    catalog
}

async fn initialize_shared_catalog() -> Arc<Mutex<Catalog>> {
    let catalog = initiate_catalog().await;
    Arc::new(Mutex::new(catalog))
}

#[tokio::main]
async fn main() {

    // initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();


    // Shared states to store
    let shared_catalog_state = initialize_shared_catalog().await; // Catalog
    let shared_negotiation_state = Arc::new(Mutex::new(HashMap::new())); // Contract Negotiation

    let catalog_route = Router::new()
        .route("/request", post(catalog::get_catalog))
        .route("/datasets/:id", get(catalog::get_dataset))
        .with_state(shared_catalog_state);

    let negotiation_route = Router::new()
        .route("/:pid", get(negotiation_provider::get_negotiation))
        .route("/request", post(negotiation_provider::request_negotiation))
        .route("/:pid/request", post(negotiation_provider::make_offer))
        .route("/:pid/events", post(negotiation_provider::accept_offer))
        .route("/:pid/agreement/verification", post(negotiation_provider::verify_agreement))
        .route("/:pid/termination", post(negotiation_provider::terminate_negotiation))
        .with_state(shared_negotiation_state);
/*

    // routes for contract definitions
    let contract_definitions_routes = Router::new()
        // Contract Definitions
        // `PUT /v2/contractdefinitions` goes to `update_contract_definition`
        .route("/", put(contract_definition::update_contract_definition))
        // `POST /v2/contractdefinitions` goes to `create_contract_definition`
        .route("/", post(contract_definition::create_contract_definition))
        // `POST /v2/contractdefinitions/{id}/request` goes to `request_contract_definition`
        .route("/request", post(contract_definition::request_contract_definition))
        // `GET /v2/contractdefinitions/{id}` goes to `get_contract_definition`
        .route("/:id", get(contract_definition::get_contract_definition))
        // `DELETE /v2/contractdefinitions/{id}` goes to `delete_contract_definition`
        .route("/:id", delete(contract_definition::delete_contract_definition))
        // add shared state to the app
        .with_state(shared_contract_definitions_state);

    // routes for contract negotiations
    let contract_negotiations_routes = Router::new()
        // Contract Negotiations
        // `POST /v2/contractnegotiations` goes to `initiate_contract_negotiation`
        .route("/", post(contract_negotiation::initiate_contract_negotiation))
        // `POST /v2/contractnegotiations/request` goes to `request_contract_negotiation`
        .route("/request", post(contract_negotiation::request_contract_negotiation))
        // `GET /v2/contractnegotiations/{id}` goes to `get_contract_negotiation`
        .route("/:id", get(contract_negotiation::get_contract_negotiation))
        // `GET /v2/contractnegotiations/{id}/agreement` goes to `get_agreement_by_negotiation_id`
        .route("/:id/agreement", get(contract_negotiation::get_agreement_by_negotiation_id))
        // `GET /v2/contractnegotiations/{id}/state` goes to `get_negotiation_state`
        .route("/:id/state", get(contract_negotiation::get_negotiation_state))
        // `POST /v2/contractnegotiations/{id}/terminate` goes to `terminate_contract_negotiation`
        .route("/:id/terminate", post(contract_negotiation::terminate_contract_negotiation))
        // add shared state to the app
        .with_state(shared_contract_negotiations_state);
*/

    // Path(path): Path<String>

    // create our app by nesting the routes
    let api_routes = Router::new()
        .nest("/catalog", catalog_route)
        .nest("/negotiations", negotiation_route)
        .route("/*path", get(debug_route).post(debug_route).put(debug_route).delete(debug_route));
        /*.nest("/v2/contractagreements", contract_agreement_routes)
        .nest("/v2/contractdefinitions", contract_definitions_routes)
        .nest("/v2/contractnegotiations", contract_negotiations_routes);*/


    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("localhost:3000").await.unwrap();
    axum::serve(listener, api_routes.into_make_service()).await.unwrap();

}
