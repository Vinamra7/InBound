# PROGRESS

## Done
- Toolchain: rustc 1.99, cargo 1.99, wrangler 4.147, worker-build 0.8
- Pure gates in `src/lib.rs`: `is_allowed_model`, `remaining_quota`, `count_words`, `validate_message` + `MAX_WORDS=250`, `MAX_MESSAGE_BYTES=4096`, `MessageError`
- Tests green (user-pasted): `allow_models` 1 passed, `limits` 2 passed, `quota` 1 passed; doc-tests 0
- Skill saved: `.agents/skills/inscope-mentor/SKILL.md`
- Deps approved for Workers: `worker 0.8.7`, `serde 1.0.229`, `serde_json 1.0.151`; `[lib] cdylib+rlib`
- `GET /health` live on `wrangler dev`: `GET /health 200 OK`, browser `{"ok":true}`, `GET /favicon.ico 404`; `wrangler.toml main="build/index.js"` verified via `index.js 19.8kb` output; `src/main.rs` removed (lib-only correct)

## Deferred (non-blocking)
- `allow_models` missing empty-list + case asserts (`&[]`, `"GPT-4"` vs `"gpt-4"`)
- `tests/limits.rs:5-6` duplicates `MAX_*` consts instead of `use inscope::{MAX_WORDS, MAX_MESSAGE_BYTES}`
- No pasted `cargo fmt --check` proof yet

## In progress
- Task 8: `src/config.rs` (typed owner config, no secrets) + `tests/config.rs`, keep `/health` 200

## Next
- `security.rs` Turnstile verify via `worker::Fetch` (real HTTP, secrets via `Env`, no mocks in prod)
- Verify workers-rs DO + `wrangler.toml` migration syntax before `state.rs`
- `api.rs` routing/CORS/error codes, then DO coordinator, relevance provider, OpenRouter

## Release blockers / uncertainties
- workers-rs DO support + migration syntax not yet verified against current docs
- Relevance provider https://docs.typesafe.ai/primitives API/auth/format unverified
- OpenRouter model IDs + token accounting unverified
- Turnstile hostname/action + expiry/single-use to verify
- No deployed smoke test yet

## DoD 1-11
0/11 met. First runtime proof obtained (`/health` local + 4 tests green), no deploy yet.
