# 01_REPO_MAP.md — agentmail-cli

## Repository Structure

```
agentmail-cli/
├── cmd/agentmail/
│   ├── main.go          # Entry point
│   └── banner.go        # CLI banner
├── pkg/cmd/             # All CLI commands (generated from OpenAPI)
│   ├── cmd.go           # Root command, flag definitions, all subcommands
│   ├── version.go       # Version constant
│   ├── cmdutil.go       # Utility functions
│   ├── flagoptions.go   # Flag option handling
│   ├── agent.go / agent_test.go
│   ├── apikey.go / apikey_test.go
│   ├── domain.go / domain_test.go
│   ├── draft.go / draft_test.go
│   ├── inbox.go / inbox_test.go
│   ├── list.go / list_test.go
│   ├── metric.go / metric_test.go
│   ├── organization.go / organization_test.go
│   ├── pod.go / pod_test.go
│   ├── podapikey.go / podapikey_test.go
│   ├── poddomain.go / poddomain_test.go
│   ├── poddraft.go / poddraft_test.go
│   ├── podinbox.go / podinbox_test.go
│   ├── podlist.go / podlist_test.go
│   ├── podmetric.go / podmetric_test.go
│   ├── podthread.go / podthread_test.go
│   ├── thread.go / thread_test.go
│   ├── webhook.go / webhook_test.go
│   └── flagoptions_test.go
├── internal/
│   ├── apiform/         # Form encoding for requests
│   ├── apiquery/        # Query encoding
│   ├── autocomplete/    # Shell completion scripts
│   │   └── shellscripts/  # bash, zsh, fish, pwsh completions
│   ├── binaryparam/     # Binary file parameter handling
│   ├── debugmiddleware/ # Debug HTTP middleware
│   ├── jsonview/        # JSON explorer/viewer
│   ├── mocktest/        # Mock testing utilities
│   └── requestflag/     # Core flag parsing (handles @file, YAML, etc.)
│       ├── requestflag.go    # Flag[T] generic type, parsing logic
│       ├── requestflag_test.go
│       ├── innerflag.go      # Inner flags for nested objects
│       └── innerflag_test.go
├── npm/                 # NPM package for distribution
│   ├── package.json     # 0.4.1, agentmail-cli
│   ├── bin/agentmail
│   ├── scripts/postinstall.js
│   └── README.md
├── .github/
│   ├── actions/setup-go/
│   └── workflows/
│       ├── ci.yml
│       ├── publish-release.yml
│       ├── release-doctor.yml
│       └── auto-merge-release.yml
├── scripts/
│   ├── bootstrap
│   ├── lint
│   ├── test
│   ├── build
│   ├── format
│   ├── run
│   ├── link
│   ├── unlink
│   └── utils/upload-artifact.sh
├── .goreleaser.yml      # Build config (macos, linux, windows; nfpms)
├── release-please-config.json
├── go.mod / go.sum
├── .stats.yml           # 94 configured endpoints
├── SKILL.md             # Skill documentation
├── README.md
├── CHANGELOG.md
├── LICENSE
├── SECURITY.md
└── .release-please-manifest.json
```

## API Resources (from cmd.go)

| Resource Group | Commands |
|---|---|
| `agent` | sign-up, verify |
| `inboxes` | create, update, list, delete, get, list-metrics |
| `inboxes:drafts` | create, update, list, delete, get, get-attachment, send |
| `inboxes:messages` | update, list, forward, get, get-attachment, get-raw, reply, reply-all, send |
| `inboxes:threads` | list, delete, get, get-attachment |
| `inboxes:lists` | create, list, delete, get |
| `inboxes:api-keys` | create, list, delete |
| `pods` | create, list, delete, get |
| `pods:domains` | create, update, list, delete, get, get-zone-file, verify |
| `pods:drafts` | list, get, get-attachment |
| `pods:inboxes` | create, update, list, delete, get |
| `pods:threads` | list, delete, get, get-attachment |
| `pods:lists` | create, list, delete, get |
| `pods:api-keys` | create, list, delete |
| `pods:metrics` | query |
| `webhooks` | create, update, list, delete, get |
| `api-keys` | create, list, delete |
| `domains` | create, update, list, delete, get, get-zone-file, verify |
| `drafts` | list, get, get-attachment |
| `lists` | create, list, delete, get |
| `metrics` | list |
| `organizations` | get |
| `threads` | list, delete, get, get-attachment |

## Key Dependencies

- `github.com/agentmail-to/agentmail-go v0.14.0` — API client
- `github.com/urfave/cli/v3 v3.3.2` — CLI framework
- `github.com/charmbracelet/bubbletea v1.3.6` — TUI
- `github.com/goccy/go-yaml v1.18.0` — YAML parsing
- `github.com/tidwall/gjson v1.18.0` — JSON query/transform

## Branches

| Branch | Description |
|---|---|
| `main` | Primary branch |
| `next` | Next version branch |
| `improve-readme` | Upstream branch for README improvement |
| `jarvis/improve-readme` | Another upstream branch (PR #20) |
| `generated` | Codegen output branch |
