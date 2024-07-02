use crate::model::action::Action;
use crate::model::asset::Asset;
use crate::model::party::Party;
use crate::model::constraint::Constraint;
use crate::model::type_alias::IRI;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Rule {

    Permission(Permission),
    Prohibition(Prohibition),
    Duty(Duty),
    Obligation(Obligation),

}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {

    pub uid: Option<IRI>,
    pub action: Action,
    pub relation: Option<Asset>,
    pub function: Vec<Party>,
    pub failures: Vec<Rule>,
    pub constraints: Vec<Constraint>,

    pub target: Asset,
    pub assigner: Option<Party>,
    pub assignee: Option<Party>,
    pub duties: Vec<Duty>,

}

impl Permission {

    pub fn new(uid: Option<IRI>, action: Action, relation: Option<Asset>, function: Vec<Party>, failures: Vec<Rule>, constraints: Vec<Constraint>, target: Asset, assigner: Option<Party>, assignee: Option<Party>, duties: Vec<Duty>) -> Self {
        Permission {
            uid,
            action,
            relation,
            function,
            failures,
            constraints,
            target,
            assigner,
            assignee,
            duties
        }
    }

}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prohibition {

    pub uid: Option<IRI>,
    pub action: Action,
    pub relation: Option<Asset>,
    pub function: Vec<Party>,
    pub failures: Vec<Rule>,
    pub constraints: Vec<Constraint>,

    pub target: Asset,
    pub assigner: Option<Party>,
    pub assignee: Option<Party>,
    pub remedies: Vec<Duty>,

}

impl Prohibition {

    pub fn new(uid: Option<IRI>, action: Action, relation: Option<Asset>, function: Vec<Party>, failures: Vec<Rule>, constraints: Vec<Constraint>, target: Asset, assigner: Option<Party>, assignee: Option<Party>, remedies: Vec<Duty>) -> Self {
        Prohibition {
            uid,
            action,
            relation,
            function,
            failures,
            constraints,
            target,
            assigner,
            assignee,
            remedies
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Duty {

    pub uid: Option<IRI>,
    pub action: Action,
    pub relation: Option<Asset>,
    pub function: Vec<Party>,
    pub failures: Vec<Rule>,
    pub constraints: Vec<Constraint>,

    pub target: Option<Asset>,
    pub assigner: Option<Party>,
    pub assignee: Option<Party>,
    pub consequences: Vec<Duty>,
    pub pre_condition: Option<Vec<Duty>>,


}

impl Duty {

    pub fn new(uid: Option<IRI>, action: Action, relation: Option<Asset>, function: Vec<Party>, failures: Vec<Rule>, constraints: Vec<Constraint>, target: Option<Asset>, assigner: Option<Party>, assignee: Option<Party>, consequences: Vec<Duty>, pre_condition: Option<Vec<Duty>>) -> Self {
        Duty {
            uid,
            action,
            relation,
            function,
            failures,
            constraints,
            target,
            assigner,
            assignee,
            consequences,
            pre_condition
        }
    }

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Obligation {

    pub uid: Option<IRI>,
    pub target: Asset,
    pub assigner: Party,
    pub assignee: Party,
    pub action: Action,
    pub consequence: Vec<Duty>,

}

impl Obligation {

    pub fn new(uid: Option<IRI>, target: Asset, assigner: Party, assignee: Party, action: Action, consequence: Vec<Duty>) -> Self {
        Obligation {
            uid,
            target,
            assigner,
            assignee,
            action,
            consequence
        }
    }

}