use crate::types::{OpenAiConfig, OpenAiModel, OpenAiResponse};
use crate::OpenAiError::{ApiErrorResponse, UnexpectedApiResponse};
use crate::OpenAiResult;
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct OpenAiClient {
    config: OpenAiConfig,
    client: Client,
}

impl OpenAiClient {
    pub fn new(config: OpenAiConfig) -> Self {
        OpenAiClient {
            config,
            client: Client::new(),
        }
    }

    pub async fn get_request<T>(&self, endpoint: &str) -> OpenAiResult<T>
    where
        T: DeserializeOwned,
    {
        let res = self
            .client
            .get(self.config.endpoint_url(endpoint))
            .header(
                "Authorization",
                format!("Bearer {}", self.config.get_access_token()?),
            )
            .send()
            .await?
            .json()
            .await?;
        Ok(res)
    }

    pub async fn post_request<R,T>(&self, endpoint: &str, body: R) -> OpenAiResult<T>
        where
            T: DeserializeOwned,
            R: Serialize,
    {
        let res = self
            .client
            .post(self.config.endpoint_url(endpoint))
            .header("Authorization",
                    format!("Bearer {}", self.conf.get_access_token()?),
            )
            .json(body)
            .send()
            .await?
            .json()
            .await?;
        Ok(res)
    }

    fn unwrap_response<T>(&self, response: OpenAiResponse<T>) -> OpenAiResult<T> {
        match response {
            OpenAiResponse {
                data: Some(value),
                error: None,
                ..
            } => Ok(value),
            OpenAiResponse {
                error: Some(err), ..
            } => Err(ApiErrorResponse(err.to_owned())),
            _ => Err(UnexpectedApiResponse),
        }
    }

    pub async fn get_models(&self) -> OpenAiResult<Vec<OpenAiModel>> {
        let response = self.get_request("models").await?;
        self.unwrap_response(response)
    }

    pub async fn get_model(&self, model: &str) -> OpenAiResult<OpenAiModel> {
        self.get_request(&format!("models/{}", model)).await
    }

}
