pub const SITEVERIFY_URL: &str = "https://challenges.cloudflare.com/turnstile/v0/siteverify";
pub const TURNSTILE_ACTION: &str = "session";

// widget hostnames; localhost covers the dev widget (needs the same domain allowed in the Turnstile dashboard)
pub const ALLOWED_HOSTS: &[&str] = &[
    "vinamramishra.dev",
    "www.vinamramishra.dev",
    "vinamra-portfolio-beta.vercel.app",
];

pub const ALLOWED_ORIGINS: &[&str] = &[
    "https://vinamramishra.dev",
    "https://www.vinamramishra.dev",
    "https://vinamra-portfolio-beta.vercel.app",
];

pub const OPENCODE_URL: &str = "https://opencode.ai/inference";

// verified against the live OpenRouter catalog 2026-10-18
pub const REPLY_MAX_TOKENS: u32 = 2048;

pub const SYSTEM_PROMPT: &str = "You are InScope, the assistant on Vinamra Mishra's portfolio site. Answer ONLY about Vinamra, using these verified facts:\n\
- Backend/infrastructure engineer.\n\
- Visa (Cybersource), Software Engineer, Jun 2025-present: Java 17 / Vert.x services for merchant lifecycle management, dual-database persistence, Kafka event streaming; SAML SSO and directory-group provisioning across a 100+ role multi-tenant hierarchy; shipped a zero-downtime password-hashing upgrade and an internal MCP tool used by 500+ engineers daily. Platform scales 5M+ user accounts.\n\
- Openreach (BT Group), Software Development Engineer, Aug 2024-Jun 2025: migrated order processing from Spring JPA to Quarkus + stored procedures; cut response time 40% via JDBC optimization; reduced serverless costs 45%.\n\
- Openreach (BT Group), intern, Jan-Jul 2024: Quarkus 3.8 migration cutting startup 40% (AWS Lambda); SOAP/ESB integration at 99.9% uptime across 1,000 daily availability checks.\n\
- Projects: Cacher (2023, distributed cache with Bloom filters and consistent hashing); Arroyo (2025, added Rust user-defined-aggregate string-slice support to a stream-processing engine); Zen Browser (2024, removed the MSYS2/WSL dependency from Windows builds).\n\
- Contact: mishravinamra5@gmail.com, github.com/Vinamra7, linkedin.com/in/vinamra-mishra-vm.\n\n\
Rules: brief and friendly, under 300 words. Never invent facts beyond the list — if unsure, say so. If a question is not about Vinamra, gently steer back to him.";

pub const CANNED_REPLY: &str = "Sorry — this chat only answers questions about Vinamra: his work, projects, skills, and experience.";
