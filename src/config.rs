use worker::Env;

pub const USER_QUOTA_NORMAL: u32 = 10;
pub const USER_QUOTA_RESTRICTED: u32 = 2;
pub const GLOBAL_QUOTA: u32 = 500;

#[derive(Debug, Clone)]
pub struct Config {
    pub session: SessionConfig,
    pub chat: ChatConfig,
}

#[derive(Debug, Clone)]
pub struct SessionConfig {
    pub turnstile_secret: String,
    pub hmac_key: Vec<u8>,
    pub allow_localhost: bool,
}

#[derive(Debug, Clone)]
pub struct ChatConfig {
    pub hmac_key: Vec<u8>,
    pub llm_api_key: String,
    pub models: [ModelConfig; 3],
    pub classifier_model: String,
}

#[derive(Debug, Clone)]
pub struct ModelConfig {
    pub model: String,
    pub api: ChatApi,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChatApi {
    ChatCompletions,
    Responses,
}

impl ChatApi {
    pub fn parse(value: &str) -> Result<Self, ConfigError> {
        match value {
            "chat_completions" => Ok(Self::ChatCompletions),
            "responses" => Ok(Self::Responses),
            _ => Err(ConfigError::InvalidVariable),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ConfigError {
    MissingSecret(&'static str),
    WeakHmacKey,
    InvalidVariable,
}

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

        let variable = |name: &str| -> Result<String, ConfigError> {
            let value = env
                .var(name)
                .map_err(|_| ConfigError::InvalidVariable)?
                .to_string();
            if value.trim().is_empty() || value.len() > 200 {
                return Err(ConfigError::InvalidVariable);
            }
            Ok(value)
        };
        let model = |model_var: &str, api_var: &str| -> Result<ModelConfig, ConfigError> {
            Ok(ModelConfig {
                model: variable(model_var)?,
                api: ChatApi::parse(&variable(api_var)?)?,
            })
        };

        Ok(Self {
            session: SessionConfig {
                turnstile_secret: read("TURNSTILE_SECRET")?,
                hmac_key: hmac_key.clone(),
                allow_localhost: allow_localhost(env),
            },
            chat: ChatConfig {
                hmac_key,
                llm_api_key: read("LLM_API_KEY")?,
                models: [
                    model("LLM_1_MODEL", "LLM_1_API")?,
                    model("LLM_2_MODEL", "LLM_2_API")?,
                    model("LLM_3_MODEL", "LLM_3_API")?,
                ],
                classifier_model: variable("CLASSIFIER_MODEL")?,
            },
        })
    }
}

pub fn allow_localhost(env: &Env) -> bool {
    env.var("ALLOW_LOCALHOST")
        .map(|value| value.to_string() == "true")
        .unwrap_or(false)
}
