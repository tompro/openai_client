use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::string::ToString;

use thiserror::Error;

const BASE_URL: &str = "https://api.openai.com";
const DEFAULT_VERSION: &str = "v1";
const ENV_TOKEN: &str = "OPENAI_API_KEY";

pub type OpenAiResult<R> = Result<R, OpenAiError>;

#[derive(Error, Debug)]
pub enum OpenAiError {
    #[error("missing openAi access token in config and env OPENAI_API_KEY")]
    MissingTokenError,

    #[error("missing required parameter {name} in request {request}")]
    MissingRequestParameter { name: String, request: String },

    #[error("openAi API returned unexpected response body")]
    UnexpectedApiResponse,

    #[error("openAi API returned error")]
    ApiErrorResponse(OpenAiErrorDetails),

    #[error("openAi API returned unexpected json")]
    UnexpectedJsonResponse(Value),

    #[error("failed to execute openAi request")]
    HttpError(#[from] reqwest::Error),

    #[error("failed to parse or encode json")]
    JsonEncodeError(#[from] serde_json::Error),
}

pub struct OpenAiConfig {
    base_url: String,
    version: String,
    access_token: String,
    model_path: String,
    completion_path: String,
    edit_path: String,
    image_path: String,
    image_create: String,
    image_edits: String,
    image_variations: String,
}

/// Basic configuration params for running requests against OpenAi Api.
impl OpenAiConfig {
    pub fn new(access_token: &str) -> Self {
        OpenAiConfig::create(BASE_URL, DEFAULT_VERSION, access_token)
    }

    pub fn create(base_url: &str, version: &str, access_token: &str) -> Self {
        OpenAiConfig {
            base_url: base_url.to_string(),
            version: version.to_string(),
            access_token: access_token.to_string(),
            model_path: "models".to_string(),
            completion_path: "completions".to_string(),
            edit_path: "edits".to_string(),
            image_path: "images".to_string(),
            image_create: "generations".to_string(),
            image_edits: "edits".to_string(),
            image_variations: "variations".to_string(),
        }
    }

    /// Set the base url for the Api.
    pub fn base_url(mut self, url: &str) -> Self {
        self.base_url = url.to_string();
        self
    }

    /// Set the Api version to use.
    pub fn version(mut self, version: &str) -> Self {
        self.version = version.to_string();
        self
    }

    /// Set the access_token for billing programmatically. If not
    /// set will try to resolve from env var.
    pub fn access_token(mut self, access_token: &str) -> Self {
        self.access_token = access_token.to_string();
        self
    }

    /// Returns the Api url for given path.
    pub fn api_url(&self, path: &str) -> String {
        format!("{}/{}", self.base_url, path)
    }

    /// Returns the models path.
    pub fn get_models_path(&self) -> String {
        self.add_path_segment(&self.version, &self.model_path)
    }

    /// Returns the path for a specific model
    pub fn get_model_path(&self, model: &str) -> String {
        self.add_path_segment(&self.get_models_path(), model)
    }

    /// Returns the create image path
    pub fn get_create_image_path(&self) -> String {
        self.image_path(&self.image_create)
    }

    /// Returns the edit image path
    pub fn get_edit_image_path(&self) -> String {
        self.image_path(&self.image_edits)
    }

    /// Returns the image variations path
    pub fn get_image_variations_path(&self) -> String {
        self.image_path(&self.image_variations)
    }

    /// Returns the text edit path
    pub fn get_edit_path(&self) -> String {
        self.add_path_segment(&self.version, &self.edit_path)
    }

    /// Returns the text completions path
    pub fn get_completion_path(&self) -> String {
        self.add_path_segment(&self.version, &self.completion_path)
    }

    fn image_path(&self, segment: &str) -> String {
        format!("{}/{}/{}", self.version, self.image_path, segment)
    }

    fn add_path_segment(&self, path: &str, segment: &str) -> String {
        format!("{}/{}", path, segment)
    }

    /// Returns the OpenAi Api access token
    pub fn get_access_token(&self) -> OpenAiResult<String> {
        if self.access_token.is_empty() {
            match env::var(ENV_TOKEN) {
                Ok(token) => Ok(token),
                Err(_) => Err(OpenAiError::MissingTokenError),
            }
        } else {
            Ok(self.access_token.to_string())
        }
    }
}

impl Default for OpenAiConfig {
    fn default() -> Self {
        OpenAiConfig::new("")
    }
}

/// A wrapper around the OpenAi response payload.
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum OpenAiResponse<T> {
    Success(T),
    Error(OpenAiErrorResponse),
    Other(Value),
}

/// The payload of an OpenAi error response.
#[derive(Serialize, Deserialize, Debug)]
pub struct OpenAiErrorResponse {
    pub error: OpenAiErrorDetails,
}

/// The error details provided by OpenAi error responses.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAiErrorDetails {
    pub code: Option<String>,
    pub message: String,
    pub param: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}

/// Model permissions response.
#[derive(Serialize, Deserialize, Debug)]
pub struct OpenAiModelPermission {
    pub allow_create_engine: bool,
    pub allow_fine_tuning: bool,
    pub allow_logprobs: bool,
    pub allow_sampling: bool,
    pub allow_search_indices: bool,
    pub allow_view: bool,
    pub created: i64,
    pub group: Option<String>,
    pub id: String,
    pub is_blocking: bool,
    pub object: String,
    pub organization: String,
}

/// Model properties response
#[derive(Serialize, Deserialize, Debug)]
pub struct OpenAiModel {
    pub created: i64,
    pub id: String,
    pub object: String,
    pub owned_by: String,
    pub parent: Option<String>,
    pub permission: Vec<OpenAiModelPermission>,
    pub root: String,
}

/// Models list response.
#[derive(Serialize, Deserialize, Debug)]
pub struct OpenAiModelResponse {
    pub data: Vec<OpenAiModel>,
    pub object: Option<String>,
}

/// Multi type response that can either be a string or
/// a list of strings.
#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(untagged)]
pub enum StringOrListParam {
    StringParam(String),
    ListParam(Vec<String>),
}

impl Clone for StringOrListParam {
    fn clone(&self) -> Self {
        match self {
            StringOrListParam::StringParam(str) => StringOrListParam::StringParam(str.clone()),
            StringOrListParam::ListParam(list) => StringOrListParam::ListParam(list.clone()),
        }
    }
}

impl From<&str> for StringOrListParam {
    fn from(value: &str) -> Self {
        StringOrListParam::StringParam(value.to_string())
    }
}

impl From<Vec<&str>> for StringOrListParam {
    fn from(value: Vec<&str>) -> Self {
        StringOrListParam::ListParam(value.iter().map(|s| s.to_string()).collect())
    }
}

impl From<&Vec<&str>> for StringOrListParam {
    fn from(value: &Vec<&str>) -> Self {
        StringOrListParam::ListParam(value.iter().map(|s| s.to_string()).collect())
    }
}

/// The token usage as returned in some responses.
#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: i64,
    pub completion_tokens: Option<i64>,
    pub total_tokens: i64,
}

