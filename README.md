# About

OpenAI API client for Rust

More information about the OpenAI API:
https://beta.openai.com/docs/

# Usage

## Completions API

```rust

use openai_api_client::*;


#[actix_rt::main]
async fn main() {
    // pretty usage
    let api_key = "............";
    let model = "text-davinci-003";
    let max_tokens:u32 = 3;
    let prompt = "Is Biden president of USA?  If you ask yes or not. \
     I say:";
    let result: String = completions_pretty(prompt, model, max_tokens, &api_key).await;
    println!("result: {}", result);
    
    // hardcore usage
    let params = CompletionsParams {
        model: model.to_string(),
        temperature: 0,
        max_tokens: max_tokens,
        top_p: 1.0,
        frequency_penalty: 0.0,
        presence_penalty: 0.0,
        stop: None,
        suffix: None,
        n: 1,
        stream: false,
        logprobs: None,
        echo: false,
        best_of: 1,
        logit_bias: None,
        user: None,
    };
    let new_promt = "Is Biden president of Canada?  If you ask yes or \
     not. I say:";
    let result_hard = completions(new_promt, &params, &api_key).await;
    println!("result: {}", result_hard.unwrap().choices[0].text);

}

```

## Edits API

```rust

use openai_api_client::*;


#[actix_rt::main]
async fn main() {
    // pretty usage
    let api_key = "............";
    let result_edits:String  = edits_pretty("Helllo, Mick!", "Fix grammar", 
                                            "text-davinci-edit-001", &api_key).await;
    println!("result: {}", result_edits);


}

```