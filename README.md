# AgentMail CLI

The official CLI for the [AgentMail API](https://docs.agentmail.to).

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

## Environment variables

| Environment variable | Required |
| -------------------- | -------- |
| `AGENTMAIL_API_KEY`  | yes      |

## Global flags

- `--api-key` (can also be set with `AGENTMAIL_API_KEY` env var)
- `--help` - Show command line usage
- `--debug` - Enable debug logging (includes HTTP request/response details)
- `--version`, `-v` - Show the CLI version
- `--base-url` - Use a custom API backend URL
- `--format` - Change the output format (`auto`, `explore`, `json`, `jsonl`, `pretty`, `raw`, `yaml`)
- `--format-error` - Change the output format for errors (`auto`, `explore`, `json`, `jsonl`, `pretty`, `raw`, `yaml`)
- `--transform` - Transform the data output using [GJSON syntax](https://github.com/tidwall/gjson/blob/master/SYNTAX.md)
- `--transform-error` - Transform the error output using [GJSON syntax](https://github.com/tidwall/gjson/blob/master/SYNTAX.md)

### Passing files as arguments

To pass files to your API, you can use the `@myfile.ext` syntax:

```bash
agentmail <command> --arg @abe.jpg
```

Files can also be passed inside JSON or YAML blobs:

```bash
agentmail <command> --arg '{image: "@abe.jpg"}'
# Equivalent:
agentmail <command> <<YAML
arg:
  image: "@abe.jpg"
YAML
```

If you need to pass a string literal that begins with an `@` sign, you can
escape the `@` sign to avoid accidentally passing a file.

```bash
agentmail <command> --username '\@abe'
```

#### Explicit encoding

For JSON endpoints, the CLI tool does filetype sniffing to determine whether the
file contents should be sent as a string literal (for plain text files) or as a
base64-encoded string literal (for binary files). If you need to explicitly send
the file as either plain text or base64-encoded data, you can use
`@file://myfile.txt` (for string encoding) or `@data://myfile.dat` (for
base64-encoding). Note that absolute paths will begin with `@file://` or
`@data://`, followed by a third `/` (for example, `@file:///tmp/file.txt`).

```bash
agentmail <command> --arg @data://file.txt
```

## Documentation

[docs.agentmail.to](https://docs.agentmail.to)