/// Container for a text base result.
#[derive(Serialize, Deserialize, Debug)]
pub struct TextResult {
    pub id: Option<String>,
    pub object: String,
    pub created: i64,
    pub model: Option<String>,
    pub choices: Vec<TextChoice>,
    pub usage: Usage,
}

/// A choice result for text based operations
#[derive(Serialize, Deserialize, Debug)]
pub struct TextChoice {
    pub text: String,
    pub index: i64,
    pub logprobs: Option<i64>,
    pub finish_reason: Option<String>,
}

/// A single image item
#[derive(Serialize, Deserialize, Debug)]
pub struct ImageItem {
    pub url: Option<String>,
    pub b64_json: Option<String>,
}

/// A result returned by image operations
#[derive(Serialize, Deserialize, Debug)]
pub struct ImageResult {
    pub created: u64,
    pub data: Vec<ImageItem>,
}

/// Json data required for doing text completion requests.
#[derive(Serialize, Deserialize, Builder, Debug, Default)]
#[builder(setter(strip_option, into))]
#[cfg_attr(test, derive(PartialEq))]
pub struct CompletionRequest {
    pub model: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<StringOrListParam>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<i64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<i64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<i64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub echo: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<StringOrListParam>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<i64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<i64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_of: Option<i64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<String, i64>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// Json data required for doing text edit requests.
#[derive(Serialize, Deserialize, Builder, Debug, Default)]
#[builder(setter(strip_option, into))]
#[cfg_attr(test, derive(PartialEq))]
pub struct EditRequest {
    pub model: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    pub instruction: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<i64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<i64>,
}

/// Json data required for doing image generation requests.
#[derive(Serialize, Deserialize, Builder, Debug, Default)]
#[builder(setter(strip_option, into))]
#[cfg_attr(test, derive(PartialEq))]
pub struct CreateImageRequest {
    pub prompt: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[cfg(test)]
mod image {
    use crate::types::{CreateImageRequest, CreateImageRequestBuilder};

    #[test]
    fn should_build_an_image_create_request() {
        let request = CreateImageRequestBuilder::default()
            .prompt("A cute baby sea otter")
            .size("256x256")
            .build()
            .unwrap();
        let expected = CreateImageRequest {
            prompt: "A cute baby sea otter".to_string(),
            n: None,
            size: Some("256x256".to_string()),
            response_format: None,
            user: None,
        };
        assert_eq!(request, expected);
    }
}
#[cfg(test)]
mod config {
    use super::StringOrListParam::*;
    use super::*;

    #[test]
    fn should_create_config_new() {
        let token = "test";
        let conf = OpenAiConfig::new(token);
        assert_eq!(conf.base_url, BASE_URL);
        assert_eq!(conf.version, DEFAULT_VERSION);
        assert_eq!(conf.access_token, token);
    }

