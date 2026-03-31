# AgentMail CLI

[![npm](https://img.shields.io/npm/v/agentmail-cli)](https://www.npmjs.com/package/agentmail-cli)

The official CLI for [AgentMail](https://agentmail.to) — the email API for AI agents. Manage inboxes, send and receive email, and inspect threads from the terminal.

## Installation

```sh
npm install -g agentmail-cli
```

## Setup

```sh
export AGENTMAIL_API_KEY=am_us_xxx
```

## Usage

```sh
agentmail [resource] <command> [flags...]
```

```sh
# List inboxes
agentmail inboxes list

# Create an inbox
agentmail inboxes create --display-name "My Inbox"

# Send a message
agentmail inboxes:messages send \
  --inbox-id inb_xxx \
  --to user@example.com \
  --subject "Hello" \
  --text "Hi there"

# List threads
agentmail inboxes:threads list --inbox-id inb_xxx
```

Use `--help` on any command for details.

## Global flags

| Flag | Description |
| --- | --- |
| `--api-key` | API key (or set `AGENTMAIL_API_KEY`) |
| `--format` | Output format: `json`, `yaml`, `pretty`, `raw`, `explore` |
| `--debug` | Enable debug logging |
| `--help` | Show help |
| `--version` | Show version |

### Install via Homebrew

```sh
brew install agentmail-to/tap/agentmail
```

## Links

- [AgentMail](https://agentmail.to) — The email API for AI agents
- [Documentation](https://docs.agentmail.to)
- [Python SDK](https://github.com/agentmail-to/agentmail-python)
- [TypeScript SDK](https://github.com/agentmail-to/agentmail-node)
- [MCP Server](https://github.com/agentmail-to/agentmail-mcp)
- [Discord](https://discord.gg/ZYN7f7KPjS)
