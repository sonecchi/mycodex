#!/usr/bin/env bash
set -euo pipefail

PATCH_FILE="changes.patch"

has_patch() {
  [[ -f "$PATCH_FILE" ]]
}

is_applied() {
  # If reverse applies cleanly, patch is currently applied
  git apply -R --check "$PATCH_FILE" >/dev/null 2>&1
}

can_apply() {
  git apply --check "$PATCH_FILE" >/dev/null 2>&1
}

cmd=${1:-status}

if ! has_patch; then
  echo "[cot] $PATCH_FILE not found at repo root." >&2
  exit 1
fi

case "$cmd" in
  status)
    if is_applied; then
      echo "[cot] patch is applied ✅"
    else
      echo "[cot] patch is NOT applied ❌"
    fi
    ;;
  on)
    if is_applied; then
      echo "[cot] already applied ✅"
      exit 0
    fi
    if can_apply; then
      git apply "$PATCH_FILE"
      echo "[cot] applied patch ✅"
    else
      echo "[cot] patch cannot be applied cleanly. Resolve conflicts then retry." >&2
      exit 2
    fi
    ;;
  off)
    if is_applied; then
      git apply -R "$PATCH_FILE"
      echo "[cot] reverted patch 🔄"
    else
      echo "[cot] already not applied ✅"
    fi
    ;;
  ensure)
    if is_applied; then
      echo "[cot] already applied ✅"
      exit 0
    fi
    if can_apply; then
      git apply "$PATCH_FILE"
      echo "[cot] applied patch after merge ✅"
    else
      echo "[cot] patch cannot be applied cleanly after merge. Please rebase/resolve." >&2
      exit 2
    fi
    ;;
  *)
    echo "Usage: $0 {status|on|off|ensure}" >&2
    exit 64
    ;;
esac

