use serde::{Deserialize, Serialize};

use super::{
    definitions::{Override, Permission},
    newcalc::{Field, PermissionData, Value},
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Permissions {
    pub allow: Vec<Permission>,
    pub deny: Vec<Permission>,
}

impl Permissions {
    pub fn add_allow(&mut self, permission: Permission) -> Self {
        self.allow.push(permission);
        self.clone()
    }

    pub fn add_deny(&mut self, permission: Permission) -> Self {
        self.deny.push(permission);
        self.clone()
    }

    pub fn new() -> Self {
        Default::default()
    }

    pub fn convert(input: Vec<Permission>) -> u64 {
        input
            .into_iter()
            .map(|item| item as u64)
            .collect::<Vec<u64>>()
            .into_iter()
            .sum()
    }

    pub fn export(&self) -> PermissionData {
        let allow = Self::convert(self.allow.to_owned());
        let deny = Self::convert(self.deny.to_owned());

        PermissionData {
            value: Value { permissions: allow },
            field: Field {
                permissions: Override { allow, deny },
            },
        }
    }
}
