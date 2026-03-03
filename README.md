# AgentMail CLI

The official CLI for the [AgentMail API](https://docs.agentmail.to).

## Installation

```sh
npm install -g @agentmail/cli
```

## Setup

```sh
export AGENTMAIL_API_KEY=sk_xxx
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

## Documentation

[docs.agentmail.to](https://docs.agentmail.to)
