# InScope — Rate-Limited, In-Scope LLM Chat for a Static Portfolio (No Login)

Rust Cloudflare Worker that proxies an LLM chat for a static portfolio site.
**InScope** = keeps the LLM *in scope*: a relevance classifier (Typesafe Jev AI) gates every
message before the real LLM call, behind Turnstile-gated sessions and atomic D1 quotas.

Goal: discourage lazy scripts and single-IP spam. A valid bearer session can be replayed
with curl until expiry; it still shares the device's daily quota. Rotating device IDs and
residential proxies + CAPTCHA solvers remain accepted risks. The global quota bounds
provider calls; configure and verify OpenCode billing controls separately.

Current scope (2026-10-19): OpenCode replaces OpenRouter. Three server-configured chat
slots; the UI currently sends `llm-1`. A visitor model selector / `/model` command is deferred.

---

## 1. Hosts & Domains

| Role | Primary | Fallback (corporate DNS often blocks new domains) |
|---|---|---|
| UI (Vercel) | `https://vinamramishra.dev` | `https://vinamra-portfolio-beta.vercel.app` |
| API (Worker) | `https://api.vinamramishra.dev` (custom domain, orange cloud) | `https://inscope.<acct>.workers.dev` |

- UI picks the API base **by its own host**: `vinamramishra.dev` / `www.vinamramishra.dev` → `NEXT_PUBLIC_API_PRIMARY`; localhost / 127.0.0.1 → `NEXT_PUBLIC_API_LOCAL`; other hosts → `NEXT_PUBLIC_API_FALLBACK`. An empty fallback produces a configuration error; set the actual Worker URL before building Vercel.
- Browser calls the Worker **directly** (never via a Vercel function — the Worker would then see Vercel's IP).
- Because the fallback pair is cross-site (`vercel.app` / `workers.dev` are on the Public Suffix List), **no cookies**. Token travels in `Authorization: Bearer`.
- Domain safelisting (to-do, outside code): submit `vinamramishra.dev` as "Personal site / Technology" to Zscaler, Palo Alto, Cisco Talos, FortiGuard, Broadcom (Bluecoat), Trellix; verify in Google Search Console. Put real content up first. "Newly Registered" flags drop after ~30 days on their own.
- Test that `*.workers.dev` itself isn't blocked from a corporate network if possible.

---

## 2. Endpoints & Flows

### `POST /api/session` → `{ token, expires_at }`
Request body: `{ turnstile: String, fingerprint: { device_id, tz, lang } }` (fingerprint **required**, else 400). Body cap ~8 KB.

```
1. edge = adapters::edge::extract(&req)          (before body parse)
2. gate(edge) == Block (Tor, cf.country == T1; CF-IPCountry fallback) → 403
3. SESSION_LIMITER.limit(ip)  (Workers Rate Limiting binding, per IP)  → 429
4. bounded body read + parse; malformed device_id → 400 before siteverify
5. turnstile::verify(secret, token, ip): success && allowed hostname && action == "session" → else 403; network/timeout/malformed upstream → 502
6. fp = validate_fingerprint(payload, edge)       (strike scoring, §4)
7. q = Q_RESTRICTED if fp == Suspicious || gate == Restricted (datacenter ASN), else Q_NORMAL
8. token = sign(TokenRaw{device_id, q, iat, exp: iat+24h}, HMAC_KEY)
9. return { token, expires_at }   (UI stores in sessionStorage)
```

### `POST /api/chat` → `{ reply, usage }`
Headers: `Authorization: Bearer <token>`. Body: `{ model: "llm-1", messages: [{role, content}] }` (no token field). `model` is required; only `llm-1`, `llm-2`, `llm-3` are accepted. Raw provider names / unknown aliases → 400 before counters or provider calls. Body cap: 40 KB, enforced while reading.

```
1. edge = extract(&req)
2. deserialize allowed model alias; validate_chat_request: shape, no `system` role (400, never strip), last = user,
   per-role lengths, messages.len() ≤ 21 (10 pairs + current)          → 400
3. gate(edge) == Block (Tor)                                             → 403   (pure CPU, before any counter)
4. verify bearer token (HMAC + exp)                                      → 401 → UI re-solves Turnstile at /session
5. counter::incr — ONE D1 batch, user row + global row                  → 429 user / 503 global
6. classifier (OpenCode Jev native noul, counts toward budget) — off-topic → canned "I only answer about Vinamra" reply
7. resolve alias from Cloudflare config; OpenCode completion with server system prompt + history, 2048 output-token cap, no streaming
8. return full JSON
```

### `GET /health` → `{ok:true}`

### `OPTIONS *` → CORS preflight (204)

---

## 3. Locked Decisions

- **Two D1 counters only:** per-user `(UTC-day, device_id)` and global `(UTC-day)`.
  - Quotas: user **10/day** (`Q_RESTRICTED` → **2/day**), global **500/day**.
  - Single atomic incr-if-under-limit **before** classifier + LLM. Count attempts (no refund on failure).
  - Session-minting abuse is handled by the **Workers Rate Limiting binding** on `/api/session` (works on both hosts) + the WAF rule on the custom domain (free plan likely only allows 10s windows — check dashboard).
- **Identity = `device_id`** (FingerprintJS visitor id, 32 lowercase hex). Client-claimed → rotatable; accepted trade-off, bounded by Turnstile + session rate limit + global cap. Possible later upgrade: a per-IP daily ceiling row in the same batch (+2 statements).
- **Session token:** `base64(json).base64(hmac_sha256)`, `TokenRaw {device_id, q, iat, exp}`, 24h TTL. Signature checked with `verify_slice` (constant time). Counters live on the server, never in the token.
- **Streaming OFF.** Single JSON response.
- **Provider:** OpenCode Console inference, base `https://opencode.ai/inference`. Secret `LLM_API_KEY` must contain an OpenCode service-account API key. A Go subscription or added credit alone does not verify that the API key has inference access.
- **Model selection:** Cloudflare variables supply the three chat model IDs and API formats (§3a). Client aliases cannot override IDs, endpoints or the classifier. This is a choice of slot, not a fallback chain.
- **Classifier (Typesafe Jev AI — the project's headline feature):** private `CLASSIFIER_MODEL` variable. POST `/systemone/v1/systemone` with `{model,state,questions:{in_scope:{type:"noul",instructions}}}`. Only the current message is classified. Read `answers.in_scope.noul` as a probability in [0,1]; ≥0.5 is in scope. This is native `noul`, not generated `yes`/`no` text. Malformed answer fields → fail-open + generic log (existing availability trade-off). Transport/HTTP/invalid-JSON failures → 502. No text temperature/max_tokens fields on this native endpoint.
- **Roles:** client may send `user`, `assistant`, `agent` (alias → assistant). `system` from client → 400. Upstream always `assistant`; omit `stream`.
- **Length limits:** user 80 words / 1536 bytes; assistant 300 words / 3072 bytes. Order: shape/roles → per-entry lengths → turn cap.
- **CORS:** allowlist `["https://vinamramishra.dev", "https://www.vinamramishra.dev", "https://vinamra-portfolio-beta.vercel.app"]`; `ALLOW_LOCALHOST=true` additionally enables `http://localhost:3000` and `http://127.0.0.1:3000`. Echo the matching `Origin`; allow headers `Content-Type, Authorization`; no credentials. Preserve JSON Content-Type on every wrapped response incl. errors and 404; `Vary: Origin`, `Cache-Control: no-store`. CORS does not authenticate callers.
- **Errors:** body is `{"error": code}` only — never which limit/key. Generic configuration/classifier logs; no prompt, bearer token or provider response logging added. Codes: 400 `bad_request`, 401 `unauthorized`, 403 `human_check_failed`, 404 `not_found`, 429 `rate_limited`, 503 `budget_exhausted`, 502 `upstream`, 500 `internal`.
- **Cost bounds:** 500 admitted chat attempts/day, each with at most one classifier and one completion. Completion cap 2048 tokens, no automatic fallback/retry. Free model availability/pricing and account spending controls must be checked in OpenCode before going live; a provider spend cap has not been verified.
- **Turnstile:** widget configured for all three production UI hostnames; hostname + action + remoteip verified server-side. Local widget hostnames (`localhost`, `127.0.0.1`) require `ALLOW_LOCALHOST=true`; production defaults to false. Siteverify deadline 8s / body cap 16 KB. Classifier deadline 5s; chat deadline 20s; provider bodies capped at 128 KB.

### 3a. Cloudflare Model Configuration

| Client slot | Model variable | Initial value | API variable / initial value |
|---|---|---|---|
| `llm-1` (current UI choice) | `LLM_1_MODEL` | `deepseek-v4.1-flash-free` | `LLM_1_API=chat_completions` |
| `llm-2` | `LLM_2_MODEL` | `glm-5.3-flash-free` | `LLM_2_API=chat_completions` |
| `llm-3` | `LLM_3_MODEL` | `longcat-2.5-preview-free` | `LLM_3_API=chat_completions` |
| private classifier | `CLASSIFIER_MODEL` | `jev-1.13-free` | fixed native System One / `noul` |

The four initial IDs were found in the public OpenCode model catalog on 2026-10-19;
no inference calls or account credits were used to verify them. Account access and runtime
responses remain unverified. Change variables in Cloudflare **Settings → Variables and Secrets**;
publish the variable changes. API format must match the chosen model:

- `chat_completions` → `/openai/v1/chat/completions`, `max_tokens=2048`.
- `responses` → `/openai/v1/responses`, `max_output_tokens=2048`, `store=false`; parse message `output_text`, excluding reasoning.
- Other formats → configuration error. Changing the private classifier ID requires another model that supports the native `noul` contract.

`wrangler.toml` includes the initial values and `keep_vars=true`. **Keys explicitly in `[vars]`
are still reapplied on deploy**: sync any dashboard edits into the file before redeploying.
`keep_vars` preserves only dashboard variables omitted from the file.

### 3b. Frontend Contract and Lifecycle (`C:\vinamra_portfolio`)

- `lib/inscope.js`: `sendChat` defaults to `model:"llm-1"`; only three aliases accepted. Bearer token in sessionStorage; discard 60s before expiry. No cookies. Session/chat requests have 30s deadlines and caller cancellation; fingerprint acquisition has an 8s deadline. Do not save a cancelled/malformed session. Empty fallback URL is an explicit config error.
- `ContactTerminal.js`: passes `llm-1`; no model selector yet. Turnstile only when a valid session is absent, unmounted after success; `refresh-expired:"never"`. One 401 re-verification and automatic resend; local shortcuts preserve the queued retry. Prevent overlapping session creation.
- Reset clears conversation and cancels an in-flight chat; preserves session, daily-limit lock and session retry timer/status. Ignore stale replies after reset/unmount. Clean up timers, requests and fake typing.
- `apiHistory` stores only successful backend exchanges, at most 20 entries starting with user. Assistant history trimmed to 300 words / 3000 bytes; drop oldest pairs if JSON would exceed 40 KB. Full replies remain in display memory. Local contacts only for ≤6 words or explicit shortcut buttons. Render model output as React text.
- 429/503 lock free text; contact shortcuts remain available. Over-limit edits keep the existing draft. Privacy copy discloses OpenCode messages/history and stored device/day usage.
- Vercel public variables are baked at build time: `NEXT_PUBLIC_API_PRIMARY`, `NEXT_PUBLIC_API_FALLBACK`, `NEXT_PUBLIC_API_LOCAL`, `NEXT_PUBLIC_TURNSTILE_SITE_KEY`. Set `NEXT_PUBLIC_INSCOPE_MOCK=0` for production; only `1` enables mock behavior. Never put provider/HMAC/Turnstile secrets in public variables.

---

## 4. Fingerprint Check (minimal, strike-based)

Keep `domain/fingerprint.rs` as is, with one change: a single mismatch must not restrict a user
(VPN / mobile users often have a tz that differs from Cloudflare's geo-IP tz).

```
malformed device_id                     → 400 at session route (before Turnstile)
strike +1  payload.tz   != cf.timezone
strike +1  payload.lang ∉ Accept-Language primaries
strike +1  cf.httpProtocol == "HTTP/1.1"   (optional; Cloudflare-set, unforgeable; browsers use h2/h3)
strikes ≥ 2                             → Suspicious  → Q_RESTRICTED
```

- No client-sent IP (browser JS can't know it; `CF-Connecting-IP` is the truth). Timezone/country/ASN/protocol use Cloudflare metadata; `Accept-Language` is client-controlled and is only a weak hint, not trusted identity.
- No hash recomputation (FingerprintJS is open source → a matching hash proves nothing).
- Update `tests/fingerprint.rs`: single tz mismatch now expects `Ok`.

---

## 5. D1 Counter Design (`adapters/counter.rs`)

```sql
-- schema.sql
CREATE TABLE IF NOT EXISTS user_counters (
  day TEXT NOT NULL, device_id TEXT NOT NULL, count INTEGER NOT NULL DEFAULT 0,
  PRIMARY KEY (day, device_id));
CREATE TABLE IF NOT EXISTS global_counters (
  day TEXT PRIMARY KEY, count INTEGER NOT NULL DEFAULT 0);
```

Batch order (fixes "denied user still burns a global slot"):

```
0 INSERT OR IGNORE user row (count 0)
1 INSERT OR IGNORE global row (count 0)
2 UPDATE user   SET count=count+1 WHERE day=?1 AND device_id=?2 AND count < ?3
3 UPDATE global SET count=count+1 WHERE day=?1 AND count < ?2 AND changes() = 1   -- only if [2] succeeded
4 SELECT user count
5 SELECT global count
```

- Read `meta().changes` at indices **2** and **3**; results at **4** and **5**. Missing metadata/results → `Internal`, not a silent quota denial.
- `changes()` relies on the batch running sequentially on one connection — **verify in `wrangler dev`**.
- Replace every `.unwrap()` / `[0]` with `.ok_or(AppError::Internal)?` — a panic = 500 without CORS.
- Mapping: user denied → 429 `RateLimited`; global denied → 503 `BudgetExceeded`.
- Current accepted attempt-counting behavior: an exhausted global quota can still consume a user attempt; a user denial does not consume global quota. Denied/auth-invalid/Tor/model-spoofed requests do not call the provider.

---

## 6. Repo Tree (mind map)

Legend: ✅ done · 🔧 exists, needs fix (see §7) · ⬜ empty / to write

```
InBound/                         (crate name: inscope)
├── Cargo.toml                 ✅ deps incl. futures-util + [profile.release]
├── wrangler.toml              🔧 model variables + production localhost-off + observability; real database_id configured, deployed bindings still unverified
├── schema.sql                 ✅ user_counters + global_counters (§5)
├── .dev.vars.example          ✅ TURNSTILE_SECRET, HMAC_KEY (≥32 bytes), OpenCode LLM_API_KEY, local ALLOW_LOCALHOST=true
├── README.md                  ✅ architecture + threat-model table + run/deploy steps
├── claude-5.5.md              this plan
├── learning.md                personal notes
├── src/
│   ├── lib.rs                 ✅ #[event(fetch)], panic hook, router — CORS wrap on every response, OPTIONS 204, AppError wiring
│   ├── config.rs              ✅ required model IDs/API-format allowlist, HMAC length, explicit local-host flag
│   ├── constants.rs           ✅ OpenCode base, server prompt, caps, production host/origin lists (historical comments retained)
│   ├── error.rs               ✅ sanitized error codes; worker errors → Internal
│   ├──
│   ├── routes/                thin HTTP layer: parse → pipeline → serialize
│   │   ├── mod.rs             ✅
│   │   ├── session.rs         🔧 full §2 flow coded — needs `wrangler dev` run (rate limiter + Turnstile + token)
│   │   ├── chat.rs            🔧 full §2 flow coded — needs `wrangler dev` run (D1 + classifier + completion)
│   │   ├── cors.rs            🔧 header merge preserves JSON Content-Type; local opt-in; needs runtime preflight/error-header check
│   │   └── health.rs          ✅ GET /health
│   ├──
│   ├── pipeline/              decision steps called by routes
│   │   ├── mod.rs             ✅
│   │   ├── validate.rs        ✅ validate_chat_request: ≤21 msgs, no System, last = user, per-role lengths, MAX_BODY_BYTES 40 KB
│   │   ├── auth.rs            ✅ Bearer header → token::verify(now) → TokenRaw | 401
│   │   ├── risk.rs            ✅ Block → 403, quota_tier (fp strikes / datacenter ASN)
│   │   └── limits.rs          ✅ user_limit_for(q) + enforce: ONE counter::incr batch → 429 / 503
│   │
│   ├── domain/                pure logic, no `worker` imports, `cargo test` natively
│   │   ├── mod.rs             ✅
│   │   ├── message.rs         ✅ ChatRequestDto {model: ChatModelSlot, messages}, no token/raw model names; Role agent alias; System rejected explicitly
│   │   ├── token.rs           ✅ SessionRequestDto (with fingerprint), SessionResponseDto {token, expires_at}, TokenRaw, sign/verify, Q_NORMAL/Q_RESTRICTED — HMAC checked before parse
│   │   ├── fingerprint.rs     ✅ FingerprintPayload {device_id, tz, lang}, validate_fingerprint — strike scoring, ≥2 strikes → Suspicious
│   │   ├── risk.rs            ✅ RiskDecision {Block, Restricted, Allow}, DATA_CENTER_ASNS, gate()
│   │   └── edge.rs            ✅ EdgeInfo {ip, country, asn, tz, lang, http_protocol}, is_tor()
│   │
│   └── adapters/              Cloudflare I/O as free functions, no trait layer
│       ├── mod.rs             ✅
│       ├── edge.rs            ✅ Cloudflare country first, CF-IPCountry fallback; edge facts + weak Accept-Language hint
│       ├── http.rs            🔧 bounded stream reads + fetch abort deadlines; Wasm check passes, runtime still unverified
│       ├── turnstile.rs       🔧 pure hostname/action checks tested; bounded timed siteverify needs real widget test
│       ├── counter.rs         🔧 incr reordered + changes()=1 gate + unwraps removed — code compiles; still UNTESTED on real D1 (verify `changes()` in `wrangler dev`)
│       ├── clock.rs           ✅ now_secs() + utc_day() (pure civil-date math, unit-tested)
│       └── llm/
│           ├── mod.rs         ✅ module wiring
│           └── opencode.rs    🔧 native noul classifier + completion/Responses formats tested; no live inference run (openrouter.rs removed)
│
├── tests/                     native unit tests (pure fns only)
│   ├── error.rs               ✅ AppError status/code
│   ├── fingerprint.rs         ✅ strike scoring — single mismatch Ok, ≥2 Suspicious
│   ├── limits.rs              ✅ message length rules
│   ├── models.rs              ✅ three aliases, raw/unknown/missing rejection, API-format allowlist
│   ├── quota.rs               ✅ user_limit_for
│   ├── risk.rs                ✅ Tor / datacenter ASN gate
│   ├── security.rs            ✅ Turnstile production hosts, wrong action/host rejection, explicit localhost opt-in
│   └── token.rs               ✅ sign/verify/expiry/tamper
│
└── scripts/
    └── smoke.sh               🔧 curl checks written — not yet run against `wrangler dev`
```

Frontend files (outside this repo): `lib/inscope.js` ✅ six helper regression tests;
`components/ContactTerminal.js` 🔧 production build/control-flow review pass, browser interaction
still pending; `components/ChatVerification.js` unchanged (action/cleanup/test-key guard reviewed);
`tests/inscope.test.mjs` ✅ Node built-in regression suite.

Testing: pure functions → `cargo test`; anything touching `worker` types (D1, Fetch, Request) → `wrangler dev` + `smoke.sh`.

Rust-on-Wasm reminders: no `SystemTime::now()` (use `worker::Date`), no tokio/reqwest (use `worker::Fetch`), `console_error_panic_hook` on, add release profile:

```toml
[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
strip = true
```

---

## 7. Fix List for Existing Code (small, do first)

1. ~~Delete `src/durable/` and `pub mod durable;`~~ ✅ done. ~~Delete empty `tests/pipeline.rs`~~ ✅ done.
2. ~~`domain/message.rs`: remove `token` from `ChatRequestDto`; reject `Role::System` explicitly (not via max_bytes = 0)~~ ✅ done
3. ~~`domain/token.rs`: `SessionRequestDto` gains `fingerprint: FingerprintPayload`. In `verify`, check HMAC on the raw payload string **before** base64-decoding/parsing JSON.~~ ✅ done (`SessionResponseDto {token, expires_at}` kept as-is)
4. ~~`domain/fingerprint.rs`: strike scoring (§4) + test updates~~ ✅ done (also wired `http_protocol` into `EdgeInfo` + `adapters/edge.rs`)
5. ~~`constants.rs` / `adapters/turnstile.rs`: `SITE_HOST` → `ALLOWED_HOSTS: &[&str]`~~ ✅ done
6. ~~`adapters/counter.rs`: batch reorder + `changes() = 1` + remove unwraps (§5)~~ ✅ coded (compile + tests pass) — D1 behavior still to verify in `wrangler dev`
7. ~~`error.rs`: `From<worker::Error>` → `Internal`~~ ✅ done (adapters map deliberate fetch failures to `Upstream` explicitly)
8. ~~`routes/health.rs`: drop the redundant path check~~ ✅ done
9. ~~`Cargo.toml`: release profile~~ ✅ done
10. ~~Replace OpenRouter with OpenCode; configure three allowed aliases and one private native `noul` classifier; support per-slot API format~~ ✅ 48 Rust tests + Wasm check pass; live provider check pending.
11. ~~Bound streamed request/upstream bodies; add Turnstile/classifier/completion deadlines; reject blank replies; preserve CORS JSON Content-Type; default localhost off; missing D1 metadata → Internal; prefer CF country metadata~~ ✅ code compiles and pure tests pass; Worker runtime checks pending.
12. ~~Frontend: send llm-1, request cancellation/deadlines, reset stale-reply guard, preserve session retry state and queued auth resend, cap successful history, preserve over-limit draft, disclose provider sharing~~ ✅ six helper tests + production build; interactive verification pending.
13. ~~Update smoke payloads for model aliases, hostile roles/turn cap, preflight Authorization and CORS headers; update README/local config/this plan~~ ✅ files reviewed; smoke script execution against Worker pending.

---

## 8. Progress Ledger (re-audited 2026-10-19)

| Area (weight) | Module | % | Remaining |
|---|---|---|---|
| Domain (15) | message/model aliases, token, edge/risk, fingerprint | 100 | pure tests pass |
| Adapters (25) | edge/clock, http, Turnstile, D1 counter, OpenCode | 88 | live D1 batch semantics, fetch deadlines and real provider/widget responses |
| Pipeline (15) | validate/auth/risk/limits | 100 | done |
| Routes (15) | routing, session/chat, CORS/OPTIONS | 90 | Worker runtime smoke and end-to-end paths |
| Frontend (15) | helpers, ContactTerminal, ChatVerification | 85 | real browser session/401/reset/shortcut interaction and deployed host configuration |
| Infra & deploy (15) | schema, vars, Wrangler, smoke, docs | 60 | DB id configured; local smoke, secrets, account limits, deploy and real-host checks remain |

**Overall: ~87% (estimate).** Implementation and offline checks pass. Integration and deployment
are not complete. Weights now include the authorized frontend work; this is not a claim of
production readiness.

### Verification (2026-10-19)
- `cargo test`: **48 passed**, no failures.
- `cargo check --target wasm32-unknown-unknown`: **passed**.
- `cargo fmt`: **passed**.
- Frontend `node --test tests/inscope.test.mjs`: **6 passed** (aliases, byte cap/history, errors/blank reply, cancellation, 30s timeout, session expiry/cancelled storage).
- Frontend `npm.cmd run build`: **passed**.
- Not run by the agent: `wrangler dev`, D1 batch concurrency/`changes()` integration, real Turnstile, browser interaction, authenticated OpenCode inference, deployment. The user created D1 and supplied its real ID; the local config uses binding `DB`. Remote schema/binding behavior is still unverified. No paid inference performed.

### Review log
- 2026-10-18 — Gemini 4 Argon cross-repo review. Applied: fail-open classifier (was 502/contain-yes), turnstile send→502, chat body cap 8KB→40KB, device_id shape 400 before Turnstile, vercel/localhost into host+origin lists, `ratelimits` `simple` block, health-before-config, JSON 404, numeric D1 binds + get() indexing, 5 more datacenter ASNs, official dummy Turnstile key, deleted dead `AssistantConfig` (+ its tests). **Rejected:** "invented model ids" — all three verified in the live catalog same day.
- 2026-10-19 — OpenCode migration + cross-repo fixes: three Cloudflare-configured aliases, native Jev noul, optional Responses adapter, bounded/timed I/O, CORS Content-Type fix, production localhost opt-in, D1 metadata failure handling, CF country source. Frontend fixes: request abort/deadlines, queued 401 retry preserved across local shortcuts, Reset preserves session retry state and blocks stale responses, successful history limited to 20 entries/40 KB, over-limit draft preserved, privacy updated. Six helper tests and production build pass. No UI model selector added.
- Historical comments in `constants.rs` / `error.rs` still mention OpenRouter / localhost; original provider tuning comments moved intact into `opencode.rs` as historical notes. Preserved under the project's user-comment rule. §3/§3a and actual configuration are current.
- `claude-5.5.md` is explicitly included in the release-preparation commit despite the existing `*.md` ignore rule. Future edits to the tracked plan appear in normal Git diffs. Ignore rules were left unchanged.
- 2026-10-19 — First-launch instructions added to README: Wrangler login, real D1 UUID/schema, dry-run/deploy, production Turnstile/secrets, Vercel public variables/rebuild, and separate UI/API DNS. No Cloudflare/Vercel resources were created or changed by this documentation work.
- 2026-10-19 — User created D1 `inscope` (`d71e1719-fcad-48a0-8fa2-cfd8f6bf74b8`). Restored the local binding to `DB` to match `env.d1("DB")`; database name remains `inscope`. Static TOML/code alignment verified; schema execution and Worker runtime remain pending.

### Build order from here (updated 2026-10-19)
1. ~~Implementation fixes in §7~~ ✅ offline checks pass; integration caveats remain above.
2. ~~Infra files and real D1 id~~ ✅ written/configured — database `inscope`, binding `DB`; copy `.dev.vars.example` → `.dev.vars` for local checks.
3. `wrangler dev` + `scripts/smoke.sh` — verify session + chat + counter `changes()` gate on D1.
4. Configure OpenCode service-account `LLM_API_KEY` and verify one real Jev + llm-1 exchange, only when live inference testing is authorized. Check free-model access and account spending controls.
5. Test real frontend session creation, one 401 resend, Reset during a request, daily-limit shortcuts, and preflight from each real UI origin.
6. Deploy after review: production secrets, all Turnstile hostnames, custom domain + WAF, Vercel public URLs / mock off. Preserve dashboard model settings when redeploying (§3a).

**Deferred:** visitor `/model` selector, streaming, full TLS/cipher scoring, per-IP daily counter (possible v2). Frontend fixes are now in scope; live deployment remains a separate step.

---

## 9. Cross-Repo Review (2026-10-19)

Backend paths below are relative to `C:\Git_new\InBound`; frontend paths are relative
to `C:\vinamra_portfolio`. References describe the reviewed working files.

| Contract item | Backend behavior (file:line) | Frontend behavior (file:line) | Result |
|---|---|---|---|
| API URLs | `wrangler.toml:7` custom domain, workers.dev enabled | `lib/inscope.js:18` host mapping | MATCH; actual fallback URL/config/deployment pending |
| Turnstile hostnames | `src/constants.rs:5` three production UI hosts | `lib/inscope.js:20` primary + www; fallback host supported | MATCH; dashboard widget hosts unverified |
| Turnstile action | `src/adapters/turnstile.rs:26` requires session | `components/ChatVerification.js:34` sends session | MATCH |
| Session shape | `src/domain/token.rs:17`, `src/routes/session.rs:73` | `lib/inscope.js:207`, `lib/inscope.js:219` | MATCH; token + numeric expires_at |
| Chat shape | `src/domain/message.rs:12`, `src/domain/message.rs:46` | `lib/inscope.js:246`, `lib/inscope.js:255`, `lib/inscope.js:289` | MATCH; required model + messages, reply/usage |
| Model aliases | `src/domain/message.rs:18`, `src/config.rs:96` allowlist/config resolution | `components/ContactTerminal.js:396` always llm-1 | MATCH; other slots available to future UI, raw IDs rejected |
| Roles | `src/domain/message.rs:61`, `src/pipeline/validate.rs:18` user/assistant, agent alias; system rejected | `components/ContactTerminal.js:384`, `components/ContactTerminal.js:400` user/assistant only | MATCH |
| Length/history | `src/domain/message.rs:3`, `src/pipeline/validate.rs:4` role limits, 21 messages / 40 KB | `components/ContactTerminal.js:21`, `components/ContactTerminal.js:49`, `lib/inscope.js:250` | MATCH; UI intentionally stricter at 500 chars/5 lines, 3000 assistant bytes |
| Error codes | `src/error.rs:23`, `src/error.rs:36` stable status/code pairs | `lib/inscope.js:282`, `components/ContactTerminal.js:316` status-driven messages/locks | MATCH; timeout/network/config remain local UI errors |
| CORS | `src/routes/cors.rs:5` origins + Content-Type/Authorization, no credentials | `lib/inscope.js:204`, `lib/inscope.js:271` direct JSON fetch + bearer | MATCH; actual browser preflight pending |
| Token transport | `src/pipeline/auth.rs:9` Authorization bearer only | `lib/inscope.js:108`, `lib/inscope.js:275` sessionStorage + bearer | MATCH; no cookies |
| Expiry/retry | `src/domain/token.rs:68` HMAC before payload parse + expiry check | `lib/inscope.js:101`, `components/ContactTerminal.js:318`, `components/ContactTerminal.js:367` 60s grace + one retry | MATCH; interactive retry unverified |
| Private classifier | `src/routes/chat.rs:50`, `src/adapters/llm/opencode.rs:151` config-only native noul | `lib/inscope.js:255` no classifier selector | MATCH |

### Remaining deployment blockers / checks

- **D1 resource configured — `wrangler.toml:12`:** user-created database `inscope` with its real UUID; binding restored to `DB` to match `src/routes/chat.rs:41`. Apply `schema.sql` and verify the deployed binding before launch.
- **Blocker — live access:** configure production Turnstile/HMAC/OpenCode secrets and actual Vercel fallback URL; verify real widget hostnames and OpenCode service-account inference access. Offline tests cannot establish these.
- **Required runtime verification — `src/adapters/counter.rs:43`:** run `wrangler dev`, exhaust one device's limit and compare D1 user/global rows before/after the denied request. Also send concurrent admitted requests and check neither cap is exceeded. Native tests do not prove D1 connection-local `changes()` behavior.
- **Required runtime verification — HTTP/UI:** run `sh scripts/smoke.sh` against local Worker (public validation/auth/CORS paths only, no LLM). Test actual allowed-origin preflight, 401 re-verification/resend, Reset during pending chat, 429/503 shortcuts and session timeout in a browser. Verify fetch deadlines in the Workers runtime and free-tier CPU time in observability.
- **Required account verification:** check rate-limit binding behavior on both API hosts, WAF plan/rule availability, free-model access/pricing and account spending controls. A Go subscription plus credit does not itself prove API access or a spending cap.

### Security trade-offs retained

| Attacker | What they can still do | Assessment |
|---|---|---|
| curl with a valid copied session | replay until expiry; choose any allowed slot; forge bounded user/assistant history | accepted; per-device/global quotas still apply, system role/raw model override rejected |
| Lazy script without a session | hit public endpoints, attempt Turnstile solving/session minting | Turnstile + per-IP mint limiter deter it; CORS is not authentication |
| One IP rotating fingerprints | obtain fresh device quotas after successful challenges; possibly move between rate-limit locations | accepted weakness of client-claimed identity and approximate edge rate limiting; can exhaust global daily availability |
| Residential proxies + CAPTCHA solver | distribute minting and drain the global budget | accepted trade-off; global call cap and independently verified account billing controls are needed |
| Prompt injection / fabricated assistant history | trick classifier or persuade the chat model away from scope | classifier + server prompt reduce risk, not a formal boundary; no tools/private data are exposed; malformed classifier fields deliberately fail open |

### Cleanup notes

Historical comments are retained verbatim (§8); `MAX_STORED_EXCHANGES` currently holds an
entry count (20 entries = 10 pairs), despite its older name. No selector, extra counters,
provider fallback chain or larger architectural refactor was added.
