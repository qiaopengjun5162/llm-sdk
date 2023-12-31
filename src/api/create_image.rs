use derive_builder::Builder;
use reqwest::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};

use crate::IntoRequest;

#[derive(Debug, Clone, Serialize, Builder)]
#[builder(pattern = "mutable")]
pub struct CreateImageRequest {
    /// A text description of the desired image(s). The maximum length is 1000 characters for dall-e-2 and 4000 characters for dall-e-3.
    #[builder(setter(into))]
    prompt: String,
    /// The model to use for image generation.
    #[builder(default)]
    model: ImageModel,
    /// The number of images to generate. Must be between 1 and 10. For dall-e-3, only n=1 is supported.
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    n: Option<usize>,
    /// The quality of the image that will be generated.
    /// hd creates images with finer details and greater consistency across the image.
    /// This param is only supported for dall-e-3.
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    quality: Option<ImageQuality>,
    /// The format in which the generated images are returned. Must be one of url or b64_json.
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    response_format: Option<ImageResponseFormat>,
    /// The size of the generated images. Must be one of 256x256, 512x512, or 1024x1024 for dall-e-2.
    /// Must be one of 1024x1024, 1792x1024, or 1024x1792 for dall-e-3 models.
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<ImageSize>,
    /// The style of the generated images. Must be one of vivid or natural.
    /// Vivid causes the model to lean towards generating hyper-real and dramatic images.
    /// Natural causes the model to produce more natural, less hyper-real looking images. This param is only supported for dall-e-3.
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<ImageStyle>,
    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse.
    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<String>,
}

#[derive(Debug, Clone, Serialize, Copy, PartialEq, Eq, Default)]
pub enum ImageModel {
    #[serde(rename = "dall-e-3")]
    #[default]
    DallE3,
}

#[derive(Debug, Clone, Serialize, Copy, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum ImageQuality {
    #[serde(rename = "default")]
    #[default]
    Standard,
    #[serde(rename = "hd")]
    Hd,
}

#[derive(Debug, Clone, Serialize, Copy, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum ImageResponseFormat {
    #[default]
    Url,
    B64Json,
}

#[derive(Debug, Clone, Serialize, Copy, PartialEq, Eq, Default)]
pub enum ImageSize {
    #[serde(rename = "1024x1024")]
    #[default]
    Large,
    #[serde(rename = "1792x1024")]
    LargeWide,
    #[serde(rename = "1024x1792")]
    LargeTall,
}

#[derive(Debug, Clone, Serialize, Copy, PartialEq, Eq, Default)]
pub enum ImageStyle {
    #[serde(rename = "vivid")]
    #[default]
    Vivid,
    #[serde(rename = "natural")]
    Natural,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateImageResponse {
    pub created: u64,
    pub data: Vec<ImageObject>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ImageObject {
    /// The base64-encoded JSON of the generated image, if response_format is b64_json.
    pub b64_json: Option<String>,

    // The URL of the generated image, if response_format is url (default).
    pub url: Option<String>,

    // The prompt that was used to generate the image, if there was any revision to the prompt.
    pub revised_prompt: String,
}

// https://platform.openai.com/docs/api-reference/images/create
impl IntoRequest for CreateImageRequest {
    fn into_request(self, client: Client) -> RequestBuilder {
        client
            .post("https://api.openai.com/v1/images/generations")
            .json(&self)
    }
}

impl CreateImageRequest {
    pub fn new(prompt: impl Into<String>) -> Self {
        CreateImageRequestBuilder::default()
            .prompt(prompt)
            .build()
            .unwrap()
    }
}

// impl Default for ImageModel {
//     fn default() -> Self {
//         ImageModel::DallE3
//     }
// }

// impl Default for ImageQuality {
//     fn default() -> Self {
//         ImageQuality::Standard
//     }
// }

// impl Default for ImageSize {
//     fn default() -> Self {
//         ImageSize::Large
//     }
// }

// impl Default for ImageStyle {
//     fn default() -> Self {
//         ImageStyle::Vivid
//     }
// }

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::LlmSdk;

    use super::*;
    use anyhow::Result;
    use serde_json::json;

    #[test]
    fn create_image_request_shoud_serialize() -> Result<()> {
        let req = CreateImageRequest::new("hello world");

        assert_eq!(
            serde_json::to_value(&req)?,
            json!({
                "prompt": "hello world",
                "model": "dall-e-3",
            })
        );
        Ok(())
    }

    #[test]
    fn create_image_request_custom_shoud_serialize() -> Result<()> {
        let req = CreateImageRequestBuilder::default()
            .prompt("hello world")
            .quality(ImageQuality::Hd)
            .style(ImageStyle::Natural)
            .build()?;

        assert_eq!(
            serde_json::to_value(&req)?,
            json!({
                "prompt": "hello world",
                "model": "dall-e-3",
                "quality": "hd",
                "style": "natural",
            })
        );
        Ok(())
    }

    #[tokio::test]
    async fn create_image_should_work() -> Result<()> {
        println!("OPENAI_API_KEY1: {:#?}", std::env::var("OPENAI_API_KEY")?);
        println!("OPENAI_API_KEY2: {:?}", std::env::var("OPENAI_API_KEY")?);
        let sdk = LlmSdk::new(std::env::var("OPENAI_API_KEY")?);
        let req = CreateImageRequest::new("hello girl");
        let res = sdk.create_image(req).await?;
        assert_eq!(res.data.len(), 1);
        let image = &res.data[0];
        assert!(image.url.is_some());
        assert!(image.b64_json.is_none());
        println!("image: {:#?}", image);
        fs::write(
            "/tmp/llm-sdk/caterpillar.png",
            reqwest::get(image.url.as_ref().unwrap())
                .await?
                .bytes()
                .await?,
        )?;
        Ok(())
    }
}
