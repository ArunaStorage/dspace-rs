#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum ConflictTerm {

    Perm,
    Prohibit,
    Invalid,

}

impl Default for ConflictTerm {
    fn default() -> Self {
        ConflictTerm::Perm
    }
}

