# 06_SELECTED_5_PR_PLAN.md — agentmail-cli

## Selected Candidates (Top 5)

Given the available issues and PRs, the following are ranked by impact and ease:

### 1. Issue #13 — Fix goreleaser nfpms bindir (`/usr` → `/usr/bin`)
- **File**: `.goreleaser.yml` line 71
- **Change**: `bindir: /usr` → `bindir: /usr/bin`
- **Why**: Straightforward, low-risk, already has PR #13 from mtzanidakis that can be reviewed/merged
- **Impact**: Ensures Linux packages (deb/rpm/apk) install binary to `/usr/bin` instead of `/usr`

### 2. Issue #17 — Fix colon-in-string YAML parsing bug
- **File**: `internal/requestflag/requestflag.go`
- **Change**: In `parseCLIArg`, detect when a YAML-parsed map has a single string key and treat as literal string instead of object
- **Why**: High severity bug causing API 400 errors for legitimate subject lines and other string fields
- **Impact**: Fixes CLI usability for any user whose data contains colons (e.g., "Re: S4: test", "BCC:", etc.)
- **Note**: Generated code may need corresponding fix upstream in Stainless CLI generator

### 3. PR #20 Review — README improvement
- **Action**: Review and potentially merge existing upstream PR #20 (`jarvis/improve-readme`)
- **Changes**: npm badge, better description, Homebrew install, links
- **Why**: Low effort, improves first-impression and installation options
- **Note**: Already exists as upstream PR — no new PR needed, just needs review/merge from maintainers

## What Was Not Selected

- **Issue #20 (as new PR)**: Already addressed by existing PR #20 from upstream branch. Not worth duplicating effort.
- **Codegen/generated code**: The repository is auto-generated from OpenAPI via Stainless. Direct modifications to generated files (`pkg/cmd/*.go`) would be overwritten by codegen. Changes should go through the Stainless pipeline.
- **NPM packaging**: Works correctly; no issues filed.

## Execution Plan

1. **For Issue #13**: Apply 1-line fix to `.goreleaser.yml` locally, commit, push to `okwn/agentmail-cli`, open PR targeting `agentmail-to:main`
2. **For Issue #17**: Study `parseCLIArg` fallback logic in `internal/requestflag/requestflag.go`, implement fix, add test case in `requestflag_test.go`, commit, push, open PR
3. **For PR #20**: Since it's already an upstream PR, assist by reviewing/testing the existing changes rather than duplicating

## Environment Notes

- `go` binary NOT available — cannot run `bootstrap`, `test`, or `lint` locally
- Tests must be validated via CI after PR is opened (CI runs on fork PRs)
- All Go code is syntactically parseable at least
