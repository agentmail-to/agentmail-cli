# 05_PR_CANDIDATES.md — agentmail-cli

## Open Issues

### Issue #13 — "fix: change goreleaser nfpms bindir to /usr/bin"
- **Severity**: Low (packaging fix)
- **Effort**: Very low (1-line change in `.goreleaser.yml`)
- **PR exists**: Yes — PR #13 from mtzanidakis (open)
- **Status**: Straightforward. Change `bindir: /usr` → `bindir: /usr/bin` in `.goreleaser.yml` line 71.
- **Risk**: Very low — only affects Linux package install path
- **Blocker**: None

### Issue #17 — "CLI parses string flags containing colons as YAML objects"
- **Severity**: High (functional bug — causes 400 API errors)
- **Effort**: Medium (requires understanding `requestflag.go` parsing logic)
- **PR exists**: No
- **Root cause**: In `internal/requestflag/requestflag.go`, the `parseCLIArg` function falls through to `yaml.Unmarshal` for flags typed as `any`. Valid YAML strings like `"S4: test"` get deserialized as `map[string]any{"S4": "test"}` instead of being kept as plain strings. The `allowAsLiteralString` fallback only triggers when YAML parsing *fails*.
- **Affected**: any string flag value containing a colon (`:), e.g., `--subject "S4: test"`, `--subject "BCC test"`
- **Fix approach**: Check if the YAML-parsed result is a map with a single string key before treating it as an object; if so, treat it as a literal string.
- **Risk**: Low — targeted change in parsing fallback
- **Blocker**: None

### Issue #20 — "Improve README — add badge, Homebrew install, links"
- **Severity**: Low (documentation)
- **Effort**: Very low
- **PR exists**: Yes — PR #20 from `agentmail-to:jarvis/improve-readme` (open)
- **Status**: PR already exists on upstream. README is already fairly complete with npm install instructions and usage examples.
- **Blocker**: None (already addressed by upstream PR)

## Open Pull Requests

### PR #13 — "fix: change goreleaser nfpms bindir to /usr/bin"
- From: `mtzanidakis/agentmail-cli` (fork)
- Target: `agentmail-to:main`
- Head: `main` (ff159d828d4e1c1116c461202acaa0ce35d0e287)
- Mergeable: Unknown (needs CI check)
- Status: Needs review/merge

### PR #20 — "Improve README — add badge, Homebrew install, links"
- From: `agentmail-to:jarvis/improve-readme`
- Target: `agentmail-to:main`
- Head: `187b67d10664f774f5666ae18372449274b080a9`
- Mergeable: Unknown
- Status: Needs review/merge

## Notes

- The `internal/requestflag/` code is generated from Stainless API spec (`stainless-api/stainless-api-cli`), so any fix to #17 may need to be applied upstream in the generator as well.
- Bootstrap script requires `go` binary — not available in this environment, so tests cannot be run locally.
