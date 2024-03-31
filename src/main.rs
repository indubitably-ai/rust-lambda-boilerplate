// This example requires the following input to succeed:
// { "command": "do something" }

use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};

/// This is also a made-up example. Requests come into the runtime as unicode
/// strings in json format, which can map to any structure that implements `serde::Deserialize`
/// The runtime pays no attention to the contents of the request payload.
#[derive(Deserialize)]
struct Request {
    command: String,
}

/// This is also a made-up example of what a response structure may look like.
/// There is no restriction on what it can be. The runtime requires responses
/// to be serialized into json. The runtime pays no attention
/// to the contents of the response payload.
#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // required to enable CloudWatch error logging by the runtime
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    let func = service_fn(my_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

use reqwest::{Client, Error as ReqwestError};
use serde_json::json;

// Configuration for the chatgpt API
#[derive(Deserialize)]
struct ChatGPTConfig {
    model: String,
    temperature: f64,
    max_tokens: u32,
}

pub(crate) async fn my_handler(event: LambdaEvent<Request>, config: ChatGPTConfig) -> Result<Response, Error> {
    // extract some useful info from the request
    let command = event.payload.command;

    // Prepare the request to the chatgpt API
    let client = Client::new();
    let api_url = "https://api.openai.com/v1/engines/chatgpt/completions"; // Replace with the actual chatgpt API URL
    let request_body = json!({
        "model": config.model,
        "prompt": command,
        "temperature": config.temperature,
        "max_tokens": config.max_tokens,
    });

    // Send the request to the chatgpt API
    let api_response = client.post(api_url)
        .bearer_auth("YOUR_API_KEY") // Replace with your actual API key
        .json(&request_body)
        .send()
        .await
        .map_err(|e| Error::from(ReqwestError::from(e)))?;

    // Check if the response from the API is successful
    if !api_response.status().is_success() {
        return Err(Error::from(ReqwestError::new()));
    }

    // Deserialize the response from the chatgpt API
    let api_response_body = api_response.json::<serde_json::Value>().await
        .map_err(|e| Error::from(ReqwestError::from(e)))?;

    // prepare the response
    let resp = Response {
        req_id: event.context.request_id,
        msg: format!("Command {} executed. Response from chatgpt API: {:?}", command, api_response_body),
    };

    // return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use crate::{my_handler, Request, ChatGPTConfig};
    use lambda_runtime::{Context, LambdaEvent};

    #[tokio::test]
    async fn response_is_good_for_simple_input() {
        let id = "ID";

        let mut context = Context::default();
        context.request_id = id.to_string();

        let payload = Request {
            command: "X".to_string(),
        };
        let event = LambdaEvent { payload, context };

        let config = ChatGPTConfig {
            model: "gpt-3.5-turbo".to_string(),
            temperature: 0.7,
            max_tokens: 150,
        };

        let result = my_handler(event, config).await.unwrap();

        assert_eq!(result.msg, "Command X executed.");
        assert_eq!(result.req_id, id.to_string());
    }
}