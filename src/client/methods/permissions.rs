use crate::{
    client::{Client, Result},
    reywen_http::driver::Method,
    structures::{
        channels::Channel,
        permissions::{definitions::PermissionData, DataEditRole, DataRoleCreate, NewRoleResponse},
        server::{Role, Server},
    },
};

impl Client {
    pub async fn server_permission_set(
        &self,
        server: impl Into<String> + std::fmt::Display,
        role_id: impl Into<String> + std::fmt::Display,
        data: impl Into<&PermissionData>,
    ) -> Result<Server> {
        self.http
            .request(
                Method::PUT,
                format!("/servers/{server}/permissions/{role_id}"),
                &data.into().field,
            )
            .await
    }
    pub async fn server_permission_set_default(
        &self,
        server: impl Into<String> + std::fmt::Display,
        data: impl Into<&PermissionData>,
    ) -> Result<Server> {
        self.http
            .request(
                Method::PUT,
                format!("/servers/{server}/permissions/default"),
                &data.into().value,
            )
            .await
    }
    pub async fn roles_create(
        &self,
        server: impl Into<String> + std::fmt::Display,
        data: impl Into<&DataRoleCreate>,
    ) -> Result<NewRoleResponse> {
        self.http
            .request(
                Method::POST,
                format!("/servers/{server}/roles"),
                data.into(),
            )
            .await
    }
    pub async fn roles_delete(
        &self,
        server: impl Into<String> + std::fmt::Display,
        role_id: impl Into<String> + std::fmt::Display,
    ) -> Result<()> {
        self.http
            .request(
                Method::DELETE,
                format!("/servers/{server}/roles/{role_id}"),
                None,
            )
            .await
    }

    pub async fn roles_edit(
        &self,
        server: impl Into<String> + std::fmt::Display,
        role_id: impl Into<String> + std::fmt::Display,
        data: impl Into<&DataEditRole>,
    ) -> Result<Role> {
        self.http
            .request(
                Method::PATCH,
                format!("/servers/{server}/roles/{role_id}"),
                data.into(),
            )
            .await
    }

    pub async fn channel_permissions_set(
        &self,
        channel: impl Into<String> + std::fmt::Display,
        role_id: impl Into<String> + std::fmt::Display,
        data: impl Into<&PermissionData>,
    ) -> Result<Channel> {
        self.http
            .request(
                Method::PUT,
                format!("/channels/{channel}/permissions/{role_id}"),
                &data.into().field,
            )
            .await
    }
    pub async fn channel_permissions_set_default(
        &self,
        channel: impl Into<String> + std::fmt::Display,
        data: impl Into<&PermissionData>,
    ) -> Result<Channel> {
        self.http
            .request(
                Method::PUT,
                format!("/channels/{channel}/permissions/default"),
                &data.into().field,
            )
            .await
    }

    pub async fn group_permissions_set_default(
        &self,
        group: impl Into<String> + std::fmt::Display,
        data: impl Into<&PermissionData>,
    ) -> Result<Channel> {
        self.http
            .request(
                Method::PUT,
                format!("/channels/{group}/permissions/default"),
                &data.into().field,
            )
            .await
    }
}
