// TODO: Serialize rules manually since they are represented as arrays of permissions, prohibitions and obligations

use serde::{Serialize, Serializer, ser::SerializeStruct};
use crate::model::policy::{AgreementPolicy, OfferPolicy, SetPolicy};
use crate::model::rule::{Rule, Permission, Prohibition, Duty};

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
                permission_map.insert("target".to_string(), serde_json::json!(p.target.uid.as_ref().unwrap_or(&String::new())));
                if let Some(assigner) = &p.assigner {
                    if let Some(assigner_uid) = &assigner.uid {
                        permission_map.insert("assigner".to_string(), serde_json::json!(assigner_uid));
                    }
                }
                if let Some(assignee) = &p.assignee {
                    if let Some(assignee_uid) = &assignee.uid {
                        permission_map.insert("assignee".to_string(), serde_json::json!(assignee_uid));
                    }
                }
                permission_map.insert("action".to_string(), serde_json::json!(p.action.name.clone()));
                serde_json::Value::Object(permission_map)
            }).collect();
            state.serialize_field("permission", &serialized_permissions)?;
        }

        // TODO: Serialize prohibitions and obligations

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