#!/usr/bin/env bash
set -euo pipefail

repo_root=$(git rev-parse --show-toplevel 2>/dev/null || pwd)

# Support multiple patch files. Order matters when applying.
PATCH_FILES=(
  "$repo_root/changes.patch"
  "$repo_root/prompt-build.patch"
)

has_any_patch() {
  local any=false
  for p in "${PATCH_FILES[@]}"; do
    if [[ -f "$p" ]]; then any=true; fi
  done
  $any
}

is_applied() {
  local p="$1"
  git -C "$repo_root" apply -R --check "$p" >/dev/null 2>&1
}

can_apply() {
  local p="$1"
  git -C "$repo_root" apply --check "$p" >/dev/null 2>&1
}

cmd=${1:-status}

if ! has_any_patch; then
  echo "[cot] no patch files found (changes.patch / prompt-build.patch)" >&2
  exit 1
fi

case "$cmd" in
  status)
    for p in "${PATCH_FILES[@]}"; do
      [[ -f "$p" ]] || continue
      if is_applied "$p"; then
        echo "[cot] $(basename "$p") is applied ✅"
      else
        echo "[cot] $(basename "$p") is NOT applied ❌"
      fi
    done
    ;;
  on)
    for p in "${PATCH_FILES[@]}"; do
      [[ -f "$p" ]] || continue
      if is_applied "$p"; then
        echo "[cot] $(basename "$p") already applied ✅"
        continue
      fi
      if can_apply "$p"; then
        git -C "$repo_root" apply "$p"
        echo "[cot] applied $(basename "$p") ✅"
      else
        echo "[cot] $(basename "$p") cannot be applied cleanly. Resolve conflicts then retry." >&2
        exit 2
      fi
    done
    ;;
  off)
    for p in "${PATCH_FILES[@]}"; do
      [[ -f "$p" ]] || continue
      if is_applied "$p"; then
        git -C "$repo_root" apply -R "$p"
        echo "[cot] reverted $(basename "$p") 🔄"
      else
        echo "[cot] $(basename "$p") already not applied ✅"
      fi
    done
    ;;
  ensure)
    for p in "${PATCH_FILES[@]}"; do
      [[ -f "$p" ]] || continue
      if is_applied "$p"; then
        echo "[cot] $(basename "$p") already applied ✅"
        continue
      fi
      if can_apply "$p"; then
        git -C "$repo_root" apply "$p"
        echo "[cot] applied $(basename "$p") after merge ✅"
      else
        echo "[cot] $(basename "$p") cannot be applied cleanly after merge. Please rebase/resolve." >&2
        exit 2
      fi
    done
    ;;
  *)
    echo "Usage: $0 {status|on|off|ensure}" >&2
    exit 64
    ;;
esac
