# InScope

Rate-limited, in-scope LLM chat for a static portfolio — no login.

A Rust Cloudflare Worker that proxies an LLM chat for [vinamramishra.dev](https://vinamramishra.dev).
**InScope** = every message is gated by a relevance classifier (Typesafe Jev AI via OpenCode)
before the real LLM call, behind Turnstile-verified sessions and atomic D1 daily quotas.

## Architecture

```
Browser (Vercel, static site)
   │  POST /api/session {turnstile, fingerprint}     → Turnstile ✓ + fp check → HMAC token (24h)
   │  POST /api/chat   {model:"llm-1",messages} + Authorization: Bearer
   ▼
Cloudflare Worker (this repo)
   1. edge facts (IP, country, ASN, tz, lang, protocol)
   2. risk gate            → Tor blocked (403), datacenter ASN → restricted quota tier
   3. Turnstile verify     → 403 on failure
   4. fingerprint strikes  → ≥2 strikes → restricted quota tier
   5. D1 counters (atomic) → per-device 10/day (restricted: 2), global 500/day → 429/503
   6. classifier           → off-topic → canned reply (counts toward quota)
   7. OpenCode chat        → {reply, usage}
```

Quotas are counted **before** the classifier and LLM run — attempts, not successes. One
atomic D1 batch increments the user row and the global row; the global bump only happens
if the user bump succeeded (`changes() = 1`).

Identity is `device_id` (FingerprintJS visitor id). Client-claimed and therefore rotatable —
accepted trade-off, bounded by Turnstile + the session rate limiter + the global cap.

## Threat model

| Attack | What stops it |
|---|---|
| Minting a session without solving Turnstile | server-side siteverify, hostname and action checks |
| Replaying a valid bearer token from curl | permitted until expiry; same device/day quota, no fresh quota |
| Turnstile token replay | siteverify + action + hostname check; token single-use at Cloudflare |
| Session-minting flood | Workers Rate Limiting binding per IP on `/api/session` |
| One device burning all global slots | per-device 10/day cap in the same D1 batch |
| Distributed flood (many devices) | global 500/day cap |
| Tor exits identified by Cloudflare country T1 | blocked before any D1 counter runs; other VPNs are not universally blocked |
| Paid residential proxies + CAPTCHA solvers | get through — bounded by global admitted-attempt cap |
| Unexpected provider charges | configure/verify OpenCode account billing controls; free model IDs are not a guaranteed spend cap |

## OpenCode configuration

The browser sends a slot alias. The Worker accepts only `llm-1`, `llm-2`, `llm-3` and
resolves the real model from Cloudflare variables. Raw model names and missing/unknown
aliases return 400 before quota writes. The current UI always sends `llm-1`; `/model`
selection is deferred. Slots are selectable choices, not a fallback chain.

| Slot | Cloudflare model variable | Current model | API format variable |
|---|---|---|---|
| `llm-1` | `LLM_1_MODEL` | `glm-5.3-flash` (paid) | `LLM_1_API=chat_completions` |
| `llm-2` | `LLM_2_MODEL` | `glm-5.3-flash-free` | `LLM_2_API=chat_completions` |
| `llm-3` | `LLM_3_MODEL` | `longcat-2.5-preview-free` | `LLM_3_API=chat_completions` |
| private classifier | `CLASSIFIER_MODEL` | `jev-1.13-free` | native System One `noul` |

Set secret `LLM_API_KEY` to an **OpenCode Console service-account key**, not an OpenRouter
key. Subscription/credit and inference API access must be checked separately.

Edit model variables in Cloudflare **Settings → Variables and Secrets**, then publish
the changes. Set each slot's API format to `chat_completions` or `responses` to match the
chosen model. Base URL stays `https://opencode.ai/inference`; the Worker selects
`/openai/v1/chat/completions` or `/openai/v1/responses`. Output is capped at 2048 tokens;
streaming and automatic provider retries are off. Responses requests set `store=false`.

The classifier uses `/systemone/v1/systemone` with the latest message and validated conversation history, plus explicit yes/no criteria: a native `noul` probability, with ≥0.5
meaning in scope. Visitors cannot select it. Malformed answer fields retain the
previous fail-open policy; transport/HTTP/invalid-JSON failures return 502. A different
classifier ID must support this native schema.

Defaults are in `wrangler.toml`. Although `keep_vars=true`, keys explicitly present
in `[vars]` are reapplied by deployment. Sync dashboard changes into that file before
redeploying. The original free chat IDs returned upstream `403 FreeTierError`, so the
user approved paid `glm-5.3-flash` for `llm-1` ($0.15/million input tokens and
$0.50/million output, per current OpenCode docs). Slots 2/3 remain free and unavailable
in these checks. Verify successful production completion and account billing controls.

## Repo layout

```
src/
  lib.rs            #[event(fetch)], routing, CORS wrap, OPTIONS
  config.rs         secrets, quotas, model variables and API formats
  constants.rs      hosts, origins, provider base, prompts, caps
  error.rs          AppError → status/code/{"error": code}
  routes/           session.rs, chat.rs, health.rs, cors.rs
  pipeline/         validate.rs, auth.rs, risk.rs, limits.rs
  domain/           pure logic — message.rs, token.rs, fingerprint.rs, risk.rs, edge.rs
  adapters/         edge.rs, http.rs, turnstile.rs, counter.rs, clock.rs, llm/opencode.rs
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
cargo check --target wasm32-unknown-unknown
wrangler dev                     # then: sh scripts/smoke.sh
```

The example opts in to localhost for local CORS and Turnstile hostname checks.
Production defaults to `ALLOW_LOCALHOST=false`. Configure an appropriate widget/key
for real browser testing; the smoke script only exercises validation/auth/CORS paths.
The example contains an official test secret, which must not be deployed.

## Deploy

First deployment from PowerShell (Rust/Cargo and Node.js must be installed):

```powershell
Set-Location C:\Git_new\InBound
npx.cmd wrangler@4 login
npx.cmd wrangler@4 d1 create inscope
```

Copy the returned database UUID into `database_id` in `wrangler.toml`; keep binding
`DB` and database name `inscope`. Then initialize the database before deployment:

```powershell
npx.cmd wrangler@4 d1 execute inscope --remote --file=schema.sql
rustup target add wasm32-unknown-unknown
npx.cmd wrangler@4 deploy --dry-run
npx.cmd wrangler@4 deploy
```

Wrangler runs the existing Rust build command, uploads the Worker, and configures
`api.vinamramishra.dev` from the custom-domain entry. The zone must be active in the
same Cloudflare account. Record the printed `https://inscope.<subdomain>.workers.dev`
URL for the frontend fallback. The API will return configuration errors until its
production secrets are set; health is intentionally available without secrets.

In Cloudflare **Turnstile → Add widget**, create a Managed widget for these production
hostnames (no scheme/path): `vinamramishra.dev`, `www.vinamramishra.dev`,
`vinamra-portfolio-beta.vercel.app`. Pre-clearance is not needed. Keep the site key
for Vercel and the secret key for the Worker. If a widget already exists, verify its
hostnames and use that matching site-key/secret pair.

Set secrets through the CLI prompts, or under the Worker's **Settings → Variables
and Secrets** using the Secret type. Do not put values into Git or chat:

```sh
npx.cmd wrangler@4 secret put TURNSTILE_SECRET
npx.cmd wrangler@4 secret put HMAC_KEY
npx.cmd wrangler@4 secret put LLM_API_KEY
```

Set a random HMAC key of at least 32 bytes. Configure Turnstile for `vinamramishra.dev`,
`www.vinamramishra.dev`, and `vinamra-portfolio-beta.vercel.app`; action is `session`.
Verify the custom domain `api.vinamramishra.dev`, WAF rule for `/api/session`, deployed
D1/rate-limit bindings, and OpenCode account billing controls before opening the chat.

Frontend (`C:\vinamra_portfolio`) calls the Worker directly. Configure Vercel public
variables before building: `NEXT_PUBLIC_API_PRIMARY`, actual `NEXT_PUBLIC_API_FALLBACK`,
`NEXT_PUBLIC_API_LOCAL`, `NEXT_PUBLIC_TURNSTILE_SITE_KEY`, `NEXT_PUBLIC_INSCOPE_MOCK=0`.
Tokens use sessionStorage + Authorization; no cookies. Reset keeps the session and
daily lock. Short contact requests/buttons are local and remain usable after quota denial.

In Vercel **Project → Settings → Environment Variables**, set the following for
Production and, if needed, Preview; rebuild/redeploy after changing public variables:

| Variable | Value |
|---|---|
| `NEXT_PUBLIC_API_PRIMARY` | `https://api.vinamramishra.dev` |
| `NEXT_PUBLIC_API_FALLBACK` | actual `https://inscope.<subdomain>.workers.dev` URL |
| `NEXT_PUBLIC_TURNSTILE_SITE_KEY` | public key from the matching production widget |
| `NEXT_PUBLIC_INSCOPE_MOCK` | `0` |
| `NEXT_PUBLIC_API_LOCAL` | optional local-development URL, default `http://localhost:8787` |

Keep `vinamramishra.dev` and `www.vinamramishra.dev` assigned to this Vercel project.
In Cloudflare DNS, retain the apex/`www` records that Vercel's Domains screen specifies;
DNS-only is the simplest initial setup for those UI records. The API custom domain
belongs to the Worker; do not point `api` at Vercel or attach a Worker route to the UI
apex/`www`. Do not guess Vercel IP/CNAME targets; use the values shown for this project.
Arbitrary Vercel preview hostnames are not in the API/Turnstile allowlists; validate
using the three configured production hostnames.

The deploy config already supplies `DB`, `SESSION_LIMITER` (5 session attempts per IP
per 60-second window), model variables, production localhost-off and observability.
Verify them in the Worker dashboard. A custom-domain WAF rule can supplement the
session limiter; it does not replace it and does not protect the workers.dev fallback.

Check `/health` on both API hosts, then test a real session/chat from the portfolio.
Inspect Worker logs for generic errors. Verify D1 quota/`changes()` behavior and the
401/reset/limit UI flows before announcing the chat; see the plan's remaining checks.

GitHub and Cloudflare are separate: a local Git commit or push does not deploy this
backend until a deployment workflow is configured. Start with Wrangler; add a GitHub
build/deploy workflow later if desired. Vercel may deploy automatically when the
frontend production branch is pushed, so configure its variables before that push.

### Commit the reviewed changes locally

If the coding agent cannot get permission to write Git metadata, these PowerShell
commands commit the current reviewed files. Check `git status` first. The backend
`git add -u` assumes its tracked changes still match this review; the frontend list
deliberately stages only the three files changed for this feature.

```powershell
Set-Location C:\Git_new\InBound
git add -u
git add -- src/adapters/http.rs src/adapters/llm/opencode.rs tests/models.rs
git add -f -- claude-5.5.md
git commit -m "Migrate InScope to configurable OpenCode models and harden request handling"
Set-Location C:\vinamra_portfolio
git add -- components/ContactTerminal.js lib/inscope.js tests/inscope.test.mjs
git commit -m "Use InScope model slots and fix chat session lifecycle"
```

No push or deployment is included in these commands. Configure Vercel variables before
pushing its production branch. The plan is force-added once because of the existing
`*.md` ignore rule; subsequent edits to a tracked plan can be committed normally.

## Testing

- Pure functions (domain, clock day-math, error mapping): `cargo test`
- Anything touching `worker` types (D1, Fetch, Request): `wrangler dev` + `scripts/smoke.sh`
- Frontend helpers: `node --test tests/inscope.test.mjs` from the frontend repo
- Frontend production build: `npm.cmd run build` from the frontend repo
- Full Rust-to-Worker packaging: `worker-build --release`, then `npx.cmd wrangler@4 deploy --dry-run`

The release profile uses `strip="debuginfo"` so wasm-bindgen can read the Wasm
`target_features` metadata. With Rust 1.99.0 / wasm-bindgen 0.2.129, `strip=true`
removed that metadata and failed with `externref table required for catch wrappers`.
The full build and Wrangler 4.151.1 dry run pass with the corrected setting.

Verified on 2026-10-19: 48 Rust tests, Wasm target check, 6 frontend helper tests and
Next.js production build pass. Live D1/Turnstile/OpenCode integration and browser
interaction are still pending. The real D1 ID is configured with binding `DB`, and the user confirmed remote schema import succeeded. Worker database access and quota behavior remain unverified. See
[claude-5.5.md](claude-5.5.md) for current flows, fix history and deployment checklist.
