use crate::{
    impl_to_vec,
    structures::server::{FieldsRole, Role},
};
use serde::{Deserialize, Serialize};

pub mod calculator;
pub mod definitions;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DataRoleCreate {
    pub name: String,
    pub rank: Option<u32>,
}
impl_to_vec!(DataRoleCreate);
impl DataRoleCreate {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }
    pub fn set_name(&mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self.clone()
    }

    pub fn set_rank(&mut self, rank: u32) -> Self {
        self.rank = Some(rank);
        self.clone()
    }
}

/// # New Role Response
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewRoleResponse {
    /// Id of the role
    pub id: String,
    /// New role
    pub role: Role,
}

/// # Role Data
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct DataEditRole {
    /// Role name
    pub name: Option<String>,
    /// Role colour
    pub colour: Option<String>,
    /// Whether this role should be displayed separately
    pub hoist: Option<bool>,
    /// Ranking position
    ///
    /// Smaller values take priority.
    pub rank: Option<i64>,
    /// Fields to remove from role object
    pub remove: Option<Vec<FieldsRole>>,
}
impl_to_vec!(DataEditRole);
impl DataEditRole {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_name(&mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self.clone()
    }
    pub fn set_colour(&mut self, colour: impl Into<String>) -> Self {
        self.colour = Some(colour.into());
        self.clone()
    }
    pub fn set_color(&mut self, color: &str) -> Self {
        self.colour = Some(color.into());
        self.clone()
    }

    pub fn set_hoist(&mut self, is_hoisted: bool) -> Self {
        self.hoist = Some(is_hoisted);
        self.to_owned()
    }

    pub fn set_rank(&mut self, rank: i64) -> Self {
        self.rank = Some(rank);
        self.to_owned()
    }
    pub fn set_remove(&mut self, remove: impl Into<Vec<FieldsRole>>) -> Self {
        self.remove = Some(remove.into());
        self.to_owned()
    }
    pub fn add_remove(&mut self, remove: impl Into<FieldsRole>) -> Self {
        match self.remove.clone() {
            Some(mut original) => {
                original.push(remove.into());
                self.remove = Some(original.to_owned());
            }
            None => {
                self.set_remove(vec![remove.into()]);
            }
        };

        self.clone()
    }
}
