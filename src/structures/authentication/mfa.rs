use serde::{Deserialize, Serialize};

/// MFA method
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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

impl MFAResponse {
    pub fn password(password: &str) -> Self {
        Self::Password {
            password: String::from(password),
        }
    }
    pub fn recovery(recovery_code: &str) -> Self {
        Self::Recovery {
            recovery_code: String::from(recovery_code),
        }
    }
    pub fn totp(totp_code: &str) -> Self {
        Self::Totp {
            totp_code: String::from(totp_code),
        }
    }
}
