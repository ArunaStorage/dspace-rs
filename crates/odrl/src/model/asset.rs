use crate::model::policy::Policy;
use crate::model::constraint::Constraint;
use crate::model::type_alias::IRI;

extern crate edc_api as api;


#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Relation {

    pub target: Option<Box<Asset>>,

}

impl Relation {

    pub fn new(target: Option<Asset>) -> Relation {
        Relation {
            target: match target {
                Some(asset) => Some(Box::new(asset)),
                None => None,
            }
        }
    }

}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct AssetCollection {

    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub source: Option<IRI>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub edc_type : Option<String>,
    #[serde(rename = "@type", skip)]
    pub refinement: Vec<Constraint>,

}

impl AssetCollection {

    pub fn new(source: Option<IRI>, edc_type : Option<String>, refinement: Vec<Constraint>) -> AssetCollection {
        AssetCollection {
            source,
            edc_type,
            refinement,
        }
    }

}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Asset {

    // Based on the ODRL model
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub context: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub uid: Option<IRI>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub edc_type : Option<String>,
    #[serde(rename = "partOf", skip)]
    pub part_of: Vec<AssetCollection>,
    #[serde(rename = "relation", skip)]
    pub relation: Relation,
    #[serde(rename = "hasPolicy", skip_serializing_if = "Option::is_none")]
    pub has_policy: Option<Policy>, // Target Policy Property

    // Needed for API
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "privateProperties", skip_serializing_if = "Option::is_none")]
    pub private_properties: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "dataAddress", skip_serializing_if = "Option::is_none")]
    pub data_address: Option<Box<api::DataAddress>>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
}

impl Asset {

    pub fn new(context: Option<std::collections::HashMap<String, serde_json::Value>>, uid: Option<IRI>, edc_type : Option<String>, part_of: Vec<AssetCollection>, relation: Relation, has_policy: Option<Policy>,
               properties: Option<::std::collections::HashMap<String, serde_json::Value>>, private_properties: Option<::std::collections::HashMap<String, serde_json::Value>>, data_address: Option<Box<api::DataAddress>>, created_at: Option<i64>) -> Asset {
        Asset {
            context,
            uid,
            edc_type,
            part_of,
            relation,
            has_policy,
            properties,
            private_properties,
            data_address,
            created_at,
        }
    }

}