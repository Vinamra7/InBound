# PROGRESS

## Done
- Toolchain: rustc 1.99, cargo 1.99, wrangler 4.147, worker-build 0.8
- Old pure gates: `is_allowed_model`, `remaining_quota`, `count_words`, `validate_message` (250/4096)
- `src/config.rs`: Deserialize+validate (IDs unverified)
- `GET /health` live 200 local; lib-only
- `src/security.rs` pure Turnstile + 4 tests
- Cloudflare: account, `inscope` shell, `TURNSTILE_SECRET` stored; widget hosts vercel app + localhost
- Task 12 done: `utf16_len` via `encode_utf16().count()` + `MAX_USER_WORDS=80`, `MAX_USER_UTF16=500`, `MAX_USER_BYTES=2048`; `tests/limits.rs::test_utf16_len` covers hello/emoji/empty

## Mismatches vs updated skill (remaining)
- `validate_message` still old bounds; need new per-user-message validator using 80/500/2048
- No history array validation yet (9 pairs max, alternating, role/order, per-entry + total budget)
- Config too narrow; Turnstile prod-localhost handling; missing api/state/providers/chat; DO verify pending

## In progress
- Task 13: new-bounds user-message validator

## Blockers
- typesafe.ai API unverified; OpenRouter IDs unverified; no deploy yet

## DoD 1-11
0/11. Local health + pure tests only.
