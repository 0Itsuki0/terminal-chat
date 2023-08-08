use reqwest::{Client, Response, header::HeaderMap, Url};
use reqwest::header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde_json::Value;

use std::{io::{Write, stdout, Stdout}, error::Error};
use core::result::Result;
use std::env;


pub struct OpenAIClient {
    client: Client,
    headers: HeaderMap,
    endpoint_url: String,
    request_body: OpenAIRequestBody,
}


impl OpenAIClient {
    pub fn new() -> Result<OpenAIClient, Box<dyn Error>> {
        return Ok(OpenAIClient{
            client:Client::new(),
            headers: OpenAIClient::initialize_header()?,
            endpoint_url: OpenAIClient:: initialize_url(),
            request_body: OpenAIClient::initialize_base_data(),
        })
    }

    pub async fn send_message(&mut self, text: &str) -> Result<String, Box<dyn Error>> {
        self.request_body.messages.push(Message{
            role: ROLE::value(&ROLE::User),
            content: String::from(text),
        });
        // println!("{:?}",  self.request_body.messages);
        // Serialize it to a JSON string.
        let request_body_string = serde_json::to_string(&self.request_body)?;
        let url = Url::parse(&format!("{}", self.endpoint_url))?;

        let res = self.client
                                .post(url)
                                .headers(self.headers.clone())
                                .body(request_body_string)
                                .send().await?;
        
        // println!("Status: {}", res.status());
        // println!("Headers:\n{:#?}", res.headers());
        let body = res.text().await?;
        // println!("Body:\n{}", body);
        let response_message = OpenAIClient::parse_response_body_string(&body)?;
        
        self.request_body.messages.push(Message{
            role: ROLE::value(&ROLE::Assistant),
            content: response_message.clone(),
        });

        return Ok(response_message);


    }

    fn parse_response_body_string(s: &str) -> Result<String, Box<dyn Error>> {
        let v: Value = serde_json::from_str(s)?;
        println!("{}", v);
        let mut content: String = String::from(v["choices"][0]["message"]["content"].as_str().unwrap());
        content = content.replace("\n", "\n\r");
        return Ok(content);   
    }


    fn initialize_base_data() -> OpenAIRequestBody {
        let system_message = Message {
            role: ROLE::value(&ROLE::System),
            content: String::from("You are a helpful assistant."),
        };
        let base_data = OpenAIRequestBody {
            model: String::from("gpt-3.5-turbo"),
            messages: vec![system_message],
        };
        return base_data;
    }

    fn initialize_url() -> String {
        return String::from("https://api.openai.com/v1/chat/completions");
    }

    fn initialize_header() -> Result<HeaderMap, Box<dyn Error>> {
        let openai_api_key = "OPENAI_API_KEY";
        let key = env::var(openai_api_key)?;

        let mut headers: HeaderMap = HeaderMap::new();
        let header_string = format!("Bearer {}", key).parse::<String>()?;
        let header_value = HeaderValue::from_str(&header_string)?;
        headers.insert(AUTHORIZATION, header_value);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        return Ok(headers);
    }
}


use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
struct OpenAIRequestBody {
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    role: String,
    content: String,
}




#[derive(Serialize, Deserialize, Debug)]
enum ROLE {
    User,
    Assistant, 
    System
}
impl ROLE {
    fn value(&self) -> String {
        match *self {
            ROLE::User => String::from("user"),
            ROLE::Assistant => String::from("assistant"),
            ROLE::System => String::from("system"),
        }
    }
}
