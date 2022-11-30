use reqwest::Client;
use serde::de::DeserializeOwned;
use crate::types::{OpenAiConfig, OpenAiModel, OpenAiResponse, OpenAiResult};
use crate::types::OpenAiError::{ApiErrorResponse, UnexpectedApiResponse};

pub struct OpenAiClient {
    config: OpenAiConfig,
    client: Client,
}

impl OpenAiClient {

    pub fn new(config: OpenAiConfig) -> Self {
        OpenAiClient { config, client: Client::new() }
    }

    async fn get_request<T>(self, endpoint: &str) -> OpenAiResult<T>
    where T: DeserializeOwned {
        let response = self.client.get(self.config.endpoint_url(endpoint))
            .header("Authorization", format!("Bearer {}", self.config.get_access_token()?))
            .send().await?.json().await?;
        self.unwrap_response(response)
    }

    fn unwrap_response<T>(self, response: OpenAiResponse<T>) -> OpenAiResult<T> {
        match response {
            OpenAiResponse { data: Some(value), error: None, .. } => Ok(value),
            OpenAiResponse { error: Some(err), .. } => Err(ApiErrorResponse(err.to_owned())),
            _ => Err(UnexpectedApiResponse),
        }

    }

    pub async fn get_models(self) -> OpenAiResult<Vec<OpenAiModel>> {
        self.get_request("models").await
    }
}
