# Convenience justfile at repo root for CoT patch helpers

cot-status:
    bash -lc '"$(git rev-parse --show-toplevel)/scripts/cot-patch.sh" status'

cot-on:
    bash -lc '"$(git rev-parse --show-toplevel)/scripts/cot-patch.sh" on'

cot-off:
    bash -lc '"$(git rev-parse --show-toplevel)/scripts/cot-patch.sh" off'

