use crate::models::general::llm::{Message, ChatCompletion, APIResponse};
use dotenv::dotenv;
use reqwest::Client;
use std::{env, str::FromStr};

use reqwest::header::{HeaderMap, HeaderValue};

//Call Large Language Model (e.g GPT4)
pub async fn call_gpt(messages: Vec<Message>)->Result<String,Box<dyn std::error::Error + Send>>{
dotenv().ok();

//Extract API Key Information
let api_key:String = env::var("OPEN_AI_KEY").expect("OPEN_AI_KEY IS NOT FOUND IN ENVIRONMENT VARIABLE");
let api_org:String = env::var("OPEN_AI_ORG").expect("OPEN_AI_ORG IS NOT FOUND IN ENVIRONMENT VARIABLE");

//Confirm Endpoint
let url: &str ="https://api.openai.com/v1/chat/completions";

//Create Headers
let mut headers = HeaderMap::new();

//Create api key header
headers.insert(
    "authorization", 
    HeaderValue::from_str(&format!("Bearer {}",api_key))
    .map_err(|e|->Box<dyn std::error::Error + Send>{Box::new(e)})?
);


//Create org  header
headers.insert(
    "OpenAI-Organization", 
    HeaderValue::from_str(api_org.as_str())
    .map_err(|e|->Box<dyn std::error::Error + Send>{Box::new(e)})?
);

//Create Client
let client = Client::builder()
    .default_headers(headers)
    .build()
    .map_err(|e|->Box<dyn std::error::Error + Send>{Box::new(e)})?;

//Create Chat Completion

let chat_completion: ChatCompletion = ChatCompletion{
    model: "gpt-4".to_string(),
     messages,
     temperature:0.1
};

//Troubleshooting
// let res_raw = client
// .post(url)
// .json(&chat_completion)
// .send()
// .await
// .unwrap();

// dbg!(res_raw.text().await.unwrap());

//Get API Response
let res: APIResponse = client
    .post(url)
    .json(&chat_completion)
    .send()
    .await
    .map_err(|e|->Box<dyn std::error::Error + Send>{Box::new(e)})?
    .json()
    .await
    .map_err(|e|->Box<dyn std::error::Error + Send>{Box::new(e)})?;

Ok(res.choices[0].message.content.clone())
}

#[cfg(test)]
mod tests{
    use super::*;
    #[tokio::test]
    async fn tests_call_to_openai(){
        let message: Message = Message{
            role:"user".to_string(),
            content:"Hi there, give me a short response.".to_string()


        };

        let messages = vec!(message);
        let res = call_gpt(messages).await;
        // if let Ok(res_str)= res{
        //     dbg!(res_str);
        //     assert!(true)
        // }
        // else {
        //     assert!(false)
        // }

        match res {
            Ok(res_str) =>{
                dbg!(res_str);
                assert!(true)
            },
            Err(_)=>{
                assert!(false)
            }

            
        }
    }
}