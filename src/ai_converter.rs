//! AI转换模块

use anyhow::{Context, Result};
use base64::{engine::general_purpose, Engine};
use futures::future;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs;
use tokio::task;

/// AI转换类
pub struct AIConverter {
    api_key: String,
    model: String,
    client: Client,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatMessage {
    role: String,
    content: Vec<ChatContent>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum ChatContent {
    Text { text: String },
    ImageUrl { image_url: ImageUrl },
}

#[derive(Serialize, Deserialize, Debug)]
struct ImageUrl {
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    max_tokens: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Choice {
    message: Message,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    content: String,
}

impl AIConverter {
    /// 创建新的AI转换器
    pub fn new(api_key: &str, model: &str) -> Self {
        AIConverter {
            api_key: api_key.to_string(),
            model: model.to_string(),
            client: Client::new(),
        }
    }

    /// 将图片转换为Markdown
    pub async fn image_to_markdown(&self, image_path: &str) -> Result<String> {
        // 读取图片并转换为base64
        let image_data = fs::read(image_path).context(format!("无法读取图片: {}", image_path))?;
        let base64_image = general_purpose::STANDARD.encode(&image_data);

        // 构建请求
        let request = ChatRequest {
            model: self.model.clone(),
            messages: vec![ChatMessage {
                role: "user".to_string(),
                content: vec![
                    ChatContent::Text {
                        text: "将图片转换为 Markdown".to_string(),
                    },
                    ChatContent::ImageUrl {
                        image_url: ImageUrl {
                            url: format!("data:image/jpeg;base64,{}", base64_image),
                        },
                    },
                ],
            }],
            max_tokens: 4096,
        };

        // 发送请求
        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("无法连接到OpenAI API")?;

        // 处理响应
        let response_status = response.status();
        if !response_status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "无法获取错误信息".to_string());
            return Err(anyhow::anyhow!(
                "API请求失败: {} - {}",
                response_status,
                error_text
            ));
        }

        let chat_response: ChatResponse = response.json().await.context("无法解析API响应")?;

        if chat_response.choices.is_empty() {
            return Err(anyhow::anyhow!("API返回空结果"));
        }

        Ok(chat_response.choices[0].message.content.trim().to_string())
    }

    /// 并发转换多个图片
    pub async fn convert_images(
        &self,
        images: Vec<String>,
        _workers: usize,
    ) -> Result<Vec<String>> {
        // 创建任务
        let tasks: Vec<_> = images
            .iter()
            .map(|img| {
                let img = img.clone();
                let converter = self.clone();
                task::spawn(async move {
                    match converter.image_to_markdown(&img).await {
                        Ok(markdown) => markdown,
                        Err(e) => {
                            log::error!("图片转换失败 {}: {}", img, e);
                            String::new()
                        }
                    }
                })
            })
            .collect();

        // 等待所有任务完成
        let results = future::join_all(tasks).await;

        // 处理结果
        let markdown_parts: Vec<String> = results
            .into_iter()
            .filter_map(|r| r.ok())
            .filter(|s| !s.is_empty())
            .collect();

        Ok(markdown_parts)
    }
}

impl Clone for AIConverter {
    fn clone(&self) -> Self {
        AIConverter {
            api_key: self.api_key.clone(),
            model: self.model.clone(),
            client: Client::new(),
        }
    }
}
