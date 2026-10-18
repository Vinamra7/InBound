#!/bin/sh
# Smoke tests against `wrangler dev` (default http://127.0.0.1:8787).
# Needs a .dev.vars with TURNSTILE_SECRET / HMAC_KEY / LLM_API_KEY.
# Turnstile-protected paths can't pass siteverify locally — expect 403 there;
# this script checks the surrounding behavior (400/401/404/CORS), not a live chat.

BASE=${BASE:-http://127.0.0.1:8787}
fail=0

check() {
  desc="$1"; want="$2"; got="$3"
  if [ "$want" = "$got" ]; then
    echo "ok    $desc ($got)"
  else
    echo "FAIL  $desc (want $want, got $got)"
    fail=1
  fi
}

code_of() {
  curl -s -o /dev/null -w "%{http_code}" "$@"
}

status=$(code_of "$BASE/health")
check "GET /health" 200 "$status"

headers=$(curl -sS -X OPTIONS "$BASE/api/chat" -H "Origin: https://vinamramishra.dev" -H "Access-Control-Request-Method: POST" -H "Access-Control-Request-Headers: content-type,authorization" -D - -o /dev/null)
status=$(printf '%s\n' "$headers" | head -n 1 | awk '{print $2}')
check "OPTIONS preflight" 204 "$status"
status=$(printf '%s\n' "$headers" | grep -i 'access-control-allow-headers:' | grep -ic 'authorization')
check "preflight allows Authorization" 1 "$status"

status=$(code_of "$BASE/api/nope" -H "Origin: https://vinamramishra.dev")
check "unknown path 404" 404 "$status"

status=$(code_of -X POST "$BASE/api/chat" -H "Content-Type: application/json" -H "Origin: https://vinamramishra.dev" -d '{"model":"llm-1","messages":[]}')
check "chat empty messages 400" 400 "$status"

status=$(code_of -X POST "$BASE/api/chat" -H "Content-Type: application/json" -H "Origin: https://vinamramishra.dev" -d '{"model":"llm-1","messages":[{"role":"system","content":"hi"},{"role":"user","content":"Work?"}]}')
check "chat system role 400" 400 "$status"

status=$(code_of -X POST "$BASE/api/chat" -H "Content-Type: application/json" -H "Origin: https://vinamramishra.dev" -d '{"model":"llm-1","messages":[{"role":"user","content":"hello"}]}')
check "chat no token 401" 401 "$status"

status=$(code_of -X POST "$BASE/api/chat" -H "Content-Type: application/json" -H "Origin: https://vinamramishra.dev" -H "Authorization: Bearer fake.token.here" -d '{"model":"llm-1","messages":[{"role":"user","content":"hello"}]}')
check "chat bad token 401" 401 "$status"

status=$(code_of -X POST "$BASE/api/session" -H "Content-Type: application/json" -H "Origin: https://vinamramishra.dev" -d '{}')
check "session missing fingerprint 400" 400 "$status"

status=$(code_of -X POST "$BASE/api/session" -H "Content-Type: application/json")
check "session no body 400" 400 "$status"

status=$(code_of -X POST "$BASE/api/chat" -H "Content-Type: application/json" -d '{"model":"deepseek-v4.1-flash-free","messages":[{"role":"user","content":"Work?"}]}')
check "raw provider model rejected 400" 400 "$status"

messages=''
i=0
while [ "$i" -lt 22 ]; do
  if [ "$i" -gt 0 ]; then messages="$messages,"; fi
  messages="$messages{\"role\":\"user\",\"content\":\"hi\"}"
  i=$((i + 1))
done
status=$(code_of -X POST "$BASE/api/chat" -H "Content-Type: application/json" -d "{\"model\":\"llm-1\",\"messages\":[$messages]}")
check "22 messages rejected 400" 400 "$status"

for path in health api/nope; do
  headers=$(curl -sS "$BASE/$path" -H "Origin: https://vinamramishra.dev" -D - -o /dev/null)
  status=$(printf '%s\n' "$headers" | grep -i 'access-control-allow-origin:' | grep -c 'https://vinamramishra.dev')
  check "CORS header on $path" 1 "$status"
  status=$(printf '%s\n' "$headers" | grep -i 'content-type:' | grep -ic 'application/json')
  check "JSON Content-Type on $path" 1 "$status"
done
headers=$(curl -sS -X POST "$BASE/api/chat" -H "Content-Type: application/json" -H "Origin: https://vinamramishra.dev" -d '{"model":"llm-1","messages":[]}' -D - -o /dev/null)
status=$(printf '%s\n' "$headers" | grep -i 'access-control-allow-origin:' | grep -c 'https://vinamramishra.dev')
check "CORS header on chat error" 1 "$status"
headers=$(curl -sS "$BASE/health" -H "Origin: https://evil.com" -D - -o /dev/null)
status=$(printf '%s\n' "$headers" | grep -ic 'access-control-allow-origin:')
check "foreign origin not allowed" 0 "$status"

exit $fail
