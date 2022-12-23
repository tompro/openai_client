extern crate core;

#[macro_use]
extern crate derive_builder;

mod client;
mod reqwest_client;
mod types;

pub use types::{
    CompletionRequest, CompletionRequestBuilder, CreateImageRequest, EditRequest,
    EditRequestBuilder, ImageItem, ImageResult, OpenAiConfig, OpenAiError, OpenAiErrorResponse,
    OpenAiModel, OpenAiModelPermission, OpenAiModelResponse, OpenAiResponse, OpenAiResult,
};

pub use client::OpenAiClient;
pub use reqwest_client::OpenAiReqwestClient;
