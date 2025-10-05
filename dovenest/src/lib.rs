use anyhow::Result;
use async_openai::types::responses::{CreateResponseArgs, Input, ResponseEvent};
use async_openai::{Client, config::OpenAIConfig};
use futures::StreamExt;

// ---------- Public, provider-neutral API ----------
pub mod api {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ChatMessage {
        pub role: Role,
        pub text: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Role {
        User,
        Assistant,
        System,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum ChatEvent {
        TextDelta(String),
        TextDone,
        Done,
        Error(String),
    }
}
// Re-export, so `use dovenest::{DoveNest, ChatEvent, ...}` works:
pub use api::{ChatEvent, ChatMessage, Role};

// ---------- DoveNest core ----------
pub struct DoveNest {
    client: Client<OpenAIConfig>,
}

impl DoveNest {
    pub fn new() -> Self {
        // Reads OPENAI_API_KEY from env automatically
        let client = Client::new();
        Self { client }
    }

    // Back-compat with earlier examples
    pub fn from_env() -> Self {
        Self::new()
    }

    /// One-shot call (no streaming)
    pub async fn chat_once(&self, model: &str, user_text: &str) -> Result<String> {
        let req = CreateResponseArgs::default()
            .model(model)
            .input(Input::Text(user_text.to_owned()))
            .build()?;

        let resp = self.client.responses().create(req).await?;
        Ok(resp.output_text.unwrap_or_default())
    }

    /// Streaming via callback: we stay provider-neutral for the CLI
    pub async fn chat_stream_to<F>(
        &self,
        model: &str,
        user_text: &str,
        mut on_chunk: F,
    ) -> Result<()>
    where
        F: FnMut(&str) -> Result<()>,
    {
        let req = CreateResponseArgs::default()
            .model(model)
            .input(Input::Text(user_text.to_owned()))
            .stream(true) // SSE streaming
            .build()?;

        let mut stream = self.client.responses().create_stream(req).await?;

        while let Some(evt) = stream.next().await {
            match evt? {
                ResponseEvent::ResponseOutputTextDelta(delta) => {
                    on_chunk(&delta.delta)?;
                }
                ResponseEvent::ResponseOutputTextDone(_) | ResponseEvent::ResponseCompleted(_) => {
                    break;
                }
                _ => {}
            }
        }
        Ok(())
    }
}

// Clippy: `new_without_default`
impl Default for DoveNest {
    fn default() -> Self {
        Self::new()
    }
}
