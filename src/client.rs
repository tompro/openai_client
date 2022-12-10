use crate::requests::completion::*;
use crate::types::{OpenAiConfig, OpenAiModel, OpenAiModelResponse, OpenAiResponse};
use crate::OpenAiError::ApiErrorResponse;
use crate::OpenAiResult;
use async_trait::async_trait;
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;

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

    pub async fn post_request<R, T>(&self, endpoint: &str, body: R) -> OpenAiResult<T>
    where
        T: DeserializeOwned,
        R: Serialize,
    {
        let res = self
            .client
            .post(self.config.endpoint_url(endpoint))
            .header(
                "Authorization",
                format!("Bearer {}", self.config.get_access_token()?),
            )
            .json(&body)
            .send()
            .await?
            .json()
            .await?;
        Ok(res)
    }

    fn unwrap_response<T>(&self, response: OpenAiResponse<T>) -> OpenAiResult<T> {
        match response {
            OpenAiResponse::Success(res) => Ok(res),
            OpenAiResponse::Failure(err) => Err(ApiErrorResponse(err)),
        }
    }

    pub async fn get_models(&self) -> OpenAiResult<OpenAiModelResponse> {
        let response = self.get_request("models").await?;
        self.unwrap_response(response)
    }

    pub async fn get_model(&self, model: &str) -> OpenAiResult<OpenAiModel> {
        let resp = self.get_request(&format!("models/{}", model)).await?;
        self.unwrap_response(resp)
    }
}

#[async_trait]
impl CompletionsRequest for OpenAiClient {
    async fn get_completions(
        &self,
        request: CompletionRequest,
    ) -> OpenAiResult<CompletionResponse> {
        self.unwrap_response(self.post_request(&COMPLETION_PATH, request).await?)
    }

    async fn get_completions_json(&self, request: CompletionRequest) -> OpenAiResult<Value> {
        self.post_request(&COMPLETION_PATH, request).await
    }
}
