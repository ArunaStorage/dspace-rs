// types
pub const DSPACE_TYPE_CONTRACT_NEGOTIATION: &'static str = concat!("https://w3id.org/dspace/v0.8/", "ContractNegotiation");
pub const DSPACE_TYPE_CONTRACT_NEGOTIATION_ERROR: &'static str = concat!("https://w3id.org/dspace/v0.8/", "ContractNegotiationError");

// messages
pub const DSPACE_TYPE_CONTRACT_REQUEST_MESSAGE: &'static str = concat!("https://w3id.org/dspace/v0.8/", "ContractRequestMessage");
pub const DSPACE_TYPE_CONTRACT_OFFER_MESSAGE: &'static str = concat!("https://w3id.org/dspace/v0.8/", "ContractOfferMessage");
pub const DSPACE_TYPE_CONTRACT_NEGOTIATION_EVENT_MESSAGE: &'static str = concat!("https://w3id.org/dspace/v0.8/", "ContractNegotiationEventMessage");
pub const DSPACE_TYPE_CONTRACT_AGREEMENT_MESSAGE: &'static str = concat!("https://w3id.org/dspace/v0.8/", "ContractAgreementMessage");
pub const DSPACE_TYPE_CONTRACT_AGREEMENT_VERIFICATION_MESSAGE: &'static str = concat!("https://w3id.org/dspace/v0.8/", "ContractAgreementVerificationMessage");
pub const DSPACE_TYPE_CONTRACT_AGREEMENT_TERMINATION_MESSAGE: &'static str = concat!("https://w3id.org/dspace/v0.8/", "ContractAgreementTerminationMessage");

// properties
pub const DSPACE_PROPERTY_EVENT_TYPE: &'static str = concat!("https://w3id.org/dspace/v0.8/", "eventType");
pub const DSPACE_PROPERTY_AGREEMENT: &'static str = concat!("https://w3id.org/dspace/v0.8/", "agreement");
pub const DSPACE_PROPERTY_OFFER: &'static str = concat!("https://w3id.org/dspace/v0.8/", "offer");
pub const DSPACE_PROPERTY_TIMESTAMP: &'static str = concat!("https://w3id.org/dspace/v0.8/", "timestamp");
#[deprecated(note = "Deprecated since EDC v0.5.1")]
pub const DSPACE_PROPERTY_CONSUMER_ID: &'static str = concat!("https://w3id.org/dspace/v0.8/", "consumerId");
#[deprecated(note = "Deprecated since EDC v0.5.1")]
pub const DSPACE_PROPERTY_PROVIDER_ID: &'static str = concat!("https://w3id.org/dspace/v0.8/", "providerId");

// event types
pub const DSPACE_VALUE_NEGOTIATION_EVENT_TYPE_ACCEPTED: &'static str = concat!("https://w3id.org/dspace/v0.8/", "ACCEPTED");
pub const DSPACE_VALUE_NEGOTIATION_EVENT_TYPE_FINALIZED: &'static str = concat!("https://w3id.org/dspace/v0.8/", "FINALIZED");

// negotiation states
pub const DSPACE_VALUE_NEGOTIATION_STATE_REQUESTED: &'static str = concat!("https://w3id.org/dspace/v0.8/", "REQUESTED");
pub const DSPACE_VALUE_NEGOTIATION_STATE_OFFERED: &'static str = concat!("https://w3id.org/dspace/v0.8/", "OFFERED");
pub const DSPACE_VALUE_NEGOTIATION_STATE_ACCEPTED: &'static str = concat!("https://w3id.org/dspace/v0.8/", "ACCEPTED");
pub const DSPACE_VALUE_NEGOTIATION_STATE_AGREED: &'static str = concat!("https://w3id.org/dspace/v0.8/", "AGREED");
pub const DSPACE_VALUE_NEGOTIATION_STATE_VERIFIED: &'static str = concat!("https://w3id.org/dspace/v0.8/", "VERIFIED");
pub const DSPACE_VALUE_NEGOTIATION_STATE_FINALIZED: &'static str = concat!("https://w3id.org/dspace/v0.8/", "FINALIZED");
pub const DSPACE_VALUE_NEGOTIATION_STATE_TERMINATED: &'static str = concat!("https://w3id.org/dspace/v0.8/", "TERMINATED");