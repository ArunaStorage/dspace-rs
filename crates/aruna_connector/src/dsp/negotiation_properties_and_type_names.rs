use odrl::name_spaces::DSPACE_NS;

// types
pub const DSPACE_TYPE_CONTRACT_NEGOTIATION: &'static str = concat!(DSPACE_NS, "ContractNegotiation");
pub const DSPACE_TYPE_CONTRACT_NEGOTIATION_ERROR: &'static str = concat!(DSPACE_NS, "ContractNegotiationError");

// messages
pub const DSPACE_TYPE_CONTRACT_REQUEST_MESSAGE: &'static str = concat!(DSPACE_NS, "ContractRequestMessage");
pub const DSPACE_TYPE_CONTRACT_OFFER_MESSAGE: &'static str = concat!(DSPACE_NS, "ContractOfferMessage");
pub const DSPACE_TYPE_CONTRACT_NEGOTIATION_EVENT_MESSAGE: &'static str = concat!(DSPACE_NS, "ContractNegotiationEventMessage");
pub const DSPACE_TYPE_CONTRACT_AGREEMENT_MESSAGE: &'static str = concat!(DSPACE_NS, "ContractAgreementMessage");
pub const DSPACE_TYPE_CONTRACT_AGREEMENT_VERIFICATION_MESSAGE: &'static str = concat!(DSPACE_NS, "ContractAgreementVerificationMessage");
pub const DSPACE_TYPE_CONTRACT_AGREEMENT_TERMINATION_MESSAGE: &'static str = concat!(DSPACE_NS, "ContractAgreementTerminationMessage");

// properties
pub const DSPACE_PROPERTY_EVENT_TYPE: &'static str = concat!(DSPACE_NS, "eventType");
pub const DSPACE_PROPERTY_AGREEMENT: &'static str = concat!(DSPACE_NS, "agreement");
pub const DSPACE_PROPERTY_OFFER: &'static str = concat!(DSPACE_NS, "offer");
pub const DSPACE_PROPERTY_TIMESTAMP: &'static str = concat!(DSPACE_NS, "timestamp");
#[deprecated(note = "Deprecated since EDC v0.5.1")]
pub const DSPACE_PROPERTY_CONSUMER_ID: &'static str = concat!(DSPACE_NS, "consumerId");
#[deprecated(note = "Deprecated since EDC v0.5.1")]
pub const DSPACE_PROPERTY_PROVIDER_ID: &'static str = concat!(DSPACE_NS, "providerId");

// event types
pub const DSPACE_VALUE_NEGOTIATION_EVENT_TYPE_ACCEPTED: &'static str = concat!(DSPACE_NS, "ACCEPTED");
pub const DSPACE_VALUE_NEGOTIATION_EVENT_TYPE_FINALIZED: &'static str = concat!(DSPACE_NS, "FINALIZED");

// negotiation states
pub const DSPACE_VALUE_NEGOTIATION_STATE_REQUESTED: &'static str = concat!(DSPACE_NS, "REQUESTED");
pub const DSPACE_VALUE_NEGOTIATION_STATE_OFFERED: &'static str = concat!(DSPACE_NS, "OFFERED");
pub const DSPACE_VALUE_NEGOTIATION_STATE_ACCEPTED: &'static str = concat!(DSPACE_NS, "ACCEPTED");
pub const DSPACE_VALUE_NEGOTIATION_STATE_AGREED: &'static str = concat!(DSPACE_NS, "AGREED");
pub const DSPACE_VALUE_NEGOTIATION_STATE_VERIFIED: &'static str = concat!(DSPACE_NS, "VERIFIED");
pub const DSPACE_VALUE_NEGOTIATION_STATE_FINALIZED: &'static str = concat!(DSPACE_NS, "FINALIZED");
pub const DSPACE_VALUE_NEGOTIATION_STATE_TERMINATED: &'static str = concat!(DSPACE_NS, "TERMINATED");