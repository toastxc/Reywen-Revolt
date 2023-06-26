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
    // converts from readable to bitwise

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

    pub fn export(&self) -> PermissionData {
        // define channel
        let mut channel = Override::new();

        for x in self.allow.clone() {
            channel.allow += x as u64;
        }
        for x in self.deny.clone() {
            channel.deny += x as u64;
        }

        // define group

        let mut group = 0;

        for x in self.allow.clone() {
            group += x as u64;
        }

        // deny is simply dropped from scope

        PermissionData {
            value: Value { permissions: group },
            field: Field {
                permissions: channel,
            },
        }
    }
}
