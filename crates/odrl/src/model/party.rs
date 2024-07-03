use crate::model::constraint::Constraint;
use crate::model::type_alias::IRI;


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Function {

    Assigner,
    Assignee,

}

impl Default for Function {
    fn default() -> Function {
        Function::Assigner
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Party {

    pub uid: Option<IRI>,
    pub part_of: Vec<PartyCollection>,
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

    pub source: Option<IRI>,
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
