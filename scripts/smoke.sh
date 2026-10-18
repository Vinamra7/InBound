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

status=$(code_of -X OPTIONS "$BASE/api/chat" -H "Origin: https://vinamramishra.dev")
check "OPTIONS preflight" 204 "$status"

status=$(code_of "$BASE/api/nope" -H "Origin: https://vinamramishra.dev")
check "unknown path 404" 404 "$status"

status=$(code_of -X POST "$BASE/api/chat" -H "Content-Type: application/json" -H "Origin: https://vinamramishra.dev" -d '{"messages":[]}')
check "chat empty messages 400" 400 "$status"

status=$(code_of -X POST "$BASE/api/chat" -H "Content-Type: application/json" -H "Origin: https://vinamramishra.dev" -d '{"messages":[{"role":"system","content":"hi"}]}')
check "chat system role 400" 400 "$status"

status=$(code_of -X POST "$BASE/api/chat" -H "Content-Type: application/json" -H "Origin: https://vinamramishra.dev" -d '{"messages":[{"role":"user","content":"hello"}]}')
check "chat no token 401" 401 "$status"

status=$(code_of -X POST "$BASE/api/chat" -H "Content-Type: application/json" -H "Origin: https://vinamramishra.dev" -H "Authorization: Bearer fake.token.here" -d '{"messages":[{"role":"user","content":"hello"}]}')
check "chat bad token 401" 401 "$status"

status=$(code_of -X POST "$BASE/api/session" -H "Content-Type: application/json" -H "Origin: https://vinamramishra.dev" -d '{}')
check "session missing fingerprint 400" 400 "$status"

status=$(code_of -X POST "$BASE/api/session" -H "Content-Type: application/json" -H "Origin: https://evil.com" -d '{"turnstile":"x","fingerprint":{"device_id":"3a7f2b9c8d41e5f6a0b3c2d1e4f7a8b6","tz":"Asia/Kolkata","lang":"en-IN"}}')
check "session foreign origin 403" 403 "$status"

status=$(code_of "$BASE/health" -D - -o /dev/null | grep -i "access-control-allow-origin" | grep -c "vinamramishra.dev")
check "CORS header on response" 1 "$status"

exit $fail
