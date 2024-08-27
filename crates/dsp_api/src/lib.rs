#[macro_use]
extern crate serde_derive;

pub mod common {
    pub mod dsp_version;
    pub use self::dsp_version::DspVersion;
    pub use self::dsp_version::ProtocolVersion;
}

pub mod catalog {
    pub mod catalog_request_message;
    pub use self::catalog_request_message::CatalogRequestMessage;
    pub mod dataset_request_message;
    pub use self::dataset_request_message::DatasetRequestMessage;
    pub mod catalog;
    pub use self::catalog::Catalog;
    pub mod dataset;
    pub use self::dataset::Dataset;
    pub use self::dataset::AbstractDataset;
    pub use self::dataset::Resource;
    pub use self::dataset::Distribution;
    pub use self::dataset::DataService;
    pub use self::dataset::Reference;
    pub use self::dataset::MultiLanguage;
    pub mod catalog_error;
    pub use self::catalog_error::CatalogError;
}

pub mod contract_negotiation {
    pub mod contract_request_message;
    pub use self::contract_request_message::ContractRequestMessage;
    pub mod contract_offer_message;
    pub use self::contract_offer_message::ContractOfferMessage;
    pub mod contract_agreement_message;
    pub use self::contract_agreement_message::ContractAgreementMessage;
    pub mod contract_agreement_verification_message;
    pub use self::contract_agreement_verification_message::ContractAgreementVerificationMessage;
    pub mod contract_negotiation_event_message;
    pub use self::contract_negotiation_event_message::ContractNegotiationEventMessage;
    pub mod contract_negotiation_termination_message;
    pub use self::contract_negotiation_termination_message::ContractNegotiationTerminationMessage;
    pub mod contract_negotiation;
    pub use self::contract_negotiation::ContractNegotiation;
    pub mod contract_negotiation_error;
    pub use self::contract_negotiation_error::ContractNegotiationError;
}