---
name: lint
description: "Run the full 4-step verification pipeline: fmt, clippy, test, doc. Manual: invoke with /lint."
---

# /lint - Full Verification Pipeline

Run all four checks in sequence. Stop on first failure.

## Steps

1. **Format check**
   ```bash
   cargo fmt -- --check
   ```
   If it fails, run `cargo fmt` to fix, then re-check.

2. **Clippy lint**
   ```bash
   cargo lint
   ```
   This is an alias for `cargo clippy --all-targets -- -D warnings`.
   Show the first 3 errors if it fails.

3. **Test**
   ```bash
   cargo ta
   ```
   This is an alias for `cargo test --all-targets`.
   Show failing test names and first assertion failure.

4. **Doc check**
   ```bash
   cargo doc --no-deps 2>&1
   ```
   Check for broken doc links or missing types.

## Output

Report results as a summary table:

```
| Step    | Result |
|---------|--------|
| fmt     | pass   |
| clippy  | pass   |
| test    | pass   |
| doc     | pass   |
```

On failure, show the step that failed, first 3 errors, and stop. Do not continue to later steps.
