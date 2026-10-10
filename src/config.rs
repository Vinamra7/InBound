use crate::domain::message::{MAX_USER_BYTES, MAX_USER_WORDS};
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct AssistantConfig {
    pub allowed_models: Vec<String>,
    pub max_words: usize,
    pub max_bytes: usize,
}

impl AssistantConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.allowed_models.is_empty() {
            return Err("allowed_models must not be empty".to_string());
        }
        if self.max_words == 0 || self.max_bytes == 0 {
            return Err("limits must be greater than zero".to_string());
        }
        Ok(())
    }
}

pub fn load_default() -> AssistantConfig {
    AssistantConfig {
        allowed_models: vec![
            "~typesafe/jev-latest".to_string(),
            "meta/muse-spark-1.3-contributor".to_string(),
            "openai/gpt-6-luna".to_string(),
            "z-ai/glm-5.3-flash".to_string(),
        ],
        max_words: MAX_USER_WORDS,
        max_bytes: MAX_USER_BYTES,
    }
}

pub fn is_allowed_model(model: &str, allowed: &[String]) -> bool {
    allowed.iter().any(|m| m == model)
}
