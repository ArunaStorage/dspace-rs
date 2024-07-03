#[cfg(test)]
mod json_parser_test {
    extern crate odrl;

    use odrl::functions::json_parser;
    use odrl::model::{policy, asset, action, rule, party};
    use odrl::model::party::Function::{Assigner, Assignee};

    #[test]
    fn test_parse_json_set_policy() {
        let json_set_policy = r#"
        {
            "@context": "http://www.w3.org/ns/odrl.jsonld",
            "@type": "Set",
            "uid": "https://example.com/policy:1010",
            "permission": [{
                "target": "https://example.com/asset:9898.movie",
                "action": "use"
            }]
        }
        "#;

        let parsed_data = json_parser::parse(json_set_policy).unwrap();
        assert_eq!(parsed_data.parsed_policies.len(), 1);

        let comp_asset = asset::Asset {
            uid: Option::from("https://example.com/asset:9898.movie".to_string()),
            ..Default::default()
        };

        let comp_action = action::Action {
            name: "use".to_string(),
            ..Default::default()
        };

        let comp_rule = rule::Rule::Permission(rule::Permission {
            target: comp_asset,
            action: comp_action,
            ..Default::default()
        });

        let comp_policy = policy::Policy::SetPolicy(policy::SetPolicy {
            uid: "https://example.com/policy:1010".to_string(),
            rules: vec![comp_rule],
            ..Default::default()
        });

        assert_eq!(parsed_data.parsed_policies[0], comp_policy);

    }

    #[test]
    fn test_parse_json_offer_policy() {

        let json_offer_policy = r#"
        {
            "@context": "http://www.w3.org/ns/odrl.jsonld",
            "@type": "Offer",
            "uid": "https://example.com/policy:1011",
            "profile": "https://example.com/odrl:profile:01",
                "permission": [{
                "target": "https://example.com/asset:9898.movie",
                "assigner": "https://example.com/party:org:abc",
                "action": "play"
            }]
        }
        "#;

        let parsed_data = json_parser::parse(json_offer_policy).unwrap();
        assert_eq!(parsed_data.parsed_policies.len(), 1);

        let comp_asset = asset::Asset {
            uid: Option::from("https://example.com/asset:9898.movie".to_string()),
            ..Default::default()
        };

        let comp_action = action::Action {
            name: "play".to_string(),
            ..Default::default()
        };

        let comp_party = party::Party {
            uid: Option::from("https://example.com/party:org:abc".to_string()),
            function: Assigner,
            ..Default::default()
        };

        let comp_rule = rule::Rule::Permission(rule::Permission {
            target: comp_asset,
            action: comp_action,
            assigner: Some(comp_party.clone()),
            ..Default::default()
        });

        let comp_policy = policy::Policy::OfferPolicy(policy::OfferPolicy {
            uid: "https://example.com/policy:1011".to_string(),
            rules: vec![comp_rule],
            assigner: comp_party.clone(),
            ..Default::default()
        });

        assert_eq!(parsed_data.parsed_policies[0], comp_policy);

    }

    #[test]
    fn test_parse_json_agreement_policy() {

        let json_agreement_policy = r#"
        {
            "@context": "http://www.w3.org/ns/odrl.jsonld",
            "@type": "Agreement",
            "uid": "https://example.com/policy:1012",
            "profile": "https://example.com/odrl:profile:01",
            "permission": [{
                "target": "https://example.com/asset:9898.movie",
                "assigner": "https://example.com/party:org:abc",
                "assignee": "https://example.com/party:person:billie",
                "action": "play"
            }]
        }
        "#;

        let parsed_data = json_parser::parse(json_agreement_policy).unwrap();
        assert_eq!(parsed_data.parsed_policies.len(), 1);

        let comp_asset = asset::Asset {
            uid: Option::from("https://example.com/asset:9898.movie".to_string()),
            ..Default::default()
        };

        let comp_action = action::Action {
            name: "play".to_string(),
            ..Default::default()
        };

        let comp_assigner = party::Party {
            uid: Option::from("https://example.com/party:org:abc".to_string()),
            function: Assigner,
            ..Default::default()
        };

        let comp_assignee = party::Party {
            uid: Option::from("https://example.com/party:person:billie".to_string()),
            function: Assignee,
            ..Default::default()
        };

        let comp_rule = rule::Rule::Permission(rule::Permission {
            target: comp_asset,
            action: comp_action,
            assigner: Some(comp_assigner.clone()),
            assignee: Some(comp_assignee.clone()),
            ..Default::default()
        });

        let comp_policy = policy::Policy::AgreementPolicy(policy::AgreementPolicy {
            uid: "https://example.com/policy:1012".to_string(),
            rules: vec![comp_rule],
            assigner: comp_assigner.clone(),
            assignee: comp_assignee.clone(),
            ..Default::default()
        });

        assert_eq!(parsed_data.parsed_policies[0], comp_policy);

    }
}