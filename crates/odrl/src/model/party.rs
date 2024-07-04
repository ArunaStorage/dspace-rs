use crate::model::constraint::Constraint;
use crate::model::type_alias::IRI;


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Function {

    #[serde(rename = "assigner")]
    Assigner,
    #[serde(rename = "assignee")]
    Assignee,

}

impl Default for Function {
    fn default() -> Function {
        Function::Assigner
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Party {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<IRI>,
    #[serde(rename = "partOf", skip_serializing_if = "Vec::is_empty")]
    pub part_of: Vec<PartyCollection>,
    #[serde(skip_serializing)]
    pub function: Function,

}

impl Party {

    pub fn new(uid: Option<IRI>, part_of: Vec<PartyCollection>, function: Function) -> Party {
        Party {
            uid,
            part_of,
            function
        }
    }

}


#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct PartyCollection {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<IRI>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub refinement: Vec<Constraint>,

}

impl PartyCollection {

    pub fn new(source: Option<IRI>, refinement: Vec<Constraint>) -> PartyCollection {
        PartyCollection {
            source,
            refinement
        }
    }

}
