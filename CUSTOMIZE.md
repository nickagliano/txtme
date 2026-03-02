# txtme — Customization Guide

txtme is a personal SMS notification endpoint. It runs as a persistent service and
exposes a single route — `POST /notify` — that any other EPS on your tailnet can hit
to send you a text message. It ships with an email-to-SMS provider using your carrier's
gateway address (no extra accounts or SDKs required).

## Ports

### `SMS_PROVIDER`

**What it does:** Controls how notifications are delivered.
**Default:** `providers/email_sms.py` — sends SMS via your carrier's email gateway over SMTP.
**How to customize:** Open `main.py` and swap the import at the top:

```python
# current default
from providers.email_sms import EmailSMSProvider
PROVIDER = EmailSMSProvider()

# dev mode — prints to stdout, no SMS sent
from providers.log import LogProvider
PROVIDER = LogProvider()
```

To add a new provider, create a file in `providers/` that implements `SMSProvider`:

```python
from .base import SMSProvider

class MyProvider(SMSProvider):
    def send(self, message: str) -> None:
        # your delivery logic here
        ...
```

Then import and assign it in `main.py`.

### `SMS_GATEWAY`

**What it does:** The email address your carrier uses to deliver SMS.
**Default:** None — you must set this.
**How to customize:** Set `SMS_GATEWAY` in your `.env` file:

```
SMS_GATEWAY=5551234567@vtext.com    # Verizon
SMS_GATEWAY=5551234567@txt.att.net  # AT&T
SMS_GATEWAY=5551234567@tmomail.net  # T-Mobile
```

### `SMTP_USER` / `SMTP_PASS`

**What it does:** The email account txtme sends from.
**Default:** None — you must set these.
**How to customize:** Set in `.env`. For Gmail, generate an App Password at
`https://myaccount.google.com/apppasswords` (requires 2FA on your Google account).

```
SMTP_USER=you@gmail.com
SMTP_PASS=xxxx xxxx xxxx xxxx
```

To use a different SMTP provider, also set `SMTP_HOST` and `SMTP_PORT`:

```
SMTP_HOST=smtp.fastmail.com
SMTP_PORT=587
```

### `TXTME_API_KEY`

**What it does:** A shared secret that callers must include in the `X-Api-Key` header.
Prevents anyone else on your network from triggering texts.
**Default:** Empty string — auth is disabled.
**How to customize:** Set `TXTME_API_KEY` in `.env` to any string. Callers then include:

```
X-Api-Key: your-secret-here
```

Leave empty if txtme is only reachable on a private tailnet and you trust all devices on it.

## Getting Started

1. Clone: `git clone <repository>`
2. Copy config: `cp .env.example .env`
3. Edit `.env` — fill in `SMTP_USER`, `SMTP_PASS`, and `SMS_GATEWAY`
4. Make serve.sh executable: `chmod +x serve.sh`
5. Run: `./serve.sh`

Test it:

```bash
curl -X POST http://localhost:5543/notify \
  -H "Content-Type: application/json" \
  -H "X-Api-Key: your-secret-here" \
  -d '{"message": "hello from txtme"}'
```

## Calling txtme from another EPS

```bash
curl -s -X POST http://<tailscale-hostname>:5543/notify \
  -H "Content-Type: application/json" \
  -H "X-Api-Key: $TXTME_API_KEY" \
  -d "{\"message\": \"$MESSAGE\"}"
```

Or in Python (stdlib only):

```python
import json, urllib.request

def notify(message, host="http://localhost:5543", api_key=""):
    req = urllib.request.Request(
        f"{host}/notify",
        data=json.dumps({"message": message}).encode(),
        headers={"Content-Type": "application/json", "X-Api-Key": api_key},
        method="POST",
    )
    urllib.request.urlopen(req)
```
