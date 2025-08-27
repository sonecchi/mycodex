CoT visibility patch workflow
=============================

Purpose
-------
- Keep the standard TUI view showing live reasoning deltas ("thinking") even after pulling upstream.

What’s included
---------------
- `changes.patch`: the diff that enables streaming reasoning into the normal view and updates snapshots.
- `scripts/cot-patch.sh`: helper to apply/revert/check the patch.
- `.githooks/post-merge`: auto-reapply the patch after `git merge`/`git pull`.
- `just` tasks: `cot-on`, `cot-off`, `cot-status`.

Setup (one-time)
----------------
1) Enable repo-local hooks:

```
git config core.hooksPath .githooks
```

2) Make sure the script is executable (checked in as such):

```
chmod +x scripts/cot-patch.sh .githooks/post-merge
```

Daily usage
-----------
- Check status:

```
just cot-status
```

- Apply (turn CoT on in standard view):

```
just cot-on
```

- Revert (back to upstream behavior):

```
just cot-off
```

Notes
-----
- If the patch cannot be applied cleanly after an upstream update, rebase and resolve, then re-run `just cot-on`.
- For longer-term maintainability, consider upstreaming a config gate like `tui.thinking_visibility = "auto|always|never"` so local preference can be toggled without code changes.

