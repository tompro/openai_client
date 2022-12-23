extern crate core;

#[macro_use]
extern crate derive_builder;

mod client;
mod client_api;
mod types;

pub use types::{
    CompletionRequest, CompletionRequestBuilder, CreateImageRequest, EditRequest,
    EditRequestBuilder, ImageItem, ImageResult, OpenAiConfig, OpenAiError, OpenAiErrorResponse,
    OpenAiModel, OpenAiModelPermission, OpenAiModelResponse, OpenAiResponse, OpenAiResult,
};

pub use client::OpenAiClient;
pub use client_api::ClientApi;
