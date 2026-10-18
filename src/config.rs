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
        })
    }
}
