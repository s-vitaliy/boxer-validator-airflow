use async_trait::async_trait;

pub type TokenIssuerError = Box<dyn std::error::Error + Send + Sync>;

#[async_trait]
pub trait TokenIssuer: Send + Sync {
    async fn create_token(
        &self,
        external_token: &str,
        base_url: &str,
        identity_provider: &str,
    ) -> Result<String, TokenIssuerError>;
}

pub struct ReqwestTokenIssuer {
    client: reqwest::Client,
}

impl ReqwestTokenIssuer {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl TokenIssuer for ReqwestTokenIssuer {
    async fn create_token(
        &self,
        external_token: &str,
        base_url: &str,
        identity_provider: &str,
    ) -> Result<String, TokenIssuerError> {
        let url = format!(
            "{}/api/v1/token/{identity_provider}",
            base_url.trim_end_matches('/')
        );
        let response = self
            .client
            .get(url)
            .bearer_auth(external_token)
            .send()
            .await?
            .error_for_status()?;

        Ok(response.text().await?)
    }
}
