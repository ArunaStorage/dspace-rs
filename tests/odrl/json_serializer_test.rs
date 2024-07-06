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

    #[test]
    fn test_set_policy_with_two_rules_serialize() {
        let action_1 = Action::new("play", None, None, vec![]);
        let mut asset_1 = Asset::default();
        asset_1.uid = Some("https://example.com/asset:9898.movie".to_string());
        let mut permission_1 = Permission::default();
        permission_1.target = asset_1;
        permission_1.action = action_1;

        let action_2 = Action::new("use", None, None, vec![]);
        let mut asset_2 = Asset::default();
        asset_2.uid = Some("https://example.com/asset:1212".to_string());
        let mut permission_2 = Permission::default();
        permission_2.target = asset_2;
        permission_2.action = action_2;

        let mut policy = SetPolicy::default();
        policy.uid = "https://example.com/policy:1010".to_string();
        policy.rules = vec![Rule::Permission(permission_1), Rule::Permission(permission_2)];

        let serialized_policy = serde_json::to_string_pretty(&policy).unwrap();
        let expected_policy = r#"
        {
            "@type": "Set",
            "uid": "https://example.com/policy:1010",
            "permission": [{
            "target": "https://example.com/asset:9898.movie",
            "action": "play"
            },
            {
            "target": "https://example.com/asset:1212",
            "action": "use"
            }]
        }"#.to_string();

        assert_eq!(
            serialized_policy.parse::<serde_json::Value>().unwrap(),
            expected_policy.parse::<serde_json::Value>().unwrap()
        );
    }

}