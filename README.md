# txtme

An SMS notification endpoint for your personal EPS network. Any EPS on your tailnet can `POST /notify` to send you a text message.

Ships with email-to-SMS — no extra accounts or SDKs needed. Just a Gmail App Password and your carrier gateway address.

## Setup

### 1. Get a Gmail App Password

Google no longer lets apps use your actual password. You need to generate an App Password:

1. Go to https://myaccount.google.com/apppasswords
2. Name it anything (e.g. "txtme")
3. Copy the 16-character password it gives you

> You must have 2-Step Verification enabled on your Google account for this to appear.

### 2. Configure `.env`

Copy the example and fill in your details:

```bash
cp .env.example .env
```

Then edit `.env`:

```
SMTP_USER=you@gmail.com          # your Gmail address
SMTP_PASS=xxxx xxxx xxxx xxxx   # the App Password from step 1

SMS_GATEWAY=5551234567@vtext.com # your number + carrier gateway (see table below)

TXTME_API_KEY=pick-any-secret    # other EPS include this in X-Api-Key header
```

**Carrier gateway addresses:**

| Carrier  | Gateway address              |
|----------|------------------------------|
| Verizon  | `{number}@vtext.com`         |
| AT&T     | `{number}@txt.att.net`       |
| T-Mobile | `{number}@tmomail.net`       |
| Sprint   | `{number}@messaging.sprintpcs.com` |

### 3. Build and run

```bash
chmod +x serve.sh
./serve.sh
```

### 4. Test it

```bash
curl -X POST http://localhost:5543/notify \
  -H "Content-Type: application/json" \
  -H "X-Api-Key: your-secret" \
  -d '{"message": "txtme is working"}'
```

Check your phone. You're done.

---

## Calling txtme from another EPS

**Shell:**
```bash
curl -s -X POST http://<tailscale-host>:5543/notify \
  -H "Content-Type: application/json" \
  -H "X-Api-Key: $TXTME_API_KEY" \
  -d "{\"message\": \"$MESSAGE\"}"
```

**Rust:**
```rust
let client = reqwest::Client::new();
client.post("http://<host>:5543/notify")
    .header("X-Api-Key", api_key)
    .json(&serde_json::json!({"message": message}))
    .send()
    .await?;
```

**Python (stdlib):**
```python
import json, urllib.request

req = urllib.request.Request(
    "http://<host>:5543/notify",
    data=json.dumps({"message": "hello"}).encode(),
    headers={"Content-Type": "application/json", "X-Api-Key": "your-secret"},
    method="POST",
)
urllib.request.urlopen(req)
```
