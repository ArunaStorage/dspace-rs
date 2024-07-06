// Examples based on https://www.w3.org/TR/odrl-model

#[cfg(test)]
mod json_ld_serializer_test {
    extern crate odrl;

    use odrl::model::policy::{SetPolicy};
    use odrl::model::rule::{Rule, Permission};
    use odrl::model::action::Action;
    use odrl::model::asset::Asset;

    #[test]
    fn test_set_policy_with_one_rule_serialize() {
        let action = Action::new("use", None, None, vec![]);
        let mut asset = Asset::default();
        asset.uid = Some("https://example.com/asset:9898.movie".to_string());
        let mut permission = Permission::default();
        permission.target = asset;
        permission.action = action;

        let mut policy = SetPolicy::default();
        policy.uid = "https://example.com/policy:1010".to_string();
        policy.rules = vec![Rule::Permission(permission)];

        let serialized_policy = serde_json::to_string_pretty(&policy).unwrap();
        let expected_policy = r#"
        {
            "@type": "Set",
            "uid": "https://example.com/policy:1010",
            "permission": [{
            "target": "https://example.com/asset:9898.movie",
            "action": "use"
            }]
        }"#.to_string();

        assert_eq!(
            serialized_policy.parse::<serde_json::Value>().unwrap(),
            expected_policy.parse::<serde_json::Value>().unwrap()
        );
    }

}