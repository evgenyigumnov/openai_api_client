# About

OpenAI API client for Rust

More information about the OpenAI API:
https://beta.openai.com/docs/

# Usage

```rust

use openai_api_client::*;


#[actix_rt::main]
async fn main() {
    let api_key = "............";
    let model = "text-davinci-003";
    let max_tokens:u32 = 3;
    let result = completions_pretty("Is Biden president of USA?  If you ask yes or not. I say:", model, max_tokens, &api_key).await;
    println!("result: {:?}", result);
    let params = Params {
        model: "text-davinci-003".to_string(),
        temperature: 0,
        max_tokens: 3,
        top_p: 1.0,
        frequency_penalty: 0.0,
        presence_penalty: 0.0,
        stop: vec!["\"\"\"".to_string()],
    };
    let result_hard = completions("Is Biden president of Canada?  If you ask yes or not. I say:", &params, &api_key).await;
    println!("result: {}", result_hard.unwrap().choices[0].text);

}

```