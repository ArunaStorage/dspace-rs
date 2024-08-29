// DCAT
pub const DCAT_CATALOG_TYPE: &'static str = concat!("http://www.w3.org/ns/dcat#", "Catalog");
pub const DCAT_DATASET_TYPE: &'static str = concat!("http://www.w3.org/ns/dcat#", "Dataset");
pub const DCAT_DISTRIBUTION_TYPE: &'static str = concat!("http://www.w3.org/ns/dcat#", "Distribution");
pub const DCAT_DATA_SERVICE_TYPE: &'static str = concat!("http://www.w3.org/ns/dcat#", "DataService");
pub const DCAT_DATA_SERVICE_ATTRIBUTE: &'static str = concat!("http://www.w3.org/ns/dcat#", "service");
pub const DCAT_DATASET_ATTRIBUTE: &'static str = concat!("http://www.w3.org/ns/dcat#", "dataset");
pub const DCAT_DISTRIBUTION_ATTRIBUTE: &'static str = concat!("http://www.w3.org/ns/dcat#", "distribution");
pub const DCAT_ACCESS_SERVICE_ATTRIBUTE: &'static str = concat!("http://www.w3.org/ns/dcat#", "accessService");
pub const DCAT_ENDPOINT_URL_ATTRIBUTE: &'static str = concat!("http://www.w3.org/ns/dcat#", "endpointUrl");
pub const DCAT_ENDPOINT_DESCRIPTION_ATTRIBUTE: &'static str = concat!("http://www.w3.org/ns/dcat#", "endpointDescription");

// EDC
pub const EDC_CREATED_AT: &'static str = concat!("https://w3id.org/edc/v0.0.1/ns/", "createdAt");

// DCTERMS
pub const DCTERMS_FORMAT_ATTRIBUTE: &'static str = concat!("http://purl.org/dc/terms/", "format");
pub const DCTERMS_TERMS_ATTRIBUTE: &'static str = concat!("http://purl.org/dc/terms/", "terms");
pub const DCTERMS_ENDPOINT_URL_ATTRIBUTE: &'static str = concat!("http://purl.org/dc/terms/", "endpointUrl");

// ODRL
pub const ODRL_POLICY_ATTRIBUTE: &'static str = concat!("http://www.w3.org/ns/odrl/2/", "hasPolicy");
pub const ODRL_POLICY_TYPE_SET: &'static str = concat!("http://www.w3.org/ns/odrl/2/", "Set");
pub const ODRL_POLICY_TYPE_OFFER: &'static str = concat!("http://www.w3.org/ns/odrl/2/", "Offer");
pub const ODRL_POLICY_TYPE_AGREEMENT: &'static str = concat!("http://www.w3.org/ns/odrl/2/", "Agreement");
pub const ODRL_CONSTRAINT_TYPE: &'static str = concat!("http://www.w3.org/ns/odrl/2/", "Constraint");
pub const ODRL_LOGICAL_CONSTRAINT_TYPE: &'static str = concat!("http://www.w3.org/ns/odrl/2/", "LogicalConstraint");
pub const ODRL_TARGET_ATTRIBUTE: &'static str = concat!("http://www.w3.org/ns/odrl/2/", "target");
pub const ODRL_ASSIGNEE_ATTRIBUTE: &'static str = concat!("http://www.w3.org/ns/odrl/2/", "assignee");
pub const ODRL_ASSIGNER_ATTRIBUTE: &'static str = concat!("http://www.w3.org/ns/odrl/2/", "assigner");
pub const ODRL_PERMISSION_ATTRIBUTE: &'static str = concat!("http://www.w3.org/ns/odrl/2/", "permission");
pub const ODRL_PROHIBITION_ATTRIBUTE: &'static str = concat!("http://www.w3.org/ns/odrl/2/", "prohibition");
pub const ODRL_OBLIGATION_ATTRIBUTE: &'static str = concat!("http://www.w3.org/ns/odrl/2/", "obligation");
pub const ODRL_ACTION_ATTRIBUTE: &'static str = concat!("http://www.w3.org/ns/odrl/2/", "action");
pub const ODRL_ACTION_TYPE_ATTRIBUTE: &'static str = concat!("http://www.w3.org/ns/odrl/2/", "type");
pub const ODRL_CONSEQUENCE_ATTRIBUTE: &'static str = concat!("http://www.w3.org/ns/odrl/2/", "consequence");
pub const ODRL_REMEDY_ATTRIBUTE: &'static str = concat!("http://www.w3.org/ns/odrl/2/", "remedy");
pub const ODRL_INCLUDED_IN_ATTRIBUTE: &'static str = concat!("http://www.w3.org/ns/odrl/2/", "includedIn");
pub const ODRL_REFINEMENT_ATTRIBUTE: &'static str = concat!("http://www.w3.org/ns/odrl/2/", "refinement");
pub const ODRL_CONSTRAINT_ATTRIBUTE: &'static str = concat!("http://www.w3.org/ns/odrl/2/", "constraint");
pub const ODRL_LEFT_OPERAND_ATTRIBUTE: &'static str = concat!("http://www.w3.org/ns/odrl/2/", "leftOperand");
pub const ODRL_OPERATOR_TYPE: &'static str = concat!("http://www.w3.org/ns/odrl/2/", "Operator");
pub const ODRL_OPERATOR_ATTRIBUTE: &'static str = concat!("http://www.w3.org/ns/odrl/2/", "operator");
pub const ODRL_RIGHT_OPERAND_ATTRIBUTE: &'static str = concat!("http://www.w3.org/ns/odrl/2/", "rightOperand");
pub const ODRL_DUTY_ATTRIBUTE: &'static str = concat!("http://www.w3.org/ns/odrl/2/", "duty");
pub const ODRL_AND_CONSTRAINT_ATTRIBUTE: &'static str = concat!("http://www.w3.org/ns/odrl/2/", "and");
pub const ODRL_AND_SEQUENCE_CONSTRAINT_ATTRIBUTE: &'static str = concat!("http://www.w3.org/ns/odrl/2/", "andSequence");
pub const ODRL_OR_CONSTRAINT_ATTRIBUTE: &'static str = concat!("http://www.w3.org/ns/odrl/2/", "or");
pub const ODRL_XONE_CONSTRAINT_ATTRIBUTE: &'static str = concat!("http://www.w3.org/ns/odrl/2/", "xone");
pub const ODRL_USE_ACTION_ATTRIBUTE: &'static str = concat!("http://www.w3.org/ns/odrl/2/", "use");
pub const ODRL_PROFILE_ATTRIBUTE: &'static str = concat!("http://www.w3.org/ns/odrl/2/", "profile");

// DSAPCE
pub const DSPACE_PROPERTY_PARTICIPANT_ID: &'static str = concat!("https://w3id.org/dspace/v0.8/", "participantId");