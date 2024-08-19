use odrl::name_spaces::{DCAT_NS, DCTERMS_NS, DSPACE_NS, EDC_NS, ODRL_NS};

// DCAT
pub const DCAT_CATALOG_TYPE: &'static str = concat!(DCAT_NS, "Catalog");
pub const DCAT_DATASET_TYPE: &'static str = concat!(DCAT_NS, "Dataset");
pub const DCAT_DISTRIBUTION_TYPE: &'static str = concat!(DCAT_NS, "Distribution");
pub const DCAT_DATA_SERVICE_TYPE: &'static str = concat!(DCAT_NS, "DataService");
pub const DCAT_DATA_SERVICE_ATTRIBUTE: &'static str = concat!(DCAT_NS, "service");
pub const DCAT_DATASET_ATTRIBUTE: &'static str = concat!(DCAT_NS, "dataset");
pub const DCAT_DISTRIBUTION_ATTRIBUTE: &'static str = concat!(DCAT_NS, "distribution");
pub const DCAT_ACCESS_SERVICE_ATTRIBUTE: &'static str = concat!(DCAT_NS, "accessService");
pub const DCAT_ENDPOINT_URL_ATTRIBUTE: &'static str = concat!(DCAT_NS, "endpointUrl");
pub const DCAT_ENDPOINT_DESCRIPTION_ATTRIBUTE: &'static str = concat!(DCAT_NS, "endpointDescription");

// EDC
pub const EDC_CREATED_AT: &'static str = concat!(EDC_NS, "createdAt");

// DCTERMS
pub const DCTERMS_FORMAT_ATTRIBUTE: &'static str = concat!(DCTERMS_NS, "format");
pub const DCTERMS_TERMS_ATTRIBUTE: &'static str = concat!(DCTERMS_NS, "terms");
pub const DCTERMS_ENDPOINT_URL_ATTRIBUTE: &'static str = concat!(DCTERMS_NS, "endpointUrl");

// ODRL
pub const ODRL_POLICY_ATTRIBUTE: &'static str = concat!(ODRL_NS, "hasPolicy");
pub const ODRL_POLICY_TYPE_SET: &'static str = concat!(ODRL_NS, "Set");
pub const ODRL_POLICY_TYPE_OFFER: &'static str = concat!(ODRL_NS, "Offer");
pub const ODRL_POLICY_TYPE_AGREEMENT: &'static str = concat!(ODRL_NS, "Agreement");
pub const ODRL_CONSTRAINT_TYPE: &'static str = concat!(ODRL_NS, "Constraint");
pub const ODRL_LOGICAL_CONSTRAINT_TYPE: &'static str = concat!(ODRL_NS, "LogicalConstraint");
pub const ODRL_TARGET_ATTRIBUTE: &'static str = concat!(ODRL_NS, "target");
pub const ODRL_ASSIGNEE_ATTRIBUTE: &'static str = concat!(ODRL_NS, "assignee");
pub const ODRL_ASSIGNER_ATTRIBUTE: &'static str = concat!(ODRL_NS, "assigner");
pub const ODRL_PERMISSION_ATTRIBUTE: &'static str = concat!(ODRL_NS, "permission");
pub const ODRL_PROHIBITION_ATTRIBUTE: &'static str = concat!(ODRL_NS, "prohibition");
pub const ODRL_OBLIGATION_ATTRIBUTE: &'static str = concat!(ODRL_NS, "obligation");
pub const ODRL_ACTION_ATTRIBUTE: &'static str = concat!(ODRL_NS, "action");
pub const ODRL_ACTION_TYPE_ATTRIBUTE: &'static str = concat!(ODRL_NS, "type");
pub const ODRL_CONSEQUENCE_ATTRIBUTE: &'static str = concat!(ODRL_NS, "consequence");
pub const ODRL_REMEDY_ATTRIBUTE: &'static str = concat!(ODRL_NS, "remedy");
pub const ODRL_INCLUDED_IN_ATTRIBUTE: &'static str = concat!(ODRL_NS, "includedIn");
pub const ODRL_REFINEMENT_ATTRIBUTE: &'static str = concat!(ODRL_NS, "refinement");
pub const ODRL_CONSTRAINT_ATTRIBUTE: &'static str = concat!(ODRL_NS, "constraint");
pub const ODRL_LEFT_OPERAND_ATTRIBUTE: &'static str = concat!(ODRL_NS, "leftOperand");
pub const ODRL_OPERATOR_TYPE: &'static str = concat!(ODRL_NS, "Operator");
pub const ODRL_OPERATOR_ATTRIBUTE: &'static str = concat!(ODRL_NS, "operator");
pub const ODRL_RIGHT_OPERAND_ATTRIBUTE: &'static str = concat!(ODRL_NS, "rightOperand");
pub const ODRL_DUTY_ATTRIBUTE: &'static str = concat!(ODRL_NS, "duty");
pub const ODRL_AND_CONSTRAINT_ATTRIBUTE: &'static str = concat!(ODRL_NS, "and");
pub const ODRL_AND_SEQUENCE_CONSTRAINT_ATTRIBUTE: &'static str = concat!(ODRL_NS, "andSequence");
pub const ODRL_OR_CONSTRAINT_ATTRIBUTE: &'static str = concat!(ODRL_NS, "or");
pub const ODRL_XONE_CONSTRAINT_ATTRIBUTE: &'static str = concat!(ODRL_NS, "xone");
pub const ODRL_USE_ACTION_ATTRIBUTE: &'static str = concat!(ODRL_NS, "use");
pub const ODRL_PROFILE_ATTRIBUTE: &'static str = concat!(ODRL_NS, "profile");

// DSAPCE
pub const DSPACE_PROPERTY_PARTICIPANT_ID: &'static str = concat!(DSPACE_NS, "participantId");