use crate::requests::TextResult;
use crate::OpenAiResult;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub const EDIT_PATH: &str = "edits";

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

#[async_trait]
pub trait EditsRequest {
    async fn create_edit(&self, request: EditRequest) -> OpenAiResult<TextResult>;
}

#[cfg(test)]
mod test {
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
