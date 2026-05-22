# 00_STATE.md — agentmail-cli

## Repository Status

| Field | Value |
| ----- | ----- |
| **Upstream** | agentmail-to/agentmail-cli |
| **Fork** | okwn/agentmail-cli (cloned to /root/oss-pr-campaign/repos/agentmail-cli) |
| **Archived** | false |
| **License** | Apache-2.0 |
| **Language** | Go |
| **Default Branch** | main |
| **Stars** | 21 |
| **Forks** | 7 |
| **Open Issues** | 3 |
| **Open PRs** | 2 |

## Current Branch

- Local: `main`
- Upstream: `upstream/main`
- All remotes: `origin/*` (fork), `upstream/*`

## CI Status

- `.github/workflows/ci.yml` — lint, build, test jobs on push/PR
- `.github/workflows/publish-release.yml`
- `.github/workflows/release-doctor.yml`
- `.github/workflows/auto-merge-release.yml`
- Go 1.25, depot-ubuntu-24.04 or ubuntu-latest

## Key Scripts

- `./scripts/bootstrap` — installs Go deps (needs `go` binary)
- `./scripts/lint` — runs lints
- `./scripts/test` — runs `go test ./...` + Windows cross-compile test
- `./scripts/build` — runs goreleaser
- `./scripts/format`, `./scripts/run`, `./scripts/link`, `./scripts/unlink`

## Go Module

- Module: `github.com/agentmail-to/agentmail-cli`
- Go version: `1.25`
- Private deps: `github.com/agentmail-to/agentmail-go`, `github.com/stainless-sdks/agentmail-go`
- NPM package: `agentmail-cli@0.4.1` (published to npm)

## Release

- Release-please configured (`release-please-config.json`)
- Goreleaser configured (`.goreleaser.yml`)
- Version tracked in: `pkg/cmd/version.go`, `README.md`
- Extra files for release-please: `pkg/cmd/version.go`, `README.md`

## Build/Release Notes

- Bootstrap requires `go` binary in PATH (not available in this environment)
- npm `postinstall` script present at `npm/scripts/postinstall.js`
- Binary published via npm (`./npm/bin/agentmail`)

## Environment

- `go` binary NOT available in this environment — cannot run tests/bootstrap
- GitHub CLI (`gh`) available for API queries
- Python not noted as required
