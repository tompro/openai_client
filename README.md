# openai_client

[![crates.io](https://img.shields.io/badge/crates.io-v0.1.0-orange)](https://crates.io/crates/openai_client)
![Continuous integration](https://github.com/tompro/openai_client/workflows/Continuous%20integration/badge.svg)

`openai_client` provides configuration, models an a http client for working with 
the API of [OpenAi](https://beta.openai.com/docs/api-reference/) in Rust.

## Cargo

 ```ini
 [dependencies]
 openai_client = "0.1.0"
 ```

 Or via git:

 ```ini
 [dependencies.redis_ts]
 git = "https://github.com/tompro/openai_client.git"
 ```
 
 ## Usage 
 
 ```rust
 use openai_client::*;

// Create client
let client = OpenAiClient::new(OpenAiConfig::new("<ACCESS_TOKEN>"));

// Create request
let request = EditRequestBuilder::default()
    .model("text-davinci-edit-001")
    .input("What day of the wek is it?")
    .instruction("Fix the spelling mistakes")
    .build()
    .unwrap();

// Send request
let result = client.create_edit(request).await?;
```

### Features
- [x] Models
- [x] Completions
- [x] Edits
- [ ] Images
    - [x] Create
    - [ ] Edit
    - [ ] Variations
- [ ] Embeddings
- [ ] Files
- [ ] Fine Tunes
- [ ] Moderations
- [ ] Engines
- [ ] Parameter details
