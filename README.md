# InScope

Rate-limited, in-scope LLM chat for a static portfolio — no login.

A Rust Cloudflare Worker that proxies an LLM chat for [vinamramishra.dev](https://vinamramishra.dev).
**InScope** = every message is gated by a relevance classifier (Typesafe Jev AI via OpenRouter)
before the real LLM call, behind Turnstile-verified sessions and atomic D1 daily quotas.

## Architecture

```
Browser (Vercel, static site)
   │  POST /api/session {turnstile, fingerprint}     → Turnstile ✓ + fp check → HMAC token (24h)
   │  POST /api/chat   {messages} + Authorization: Bearer
   ▼
Cloudflare Worker (this repo)
   1. edge facts (IP, country, ASN, tz, lang, protocol)
   2. risk gate            → Tor blocked (403), datacenter ASN → restricted quota tier
   3. Turnstile verify     → 403 on failure
   4. fingerprint strikes  → ≥2 strikes → restricted quota tier
   5. D1 counters (atomic) → per-device 10/day (restricted: 2), global 500/day → 429/503
   6. classifier           → off-topic → canned reply (counts toward quota)
   7. OpenRouter chat      → {reply, usage}
```

Quotas are counted **before** the classifier and LLM run — attempts, not successes. One
atomic D1 batch increments the user row and the global row; the global bump only happens
if the user bump succeeded (`changes() = 1`).

Identity is `device_id` (FingerprintJS visitor id). Client-claimed and therefore rotatable —
accepted trade-off, bounded by Turnstile + the session rate limiter + the global cap.

## Threat model

| Attack | What stops it |
|---|---|
| curl/Postman replay without browser | Turnstile (browser-only widget) |
| Turnstile token replay | siteverify + action + hostname check; token single-use at Cloudflare |
| Session-minting flood | Workers Rate Limiting binding per IP on `/api/session` |
| One device burning all global slots | per-device 10/day cap in the same D1 batch |
| Distributed flood (many devices) | global 500/day cap |
| Tor / anonymizers | blocked at the edge before any counter runs |
| Paid residential proxies + CAPTCHA solvers | get through — bounded by global cap + OpenRouter spend cap |
| Surprise bill | OpenRouter provider spend cap (set in dashboard) |

## Repo layout

```
src/
  lib.rs            #[event(fetch)], routing, CORS wrap, OPTIONS
  config.rs         secrets, quotas
  constants.rs      hosts, origins, models, prompts, caps
  error.rs          AppError → status/code/{"error": code}
  routes/           session.rs, chat.rs, health.rs, cors.rs
  pipeline/         validate.rs, auth.rs, risk.rs, limits.rs
  domain/           pure logic — message.rs, token.rs, fingerprint.rs, risk.rs, edge.rs
  adapters/         edge.rs, turnstile.rs, counter.rs, clock.rs, llm/openrouter.rs
tests/              native unit tests (cargo test)
scripts/smoke.sh    curl checks against wrangler dev
schema.sql          D1 tables
.dev.vars.example   local secrets template
```

## Local dev

```sh
cp .dev.vars.example .dev.vars   # fill in real values
wrangler d1 create inscope       # put the id into wrangler.toml
wrangler d1 execute inscope --local --file=schema.sql
cargo test
wrangler dev                     # then: sh scripts/smoke.sh
```

## Deploy

```sh
wrangler deploy
wrangler d1 execute inscope --remote --file=schema.sql
```

Then in the dashboards: Turnstile widget for both UI hostnames, custom domain
`api.vinamramishra.dev`, WAF rule for `/api/session`, OpenRouter spend cap.

## Testing

- Pure functions (domain, clock day-math, error mapping): `cargo test`
- Anything touching `worker` types (D1, Fetch, Request): `wrangler dev` + `scripts/smoke.sh`
