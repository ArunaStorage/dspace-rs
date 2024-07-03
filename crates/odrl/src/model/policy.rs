use crate::model::party::Party;
use crate::model::rule::*;
use crate::model::conflict_term::ConflictTerm;
use crate::model::type_alias::IRI;


// A Policy MAY include an obligation to fulfil a Duty. The obligation is fulfilled if all constraints are satisfied and if its action, with all refinements satisfied, has been exercised.


/// Default Policy of type Set
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetPolicy {

    pub uid: IRI,
    pub rules: Vec<Rule>,
    pub profiles: Vec<IRI>,
    pub inherit_from: Vec<IRI>,
    pub conflict: Option<ConflictTerm>,
    pub obligation: Vec<Rule>,

}

impl SetPolicy {

    pub fn new(uid: IRI, rules: Vec<Rule>, profiles: Vec<IRI>, inherit_from: Vec<IRI>, conflict: Option<ConflictTerm>, obligation: Vec<Rule>) -> Self {
        SetPolicy {
            uid,
            rules,
            profiles,
            inherit_from,
            conflict,
            obligation
        }
    }

}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OfferPolicy {

    pub uid: IRI,
    pub assigner: Party,
    pub rules: Vec<Rule>,
    pub profiles: Vec<IRI>,
    pub inherit_from: Vec<IRI>,
    pub conflict: Option<ConflictTerm>,
    pub obligation: Vec<Rule>

}

impl OfferPolicy {

    pub fn new(uid: IRI, assigner: Party, rules: Vec<Rule>, profiles: Vec<IRI>, inherit_from: Vec<IRI>, conflict: Option<ConflictTerm>, obligation: Vec<Rule>) -> Self {
        OfferPolicy {
            uid,
            assigner,
            rules,
            profiles,
            inherit_from,
            conflict,
            obligation
        }
    }

}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AgreementPolicy {

    pub uid: IRI,
    pub assigner: Party,
    pub assignee: Party,
    pub rules: Vec<Rule>,
    pub profiles: Vec<IRI>,
    pub inherit_from: Vec<IRI>,
    pub conflict: Option<ConflictTerm>,
    pub obligation: Vec<Rule>

}

impl AgreementPolicy {

    pub fn new(uid: IRI, assigner: Party, assignee: Party, rules: Vec<Rule>, profiles: Vec<IRI>, inherit_from: Vec<IRI>, conflict: Option<ConflictTerm>, obligation: Vec<Rule>) -> Self {
        AgreementPolicy {
            uid,
            assigner,
            assignee,
            rules,
            profiles,
            inherit_from,
            conflict,
            obligation
        }
    }

}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Policy {

    SetPolicy(SetPolicy),
    OfferPolicy(OfferPolicy),
    AgreementPolicy(AgreementPolicy),

}

impl Default for Policy {
    fn default() -> Self {
        // Default to SetPolicy
        Policy::SetPolicy(SetPolicy::default())
    }
}