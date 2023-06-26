use serde::{Deserialize, Serialize};

use super::definitions::Override;

/// # Permission Value
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Field {
    /// Allow / deny values to set for this role
    pub permissions: Override,
}

/// Permission values to set for members in a `Group`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Value {
    /// Allow / deny values to set for this role
    pub permissions: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PermissionData {
    pub value: Value,
    pub field: Field,
}

// channel perm - FIELD
// channel default perm - FIELD
// channel default perm group - VALUE
// server perm - FIELD
// server default perm - VALUE
