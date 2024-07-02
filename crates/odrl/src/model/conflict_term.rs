#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum ConflictTerm {

    Perm,
    Prohibit,
    Invalid,

}

