use crate::model::constraint::Constraint;
use crate::model::constraint::LogicalConstraint;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Refinements {
    Constraints(Vec<Constraint>),
    LogicalConstraints(Vec<LogicalConstraint>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {

    pub name: String,
    pub refinements: Option<Refinements>,
    pub included_in: Option<Box<Action>>, // Use Box to allow recursive type definition
    pub implies: Vec<Box<Action>>,

}

impl Action {

    pub fn new(name: &str, refinements: Option<Refinements>, included_in: Option<Action>, implies: Vec<Action>) -> Action {
        Action {
            name: name.to_string(),
            refinements,
            included_in: match included_in {
                Some(action) => Some(Box::new(action)),
                None => None,
            },
            implies: implies.into_iter().map(Box::new).collect(),
        }
    }

    // Function to create the two top level actions "use" and "transfer"
    pub fn init_top_level() -> (Action, Action) {
        let use_action = Action {
            name: "use".to_string(),
            refinements: None,
            included_in: None,
            implies: vec![],
        };
        let transfer_action = Action {
            name: "transfer".to_string(),
            refinements: None,
            included_in: None,
            implies: vec![],
        };

        (use_action, transfer_action)
    }

}
