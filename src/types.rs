use crate::{OpenAiError, OpenAiResult};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use std::string::ToString;

const BASE_URL: &str = "https://api.openai.com";
const DEFAULT_VERSION: &str = "v1";
const ENV_TOKEN: &str = "OPENAI_API_KEY";

pub struct OpenAiConfig {
    base_url: String,
    version: String,
    access_token: String,
}

impl OpenAiConfig {
    pub fn new(access_token: &str) -> Self {
        OpenAiConfig::create(BASE_URL, DEFAULT_VERSION, access_token)
    }

    pub fn create(base_url: &str, version: &str, access_token: &str) -> Self {
        OpenAiConfig {
            base_url: base_url.to_string(),
            version: version.to_string(),
            access_token: access_token.to_string(),
        }
    }

    pub fn base_url(mut self, url: &str) -> Self {
        self.base_url = url.to_string();
        self
    }

    pub fn version(mut self, version: &str) -> Self {
        self.version = version.to_string();
        self
    }

    pub fn access_token(mut self, access_token: &str) -> Self {
        self.access_token = access_token.to_string();
        self
    }

    pub fn api_url(&self) -> String {
        format!("{}/{}", self.base_url, self.version)
    }

    pub fn endpoint_url(&self, path: &str) -> String {
        format!("{}/{}", self.api_url(), path)
    }

    pub fn get_access_token(&self) -> OpenAiResult<String> {
        if self.access_token == "" {
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

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum OpenAiResponse<T> {
    Success(T),
    Error(OpenAiErrorResponse),
    Other(Value),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenAiErrorResponse {
    pub error: OpenAiErrorDetails,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAiErrorDetails {
    pub code: Option<String>,
    pub message: String,
    pub param: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}

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

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenAiModelResponse {
    pub data: Vec<OpenAiModel>,
    pub object: Option<String>,
}

#[cfg(test)]
mod test {
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
}
