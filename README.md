# AgentMail CLI

[![npm](https://img.shields.io/npm/v/agentmail-cli)](https://www.npmjs.com/package/agentmail-cli)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

The official command-line interface for [AgentMail](https://agentmail.to) — the email API built for AI agents.

Manage inboxes, send and receive emails, and inspect threads directly from your terminal. Built for developers and CI/CD pipelines.

## Installation

```sh
npm install -g agentmail-cli
```

Or via Homebrew:

```sh
brew tap agentmail-to/tap
brew install agentmail-cli
```

## Setup

```sh
export AGENTMAIL_API_KEY=am_us_xxx
```

Get your API key at [console.agentmail.to](https://console.agentmail.to).

## Usage

```sh
agentmail [resource] <command> [flags...]
```

### Inboxes

```sh
# List all inboxes
agentmail inboxes list

# Create a new inbox
agentmail inboxes create --display-name "Sales Agent"

# Get inbox details
agentmail inboxes get <inbox-id>

# Delete an inbox
agentmail inboxes delete <inbox-id>
```

### Messages

```sh
# Send an email
agentmail messages send <inbox-id> \
  --to "hello@example.com" \
  --subject "Hello from my agent" \
  --body "This email was sent by an AI agent."

# List messages in an inbox
agentmail messages list <inbox-id>

# Read a specific message
agentmail messages get <inbox-id> <message-id>
```

### Threads

```sh
# List threads
agentmail threads list <inbox-id>

# View a thread
agentmail threads get <inbox-id> <thread-id>
```

## CI/CD Usage

The CLI works great in automated pipelines:

```sh
# Create an inbox for a test agent, capture the address
INBOX=$(agentmail inboxes create --display-name "CI Test Agent" --json | jq -r '.address')
echo "Agent email: $INBOX"

# Send a test email
agentmail messages send "$INBOX" --to "test@example.com" --subject "Automated test"
```

## Links

- [AgentMail Website](https://agentmail.to)
- [API Documentation](https://docs.agentmail.to)
- [Python SDK](https://github.com/agentmail-to/agentmail-python)
- [TypeScript SDK](https://github.com/agentmail-to/agentmail-node)
- [MCP Server](https://github.com/agentmail-to/agentmail-mcp)

## License

MIT
