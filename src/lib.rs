//! `openai_client` provides configuration, models an a http client for working with
//! the API of [OpenAi](https://beta.openai.com/docs/api-reference/) in Rust.
//!
//! # Cargo
//!
//! ```ini
//! [dependencies]
//! openai_client = "0.1.0"
//! ```
//! Or via git:
//!
//! ```ini
//! [dependencies.redis_ts]
//! git = "https://github.com/tompro/openai_client.git"
//! ```
//!
//! # Usage
//!
//! ```rust,no_run
//! # async fn run() -> openai_client::OpenAiResult<()> {
//! use openai_client::*;
//!
//! // Create client
//! let client = OpenAiClient::new(OpenAiConfig::new("<ACCESS_TOKEN>"));
//!
//! // Create request
//! let request = EditRequestBuilder::default()
//!     .model("text-davinci-edit-001")
//!     .input("What day of the wek is it?")
//!     .instruction("Fix the spelling mistakes")
//!     .build()
//!     .unwrap();
//!
//! // Send request
//! let result = client.create_edit(request).await?;
//! # Ok(())}
//! ```
//!
//! # Supported operations
//!
//! To create a client you can either provide your own configuration with the
//! access token required by OpenAi or use the default which will in turn use
//! the default configuration. The default configuration expects the Api token
//! environment variable OPENAI_API_KEY to be populated with your credentials.
//!
//! All currently supported operations have a builder for the request payload,
//! can be configured via a config struct and return either a struct with the
//! expected success response or an error of type OpenAiError.
//!
//! ## Client
//!
//! ```rust,no_run
//! # use openai_client::*;
//!
//! // custom configuration
//! let client = OpenAiClient::new(OpenAiConfig::new("<ACCESS_TOKEN>"));
//!
//! // default client access token from env
//! let client = OpenAiClient::default();
//! ```
//!
//! ## Models
//! List and describe the various models available in the API.
//!
//! ```rust,no_run
//! # use openai_client::ClientApi;
//!  async fn run() -> openai_client::OpenAiResult<()> {
//! # use openai_client::*;
//! # let client = OpenAiClient::default();
//! // fetch all models provided by OpenAi
//! let models = client.get_models().await?;
//!
//! // fetch a specific model
//! let model: OpenAiModel = client.get_model("text-davinci-003").await?;
//! # Ok(())}
//! ```
//!
//! ## Edits
//! Given a prompt and an instruction, the model will return an edited version of the prompt.
//!
//! ```rust,no_run
//! # use openai_client::ClientApi;
//!  async fn run() -> openai_client::OpenAiResult<()> {
//! # use openai_client::*;
//! # let client = OpenAiClient::default();
//! let request = EditRequestBuilder::default()
//!     .model("text-davinci-edit-001")
//!     .input("What day of the wek is it?")
//!     .instruction("Fix the spelling mistakes")
//!     .build()
//!     .unwrap();
//!
//! let result: TextResult = client.create_edit(request).await?;
//! assert!(!result.choices.is_empty());
//! # Ok(())}
//! ```
//!
//! ## Completions
//! Given a prompt, the model will return one or more predicted completions,
//! and can also return the probabilities of alternative tokens at each position.
//!
//! ```rust,no_run
//! # use openai_client::ClientApi;
//!  async fn run() -> openai_client::OpenAiResult<()> {
//! # use openai_client::*;
//! # let client = OpenAiClient::default();
//! let request = CompletionRequestBuilder::default()
//!     .model("text-davinci-003")
//!     .prompt("I am so tired I could")
//!     .build()
//!     .unwrap();
//!
//! let result: TextResult = client.create_completion(request).await?;
//! assert!(!result.choices.is_empty());
//! # Ok(())}
//! ```
//!
//! ## Generate Image
//! Creates an image given a prompt.
//!
//! ```rust,no_run
//! # use openai_client::ClientApi;
//!  async fn run() -> openai_client::OpenAiResult<()> {
//! # use openai_client::*;
//! # let client = OpenAiClient::default();
//! let request = CreateImageRequestBuilder::default()
//!     .prompt("A cute baby sea otter")
//!     .size("1024x1024")
//!     .n(2)
//!     .build()
//!     .unwrap();
//! let result: ImageResult = client.create_image(request).await?;
//! assert_eq!(!result.data.len(), 2);
//! # Ok(())}
//! ```
//!
//!
extern crate core;

#[macro_use]
extern crate derive_builder;

mod client;
mod client_api;
mod types;

pub use types::{
    CompletionRequest, CompletionRequestBuilder, CreateImageRequest, CreateImageRequestBuilder,
    EditRequest, EditRequestBuilder, ImageItem, ImageResult, OpenAiConfig, OpenAiError,
    OpenAiErrorResponse, OpenAiModel, OpenAiModelPermission, OpenAiModelResponse, OpenAiResponse,
    OpenAiResult, TextChoice, TextResult,
};

pub use client::OpenAiClient;
pub use client_api::ClientApi;