    #[test]
    fn should_create_conf_default_from_env() {
        let token = "env_token";
        env::set_var(ENV_TOKEN, token);
        let conf = OpenAiConfig::default();
        assert_eq!(conf.base_url, BASE_URL);
        assert_eq!(conf.version, DEFAULT_VERSION);
        assert_eq!(conf.get_access_token().unwrap(), token);
        env::remove_var(ENV_TOKEN);
    }

    #[test]
    fn must_serde_string() {
        let test: StringOrListParam = StringParam("test_string".to_string());
        let value: Value = serde_json::to_value(&test).unwrap();
        let res: StringOrListParam = serde_json::from_value(value).unwrap();
        assert_eq!(test, res);
    }

    #[test]
    fn must_serde_list() {
        let test: StringOrListParam =
            ListParam(vec!["test_string".to_string(), "test_string2".to_string()]);
        let value: Value = serde_json::to_value(&test).unwrap();
        let res: StringOrListParam = serde_json::from_value(value).unwrap();
        match res {
            StringParam(_) => assert!(false),
            ListParam(ref list) => assert_eq!(list.len(), 2),
        }
        assert_eq!(test, res);
    }
}

#[cfg(test)]
mod completion {
    use super::StringOrListParam::*;
    use super::*;

    #[test]
    fn builder_must_fail_on_empty_model() {
        let res = CompletionRequestBuilder::default().build();
        match res {
            Ok(_) => assert!(false, "expected required param error"),
            Err(_) => assert!(true),
        }
    }

    #[test]
    fn builder_must_set_model() {
        let req = CompletionRequestBuilder::default()
            .model("test")
            .build()
            .unwrap();
        assert_eq!(req.model, "test".to_string())
    }

    #[test]
    fn builder_must_set_suffix() {
        let req = CompletionRequestBuilder::default()
            .model("test")
            .suffix("test")
            .build()
            .unwrap();
        assert_eq!(req.suffix, Some("test".to_string()))
    }

    #[test]
    fn builder_must_set_string_prompt() {
        let req = CompletionRequestBuilder::default()
            .model("test")
            .prompt("test")
            .build()
            .unwrap();
        match req.prompt {
            Some(StringParam(s)) => assert_eq!(s, "test".to_string()),
            _ => assert!(false, "prompt did not match a StringParam"),
        }
    }

    #[test]
    fn builder_must_set_list_prompt() {
        let req = CompletionRequestBuilder::default()
            .model("test")
            .prompt(vec!["a", "b"])
            .build()
            .unwrap();
        match req.prompt {
            Some(ListParam(s)) => assert_eq!(s, vec!["a", "b"]),
            _ => assert!(false, "prompt did not match a ListParam"),
        }
    }

    #[test]
    fn builder_must_set_ref_list_prompt() {
        let list = vec!["test1", "test2"];
        let req = CompletionRequestBuilder::default()
            .model("test")
            .prompt(&list)
            .build()
            .unwrap();
        match req.prompt {
            Some(ListParam(s)) => assert_eq!(s, list),
            _ => assert!(false, "prompt did not match a ListParam"),
        }
    }

    #[test]
    fn must_correctly_build() {
        let req = CompletionRequestBuilder::default()
            .model("model")
            .n(100)
            .prompt("prompt")
            .suffix("suffix")
            .best_of(true)
            .echo(true)
            .stream(true)
            .build()
            .unwrap();

        assert_eq!(
            req,
            CompletionRequest {
                model: "model".to_string(),
                prompt: Some(StringParam("prompt".to_string())),
                suffix: Some("suffix".to_string()),
                max_tokens: None,
                temperature: None,
                top_p: None,
                n: Some(100),
                stream: Some(true),
                logprobs: None,
                echo: Some(true),
                stop: None,
                presence_penalty: None,
                frequency_penalty: None,
                best_of: Some(1),
                logit_bias: None,
                user: None,
            }
        )
    }
}

#[cfg(test)]
mod edit {
    use super::*;

    #[test]
    fn builder_must_fail_on_empty_model_or_prompt() {
        match EditRequestBuilder::default()
            .instruction("instruction")
            .build()
        {
            Ok(_) => assert!(false, "expected missing model err"),
            Err(_) => assert!(true),
        }
        match EditRequestBuilder::default().model("model").build() {
            Ok(_) => assert!(false, "expected missing instructions err"),
            Err(_) => assert!(true),
        }
    }

    #[test]
    fn builder_must_create_successful_request() {
        let request = EditRequestBuilder::default()
            .model("model")
            .input("input")
            .instruction("instructions")
            .build()
            .unwrap();

        assert_eq!(
            request,
            EditRequest {
                model: "model".to_string(),
                input: Some("input".to_string()),
                instruction: "instructions".to_string(),
                n: None,
                temperature: None,
                top_p: None,
            }
        )
    }
}
