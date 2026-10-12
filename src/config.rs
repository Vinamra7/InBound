use serde::Deserialize;
use worker::Env;

pub const CLASSIFIER_MODEL: &str = "openai/gpt-6-luna";

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct AssistantConfig {
    pub allowed_models: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub session: SessionConfig,
    pub chat: ChatConfig,
    pub assistant: AssistantConfig,
}

#[derive(Debug, Clone)]
pub struct SessionConfig {
    pub turnstile_secret: String,
    pub hmac_key: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct ChatConfig {
    pub hmac_key: Vec<u8>,
    pub llm_api_key: String,
}

#[derive(Debug, PartialEq)]
pub enum ConfigError {
    MissingSecret(&'static str),
    WeakHmacKey,
}

// ---------- functions ----------

pub fn load_default() -> AssistantConfig {
    AssistantConfig {
        allowed_models: vec![
            "meta/muse-spark-1.3-contributor".to_string(),
            "openai/gpt-6-luna".to_string(),
            "z-ai/glm-5.3-flash".to_string(),
        ],
    }
}

pub fn is_allowed_model(model: &str, allowed: &[String]) -> bool {
    allowed.iter().any(|m| m == model)
}

// ---------- impls ----------

impl Config {
    pub fn from_env(env: &Env) -> Result<Self, ConfigError> {
        let read = |name: &'static str| -> Result<String, ConfigError> {
            env.secret(name)
                .map_err(|_| ConfigError::MissingSecret(name))
                .map(|s| s.to_string())
        };

        let hmac_key = read("HMAC_KEY")?.into_bytes();
        if hmac_key.len() < 32 {
            return Err(ConfigError::WeakHmacKey);
        }

        Ok(Self {
            session: SessionConfig {
                turnstile_secret: read("TURNSTILE_SECRET")?,
                hmac_key: hmac_key.clone(),
            },
            chat: ChatConfig {
                hmac_key,
                llm_api_key: read("LLM_API_KEY")?,
            },
            assistant: load_default(),
        })
    }
}
