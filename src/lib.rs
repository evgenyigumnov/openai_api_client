use std::io::Error;
use awc::Client;
use serde::Deserialize;
use serde::Serialize;


pub async fn completions(prompt: &str, params: &Params, api_key: &str) -> std::io::Result<Response> {
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
    };


    let mut resp = client.post("https://api.openai.com/v1/completions")
        .insert_header(("Content-Type", "application/json"))
        .insert_header(("Authorization", format!("Bearer {}", api_key)))
        .send_json(&request)
        .await.unwrap();

    let result_bytes = resp.body().await.unwrap();
    let result_string = String::from_utf8(result_bytes.to_vec()).unwrap();
    let result: Result<Response, serde_json::Error> = serde_json::from_str(&result_string);
    match result {
        Ok(response) => Ok(response),
        Err(_e) => {
            let result_err: ErrorResponse = serde_json::from_str(&result_string).unwrap();
            Err(Error::new(std::io::ErrorKind::Other, result_err.error.message))
        }
    }
}


pub async fn completions_pretty(prompt: &str, model: &str, max_tokens: u32, api_key: &str) -> String {

    let params = Params {
        model: model.to_string(),
        temperature: 0,
        max_tokens: max_tokens,
        top_p: 1.0,
        frequency_penalty: 0.0,
        presence_penalty: 0.0,
        stop: vec!["\"\"\"".to_string()],
    };

    let res = completions(prompt,& params, api_key).await;
    match res {
        Ok(response) => {
            let mut result = String::new();
            for choice in response.choices {
                result.push_str(&choice.text);
                result.push_str(" ");
            }
            result
        }
        Err(e) => e.to_string(),
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub error: ErrorResponseObject
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponseObject {
    pub message: String,
    pub r#type: String,
    pub param: Option<String>,
    pub code: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Params {
    pub model: String,
    pub temperature: u32,
    pub max_tokens: u32,
    pub top_p: f32,
    pub frequency_penalty: f32,
    pub presence_penalty: f32,
    pub stop: Vec<String>,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct Response {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Choice {
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
    pub stop: Vec<String>,
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
