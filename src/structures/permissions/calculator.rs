use serde::{Deserialize, Serialize};

use super::definitions::{Field, Override, Permission, Value};

/// Permission data body for requests
/// `allow` and `deny` provide abstraction for setting permissions
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Permissions {
    /// Allowed permission for target
    pub allow: Vec<Permission>,
    /// Denied permissions for target
    pub deny: Vec<Permission>,
    /// Low level field for absolute permissions
    pub value: Value,
    /// Low level field for Override permissions
    pub field: Field,
}

impl Permissions {
    /// Adds an allowed permission for target
    pub fn add_allow(&mut self, permission: Permission) -> Self {
        self.allow.push(permission);
        self.recalc()
    }
    /// Adds an denied permission for target
    pub fn add_deny(&mut self, permission: Permission) -> Self {
        self.deny.push(permission);
        self.recalc()
    }

    /// Recalculate low level permissions - trigger by setters
    pub fn recalc(&mut self) -> Self {
        self.field.permissions = Override {
            allow: Self::convert(&self.allow),
            deny: Self::convert(&self.deny),
        };
        self.value.permissions = Self::convert(&self.allow);
        self.clone()
    }

    pub fn new() -> Self {
        Default::default()
    }

    pub fn convert(input: &[Permission]) -> u64 {
        input
            .iter()
            .map(|item| item.clone() as u64)
            .collect::<Vec<u64>>()
            .into_iter()
            .sum()
    }
}
