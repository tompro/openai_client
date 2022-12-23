use crate::types::TextResult;
use crate::{
    CompletionRequest, CreateImageRequest, EditRequest, ImageResult, OpenAiModel,
    OpenAiModelResponse, OpenAiResult,
};
use async_trait::async_trait;

#[async_trait]
pub trait ClientApi {
    async fn create_completion(&self, request: CompletionRequest) -> OpenAiResult<TextResult>;
    async fn create_edit(&self, request: EditRequest) -> OpenAiResult<TextResult>;
    async fn get_models(&self) -> OpenAiResult<OpenAiModelResponse>;
    async fn get_model(&self, model: &str) -> OpenAiResult<OpenAiModel>;
    async fn create_image(&self, request: CreateImageRequest) -> OpenAiResult<ImageResult>;
}
