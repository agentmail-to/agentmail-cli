---
name: agentmail-custom-commands
description: How to author custom commands for the agentmail CLI using the co-generated SDK.
---

# Custom Commands for `agentmail`

## Overview

The `agentmail` CLI supports user-authored custom commands that are
compiled into the binary alongside the auto-generated API commands.
Custom commands get a fully-wired SDK client that inherits the CLI's
auth, retries, TLS, base URL, and global headers — zero configuration required.

## Architecture

```
cli/agentmail/custom.rs    ← Your command handlers (protected by .fernignore)
cli/agentmail/sdk.rs       ← Generated bridge: client() + block_on()
cli/agentmail/main.rs      ← Generated entrypoint (calls custom::register)
agentmail-sdk/             ← Co-generated typed SDK crate
agentmail-types/           ← Co-generated typed model crate
```

## Adding a Custom Command

### 1. Edit `cli/agentmail/custom.rs`

This file is protected by `.fernignore` — `fern generate` will never
overwrite it. Register commands in the `register()` function:

```rust
use agentmail_sdk::api::*;

pub fn register(app: CliApp) -> CliApp {
    let app = app.command(
        clap::Command::new("get")
            .about("Get Inbox")
            .arg(clap::Arg::new("inbox_id").required(true))
        ,
        |matches, ctx| {
            let inbox_id = matches.get_one::<String>("inbox_id").unwrap();
            let client = super::sdk::client(ctx);
            let result = super::sdk::block_on(
                client.inboxes.get(inbox_id),
            )?;
            println!("{}", serde_json::to_string_pretty(&result).unwrap());
            Ok(())
        },
    );
    app
}
```

Then build and test:
```bash
cargo build
agentmail get <inbox_id>
```

### 2. Available SDK Clients

The `super::sdk::client(ctx)` call returns a `agentmail_sdk::api::Client`
with the following sub-clients:

| Field | Type | Description |
|-------|------|-------------|
| `client.inboxes` | `agentmail_sdk::api::InboxesClient` | inboxes operations |
| `client.api_keys` | `agentmail_sdk::api::ApiKeysClient2` | api_keys operations |
| `client.browser_credentials` | `agentmail_sdk::api::BrowserCredentialsClient` | browser_credentials operations |
| `client.drafts` | `agentmail_sdk::api::DraftsClient2` | drafts operations |
| `client.events` | `agentmail_sdk::api::EventsClient` | events operations |
| `client.lists` | `agentmail_sdk::api::ListsClient2` | lists operations |
| `client.messages` | `agentmail_sdk::api::MessagesClient` | messages operations |
| `client.metrics` | `agentmail_sdk::api::MetricsClient2` | metrics operations |
| `client.threads` | `agentmail_sdk::api::ThreadsClient2` | threads operations |
| `client.webhooks` | `agentmail_sdk::api::WebhooksClient2` | webhooks operations |
| `client.pods` | `agentmail_sdk::api::PodsClient` | pods operations |
| `client.api_keys` | `agentmail_sdk::api::ApiKeysClient3` | api_keys operations |
| `client.domains` | `agentmail_sdk::api::DomainsClient2` | domains operations |
| `client.drafts` | `agentmail_sdk::api::DraftsClient3` | drafts operations |
| `client.inboxes` | `agentmail_sdk::api::InboxesClient2` | inboxes operations |
| `client.lists` | `agentmail_sdk::api::ListsClient3` | lists operations |
| `client.metrics` | `agentmail_sdk::api::MetricsClient3` | metrics operations |
| `client.threads` | `agentmail_sdk::api::ThreadsClient3` | threads operations |
| `client.webhooks` | `agentmail_sdk::api::WebhooksClient3` | webhooks operations |
| `client.webhooks` | `agentmail_sdk::api::WebhooksClient` | webhooks operations |
| `client.agent` | `agentmail_sdk::api::AgentClient` | agent operations |
| `client.api_keys` | `agentmail_sdk::api::ApiKeysClient` | api_keys operations |
| `client.auth` | `agentmail_sdk::api::AuthClient` | auth operations |
| `client.domains` | `agentmail_sdk::api::DomainsClient` | domains operations |
| `client.drafts` | `agentmail_sdk::api::DraftsClient` | drafts operations |
| `client.lists` | `agentmail_sdk::api::ListsClient` | lists operations |
| `client.metrics` | `agentmail_sdk::api::MetricsClient` | metrics operations |
| `client.organizations` | `agentmail_sdk::api::OrganizationsClient` | organizations operations |
| `client.threads` | `agentmail_sdk::api::ThreadsClient` | threads operations |

### 3. Key Patterns

**Get the SDK client** (execution-sharing, fully authenticated):
```rust
let client = super::sdk::client(ctx);
```

**Run an async SDK call from a sync handler:**
```rust
let result = super::sdk::block_on(
    client.some_resource.some_method(args),
)?;
```

**Use typed models for request/response serialization:**
```rust
use agentmail_sdk::api::*;
```

### 4. Authentication

Custom commands automatically inherit the CLI's authentication.
The following auth schemes are configured:

- **BearerAuth** (bearer): env `AGENTMAIL_API_KEY`
- **TokenAuth** (bearer): env `AGENTMAIL_TOKEN`

No manual auth wiring is needed in custom command handlers.

## Regeneration Safety

| File | Regenerated? | Notes |
|------|-------------|-------|
| `cli/agentmail/custom.rs` | **No** | Protected by `.fernignore` |
| `cli/agentmail/sdk.rs` | Yes | Bridges AppContext → SDK client |
| `cli/agentmail/main.rs` | Yes | Calls `custom::register(app)` |
| `agentmail-sdk/` | Yes | Co-generated typed SDK crate |
| `agentmail-types/` | Yes | Co-generated typed models |

After running `fern generate`, your `custom.rs` is preserved. All
generated code (SDK, types, glue, main.rs) is updated to match the
latest API spec. If the SDK surface changes (renamed methods, new
sub-clients), update your `custom.rs` to match.

## Build & Test

```bash
# Build the CLI (includes custom commands)
cargo build

# Run your custom command
agentmail <your-command> [args]

# Run with verbose output for debugging
RUST_LOG=debug agentmail <your-command> [args]
```
