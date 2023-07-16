use serde::{Deserialize, Serialize};

/// MFA method
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MFAMethod {
    Password,
    Recovery,
    Totp,
}

/// MFA response
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum MFAResponse {
    Password { password: String },
    Recovery { recovery_code: String },
    Totp { totp_code: String },
}
