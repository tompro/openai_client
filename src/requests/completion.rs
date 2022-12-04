use crate::requests::StringOrListParam;
use crate::requests::StringOrListParam::{ListParam, StringParam};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::string::ToString;
use crate::client::OpenAiClient;
use crate::OpenAiResult;

pub const COMPLETION_PATH: String = "completions".to_string();

#[derive(Serialize, Deserialize, Builder, Debug, Default)]
#[builder(setter(strip_option, into))]
#[cfg_attr(test, derive(PartialEq))]
pub struct CompletionRequest {
    pub model: String,
    #[builder(default)]
    pub prompt: Option<StringOrListParam>,
    #[builder(default)]
    pub suffix: Option<String>,
    #[builder(default = "16")]
    pub max_tokens: i64,
    #[builder(default = "1")]
    pub temperature: i64,
    #[builder(default = "1")]
    pub top_p: i64,
    #[builder(default = "1")]
    pub n: i64,
    #[builder(default = "false")]
    pub stream: bool,
    #[builder(default)]
    pub logprobs: Option<i64>,
    #[builder(default = "false")]
    pub echo: bool,
    #[builder(default)]
    pub stop: Option<StringOrListParam>,
    #[builder(default = "0")]
    pub presence_penalty: i64,
    #[builder(default = "1")]
    pub best_of: i64,
    #[builder(default)]
    pub logit_bias: Option<HashMap<String, i64>>,
    #[builder(default)]
    pub user: Option<String>,
}


#[derive(Serialize, Deserialize)]
struct Usage {
    pub prompt_tokens: i64,
    pub completion_tokens: i64,
    pub total_tokens: i64,
}

#[derive(Serialize, Deserialize)]
struct Choices {
    pub text: String,
    pub index: i64,
    pub logprobs: Option<_>,
    pub finish_reason: String,
}

#[derive(Serialize, Deserialize)]
struct CompletionResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<Choices>,
    pub usage: Usage,
}

trait CompletionsRequest {
    async fn get_completions(&self, request: CompletionRequest) -> OpenAiResult<CompletionResponse>;
}

impl CompletionsRequest for OpenAiClient {
    async fn get_completions(&self, request: CompletionRequest) -> OpenAiResult<CompletionResponse> {
        self.post_request(&COMPLETION_PATH, request)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn builder_must_fail_on_empty_model() {
        let res = CompletionRequestBuilder::default().build();
        match res {
            Ok(_) => assert!(false, "expected required param error"),
            Err(_) => assert!(true)
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
            .build().unwrap();

        assert_eq!(req, CompletionRequest {
            model: "model".to_string(),
            prompt: Some(StringParam("prompt".to_string())),
            suffix: Some("suffix".to_string()),
            max_tokens: 16,
            temperature: 1,
            top_p: 1,
            n: 100,
            stream: false,
            logprobs: None,
            echo: true,
            stop: None,
            presence_penalty: 0,
            best_of: 1,
            logit_bias: None,
            user: None,
        })
    }

}

