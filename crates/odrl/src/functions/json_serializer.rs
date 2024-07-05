// TODO: Serialize rules manually since they are represented as arrays of permissions, prohibitions and obligations

use serde::{Serialize, Serializer, ser::SerializeStruct};
use crate::model::policy::{AgreementPolicy, OfferPolicy, SetPolicy};
use crate::model::rule::{Rule, Permission, Prohibition, Obligation};
use crate::model::action::Refinements;
use crate::model::constraint::{LeftOperand, RightOperand};

impl Serialize for SetPolicy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        // docs.rs: len does not include fields which are skipped with SerializeStruct::skip_field.
        let mut state = serializer.serialize_struct("SetPolicy", 5)?;
        state.serialize_field("@type", "Set")?;
        state.serialize_field("uid", &self.uid)?;
        if !self.profiles.is_empty() {
            if self.profiles.len() == 1 {
                state.serialize_field("profile", &self.profiles[0])?;
            } else {
                state.serialize_field("profile", &self.profiles)?;
            }
        }
        if !self.inherit_from.is_empty() {
            if self.inherit_from.len() == 1 {
                state.serialize_field("inheritFrom", &self.inherit_from[0])?;
            } else {
                state.serialize_field("inheritFrom", &self.inherit_from)?;
            }
        }
        if let Some(conflict) = &self.conflict {
            state.serialize_field("conflict", conflict)?;
        }
        if !self.obligation.is_empty() {
            state.serialize_field("obligation", &self.obligation)?;
        }

        let permissions: Vec<&Permission> = self.rules.iter().filter_map(|rule| {
            if let Rule::Permission(permission) = rule {
                Some(permission)
            } else {
                None
            }
        }).collect();

        if !permissions.is_empty() {
            let serialized_permissions: Vec<_> = permissions.iter().map(|p| {
                let mut permission_map = serde_json::Map::new();

                let mut target_map = serde_json::Map::new();
                if let Some(target_type) = &p.target.edc_type {
                    target_map.insert("@type".to_string(), serde_json::json!(target_type));
                }
                if let Some(target_uid) = &p.target.uid {
                    target_map.insert("uid".to_string(), serde_json::json!(target_uid));
                }
                if target_map.len() > 1 {
                    permission_map.insert("target".to_string(), serde_json::Value::Object(target_map));
                } else {
                    permission_map.insert("target".to_string(), serde_json::json!(p.target.uid.as_ref().unwrap_or(&String::new())));
                }

                if let Some(assigner) = &p.assigner {
                    let mut assigner_map = serde_json::Map::new();
                    if let Some(assigner_type) = &assigner.r#type {
                        assigner_map.insert("@type".to_string(), serde_json::json!(assigner_type));
                    }
                    if let Some(assigner_uid) = &assigner.uid {
                        assigner_map.insert("uid".to_string(), serde_json::json!(assigner_uid));
                    }
                    if assigner.part_of.len() > 0 {
                        if assigner.part_of.len() == 1 {
                            assigner_map.insert("partOf".to_string(), serde_json::json!(assigner.part_of[0].source.as_ref().unwrap_or(&String::new())));
                        } else {
                            // Collect all PartyCollection.source into a vec
                            let part_of: Vec<_> = assigner.part_of.iter().filter_map(|pc| pc.source.as_ref()).collect();
                            assigner_map.insert("partOf".to_string(), serde_json::json!(part_of));
                        }
                    }
                    if assigner_map.len() > 1 {
                        permission_map.insert("assigner".to_string(), serde_json::Value::Object(assigner_map));
                    } else {
                        permission_map.insert("assigner".to_string(), serde_json::json!(assigner.uid.as_ref().unwrap_or(&String::new())));
                    }
                }

                if let Some(assignee) = &p.assignee {
                    let mut assignee_map = serde_json::Map::new();
                    if let Some(assigner_type) = &assignee.r#type {
                        assignee_map.insert("@type".to_string(), serde_json::json!(assigner_type));
                    }
                    if let Some(assigner_uid) = &assignee.uid {
                        assignee_map.insert("uid".to_string(), serde_json::json!(assigner_uid));
                    }
                    if assignee.part_of.len() > 0 {
                        if assignee.part_of.len() == 1 {
                            assignee_map.insert("partOf".to_string(), serde_json::json!(assignee.part_of[0].source.as_ref().unwrap_or(&String::new())));
                        } else {
                            // Collect all PartyCollection.source into a vec
                            let part_of: Vec<_> = assignee.part_of.iter().filter_map(|pc| pc.source.as_ref()).collect();
                            assignee_map.insert("partOf".to_string(), serde_json::json!(part_of));
                        }
                    }
                    if assignee_map.len() > 1 {
                        permission_map.insert("assignee".to_string(), serde_json::Value::Object(assignee_map));
                    } else {
                        permission_map.insert("assignee".to_string(), serde_json::json!(assignee.uid.as_ref().unwrap_or(&String::new())));
                    }
                }

                if (p.action.refinements.is_none()) && (p.action.included_in.is_none()) && (p.action.implies.len() == 0) {
                    permission_map.insert("action".to_string(), serde_json::json!(p.action.name.clone()));
                } else {
                    let mut action_map = serde_json::Map::new();
                    action_map.insert("name".to_string(), serde_json::json!(p.action.name.clone()));
                    if let Some(refinements) = &p.action.refinements {
                        // differentiate between constraints and logical constraints
                        match refinements {
                            Refinements::Constraints(constraints) => {
                                let mut serialized_constraints = Vec::new();
                                for constraint in constraints {
                                    let mut constraint_map = serde_json::Map::new();
                                    // differentiate between literal and iri operands
                                    match &constraint.left_operand {
                                        LeftOperand::Literal(literal) => {
                                            constraint_map.insert("leftOperand".to_string(), serde_json::json!(literal));
                                        },
                                        LeftOperand::IRI(iri) => {
                                            constraint_map.insert("leftOperand".to_string(), serde_json::json!(iri));
                                        }
                                        LeftOperand::Reference(reference) => {
                                            constraint_map.insert("leftOperand".to_string(), serde_json::json!(reference));
                                        }
                                    }
                                    constraint_map.insert("operator".to_string(), serde_json::json!(constraint.operator));
                                    // differentiate between literal and iri operands
                                    match &constraint.right_operand {
                                        RightOperand::Literal(literal) => {
                                            constraint_map.insert("rightOperand".to_string(), serde_json::json!(literal));
                                        },
                                        RightOperand::IRI(iri) => {
                                            constraint_map.insert("rightOperand".to_string(), serde_json::json!(iri));
                                        }
                                        RightOperand::Reference(reference) => {
                                            constraint_map.insert("rightOperand".to_string(), serde_json::json!(reference));
                                        }
                                    }
                                    if let Some(data_type) = &constraint.data_type {
                                        constraint_map.insert("dataType".to_string(), serde_json::json!(data_type));
                                    }
                                    if let Some(unit) = &constraint.unit {
                                        constraint_map.insert("unit".to_string(), serde_json::json!(unit));
                                    }
                                    if !constraint.status.is_empty() {
                                        constraint_map.insert("status".to_string(), serde_json::json!(constraint.status));
                                    }
                                    serialized_constraints.push(constraint_map);
                                }
                                action_map.insert("refinement".to_string(), serde_json::json!(serialized_constraints));
                            },
                            Refinements::LogicalConstraints(logical_constraints) => {
                                for logical_constraint in logical_constraints {
                                    let mut logical_constraint_map = serde_json::Map::new();
                                    if let Some(operand) = &logical_constraint.operand {
                                        logical_constraint_map.insert(serde_json::json!(operand.0).to_string().replace("\"", ""), serde_json::json!(operand.1));
                                    }
                                    let serialized_logical_constraint = serde_json::json!(logical_constraint_map);
                                    action_map.insert("refinement".to_string(), serialized_logical_constraint);
                                }
                            }
                        }
                    }
                    if let Some(included_in) = &p.action.included_in {
                        action_map.insert("includedIn".to_string(), serde_json::json!(included_in));
                    }
                    if p.action.implies.len() > 0 {
                        action_map.insert("implies".to_string(), serde_json::json!(p.action.implies.iter().map(|a| a.name.clone()).collect::<Vec<_>>()));
                    }
                    permission_map.insert("action".to_string(), serde_json::Value::Object(action_map));
                }

                serde_json::Value::Object(permission_map)
            }).collect();
            state.serialize_field("permission", &serialized_permissions)?;
        }

        let prohibitions: Vec<&Prohibition> = self.rules.iter().filter_map(|rule| {
            if let Rule::Prohibition(prohibition) = rule {
                Some(prohibition)
            } else {
                None
            }
        }).collect();

        if !prohibitions.is_empty() {
            let serialized_prohibitions: Vec<_> = prohibitions.iter().map(|p| {
                let mut prohibition_map = serde_json::Map::new();

                let mut target_map = serde_json::Map::new();
                if let Some(target_type) = &p.target.edc_type {
                    target_map.insert("@type".to_string(), serde_json::json!(target_type));
                }
                if let Some(target_uid) = &p.target.uid {
                    target_map.insert("uid".to_string(), serde_json::json!(target_uid));
                }
                if target_map.len() > 1 {
                    prohibition_map.insert("target".to_string(), serde_json::Value::Object(target_map));
                } else {
                    prohibition_map.insert("target".to_string(), serde_json::json!(p.target.uid.as_ref().unwrap_or(&String::new())));
                }

                if let Some(assigner) = &p.assigner {
                    let mut assigner_map = serde_json::Map::new();
                    if let Some(assigner_type) = &assigner.r#type {
                        assigner_map.insert("@type".to_string(), serde_json::json!(assigner_type));
                    }
                    if let Some(assigner_uid) = &assigner.uid {
                        assigner_map.insert("uid".to_string(), serde_json::json!(assigner_uid));
                    }
                    if assigner.part_of.len() > 0 {
                        if assigner.part_of.len() == 1 {
                            assigner_map.insert("partOf".to_string(), serde_json::json!(assigner.part_of[0].source.as_ref().unwrap_or(&String::new())));
                        } else {
                            // Collect all PartyCollection.source into a vec
                            let part_of: Vec<_> = assigner.part_of.iter().filter_map(|pc| pc.source.as_ref()).collect();
                            assigner_map.insert("partOf".to_string(), serde_json::json!(part_of));
                        }
                    }
                    if assigner_map.len() > 1 {
                        prohibition_map.insert("assigner".to_string(), serde_json::Value::Object(assigner_map));
                    } else {
                        prohibition_map.insert("assigner".to_string(), serde_json::json!(assigner.uid.as_ref().unwrap_or(&String::new())));
                    }
                }

                if let Some(assignee) = &p.assignee {
                    let mut assignee_map = serde_json::Map::new();
                    if let Some(assigner_type) = &assignee.r#type {
                        assignee_map.insert("@type".to_string(), serde_json::json!(assigner_type));
                    }
                    if let Some(assigner_uid) = &assignee.uid {
                        assignee_map.insert("uid".to_string(), serde_json::json!(assigner_uid));
                    }
                    if assignee.part_of.len() > 0 {
                        if assignee.part_of.len() == 1 {
                            assignee_map.insert("partOf".to_string(), serde_json::json!(assignee.part_of[0].source.as_ref().unwrap_or(&String::new())));
                        } else {
                            // Collect all PartyCollection.source into a vec
                            let part_of: Vec<_> = assignee.part_of.iter().filter_map(|pc| pc.source.as_ref()).collect();
                            assignee_map.insert("partOf".to_string(), serde_json::json!(part_of));
                        }
                    }
                    if assignee_map.len() > 1 {
                        prohibition_map.insert("assignee".to_string(), serde_json::Value::Object(assignee_map));
                    } else {
                        prohibition_map.insert("assignee".to_string(), serde_json::json!(assignee.uid.as_ref().unwrap_or(&String::new())));
                    }
                }

                if (p.action.refinements.is_none()) && (p.action.included_in.is_none()) && (p.action.implies.len() == 0) {
                    prohibition_map.insert("action".to_string(), serde_json::json!(p.action.name.clone()));
                } else {
                    let mut action_map = serde_json::Map::new();
                    action_map.insert("name".to_string(), serde_json::json!(p.action.name.clone()));
                    if let Some(refinements) = &p.action.refinements {
                        // differentiate between constraints and logical constraints
                        match refinements {
                            Refinements::Constraints(constraints) => {
                                let mut serialized_constraints = Vec::new();
                                for constraint in constraints {
                                    let mut constraint_map = serde_json::Map::new();
                                    // differentiate between literal and iri operands
                                    match &constraint.left_operand {
                                        LeftOperand::Literal(literal) => {
                                            constraint_map.insert("leftOperand".to_string(), serde_json::json!(literal));
                                        },
                                        LeftOperand::IRI(iri) => {
                                            constraint_map.insert("leftOperand".to_string(), serde_json::json!(iri));
                                        }
                                        LeftOperand::Reference(reference) => {
                                            constraint_map.insert("leftOperand".to_string(), serde_json::json!(reference));
                                        }
                                    }
                                    constraint_map.insert("operator".to_string(), serde_json::json!(constraint.operator));
                                    // differentiate between literal and iri operands
                                    match &constraint.right_operand {
                                        RightOperand::Literal(literal) => {
                                            constraint_map.insert("rightOperand".to_string(), serde_json::json!(literal));
                                        },
                                        RightOperand::IRI(iri) => {
                                            constraint_map.insert("rightOperand".to_string(), serde_json::json!(iri));
                                        }
                                        RightOperand::Reference(reference) => {
                                            constraint_map.insert("rightOperand".to_string(), serde_json::json!(reference));
                                        }
                                    }
                                    if let Some(data_type) = &constraint.data_type {
                                        constraint_map.insert("dataType".to_string(), serde_json::json!(data_type));
                                    }
                                    if let Some(unit) = &constraint.unit {
                                        constraint_map.insert("unit".to_string(), serde_json::json!(unit));
                                    }
                                    if !constraint.status.is_empty() {
                                        constraint_map.insert("status".to_string(), serde_json::json!(constraint.status));
                                    }
                                    serialized_constraints.push(constraint_map);
                                }
                                action_map.insert("refinement".to_string(), serde_json::json!(serialized_constraints));
                            },
                            Refinements::LogicalConstraints(logical_constraints) => {
                                for logical_constraint in logical_constraints {
                                    let mut logical_constraint_map = serde_json::Map::new();
                                    if let Some(operand) = &logical_constraint.operand {
                                        logical_constraint_map.insert(serde_json::json!(operand.0).to_string().replace("\"", ""), serde_json::json!(operand.1));
                                    }
                                    let serialized_logical_constraint = serde_json::json!(logical_constraint_map);
                                    action_map.insert("refinement".to_string(), serialized_logical_constraint);
                                }
                            }
                        }
                    }
                    if let Some(included_in) = &p.action.included_in {
                        action_map.insert("includedIn".to_string(), serde_json::json!(included_in));
                    }
                    if p.action.implies.len() > 0 {
                        action_map.insert("implies".to_string(), serde_json::json!(p.action.implies.iter().map(|a| a.name.clone()).collect::<Vec<_>>()));
                    }
                    prohibition_map.insert("action".to_string(), serde_json::Value::Object(action_map));
                }

                serde_json::Value::Object(prohibition_map)
            }).collect();
            state.serialize_field("prohibition", &serialized_prohibitions)?;
        }

        let obligations: Vec<&Obligation> = self.rules.iter().filter_map(|rule| {
            if let Rule::Obligation(obligation) = rule {
                Some(obligation)
            } else {
                None
            }
        }).collect();

        if !obligations.is_empty() {
            let serialized_obligations: Vec<_> = obligations.iter().map(|p| {
                let mut obligation_map = serde_json::Map::new();

                let mut target_map = serde_json::Map::new();
                if let Some(target_type) = &p.target.edc_type {
                    target_map.insert("@type".to_string(), serde_json::json!(target_type));
                }
                if let Some(target_uid) = &p.target.uid {
                    target_map.insert("uid".to_string(), serde_json::json!(target_uid));
                }
                if target_map.len() > 1 {
                    obligation_map.insert("target".to_string(), serde_json::Value::Object(target_map));
                } else {
                    obligation_map.insert("target".to_string(), serde_json::json!(p.target.uid.as_ref().unwrap_or(&String::new())));
                }

                let mut assigner_map = serde_json::Map::new();
                if let Some(assigner_type) = &p.assigner.r#type {
                    assigner_map.insert("@type".to_string(), serde_json::json!(assigner_type));
                }
                assigner_map.insert("uid".to_string(), serde_json::json!(p.assigner.uid.as_ref().unwrap_or(&String::new())));
                if p.assigner.part_of.len() > 0 {
                    if p.assigner.part_of.len() == 1 {
                        assigner_map.insert("partOf".to_string(), serde_json::json!(p.assigner.part_of[0].source.as_ref().unwrap_or(&String::new())));
                    } else {
                        // Collect all PartyCollection.source into a vec
                        let part_of: Vec<_> = p.assigner.part_of.iter().filter_map(|pc| pc.source.as_ref()).collect();
                        assigner_map.insert("partOf".to_string(), serde_json::json!(part_of));
                    }
                }
                if assigner_map.len() > 1 {
                    obligation_map.insert("assigner".to_string(), serde_json::Value::Object(assigner_map));
                } else {
                    obligation_map.insert("assigner".to_string(), serde_json::json!(p.assigner.uid.as_ref().unwrap_or(&String::new())));
                }

                let mut assignee_map = serde_json::Map::new();
                if let Some(assigner_type) = &p.assignee.r#type {
                    assignee_map.insert("@type".to_string(), serde_json::json!(assigner_type));
                }
                if let Some(assigner_uid) = &p.assignee.uid {
                    assignee_map.insert("uid".to_string(), serde_json::json!(assigner_uid));
                }
                if p.assignee.part_of.len() > 0 {
                    if p.assignee.part_of.len() == 1 {
                        assignee_map.insert("partOf".to_string(), serde_json::json!(p.assignee.part_of[0].source.as_ref().unwrap_or(&String::new())));
                    } else {
                        // Collect all PartyCollection.source into a vec
                        let part_of: Vec<_> = p.assignee.part_of.iter().filter_map(|pc| pc.source.as_ref()).collect();
                        assignee_map.insert("partOf".to_string(), serde_json::json!(part_of));
                    }
                }
                if assignee_map.len() > 1 {
                    obligation_map.insert("assignee".to_string(), serde_json::Value::Object(assignee_map));
                } else {
                    obligation_map.insert("assignee".to_string(), serde_json::json!(p.assignee.uid.as_ref().unwrap_or(&String::new())));
                }

                if (p.action.refinements.is_none()) && (p.action.included_in.is_none()) && (p.action.implies.len() == 0) {
                    obligation_map.insert("action".to_string(), serde_json::json!(p.action.name.clone()));
                } else {
                    let mut action_map = serde_json::Map::new();
                    action_map.insert("name".to_string(), serde_json::json!(p.action.name.clone()));
                    if let Some(refinements) = &p.action.refinements {
                        // differentiate between constraints and logical constraints
                        match refinements {
                            Refinements::Constraints(constraints) => {
                                let mut serialized_constraints = Vec::new();
                                for constraint in constraints {
                                    let mut constraint_map = serde_json::Map::new();
                                    // differentiate between literal and iri operands
                                    match &constraint.left_operand {
                                        LeftOperand::Literal(literal) => {
                                            constraint_map.insert("leftOperand".to_string(), serde_json::json!(literal));
                                        },
                                        LeftOperand::IRI(iri) => {
                                            constraint_map.insert("leftOperand".to_string(), serde_json::json!(iri));
                                        }
                                        LeftOperand::Reference(reference) => {
                                            constraint_map.insert("leftOperand".to_string(), serde_json::json!(reference));
                                        }
                                    }
                                    constraint_map.insert("operator".to_string(), serde_json::json!(constraint.operator));
                                    // differentiate between literal and iri operands
                                    match &constraint.right_operand {
                                        RightOperand::Literal(literal) => {
                                            constraint_map.insert("rightOperand".to_string(), serde_json::json!(literal));
                                        },
                                        RightOperand::IRI(iri) => {
                                            constraint_map.insert("rightOperand".to_string(), serde_json::json!(iri));
                                        }
                                        RightOperand::Reference(reference) => {
                                            constraint_map.insert("rightOperand".to_string(), serde_json::json!(reference));
                                        }
                                    }
                                    if let Some(data_type) = &constraint.data_type {
                                        constraint_map.insert("dataType".to_string(), serde_json::json!(data_type));
                                    }
                                    if let Some(unit) = &constraint.unit {
                                        constraint_map.insert("unit".to_string(), serde_json::json!(unit));
                                    }
                                    if !constraint.status.is_empty() {
                                        constraint_map.insert("status".to_string(), serde_json::json!(constraint.status));
                                    }
                                    serialized_constraints.push(constraint_map);
                                }
                                action_map.insert("refinement".to_string(), serde_json::json!(serialized_constraints));
                            },
                            Refinements::LogicalConstraints(logical_constraints) => {
                                for logical_constraint in logical_constraints {
                                    let mut logical_constraint_map = serde_json::Map::new();
                                    if let Some(operand) = &logical_constraint.operand {
                                        logical_constraint_map.insert(serde_json::json!(operand.0).to_string().replace("\"", ""), serde_json::json!(operand.1));
                                    }
                                    let serialized_logical_constraint = serde_json::json!(logical_constraint_map);
                                    action_map.insert("refinement".to_string(), serialized_logical_constraint);
                                }
                            }
                        }
                    }
                    if let Some(included_in) = &p.action.included_in {
                        action_map.insert("includedIn".to_string(), serde_json::json!(included_in));
                    }
                    if p.action.implies.len() > 0 {
                        action_map.insert("implies".to_string(), serde_json::json!(p.action.implies.iter().map(|a| a.name.clone()).collect::<Vec<_>>()));
                    }
                    obligation_map.insert("action".to_string(), serde_json::Value::Object(action_map));
                }

                // collect all consequences
                let serialized_consequences: Vec<_> = p.consequence.iter().map(|d| {
                    let mut consequence_map = serde_json::Map::new();
                    if let Some(uid) = &d.uid {
                        consequence_map.insert("uid".to_string(), serde_json::json!(uid));
                    }
                    if d.action.refinements.is_none() && d.action.included_in.is_none() && d.action.implies.len() == 0 {
                        consequence_map.insert("action".to_string(), serde_json::json!(d.action.name.clone()));
                    } else {
                        let mut action_map = serde_json::Map::new();
                        action_map.insert("name".to_string(), serde_json::json!(d.action.name.clone()));
                        if let Some(refinements) = &d.action.refinements {
                            // differentiate between constraints and logical constraints
                            match refinements {
                                Refinements::Constraints(constraints) => {
                                    let mut serialized_constraints = Vec::new();
                                    for constraint in constraints {
                                        let mut constraint_map = serde_json::Map::new();
                                        // differentiate between literal and iri operands
                                        match &constraint.left_operand {
                                            LeftOperand::Literal(literal) => {
                                                constraint_map.insert("leftOperand".to_string(), serde_json::json!(literal));
                                            },
                                            LeftOperand::IRI(iri) => {
                                                constraint_map.insert("leftOperand".to_string(), serde_json::json!(iri));
                                            }
                                            LeftOperand::Reference(reference) => {
                                                constraint_map.insert("leftOperand".to_string(), serde_json::json!(reference));
                                            }
                                        }
                                        constraint_map.insert("operator".to_string(), serde_json::json!(constraint.operator));
                                        // differentiate between literal and iri operands
                                        match &constraint.right_operand {
                                            RightOperand::Literal(literal) => {
                                                constraint_map.insert("rightOperand".to_string(), serde_json::json!(literal));
                                            },
                                            RightOperand::IRI(iri) => {
                                                constraint_map.insert("rightOperand".to_string(), serde_json::json!(iri));
                                            }
                                            RightOperand::Reference(reference) => {
                                                constraint_map.insert("rightOperand".to_string(), serde_json::json!(reference));
                                            }
                                        }
                                        if let Some(data_type) = &constraint.data_type {
                                            constraint_map.insert("dataType".to_string(), serde_json::json!(data_type));
                                        }
                                        if let Some(unit) = &constraint.unit {
                                            constraint_map.insert("unit".to_string(), serde_json::json!(unit));
                                        }
                                        if !constraint.status.is_empty() {
                                            constraint_map.insert("status".to_string(), serde_json::json!(constraint.status));
                                        }
                                        serialized_constraints.push(constraint_map);
                                    }
                                    action_map.insert("refinement".to_string(), serde_json::json!(serialized_constraints));
                                },
                                Refinements::LogicalConstraints(logical_constraints) => {
                                    for logical_constraint in logical_constraints {
                                        let mut logical_constraint_map = serde_json::Map::new();
                                        if let Some(operand) = &logical_constraint.operand {
                                            logical_constraint_map.insert(serde_json::json!(operand.0).to_string().replace("\"", ""), serde_json::json!(operand.1));
                                        }
                                        let serialized_logical_constraint = serde_json::json!(logical_constraint_map);
                                        action_map.insert("refinement".to_string(), serialized_logical_constraint);
                                    }
                                }
                            }
                        }
                        if let Some(included_in) = &d.action.included_in {
                            action_map.insert("includedIn".to_string(), serde_json::json!(included_in));
                        }
                        if d.action.implies.len() > 0 {
                            action_map.insert("implies".to_string(), serde_json::json!(d.action.implies.iter().map(|a| a.name.clone()).collect::<Vec<_>>()));
                        }
                        consequence_map.insert("action".to_string(), serde_json::Value::Object(action_map));
                    }
                    if let Some(relation) = &d.relation {
                        consequence_map.insert("relation".to_string(), serde_json::json!(relation.uid.as_ref().unwrap_or(&String::new())));
                    }
                    if !d.function.is_empty() {
                        if d.function.len() > 1 {
                            consequence_map.insert("compensatedParty".to_string(), serde_json::json!(d.function.iter().map(|f| f.clone().uid.expect("No UID").to_string()).collect::<Vec<_>>()));
                        } else {
                            consequence_map.insert("compensatedParty".to_string(), serde_json::json!(d.function[0].clone().uid.expect("No UID").to_string()));
                        }
                    }
                    if !d.constraints.is_empty() {
                        if d.constraints.len() > 1 {
                            let mut serialized_constraints = Vec::new();
                            for constraint in &d.constraints {
                                let serialized = serde_json::to_string(constraint).expect("Failed to serialize constraint");
                                serialized_constraints.push(serialized);
                            }
                            consequence_map.insert("constraint".to_string(), serde_json::json!(serialized_constraints));
                        } else {
                            consequence_map.insert("constraint".to_string(), serde_json::json!(serde_json::to_string(&d.constraints[0]).expect("Failed to serialize constraint")));
                        }
                    }
                    if let Some(target) = &d.target {
                        consequence_map.insert("target".to_string(), serde_json::json!(target.uid.as_ref().unwrap_or(&String::new())));
                    }
                    if let Some(assigner) = &d.assigner {
                        consequence_map.insert("assigner".to_string(), serde_json::json!(assigner.uid.as_ref().unwrap_or(&String::new())));
                    }
                    if let Some(assignee) = &d.assignee {
                        consequence_map.insert("assignee".to_string(), serde_json::json!(assignee.uid.as_ref().unwrap_or(&String::new())));
                    }
                    serde_json::Value::Object(consequence_map)
                }).collect();

                obligation_map.insert("consequence".to_string(), serde_json::json!(serialized_consequences));

                serde_json::Value::Object(obligation_map)
            }).collect();
            state.serialize_field("obligation", &serialized_obligations)?;
        }

        state.end()
    }
}


impl Serialize for OfferPolicy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        // TODO
        let mut state = serializer.serialize_struct("OfferPolicy", 7)?;
        state.end()
    }
}


impl Serialize for AgreementPolicy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        // TODO
        let mut state = serializer.serialize_struct("AgreementPolicy", 8)?;
        state.end()
    }
}