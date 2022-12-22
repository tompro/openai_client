use crate::types::TextResult;
use crate::OpenAiError::{ApiErrorResponse, UnexpectedJsonResponse};
use crate::{
    CompletionRequest, EditRequest, OpenAiClient, OpenAiConfig, OpenAiModel, OpenAiModelResponse,
    OpenAiResponse, OpenAiResult,
};
use async_trait::async_trait;
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct OpenAiReqwestClient {
    config: OpenAiConfig,
    client: Client,
}

impl OpenAiReqwestClient {
    pub fn new(config: OpenAiConfig) -> Self {
        OpenAiReqwestClient {
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
            .get(self.config.api_url(endpoint))
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
            .post(self.config.api_url(endpoint))
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
}

#[async_trait]
impl OpenAiClient for OpenAiReqwestClient {
    async fn create_completion(&self, request: CompletionRequest) -> OpenAiResult<TextResult> {
        self.unwrap_response(
            self.post_request(&self.config.get_completion_path(), request)
                .await?,
        )
    }

    async fn create_edit(&self, request: EditRequest) -> OpenAiResult<TextResult> {
        self.unwrap_response(
            self.post_request(&self.config.get_edit_path(), request)
                .await?,
        )
    }

    async fn get_models(&self) -> OpenAiResult<OpenAiModelResponse> {
        let response = self.get_request(&self.config.get_models_path()).await?;
        self.unwrap_response(response)
    }

    async fn get_model(&self, model: &str) -> OpenAiResult<OpenAiModel> {
        let resp = self.get_request(&self.config.get_model_path(model)).await?;
        self.unwrap_response(resp)
    }
}

#[cfg(test)]
mod request_client {
    use crate::request_client::test_helpers::{create_test_server_config, json_response};
    use crate::{
        CompletionRequestBuilder, EditRequestBuilder, OpenAiClient, OpenAiError,
        OpenAiReqwestClient,
    };
    use wiremock::matchers::{body_json, method, path};
    use wiremock::{Mock, ResponseTemplate};

    #[tokio::test]
    async fn should_give_http_error_for_invalid_response() {
        let (config, server) = create_test_server_config().await;
        Mock::given(method("GET"))
            .and(path(config.get_models_path()))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;

        let client = OpenAiReqwestClient::new(config);
        match client.get_models().await {
            Err(OpenAiError::HttpError(_)) => assert!(true),
            _ => assert!(false, "expected response parsing error"),
        }
    }

    #[tokio::test]
    async fn should_return_model_success() {
        let (config, server) = create_test_server_config().await;
        Mock::given(method("GET"))
            .and(path(config.get_model_path("text-davinci-003")))
            .respond_with(ResponseTemplate::new(200).set_body_json(json_response("model_response")))
            .mount(&server)
            .await;

        let client = OpenAiReqwestClient::new(config);
        match client.get_model("text-davinci-003").await {
            Ok(_) => assert!(true),
            Err(_) => assert!(false, "expected success response"),
        }
    }

    #[tokio::test]
    async fn should_return_models_success() {
        let (config, server) = create_test_server_config().await;
        Mock::given(method("GET"))
            .and(path(config.get_models_path()))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(json_response("models_response")),
            )
            .mount(&server)
            .await;

        let client = OpenAiReqwestClient::new(config);
        match client.get_models().await {
            Ok(_) => assert!(true),
            Err(_) => assert!(false, "expected success response"),
        }
    }

    #[tokio::test]
    async fn should_return_edit_response() {
        let (config, server) = create_test_server_config().await;

        let request = EditRequestBuilder::default()
            .model("text-davinci-edit-001")
            .input("What day of the wek is it?")
            .instruction("Fix the spelling mistakes")
            .build()
            .unwrap();

        let json = serde_json::to_value(&request).expect("request serialized");

        Mock::given(method("POST"))
            .and(path(config.get_edit_path()))
            .and(body_json(json))
            .respond_with(ResponseTemplate::new(200).set_body_json(json_response("edit_response")))
            .mount(&server)
            .await;

        let client = OpenAiReqwestClient::new(config);
        match client.create_edit(request).await {
            Ok(_) => assert!(true),
            Err(e) => {
                println!("ERR: {:?}", e);
                assert!(false, "expected success response")
            }
        }
    }

    #[tokio::test]
    async fn should_return_completion_response() {
        let (config, server) = create_test_server_config().await;

        let request = CompletionRequestBuilder::default()
            .model("text-davinci-003")
            .prompt("I am so tired I could")
            .build()
            .unwrap();

        let json = serde_json::to_value(&request).expect("request serialized");

        Mock::given(method("POST"))
            .and(path(config.get_completion_path()))
            .and(body_json(json))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(json_response("completion_response")),
            )
            .mount(&server)
            .await;

        let client = OpenAiReqwestClient::new(config);
        match client.create_completion(request).await {
            Ok(_) => assert!(true),
            Err(e) => {
                println!("ERR: {:?}", e);
                assert!(false, "expected success response")
            }
        }
    }
}

#[cfg(test)]
mod test_helpers {
    use crate::types::OpenAiConfig;
    use serde_json::Value;
    use std::fs::File;
    use std::io::Read;
    use wiremock::MockServer;

    pub async fn create_test_server_config() -> (OpenAiConfig, MockServer) {
        let server = MockServer::start().await;
        (get_test_config_mock(&server.uri()), server)
    }

    pub fn get_test_config_mock(base_uri: &str) -> OpenAiConfig {
        OpenAiConfig::default()
            .base_url(base_uri)
            .access_token("mock_token")
    }

    pub fn json_response(file_name: &str) -> Value {
        let mut file = File::open(&format!("test_data/{}.json", file_name))
            .expect(&format!("json test data {}.json exists", file_name));
        let mut string = String::new();
        file.read_to_string(&mut string)
            .expect("json read to buffer");
        serde_json::from_str(&string).expect("json parsed to value")
    }
}
