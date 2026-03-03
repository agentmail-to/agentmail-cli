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
agentmail inboxes list
agentmail inboxes create --display-name "My Inbox"
agentmail inboxes:messages send --inbox-id inb_xxx --to user@example.com --subject "Hello" --text "Hi"
agentmail inboxes:threads list --inbox-id inb_xxx
```

Run `agentmail --help` to see all available commands.

## Documentation

[docs.agentmail.to](https://docs.agentmail.to)
