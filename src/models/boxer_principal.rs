use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[cfg(test)]
mod tests;

pub const PRINCIPAL_KEY: &str = "boxer.sneaksanddata.com/principal";
pub const USER_ID_KEY: &str = "boxer.sneaksanddata.com/external-identity";
pub const IDENTITY_PROVIDER_KEY: &str = "boxer.sneaksanddata.com/identity-provider";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoxerPrincipal {
    principal: String,
    external_identity: String,
    identity_provider: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissingClaim {
    key: &'static str,
}

impl MissingClaim {
    pub fn key(&self) -> &'static str {
        self.key
    }
}

impl Display for MissingClaim {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "missing required claim: {}", self.key)
    }
}

impl Error for MissingClaim {}

impl BoxerPrincipal {
    pub fn deserialize_user(claims: &HashMap<String, String>) -> Result<Self, MissingClaim> {
        let principal = required_claim(claims, PRINCIPAL_KEY)?;
        let external_identity = required_claim(claims, USER_ID_KEY)?;
        let identity_provider = required_claim(claims, IDENTITY_PROVIDER_KEY)?;

        Ok(Self {
            principal,
            external_identity,
            identity_provider,
        })
    }

    pub fn serialize_user(&self) -> HashMap<String, String> {
        HashMap::from([
            (PRINCIPAL_KEY.to_string(), self.principal.clone()),
            (USER_ID_KEY.to_string(), self.external_identity.clone()),
            (
                IDENTITY_PROVIDER_KEY.to_string(),
                self.identity_provider.clone(),
            ),
        ])
    }

    pub fn get_id(&self) -> String {
        self.boxer_user_id()
    }

    pub fn get_name(&self) -> String {
        self.boxer_user_id()
    }

    fn boxer_user_id(&self) -> String {
        format!("{}/{}", self.identity_provider, self.external_identity)
    }
}

fn required_claim(
    claims: &HashMap<String, String>,
    key: &'static str,
) -> Result<String, MissingClaim> {
    claims.get(key).cloned().ok_or(MissingClaim { key })
}
