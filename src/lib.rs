extern crate core;

#[macro_use]
extern crate derive_builder;

mod client;
mod request_client;
mod types;

pub use types::{
    CompletionRequest, CompletionRequestBuilder, EditRequest, EditRequestBuilder, OpenAiConfig,
    OpenAiError, OpenAiErrorResponse, OpenAiModel, OpenAiModelPermission, OpenAiModelResponse,
    OpenAiResponse, OpenAiResult,
};

pub use client::OpenAiClient;
pub use request_client::OpenAiReqwestClient;
