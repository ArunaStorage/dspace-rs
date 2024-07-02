/*
 * management-api
 *
 * REST API documentation for the Eclipse EDC management-api.
 * https://app.swaggerhub.com/apis/eclipse-edc-bot/management-api/
 * Version: 0.7.0
 *
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuerySpec {
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub at_type: Option<String>,
    #[serde(rename = "filterExpression", skip_serializing_if = "Option::is_none")]
    pub filter_expression: Option<Vec<crate::Criterion>>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    #[serde(rename = "sortField", skip_serializing_if = "Option::is_none")]
    pub sort_field: Option<String>,
    #[serde(rename = "sortOrder", skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<SortOrder>,
}

impl QuerySpec {

    pub fn new(at_type: Option<String>, filter_expression: Option<Vec<crate::Criterion>>, limit: Option<i32>, offset: Option<i32>, sort_field: Option<String>, sort_order: Option<SortOrder>) -> QuerySpec {
        QuerySpec {
            at_type,
            filter_expression,
            limit,
            offset,
            sort_field,
            sort_order,
        }
    }

    pub fn default() -> Self {
        QuerySpec {
            at_type: Some("QuerySpec".to_string()),
            filter_expression: Some(vec![]),
            limit: Some(10),
            offset: Some(5),
            sort_field: Some("fieldName".to_string()),
            sort_order: Some(SortOrder::Desc),
        }
    }

}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SortOrder {
    #[serde(rename = "ASC")]
    Asc,
    #[serde(rename = "DESC")]
    Desc,
}

impl Default for SortOrder {
    fn default() -> SortOrder {
        Self::Asc
    }
}

