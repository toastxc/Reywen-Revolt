use reywen_http::{driver::Method, results::DeltaError};
use serde::{Deserialize, Serialize};

use crate::{
    client::Client,
    json,
    structures::{
        channels::channel::Channel,
        permissions::newcalc::PermissionData,
        server::server::{FieldsRole, Role, Server},
    },
};

impl Client {
    pub async fn server_permission_set(
        &self,
        server: &str,
        role_id: &str,
        data: &PermissionData,
    ) -> Result<Server, DeltaError> {
        self.http
            .request(
                Method::PUT,
                &format!("/servers/{server}/permissions/{role_id}"),
                json!(data.field),
            )
            .await
    }
    pub async fn server_permission_set_default(
        &self,
        server: &str,
        data: &PermissionData,
    ) -> Result<Server, DeltaError> {
        self.http
            .request(
                Method::PUT,
                &format!("/servers/{server}/permissions/default"),
                json!(data.value),
            )
            .await
    }
    pub async fn roles_create(
        &self,
        server: &str,
        data: &DataRoleCreate,
    ) -> Result<NewRoleResponse, DeltaError> {
        self.http
            .request(
                Method::POST,
                &format!("/servers/{server}/roles"),
                json!(data),
            )
            .await
    }
    pub async fn roles_delete(&self, server: &str, role_id: &str) -> Result<(), DeltaError> {
        self.http
            .request(
                Method::DELETE,
                &format!("/servers/{server}/roles/{role_id}"),
                None,
            )
            .await
    }

    pub async fn roles_edit(
        &self,
        server: &str,
        role_id: &str,
        data: &DataEditRole,
    ) -> Result<Role, DeltaError> {
        self.http
            .request(
                Method::PATCH,
                &format!("/servers/{server}/roles/{role_id}"),
                json!(data),
            )
            .await
    }

    pub async fn channel_permissions_set(
        &self,
        channel: &str,
        role_id: &str,
        data: &PermissionData,
    ) -> Result<Channel, DeltaError> {
        self.http
            .request(
                Method::PUT,
                &format!("/channels/{channel}/permissions/{role_id}"),
                json!(data.field),
            )
            .await
    }
    pub async fn channel_permissions_set_default(
        &self,
        channel: &str,
        data: &PermissionData,
    ) -> Result<Channel, DeltaError> {
        self.http
            .request(
                Method::PUT,
                &format!("/channels/{channel}/permissions/default"),
                json!(data.field),
            )
            .await
    }

    pub async fn group_permissions_set_default(
        &self,
        group: &str,
        data: &PermissionData,
    ) -> Result<Channel, DeltaError> {
        self.http
            .request(
                Method::PUT,
                &format!("/channels/{group}/permissions/default"),
                json!(data.value),
            )
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DataRoleCreate {
    pub name: String,
    pub rank: Option<u32>,
}

impl DataRoleCreate {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            ..Default::default()
        }
    }
}

/// # New Role Response
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewRoleResponse {
    /// Id of the role
    id: String,
    /// New role
    role: Role,
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

impl DataEditRole {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_name(&mut self, name: &str) -> Self {
        self.name = Some(String::from(name));
        self.to_owned()
    }
    pub fn set_colour(&mut self, colour: &str) -> Self {
        self.colour = Some(String::from(colour));
        self.to_owned()
    }
    pub fn set_color(&mut self, color: &str) -> Self {
        self.colour = Some(String::from(color));
        self.to_owned()
    }

    pub fn set_hoist(&mut self, is_hoisted: bool) -> Self {
        self.hoist = Some(is_hoisted);
        self.to_owned()
    }

    pub fn set_rank(&mut self, rank: i64) -> Self {
        self.rank = Some(rank);
        self.to_owned()
    }
    pub fn set_remove(&mut self, remove: Vec<FieldsRole>) -> Self {
        self.remove = Some(remove);
        self.to_owned()
    }
    pub fn add_remove(&mut self, remove: FieldsRole) -> Self {
        match self.remove.clone() {
            Some(mut original) => {
                original.push(remove);
                self.remove = Some(original.to_owned());
            }
            None => {
                self.set_remove(vec![remove]);
            }
        };

        self.to_owned()
    }
}
