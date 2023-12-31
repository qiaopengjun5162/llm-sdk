use crate::IntoRequest;
use derive_builder::Builder;
use reqwest::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Builder)]
pub struct ChatCompletionRequest {
    /// A list of messages comprising the conversation so far.
    #[builder(setter(into))]
    messages: Vec<ChatCompletionMessage>,
    /// ID of the model to use. See the model endpoint compatibility table for details on which models work with the Chat API.
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<ChatCompleteModel>,
    /// Number between -2.0 and 2.0. Positive values penalize new tokens based on their existing frequency in the text so far,
    /// decreasing the model's likelihood to repeat the same line verbatim.
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    frequency_penalty: Option<f32>,

    /// Modify the likelihood of specified tokens appearing in the completion.
    /// Accepts a JSON object that maps tokens (specified by their token ID in the tokenizer) to an associated bias value from -100 to 100.
    /// Mathematically, the bias is added to the logits generated by the model prior to sampling. The exact effect will vary per model,
    /// but values between -1 and 1 should decrease or increase likelihood of selection; values like -100 or 100 should result in a ban or exclusive selection of the relevant token.
    // #[builder(default, setter(strip_option))]
    // #[serde(skip_serializing_if = "Option::is_none")]
    // logit_bias: Option<HashMap<String, f32>>,

