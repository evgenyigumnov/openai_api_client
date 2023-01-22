use std::collections::HashMap;
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
        suffix: params.suffix.clone(),
        logprobs: params.logprobs,
        echo: params.echo,
        best_of: params.best_of,
        n: params.n,
        stream: params.stream,
        logit_bias: params.logit_bias.clone(),
        user: params.user.clone(),
    };


    let request_string = serde_json::to_string(&request).unwrap();
    println!("{}", request_string);
    let mut resp = client.post("https://api.openai.com/v1/completions")
        .insert_header(("Content-Type", "application/json"))
        .insert_header(("Authorization", format!("Bearer {}", api_key)))
        .send_body(request_string)
        .await.unwrap();

    let result_bytes = resp.body().await.unwrap();
    let result_string = String::from_utf8(result_bytes.to_vec()).unwrap();
    // println!("{}", result_string);
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
pub struct Params {
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

#[cfg(test)]
mod tests {
    use std::env;
    use super::*;
    use dotenv::dotenv;

    #[actix_rt::test]
    async fn it_works() {
        dotenv().ok();
        let api_key = env::var("OPEN_AI_API_KEY").expect("OPEN_AI_API_KEY must be set");

        let params = Params {
            model: "text-davinci-003".to_string(),
            temperature: 0,
            max_tokens: 5,
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


        let result_hard = completions("Is Putin president of Russia? If you ask yes or not. I say:", &params, &api_key).await;
        println!("result: {}", result_hard.unwrap().choices[0].text);
        // let model = "text-davinci-003";
        // let max_tokens:u32 = 3;
        // let result = completions_pretty("Is Putin president of USA?   If you ask yes or not. I say:", model, max_tokens, &api_key).await;
        // println!("result: {:?}", result);
    }
}
