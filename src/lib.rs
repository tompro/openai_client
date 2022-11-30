extern crate core;

pub mod client;
pub mod types;

#[cfg(test)]
mod tests {
    use crate::client::OpenAiClient;
    use crate::test_helpers::get_test_config;
    use crate::types::OpenAiError::ApiErrorResponse;

    #[tokio::test]
    async fn test_get_models() {

        let client = OpenAiClient::new(get_test_config());
        match client.get_models().await {
            Ok(models) => {
                for model in models {
                    println!("{:?}", model)
                }
            },
            Err(ApiErrorResponse(e)) => {
                println!("Api error response: {:?}", e)
            },
            Err(e) => println!("other err: {:?}", e),

        }
    }
}

#[cfg(test)]
mod test_helpers {

    use crate::types::OpenAiConfig;
    use dotenv::dotenv;

    pub fn get_test_config() -> OpenAiConfig {
        dotenv().ok();
        OpenAiConfig::default()
    }
}
