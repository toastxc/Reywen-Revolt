use reywen_http::results::{result, DeltaError};
use serde::{Deserialize, Serialize};

use crate::{
    client::Client,
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
        result(
            self.http
                .put(
                    &format!("/servers/{server}/permissions/{role_id}"),
                    Some(&serde_json::to_string(&data.field).unwrap()),
                )
                .await,
        )
        .await
    }
    pub async fn server_permission_set_default(
        &self,
        server: &str,
        data: &PermissionData,
    ) -> Result<Server, DeltaError> {
        result(
            self.http
                .put(
                    &format!("/servers/{server}/permissions/default"),
                    Some(&serde_json::to_string(&data.value).unwrap()),
                )
                .await,
        )
        .await
    }
    pub async fn roles_create(
        &self,
        server: &str,
        data: &DataRoleCreate,
    ) -> Result<NewRoleResponse, DeltaError> {
        result(
            self.http
                .post(
                    &format!("/servers/{server}/roles"),
                    Some(&serde_json::to_string(&data).unwrap()),
                )
                .await,
        )
        .await
    }
    pub async fn roles_delete(&self, server: &str, role_id: &str) -> Result<(), DeltaError> {
        result(
            self.http
                .delete(&format!("/servers/{server}/roles/{role_id}"), None)
                .await,
        )
        .await
    }

    pub async fn roles_edit(
        &self,
        server: &str,
        role_id: &str,
        data: &DataEditRole,
    ) -> Result<Role, DeltaError> {
        result(
            self.http
                .patch(
                    &format!("/servers/{server}/roles/{role_id}"),
                    Some(&serde_json::to_string(&data).unwrap()),
                )
                .await,
        )
        .await
    }

    pub async fn channel_permissions_set(
        &self,
        channel: &str,
        role_id: &str,
        data: &PermissionData,
    ) -> Result<Channel, DeltaError> {
        result(
            self.http
                .put(
                    &format!("/channels/{channel}/permissions/{role_id}"),
                    Some(&serde_json::to_string(&data.field).unwrap()),
                )
                .await,
        )
        .await
    }
    pub async fn channel_permissions_set_default(
        &self,
        channel: &str,
        data: &PermissionData,
        is_group: bool,
    ) -> Result<Channel, DeltaError> {
        let newdata = match is_group {
            true => serde_json::to_string(&data.value).unwrap(),
            false => serde_json::to_string(&data.field).unwrap(),
        };
        result(
            self.http
                .put(
                    &format!("/channels/{channel}/permissions/default"),
                    Some(&newdata),
                )
                .await,
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
    name: Option<String>,
    /// Role colour
    colour: Option<String>,
    /// Whether this role should be displayed separately
    hoist: Option<bool>,
    /// Ranking position
    ///
    /// Smaller values take priority.
    rank: Option<i64>,
    /// Fields to remove from role object
    remove: Option<Vec<FieldsRole>>,
}

impl DataEditRole {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
