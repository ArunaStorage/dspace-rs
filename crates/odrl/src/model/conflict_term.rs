#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum ConflictTerm {

    #[serde(rename = "perm")]
    Perm,
    #[serde(rename = "prohibit")]
    Prohibit,
    #[serde(rename = "invalid")]
    Invalid,

}

impl Default for ConflictTerm {
    fn default() -> Self {
        ConflictTerm::Perm
    }
}

