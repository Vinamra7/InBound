pub const SITEVERIFY_URL: &str = "https://challenges.cloudflare.com/turnstile/v0/siteverify";
pub const TURNSTILE_ACTION: &str = "session";

// replace the vercel.app entry with the real project slug after the first Vercel deploy
pub const ALLOWED_HOSTS: &[&str] = &["vinamramishra.dev", "www.vinamramishra.dev"];

pub const ALLOWED_ORIGINS: &[&str] =
    &["https://vinamramishra.dev", "https://www.vinamramishra.dev"];

pub const OPENROUTER_URL: &str = "https://openrouter.ai/api/v1/chat/completions";

// verified against the OpenRouter catalog 2026-10-18
pub const CLASSIFIER_MODEL: &str = "openai/gpt-6-luna";
pub const CHAT_MODEL: &str = "z-ai/glm-5.3-flash";

pub const CLASSIFIER_MAX_TOKENS: u32 = 8;
pub const REPLY_MAX_TOKENS: u32 = 400;

pub const SYSTEM_PROMPT: &str = "You are InScope, the assistant embedded in Vinamra Mishra's portfolio site. \
Talk only about Vinamra: his work, projects, skills, experience, and how to contact him. \
Be brief and friendly — replies under 300 words. If asked about anything else, gently steer back to Vinamra.";

pub const CANNED_REPLY: &str = "Sorry — this chat only answers questions about Vinamra: his work, projects, skills, and experience.";
