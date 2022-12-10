extern crate core;

#[macro_use]
extern crate derive_builder;

use crate::types::OpenAiErrorDetails;
use serde_json::Value;
use thiserror::Error;

pub mod client;
pub mod requests;
pub mod types;

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

// #[cfg(test)]
// mod tests {
//     use crate::client::OpenAiClient;
//     use crate::requests::completion::*;
//     use crate::requests::edits::{EditRequestBuilder, EditsRequest};
//     use crate::test_helpers::get_test_config;
//     use crate::OpenAiError::*;
//
//     #[tokio::test]
//     async fn test_get_models() {
//         let client = OpenAiClient::new(get_test_config());
//         match client.get_models().await {
//             Ok(models) => {
//                 for model in models.data {
//                     println!("{:?}", model)
//                 }
//             }
//             Err(ApiErrorResponse(e)) => {
//                 println!("Api error response: {:?}", e)
//             }
//             Err(e) => println!("other err: {:?}", e),
//         }
//     }
//
//     #[tokio::test]
//     async fn test_get_model() {
//         let client = OpenAiClient::new(get_test_config());
//         match client.get_model("text-davinci-003").await {
//             Ok(model) => {
//                 println!("{:?}", model)
//             }
//             Err(ApiErrorResponse(e)) => {
//                 println!("Api error response: {:?}", e)
//             }
//             Err(e) => println!("other err: {:?}", e),
//         }
//     }
//
//     #[tokio::test]
//     async fn test_completion() {
//         let input = "So tired I could";
//
//         let client = OpenAiClient::new(get_test_config());
//         let res = client
//             .create_completion(
//                 CompletionRequestBuilder::default()
//                     .model("text-davinci-003")
//                     .prompt(input)
//                     .build()
//                     .unwrap(),
//             )
//             .await;
//
//         match res {
//             Ok(resp) => {
//                 println!(
//                     "Completion result {}{}",
//                     input,
//                     resp.choices.first().unwrap().text
//                 )
//             }
//             Err(e) => {
//                 println!("Failed to generate completion {:?}", e)
//             }
//         }
//     }
//
//     #[tokio::test]
//     async fn test_edit() {
//         let input = "Wave–particle duality";
//         let instruction = "Explain to me as if I where a child";
//
//         let client = OpenAiClient::new(get_test_config());
//         let res = client
//             .create_edit(
//                 EditRequestBuilder::default()
//                     .model("text-davinci-edit-001")
//                     .input(input)
//                     .instruction(instruction)
//                     .build()
//                     .unwrap(),
//             )
//             .await;
//
//         match res {
//             Ok(resp) => {
//                 println!("Result: {}", resp.choices.first().unwrap().text)
//             }
//             Err(e) => {
//                 println!("Failed to generate edit {:?}", e)
//             }
//         }
//     }
// }
//
// #[cfg(test)]
// mod test_helpers {
//
//     use crate::types::OpenAiConfig;
//     use dotenv::dotenv;
//
//     pub fn get_test_config() -> OpenAiConfig {
//         dotenv().ok();
//         OpenAiConfig::default()
//     }
// }
