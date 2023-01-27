use std::collections::HashMap;
use std::time::Duration;
use awc::Client;
use serde::Deserialize;
use serde::Serialize;


pub async fn completions(prompt: &str, params: &CompletionsParams, api_key: &str) -> Result<CompletionsResponse, String> {
    let client = Client::default();

    let request: Request = Request {
        model: params.model.clone(),
        prompt: prompt.to_string(),
        temperature: params.temperature,
        max_tokens: params.max_tokens,
        top_p: params.top_p,
        frequency_penalty: params.frequency_penalty,
        presence_penalty: params.presence_penalty,
        stop: params.stop.clone(),
        suffix: params.suffix.clone(),
        logprobs: params.logprobs,
        echo: params.echo,
        best_of: params.best_of,
        n: params.n,
        stream: params.stream,
        logit_bias: params.logit_bias.clone(),
        user: params.user.clone(),
    };


    let request_result = serde_json::to_string(&request);

    match request_result {
        Ok(request_string) => {
            let resp_result = client.post("https://api.openai.com/v1/completions")
                .timeout(Duration::from_secs(30))
                .insert_header(("Content-Type", "application/json"))
                .insert_header(("Authorization", format!("Bearer {}", api_key)))
                .send_body(request_string)
                .await;
            match resp_result {
                Ok(mut resp) => {
                    let bytes_result = resp.body().await;
                    match bytes_result {
                        Ok(bytes) => {
                            let string_result = String::from_utf8(bytes.to_vec());
                            match string_result {
                                Ok(string) => {
                                    let parse_result: Result<CompletionsResponse, serde_json::Error> = serde_json::from_str(&string);
                                    match parse_result {
                                        Ok(response) => Ok(response),
                                        Err(e) => Err(e.to_string()),
                                    }
                                },
                                Err(e) => Err(e.to_string())
                            }
                        },
                        Err(e) => Err(e.to_string())
                    }

                },
                Err(e) => Err(e.to_string())
            }
        },
        Err(e) => Err(e.to_string()),
    }
}





pub async fn completions_pretty(prompt: &str, model: &str, max_tokens: u32, api_key: &str) -> String {
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

    let res = completions(prompt, &params, api_key).await;
    match res {
        Ok(response) => {
            response.choices[0].text.clone()
        }
        Err(e) => e.to_string(),
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub error: ErrorResponseObject,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponseObject {
    pub message: String,
    pub r#type: String,
    pub param: Option<String>,
    pub code: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct CompletionsParams {
    pub model: String,
    pub temperature: u32,
    pub max_tokens: u32,
    pub top_p: f32,
    pub frequency_penalty: f32,
    pub presence_penalty: f32,
    pub stop: Option<Vec<String>>,
    pub suffix: Option<String>,
    pub n: u32,
    pub stream: bool,
    pub logprobs: Option<u32>,
    pub echo: bool,
    pub best_of: u32,
    pub logit_bias: Option<HashMap<String, i32>>,
    pub user: Option<String>,
}



#[derive(Deserialize, Serialize, Debug)]
pub struct CompletionsResponse {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub model: String,
    pub choices: Vec<CompletionsChoice>,
    pub usage: Usage,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CompletionsChoice {
    pub text: String,
    pub index: u32,
    pub logprobs: Option<String>,
    pub finish_reason: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}


#[derive(Deserialize, Serialize)]
pub struct Request {
    pub model: String,
    pub prompt: String,
    pub temperature: u32,
    pub max_tokens: u32,
    pub top_p: f32,
    pub frequency_penalty: f32,
    pub presence_penalty: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    pub n: u32,
    pub stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<u32>,
    pub echo: bool,
    pub best_of: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<String, i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct EditsParams {
    pub model: String,
    pub temperature: u32,
    pub top_p: f32,
    pub n: u32,
}

#[derive(Deserialize, Serialize)]
struct RequestEdit {
    model: String,
    input: String,
    instruction: String,
    n: u32,
    temperature: u32,
    top_p: f32
}

#[derive(Deserialize, Serialize, Debug)]
pub struct EditsResponse {
    pub object: String,
    pub created: u32,
    pub choices: Vec<EditsChoice>,
    pub usage: Usage,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct EditsChoice {
    pub text: String,
    pub index: u32,
}



pub async fn edits(input: &str, instruction: &str, params: &EditsParams, api_key: &str) -> Result<EditsResponse, String> {
    let client = Client::default();

    let request: RequestEdit = RequestEdit {
        model: params.model.clone(),
        input: input.to_string(),
        instruction: instruction.to_string(),
        n: params.n,
        temperature: params.temperature,
        top_p: params.top_p,

    };


    let request_string_result = serde_json::to_string(&request);
    match request_string_result {
        Ok(request_string) => {
            let resp_result = client.post("https://api.openai.com/v1/edits")
                .timeout(Duration::from_secs(30))
                .insert_header(("Content-Type", "application/json"))
                .insert_header(("Authorization", format!("Bearer {}", api_key)))
                .send_body(request_string)
                .await;
            match resp_result {
                Ok(mut resp) => {
                    let bytes_result = resp.body().await;
                    match bytes_result   {
                        Ok(bytes) => {
                            let string_result = String::from_utf8(bytes.to_vec());
                            match string_result {
                                Ok(string) => {
                                    let parse_result: Result<EditsResponse, serde_json::Error> = serde_json::from_str(string.as_str());
                                    match parse_result {
                                        Ok(response) => Ok(response),
                                        Err(e) => Err(e.to_string())
                                    }
                                },
                                Err(e) =>  Err(e.to_string())
                            }
                        },
                        Err(e) =>  Err(e.to_string())
                    }
                },
                Err(e) =>  Err(e.to_string())
            }
        }
        Err(err) =>  Err(err.to_string())
    }
}

pub async fn edits_pretty(input: &str, instruction: &str, model: &str, api_key: &str) -> String {
    let params = EditsParams {
        model: model.to_string(),
        temperature: 0,
        top_p: 1.0,
        n: 1,
    };

    let res = edits(input, instruction, &params, api_key).await;
    match res {
        Ok(response) => {
            response.choices[0].text.clone()
        }
        Err(e) => e.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use super::*;
    use dotenv::dotenv;

    #[actix_rt::test]
    async fn it_works() {
        dotenv().ok();
        let api_key = env::var("OPEN_AI_API_KEY").expect("OPEN_AI_API_KEY must be set");

        let model = "text-davinci-003";
        let max_tokens:u32 = 3;
        let result:String = completions_pretty("Is Madonna president of USA? If you ask yes or not. I say:", model, max_tokens, &api_key).await;
        println!("result: {}", result);

        let result_edits:String  = edits_pretty("Helsllo, Mick!", "Fix grammar", "text-davinci-edit-001", &api_key).await;
        println!("result: {}", result_edits);
    }
}