    /// The maximum number of tokens to generate in the chat completion.
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<usize>,
    /// How many chat completion choices to generate for each input message.
    /// Note that you will be charged based on the number of generated tokens across all of the choices. Keep n as 1 to minimize costs.
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    n: Option<usize>,
    /// Number between -2.0 and 2.0. Positive values penalize new tokens based on whether they appear in the text so far,
    /// increasing the model's likelihood to talk about new topics.
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    presence_penalty: Option<f32>,
    /// An object specifying the format that the model must output.
    /// Setting to { "type": "json_object" } enables JSON mode, which guarantees the message the model generates is valid JSON.
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    response_format: Option<ChatResponseFormatObject>,
    /// This feature is in Beta. If specified, our system will make a best effort to sample deterministically,
    /// such that repeated requests with the same seed and parameters should return the same result.
    /// Determinism is not guaranteed, and you should refer to the system_fingerprint response parameter to monitor changes in the backend.
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    seed: Option<String>,
    /// Up to 4 sequences where the API will stop generating further tokens.
    // TODO: make this as an enum
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    stop: Option<String>,
    /// If set, partial message deltas will be sent, like in ChatGPT.
    /// Tokens will be sent as data-only server-sent events as they become available, with the stream terminated by a data: [DONE] message.
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
    /// What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random,
    /// while lower values like 0.2 will make it more focused and deterministic.
    /// We generally recommend altering this or top_p but not both.
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    /// An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass.
    /// So 0.1 means only the tokens comprising the top 10% probability mass are considered.
    /// We generally recommend altering this or temperature but not both.
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    /// A list of tools the model may call. Currently, only functions are supported as a tool.
    /// Use this to provide a list of functions the model may generate JSON inputs for.
    #[builder(default, setter(into))]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tools: Vec<Tool>,
    /// Controls which (if any) function is called by the model.
    /// none means the model will not call a function and instead generates a message.
    /// auto means the model can pick between generating a message or calling a function.
    /// Specifying a particular function via {"type: "function", "function": {"name": "my_function"}} forces the model to call that function.
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_choice: Option<ToolChoice>,
    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse.
    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
// #[serde(rename_all = "snake_case", tag = "type", content = "function")]
pub enum ToolChoice {
    #[default]
    None,
    Auto,
    // TODO: we need something like this: #[serde(tag = "type", content = "function")]
    Function {
        name: String,
    },
}

#[derive(Debug, Clone, Serialize)]
pub struct Tool {
    /// The type of the tool. Currently, only function is supported.
    r#type: ToolType,
    function: FunctionInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionInfo {
    /// A description of what the function does, used by the model to choose when and how to call the function.
    description: Option<String>,
    /// The name of the function to be called. Must be a-z, A-Z, 0-9, or contain underscores and dashes, with a maximum length of 64.
    name: String,
    /// The parameters the functions accepts, described as a JSON Schema object. See the guide for examples, and the JSON Schema reference for documentation about the format.
    /// To describe a function that accepts no parameters, provide the value {"type": "object", "properties": {}}.
    parameters: serde_json::Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChatResponseFormatObject {
    r#type: ChatResponseFormat,
}

#[derive(Debug, Clone, Serialize, Default, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ChatResponseFormat {
    Text,
    #[default]
    Json,
}

// https://serde.rs/enum-representations.html
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case", tag = "role")]
pub enum ChatCompletionMessage {
    /// A message from a system.
    System(SystemMessage),
    /// A message from a user
    User(UserMessage),
    /// A message from a assistant
    Assistant(AssistantMessage),
    /// A message from a tool
    Tool(ToolMessage),
}

#[derive(Debug, Clone, Serialize, Copy, Default, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ChatCompleteModel {
    #[default]
    #[serde(rename = "gpt-3.5-turbo-1106")]
    Gpt3Turbo,
    #[serde(rename = "gpt-3.5-turbo-instruct")]
    Gpt3TurboInstruct,
    #[serde(rename = "gpt-4-1106-preview")]
    Gpt4Turbo,
    #[serde(rename = "gpt-4-vision-preview")]
    Gpt4TurboVision,
}

#[derive(Debug, Clone, Serialize)]
pub struct SystemMessage {
    /// The contents of the system message.
    content: String,
    /// An optional name for the participant. Provides the model information to differentiate between participants of the same role.
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserMessage {
    /// The contents of the system message.
    content: String,
    /// An optional name for the participant. Provides the model information to differentiate between participants of the same role.
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantMessage {
    /// The contents of the system message.
    content: String,
    /// An optional name for the participant. Provides the model information to differentiate between participants of the same role.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    name: Option<String>,
    /// The tool calls generated by the model, such as function calls.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tool_calls: Vec<ToolCall>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    /// The ID of the tool call.
    id: String,
    /// The type of the tool. Currently, only function is supported.
    r#type: ToolType,
    /// The function that the model called.
    function: FunctionCall,
}

#[derive(Debug, Clone, Serialize, Default, PartialEq, Eq, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolType {
    #[default]
    Function,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
    /// The name of the function to call.
    name: String,
    /// The arguments to call the function with, as generated by the model in JSON format.
    /// Note that the model does not always generate valid JSON,
    /// and may hallucinate parameters not defined by your function schema.
    /// Validate the arguments in your code before calling your function.
    arguments: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ToolMessage {
    /// The contents of the system message.
    content: String,
    /// Tool call that this message is responding to.
    tool_call_id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChatCompletionResponse {
    /// A unique identifier for the chat completion.
    pub id: String,
    /// A list of chat completion choices. Can be more than one if n is greater than 1.
    pub choices: Vec<ChatCompletionChoice>,
    /// The Unix timestamp (in seconds) of when the chat completion was created.
    pub created: usize,
    /// The model used for the chat completion.
    pub model: String,
    /// This fingerprint represents the backend configuration that the model runs with.
    /// Can be used in conjunction with the seed request parameter to understand when backend changes have been made that might impact determinism.
    pub system_fingerprint: String,
    /// The object type, which is always chat.completion.
    pub object: String,
    /// Usage statistics for the completion request.
    pub usage: ChatCompleteUsage,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChatCompleteUsage {
    /// Number of tokens in the generated completion.
    pub completion_tokens: usize,
    /// Number of tokens in the prompt.
    pub prompt_tokens: usize,
    /// Total number of tokens used in the request (prompt + completion).
    pub total_tokens: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChatCompletionChoice {
    /// The reason the model stopped generating tokens.
    /// This will be stop if the model hit a natural stop point or a provided stop sequence,
    /// length if the maximum number of tokens specified in the request was reached,
    /// content_filter if content was omitted due to a flag from our content filters,
    /// tool_calls if the model called a tool, or function_call (deprecated) if the model called a function.
    pub finish_reason: FinishReason,
    /// The index of the choice in the list of choices.
    pub index: usize,
    /// A chat completion message generated by the model.
    pub message: AssistantMessage,
}

#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq, Copy)]
#[serde(rename_all = "snake_case")]
pub enum FinishReason {
    #[default]
    Stop,
    Length,
    ContentFilter,
    ToolCalls,
}

// https://platform.openai.com/docs/api-reference/chat/create
impl IntoRequest for ChatCompletionRequest {
    fn into_request(self, client: Client) -> RequestBuilder {
        client
            .post("https://api.openai.com/v1/chat/completions")
            .json(&self)
    }
}

impl ChatCompletionMessage {
    pub fn new_system(content: impl Into<String>, name: &str) -> ChatCompletionMessage {
        ChatCompletionMessage::System(SystemMessage {
            content: content.into(),
            name: Self::get_name(name),
        })
    }

    pub fn new_user(content: impl Into<String>, name: &str) -> ChatCompletionMessage {
        ChatCompletionMessage::User(UserMessage {
            content: content.into(),
            name: Self::get_name(name),
        })
    }

    fn get_name(name: &str) -> Option<String> {
        if name.is_empty() {
            None
        } else {
            Some(name.into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::LlmSdk;
    use anyhow::Result;

    #[test]
    #[ignore]
    fn chat_completion_request_tool_choice_function_serialize_should_work() {
        let req = ChatCompletionRequestBuilder::default()
            .tool_choice(ToolChoice::Function {
                // r#type: ToolType::Function,
                name: "my_function".to_string(),
            })
            .messages(vec![])
            .build()
            .unwrap();
        let json = serde_json::to_value(req).unwrap();
        assert_eq!(
            json,
            serde_json::json!({
                "messages": [],
                "tool_choice": {
                    "type": "function",
                    "function": {
                        "name": "my_function"
                    }
                }
            })
        )
    }

    #[test]
    fn chat_completion_request_tool_choice_auto_serialize_should_work() {
        let req = ChatCompletionRequestBuilder::default()
            .tool_choice(ToolChoice::Auto)
            .messages(vec![])
            .build()
            .unwrap();
        let json = serde_json::to_value(req).unwrap();
        assert_eq!(
            json,
            serde_json::json!({
                "messages": [],
                "tool_choice": "auto"
            })
        )
    }

    #[test]
    fn chat_completion_request_serialize_should_work() {
        // let messages = vec![
        //     ChatCompletionMessage::new_system("I can answer any question you ask me.", ""),
        //     ChatCompletionMessage::new_user("What is human life expectancy in the world?", "user1"),
        // ];
        // let req = ChatCompletionRequestBuilder::default()
        //     .tool_choice(ToolChoice::Auto)
        //     .messages(messages)
        //     .build()
        //     .unwrap();

        let req = get_simple_completion_request();
        let json = serde_json::to_value(req).unwrap();
        assert_eq!(
            json,
            serde_json::json!({
                "messages": [{
                    "role": "system",
                    "content": "I can answer any question you ask me.",
                },
                {
                    "role": "user",
                    "content": "What is human life expectancy in the world?",
                    "name": "user1"
                }],
                "tool_choice": "auto"
            })
        )
    }

    #[tokio::test]
    async fn simple_chat_completion_should_work() -> Result<()> {
        let sdk = LlmSdk::new(std::env::var("OPENAI_API_KEY")?);
        let req = get_simple_completion_request();

        let res = sdk.chat_completion(req).await?;
        assert_eq!(res.choices.len(), 1);
        println!("res: {:#?}", res);
        Ok(())
    }

    fn get_simple_completion_request() -> ChatCompletionRequest {
        let messages = vec![
            ChatCompletionMessage::new_system("I can answer any question you ask me.", ""),
            ChatCompletionMessage::new_user("What is human life expectancy in the world?", "user1"),
        ];
        ChatCompletionRequestBuilder::default()
            .tool_choice(ToolChoice::Auto)
            .messages(messages)
            .build()
            .unwrap()
    }
}
