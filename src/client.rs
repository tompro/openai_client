use crate::requests::completion::*;
use crate::requests::edits::{EditRequest, EditsRequest, EDIT_PATH};
use crate::requests::TextResult;
use crate::types::{OpenAiConfig, OpenAiModel, OpenAiModelResponse, OpenAiResponse};
use crate::OpenAiError::{ApiErrorResponse, UnexpectedJsonResponse};
use crate::OpenAiResult;
use async_trait::async_trait;
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
            OpenAiResponse::Error(err) => Err(ApiErrorResponse(err.error)),
            OpenAiResponse::Other(f) => Err(UnexpectedJsonResponse(f)),
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
    async fn create_completion(&self, request: CompletionRequest) -> OpenAiResult<TextResult> {
        self.unwrap_response(self.post_request(&COMPLETION_PATH, request).await?)
    }
}

#[async_trait]
impl EditsRequest for OpenAiClient {
    async fn create_edit(&self, request: EditRequest) -> OpenAiResult<TextResult> {
        self.unwrap_response(self.post_request(EDIT_PATH, request).await?)
    }
}
