#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

if [ ! -f .env ]; then
    cp .env.example .env
    echo "[txtme] Created .env — fill in your SMTP credentials and SMS_GATEWAY before starting."
fi
