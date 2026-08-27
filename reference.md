# AgentMail CLI Reference

Full command reference for `agentmail`.

## Commands

- [`agentmail agent`](#agentmail-agent)
- [`agentmail api-keys`](#agentmail-api-keys)
- [`agentmail auth`](#agentmail-auth)
- [`agentmail domains`](#agentmail-domains)
- [`agentmail drafts`](#agentmail-drafts)
- [`agentmail inboxes`](#agentmail-inboxes)
- [`agentmail inboxes api-keys`](#agentmail-inboxes-api-keys)
- [`agentmail inboxes browser-credentials`](#agentmail-inboxes-browser-credentials)
- [`agentmail inboxes drafts`](#agentmail-inboxes-drafts)
- [`agentmail inboxes events`](#agentmail-inboxes-events)
- [`agentmail inboxes lists`](#agentmail-inboxes-lists)
- [`agentmail inboxes messages`](#agentmail-inboxes-messages)
- [`agentmail inboxes metrics`](#agentmail-inboxes-metrics)
- [`agentmail inboxes threads`](#agentmail-inboxes-threads)
- [`agentmail inboxes webhooks`](#agentmail-inboxes-webhooks)
- [`agentmail lists`](#agentmail-lists)
- [`agentmail metrics`](#agentmail-metrics)
- [`agentmail organizations`](#agentmail-organizations)
- [`agentmail pods`](#agentmail-pods)
- [`agentmail pods api-keys`](#agentmail-pods-api-keys)
- [`agentmail pods domains`](#agentmail-pods-domains)
- [`agentmail pods drafts`](#agentmail-pods-drafts)
- [`agentmail pods inboxes`](#agentmail-pods-inboxes)
- [`agentmail pods lists`](#agentmail-pods-lists)
- [`agentmail pods metrics`](#agentmail-pods-metrics)
- [`agentmail pods threads`](#agentmail-pods-threads)
- [`agentmail pods webhooks`](#agentmail-pods-webhooks)
- [`agentmail threads`](#agentmail-threads)
- [`agentmail webhooks`](#agentmail-webhooks)

---

### `agentmail agent`

#### `agentmail agent sign-up`

Create a new agent organization with an inbox and API key. This endpoint is for signing up for the first time. If you've already signed up, you're all set — just use your existing API key.

A 6-digit OTP is sent to the human's email for verification.

This endpoint is idempotent. Calling it again with the same `human_email` will rotate the API key and resend the OTP if expired.

The returned API key has limited permissions until the organization is verified via the verify endpoint.

**CLI:**
```bash
agentmail agent sign-up --human-email user@example.com --username my-agent
```

`POST /v0/agent/sign-up`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `agentmail agent verify`

Verify an agent organization using the 6-digit OTP sent to the human's email during sign-up.

On success, the organization is upgraded from `agent_unverified` to `agent_verified`, the send allowlist is removed, and free plan entitlements are applied.

The OTP expires after 24 hours and allows a maximum of 10 attempts. If you run into any difficulties receiving the OTP code, you can also create an account on [console.agentmail.to](https://console.agentmail.to) using the human email address you provided to verify your account.

**CLI:**
```bash
agentmail agent verify --otp-code 123456
```

`POST /v0/agent/verify`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `agentmail api-keys`

#### `agentmail api-keys cancel-browser-enrollment`

Cancel one pending, unexpired browser enrollment intent. Requires `api_key_delete`.

`DELETE /v0/api-keys/browser-credentials/enrollments/{enrollment_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--enrollment-id` | `string (uuid)` | Yes |  |

#### `agentmail api-keys create`

**CLI:**
```bash
agentmail api-keys create --name "My Key"
```

`POST /v0/api-keys`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `agentmail api-keys create-public-key`

Register a public P-256 JWK using an existing AgentMail bearer API key
with `api_key_create`. Re-registering the same JWK creates a new
credential ID; it does not replace or recover an earlier credential.
The private key must never be sent to AgentMail.

`POST /v0/api-keys/public-keys`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `agentmail api-keys delete`

**CLI:**
```bash
agentmail api-keys delete --api-key-id <api_key_id>
```

`DELETE /v0/api-keys/{api_key_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--api-key-id` | `ApiKeyId` | Yes |  |

#### `agentmail api-keys delete-browser-consent`

Revoke one remembered AgentID client approval. Requires `api_key_delete`.

`DELETE /v0/api-keys/browser-consents/{consent_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--consent-id` | `string` | Yes |  |

#### `agentmail api-keys delete-browser-credential`

Permanently revoke one active browser credential. Requires `api_key_delete`.

`DELETE /v0/api-keys/browser-credentials/{credential_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--credential-id` | `string (uuid)` | Yes |  |

#### `agentmail api-keys list`

**CLI:**
```bash
agentmail api-keys list
```

`GET /v0/api-keys`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--limit` | `Limit` | No |  |
| `--page-token` | `PageToken` | No |  |
| `--ascending` | `Ascending` | No |  |

#### `agentmail api-keys list-browser-consents`

List remembered AgentID client approvals for one live inbox. Requires `api_key_read`.

`GET /v0/api-keys/browser-consents`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--inbox-id` | `string (email)` | Yes |  |
| `--limit` | `BrowserAuthorizationListLimit` | No |  |
| `--page-token` | `PageToken` | No |  |

#### `agentmail api-keys list-browser-credential-events`

List owner-facing browser credential and consent lifecycle events. Requires `api_key_read`.

`GET /v0/api-keys/browser-credentials/events`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--limit` | `BrowserAuthorizationListLimit` | No |  |
| `--page-token` | `PageToken` | No |  |

#### `agentmail api-keys list-browser-credentials`

List active browser credentials visible to the caller's scope. Requires `api_key_read`.

`GET /v0/api-keys/browser-credentials`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--limit` | `BrowserAuthorizationListLimit` | No |  |
| `--page-token` | `PageToken` | No |  |

#### `agentmail api-keys list-public-keys`

List only public-key credentials visible to the bearer caller's scope.
Bearer credentials are never returned, even though both credential types
share storage and pagination indexes. Requires `api_key_read`.

`GET /v0/api-keys/public-keys`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--limit` | `Limit` | No |  |
| `--page-token` | `PageToken` | No |  |
| `--ascending` | `Ascending` | No |  |

#### `agentmail api-keys revoke-all-agent-id-sign-in-keys`

Invalidate every current public-key credential in the caller's
organization by advancing its AgentID key generation. The caller must be
organization-scoped and either have `api_key_delete` or, for a verified
self-serve agent organization, use an unrestricted unmanaged bearer
credential. No request body is accepted.

`Idempotency-Key` is required and must be a UUID. Reusing the same UUID
returns the original permanent receipt without advancing the generation
again. A new UUID performs a new generation advance.

`POST /v0/api-keys/public-keys/agentid-sign-in/revoke-all`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--idempotency-key` | `string (uuid)` | Yes | Required UUID identifying this revoke-all operation permanently. |

#### `agentmail api-keys revoke-public-key`

Permanently revoke one public-key credential. This hard-deletes the
credential; repeating the request returns not found. Requires
`api_key_delete`.

`DELETE /v0/api-keys/public-keys/{api_key_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--api-key-id` | `string (uuid)` | Yes | Public-key credential ID returned by registration. |

#### `agentmail api-keys update-public-key-name`

Rename the credential. All security-relevant fields are immutable.
Requires `api_key_update`.

`PATCH /v0/api-keys/public-keys/{api_key_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--api-key-id` | `string (uuid)` | Yes | Public-key credential ID returned by registration. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `agentmail auth`

#### `agentmail auth me`

Returns the identity and scope of the authenticated credential. Useful when a client holds a pod-scoped or inbox-scoped API key and needs to discover the parent organization, pod, or inbox without prior knowledge.

**CLI:**
```bash
agentmail auth me
```

`GET /v0/auth/me`

---

### `agentmail domains`

#### `agentmail domains create`

**CLI:**
```bash
agentmail domains create --domain example.com
```

`POST /v0/domains`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `agentmail domains delete`

**CLI:**
```bash
agentmail domains delete --domain-id <domain_id>
```

`DELETE /v0/domains/{domain_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--domain-id` | `DomainId` | Yes |  |

#### `agentmail domains get`

**CLI:**
```bash
agentmail domains get --domain-id <domain_id>
```

`GET /v0/domains/{domain_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--domain-id` | `DomainId` | Yes |  |

#### `agentmail domains get-zone-file`

**CLI:**
```bash
agentmail domains get-zone-file --domain-id <domain_id>
```

`GET /v0/domains/{domain_id}/zone-file`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--domain-id` | `DomainId` | Yes |  |

#### `agentmail domains list`

**CLI:**
```bash
agentmail domains list
```

`GET /v0/domains`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--limit` | `Limit` | No |  |
| `--page-token` | `PageToken` | No |  |
| `--ascending` | `Ascending` | No |  |

#### `agentmail domains update`

**CLI:**
```bash
agentmail domains update --domain-id <domain_id>
```

`PATCH /v0/domains/{domain_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--domain-id` | `DomainId` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `agentmail domains verify`

**CLI:**
```bash
agentmail domains verify --domain-id <domain_id>
```

`POST /v0/domains/{domain_id}/verify`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--domain-id` | `DomainId` | Yes |  |

---

### `agentmail drafts`

#### `agentmail drafts get`

**CLI:**
```bash
agentmail drafts get --draft-id <draft_id>
```

`GET /v0/drafts/{draft_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--draft-id` | `DraftId` | Yes |  |

#### `agentmail drafts get-attachment`

**CLI:**
```bash
agentmail drafts get-attachment --draft-id <draft_id> --attachment-id <attachment_id>
```

`GET /v0/drafts/{draft_id}/attachments/{attachment_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--draft-id` | `DraftId` | Yes |  |
| `--attachment-id` | `AttachmentId` | Yes |  |

#### `agentmail drafts list`

**CLI:**
```bash
agentmail drafts list
```

`GET /v0/drafts`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--limit` | `Limit` | No |  |
| `--page-token` | `PageToken` | No |  |
| `--labels` | `Labels` | No |  |
| `--before` | `Before` | No |  |
| `--after` | `After` | No |  |
| `--ascending` | `Ascending` | No |  |

---

### `agentmail inboxes`

#### `agentmail inboxes create`

**CLI:**
```bash
agentmail inboxes create --display-name "My Agent" --username myagent --domain agentmail.to
```

`POST /v0/inboxes`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `agentmail inboxes delete`

**CLI:**
```bash
agentmail inboxes delete --inbox-id <inbox_id>
```

`DELETE /v0/inboxes/{inbox_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--inbox-id` | `inboxesInboxId` | Yes |  |

#### `agentmail inboxes get`

**CLI:**
```bash
agentmail inboxes get --inbox-id <inbox_id>
```

`GET /v0/inboxes/{inbox_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--inbox-id` | `inboxesInboxId` | Yes |  |

#### `agentmail inboxes list`

**CLI:**
```bash
agentmail inboxes list
```

`GET /v0/inboxes`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--limit` | `Limit` | No |  |
| `--page-token` | `PageToken` | No |  |
| `--ascending` | `Ascending` | No |  |

#### `agentmail inboxes update`

**CLI:**
```bash
agentmail inboxes update --inbox-id <inbox_id> --display-name "Updated Name"
```

`PATCH /v0/inboxes/{inbox_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--inbox-id` | `inboxesInboxId` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `agentmail inboxes api-keys`

#### `agentmail inboxes api-keys create`

**CLI:**
```bash
agentmail inboxes api-keys create --inbox-id <inbox_id> --name "My Key"
```

`POST /v0/inboxes/{inbox_id}/api-keys`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--inbox-id` | `inboxesInboxId` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `agentmail inboxes api-keys delete`

**CLI:**
```bash
agentmail inboxes api-keys delete --inbox-id <inbox_id> --api-key-id <api_key_id>
```

`DELETE /v0/inboxes/{inbox_id}/api-keys/{api_key_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--inbox-id` | `inboxesInboxId` | Yes |  |
| `--api-key-id` | `ApiKeyId` | Yes |  |

#### `agentmail inboxes api-keys list`

**CLI:**
```bash
agentmail inboxes api-keys list --inbox-id <inbox_id>
```

`GET /v0/inboxes/{inbox_id}/api-keys`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--inbox-id` | `inboxesInboxId` | Yes |  |
| `--limit` | `Limit` | No |  |
| `--page-token` | `PageToken` | No |  |

---

### `agentmail inboxes browser-credentials`

#### `agentmail inboxes browser-credentials create-enrollment`

Attach a browser enrollment intent to the inbox. Requires
`api_key_create`. Before submitting `transaction_jti`, independently
verify that the browser page's final origin is exactly
`https://auth.agentid.com`.

This endpoint is available to every organization using US production.
It is not available in EU production.

Select `inbox_id` from trusted AgentMail configuration. An AgentID
`login_hint` is not authoritative for selecting the inbox; when the
transaction includes one, it must match the path inbox.

**AgentMail API keys are sent only to `https://api.agentmail.to`; AgentID never requests them.**

A new intent returns `202`; an idempotent retry for the same pending
transaction, inbox, and bearer key returns `200` with the same receipt.
An intent lasts at most five minutes. An activated credential lasts at
most 30 days and cannot outlive its authorizing bearer API key.

Creation is limited to 20 intents per bearer API key per hour, 100 per
organization per hour, and five live unused intents per bearer API key.
Browser activation is separately limited to 20 activations per
authorizing bearer API key per UTC day. Either kind of limit can return
`429`; honor the `Retry-After` header. Cancelling an enrollment releases
its live-intent slot but does not reset the daily activation counter.

`POST /v0/inboxes/{inbox_id}/browser-credentials/enrollments`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--inbox-id` | `inboxesInboxId` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `agentmail inboxes drafts`

#### `agentmail inboxes drafts create`

Create a draft. Supply `in_reply_to` to create a reply draft (with
`reply_all` to address the whole thread), whose recipients, subject, and
threading are derived from the referenced message, or `forward_of` to
create a forward draft, which derives the subject, threading, and
forwarded content from the source but keeps recipients caller-supplied.

**CLI:**
```bash
agentmail inboxes drafts create --inbox-id <inbox_id> --to recipient@example.com --subject "Draft subject" --text "Draft body"
```

`POST /v0/inboxes/{inbox_id}/drafts`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--inbox-id` | `inboxesInboxId` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `agentmail inboxes drafts delete`

**CLI:**
```bash
agentmail inboxes drafts delete --inbox-id <inbox_id> --draft-id <draft_id>
```

`DELETE /v0/inboxes/{inbox_id}/drafts/{draft_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--inbox-id` | `inboxesInboxId` | Yes |  |
| `--draft-id` | `DraftId` | Yes |  |

#### `agentmail inboxes drafts get`

**CLI:**
```bash
agentmail inboxes drafts get --inbox-id <inbox_id> --draft-id <draft_id>
```

`GET /v0/inboxes/{inbox_id}/drafts/{draft_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--inbox-id` | `inboxesInboxId` | Yes |  |
| `--draft-id` | `DraftId` | Yes |  |

#### `agentmail inboxes drafts get-attachment`

**CLI:**
```bash
agentmail inboxes drafts get-attachment --inbox-id <inbox_id> --draft-id <draft_id> --attachment-id <attachment_id>
```

`GET /v0/inboxes/{inbox_id}/drafts/{draft_id}/attachments/{attachment_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--inbox-id` | `inboxesInboxId` | Yes |  |
| `--draft-id` | `DraftId` | Yes |  |
| `--attachment-id` | `AttachmentId` | Yes |  |

#### `agentmail inboxes drafts list`

**CLI:**
```bash
agentmail inboxes drafts list --inbox-id <inbox_id>
```

`GET /v0/inboxes/{inbox_id}/drafts`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--inbox-id` | `inboxesInboxId` | Yes |  |
| `--limit` | `Limit` | No |  |
| `--page-token` | `PageToken` | No |  |
| `--labels` | `Labels` | No |  |
| `--before` | `Before` | No |  |
| `--after` | `After` | No |  |
| `--ascending` | `Ascending` | No |  |

#### `agentmail inboxes drafts send`

**CLI:**
```bash
agentmail inboxes drafts send --inbox-id <inbox_id> --draft-id <draft_id>
```

`POST /v0/inboxes/{inbox_id}/drafts/{draft_id}/send`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--inbox-id` | `inboxesInboxId` | Yes |  |
| `--draft-id` | `DraftId` | Yes |  |
| `--idempotency-key` | `string` | No | Unique key that makes a send idempotent. A retry carrying the same key returns the original message instead of sending a second email; reusing a key with a different request returns a 409 conflict. Keys expire 24 hours after the send completes. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `agentmail inboxes drafts update`

Edit fields on an existing draft. Passing `null` clears a field (or `[]`
for a recipient field); `send_at: null` un-schedules a scheduled draft.
A draft that is already being sent cannot be edited.

**CLI:**
```bash
agentmail inboxes drafts update --inbox-id <inbox_id> --draft-id <draft_id> --subject "Updated subject"
```

`PATCH /v0/inboxes/{inbox_id}/drafts/{draft_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--inbox-id` | `inboxesInboxId` | Yes |  |
| `--draft-id` | `DraftId` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `agentmail inboxes events`

#### `agentmail inboxes events list`

List label change events for an inbox. Returns events in reverse chronological order by default. Use for IMAP UID projection or audit logging.

**CLI:**
```bash
agentmail inboxes events list --inbox-id <inbox_id>
```

`GET /v0/inboxes/{inbox_id}/events`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--inbox-id` | `inboxesInboxId` | Yes |  |
| `--limit` | `Limit` | No |  |
| `--page-token` | `PageToken` | No |  |
| `--ascending` | `Ascending` | No |  |

---

### `agentmail inboxes lists`

#### `agentmail inboxes lists create`

**CLI:**
```bash
agentmail inboxes lists create --inbox-id <inbox_id> --direction <direction> --type <type> --entry user@example.com
```

`POST /v0/inboxes/{inbox_id}/lists/{direction}/{type}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--inbox-id` | `inboxesInboxId` | Yes |  |
| `--direction` | `Direction` | Yes |  |
| `--type` | `ListType` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `agentmail inboxes lists delete`

**CLI:**
```bash
agentmail inboxes lists delete --inbox-id <inbox_id> --direction <direction> --type <type> --entry <entry>
```

`DELETE /v0/inboxes/{inbox_id}/lists/{direction}/{type}/{entry}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--inbox-id` | `inboxesInboxId` | Yes |  |
| `--direction` | `Direction` | Yes |  |
| `--type` | `ListType` | Yes |  |
| `--entry` | `string` | Yes | Email address or domain. |

#### `agentmail inboxes lists get`

**CLI:**
```bash
agentmail inboxes lists get --inbox-id <inbox_id> --direction <direction> --type <type> --entry <entry>
```

`GET /v0/inboxes/{inbox_id}/lists/{direction}/{type}/{entry}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--inbox-id` | `inboxesInboxId` | Yes |  |
| `--direction` | `Direction` | Yes |  |
| `--type` | `ListType` | Yes |  |
| `--entry` | `string` | Yes | Email address or domain. |

#### `agentmail inboxes lists list`

**CLI:**
```bash
agentmail inboxes lists list --inbox-id <inbox_id> --direction <direction> --type <type>
```

`GET /v0/inboxes/{inbox_id}/lists/{direction}/{type}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--inbox-id` | `inboxesInboxId` | Yes |  |
| `--direction` | `Direction` | Yes |  |
| `--type` | `ListType` | Yes |  |
| `--limit` | `Limit` | No |  |
| `--page-token` | `PageToken` | No |  |

---

### `agentmail inboxes messages`

#### `agentmail inboxes messages batch-get`

Fetch metadata for up to 500 messages in one request. Missing or
restricted IDs are silently omitted; compare `count` against `limit`
to detect misses.

**CLI:**
```bash
agentmail inboxes messages batch-get --inbox-id <inbox_id> --message-ids <id1> --message-ids <id2>
```

`POST /v0/inboxes/{inbox_id}/messages/batch-get`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--inbox-id` | `inboxesInboxId` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `agentmail inboxes messages batch-update`

Apply one label change to up to 50 messages in a single request. The
same add_labels and remove_labels apply to every message id, and at
least one of them must be provided. The update is atomic: either all
resolved messages are updated or none are. Missing or restricted ids
are silently excluded; compare `count` against `limit` to detect
exclusions.

**CLI:**
```bash
agentmail inboxes messages batch-update --inbox-id <inbox_id> --message-ids <id1> --message-ids <id2> --add-labels read --remove-labels unread
```

`POST /v0/inboxes/{inbox_id}/messages/batch-update`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--inbox-id` | `inboxesInboxId` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `agentmail inboxes messages delete`

Permanently deletes a message.

**CLI:**
```bash
agentmail inboxes messages delete --inbox-id <inbox_id> --message-id <message_id>
```

`DELETE /v0/inboxes/{inbox_id}/messages/{message_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--inbox-id` | `inboxesInboxId` | Yes |  |
| `--message-id` | `MessageId` | Yes |  |

#### `agentmail inboxes messages forward`

**CLI:**
```bash
agentmail inboxes messages forward --inbox-id <inbox_id> --message-id <message_id> --to recipient@example.com
```

`POST /v0/inboxes/{inbox_id}/messages/{message_id}/forward`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--inbox-id` | `inboxesInboxId` | Yes |  |
| `--message-id` | `MessageId` | Yes |  |
| `--idempotency-key` | `string` | No | Unique key that makes a send idempotent. A retry carrying the same key returns the original message instead of sending a second email; reusing a key with a different request returns a 409 conflict. Keys expire 24 hours after the send completes. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `agentmail inboxes messages get`

**CLI:**
```bash
agentmail inboxes messages get --inbox-id <inbox_id> --message-id <message_id>
```

`GET /v0/inboxes/{inbox_id}/messages/{message_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--inbox-id` | `inboxesInboxId` | Yes |  |
| `--message-id` | `MessageId` | Yes |  |

#### `agentmail inboxes messages get-attachment`

**CLI:**
```bash
agentmail inboxes messages get-attachment --inbox-id <inbox_id> --message-id <message_id> --attachment-id <attachment_id>
```

`GET /v0/inboxes/{inbox_id}/messages/{message_id}/attachments/{attachment_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--inbox-id` | `inboxesInboxId` | Yes |  |
| `--message-id` | `MessageId` | Yes |  |
| `--attachment-id` | `AttachmentId` | Yes |  |

#### `agentmail inboxes messages get-raw`

**CLI:**
```bash
agentmail inboxes messages get-raw --inbox-id <inbox_id> --message-id <message_id>
```

`GET /v0/inboxes/{inbox_id}/messages/{message_id}/raw`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--inbox-id` | `inboxesInboxId` | Yes |  |
| `--message-id` | `MessageId` | Yes |  |

#### `agentmail inboxes messages list`

Lists messages in the inbox, most recent first. Pass `from`, `to`, or
`subject` to filter by substring. Filtered requests are served by
search, which caps `limit` at 100. For relevance-ranked full-text
search across sender, recipients, subject, and message body, use
`Search Messages`.

**CLI:**
```bash
agentmail inboxes messages list --inbox-id <inbox_id>
```

`GET /v0/inboxes/{inbox_id}/messages`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--inbox-id` | `inboxesInboxId` | Yes |  |
| `--limit` | `Limit` | No |  |
| `--page-token` | `PageToken` | No |  |
| `--labels` | `Labels` | No |  |
| `--before` | `Before` | No |  |
| `--after` | `After` | No |  |
| `--ascending` | `Ascending` | No |  |
| `--include-spam` | `IncludeSpam` | No |  |
| `--include-blocked` | `IncludeBlocked` | No |  |
| `--include-unauthenticated` | `IncludeUnauthenticated` | No |  |
| `--include-trash` | `IncludeTrash` | No |  |
| `--from` | `string[]` | No | Filter to messages whose sender contains this value (substring match). Repeatable; all values must match. |
| `--to` | `string[]` | No | Filter to messages whose recipients (to, cc, or bcc) contain this value (substring match). Repeatable; all values must match. |
| `--subject` | `string[]` | No | Filter to messages whose subject contains this value (substring match). Repeatable; all values must match. |

#### `agentmail inboxes messages reply`

**CLI:**
```bash
agentmail inboxes messages reply --inbox-id <inbox_id> --message-id <message_id> --text "Reply text"
```

`POST /v0/inboxes/{inbox_id}/messages/{message_id}/reply`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--inbox-id` | `inboxesInboxId` | Yes |  |
| `--message-id` | `MessageId` | Yes |  |
| `--idempotency-key` | `string` | No | Unique key that makes a send idempotent. A retry carrying the same key returns the original message instead of sending a second email; reusing a key with a different request returns a 409 conflict. Keys expire 24 hours after the send completes. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `agentmail inboxes messages reply-all`

**CLI:**
```bash
agentmail inboxes messages reply-all --inbox-id <inbox_id> --message-id <message_id> --text "Reply text"
```

`POST /v0/inboxes/{inbox_id}/messages/{message_id}/reply-all`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--inbox-id` | `inboxesInboxId` | Yes |  |
| `--message-id` | `MessageId` | Yes |  |
| `--idempotency-key` | `string` | No | Unique key that makes a send idempotent. A retry carrying the same key returns the original message instead of sending a second email; reusing a key with a different request returns a 409 conflict. Keys expire 24 hours after the send completes. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `agentmail inboxes messages search`

Full-text search across messages in the inbox, ranked by relevance. The
query is matched against the sender, recipients, and subject (substring)
and the message body (tokenized full text). Spam, trash, blocked, and
unauthenticated messages are always excluded. `limit` cannot exceed 100.

`GET /v0/inboxes/{inbox_id}/messages/search`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--inbox-id` | `inboxesInboxId` | Yes |  |
| `--q` | `Query` | Yes |  |
| `--limit` | `Limit` | No |  |
| `--page-token` | `PageToken` | No |  |
| `--before` | `Before` | No |  |
| `--after` | `After` | No |  |

#### `agentmail inboxes messages send`

**CLI:**
```bash
agentmail inboxes messages send --inbox-id <inbox_id> --to recipient@example.com --subject "Hello" --text "Body"
```

`POST /v0/inboxes/{inbox_id}/messages/send`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--inbox-id` | `inboxesInboxId` | Yes |  |
| `--idempotency-key` | `string` | No | Unique key that makes a send idempotent. A retry carrying the same key returns the original message instead of sending a second email; reusing a key with a different request returns a 409 conflict. Keys expire 24 hours after the send completes. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `agentmail inboxes messages update`

**CLI:**
```bash
agentmail inboxes messages update --inbox-id <inbox_id> --message-id <message_id> --add-labels read --remove-labels unread
```

`PATCH /v0/inboxes/{inbox_id}/messages/{message_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--inbox-id` | `inboxesInboxId` | Yes |  |
| `--message-id` | `MessageId` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `agentmail inboxes metrics`

#### `agentmail inboxes metrics query-events`

Counts of email events (sent, delivered, bounced, etc.) over time for
the inbox. Defaults to the last 24 hours; `start` must be within the
last 90 days, and a future `end` is clamped to now. Omit `period` for
individual event counts, or set it to sum counts into buckets of that
many seconds.

**CLI:**
```bash
agentmail inboxes metrics query-events --inbox-id <inbox_id>
```

`GET /v0/inboxes/{inbox_id}/metrics/events`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--inbox-id` | `inboxesInboxId` | Yes |  |
| `--event-types` | `MetricEventTypes` | No |  |
| `--start` | `Start` | No |  |
| `--end` | `End` | No |  |
| `--period` | `Period` | No |  |
| `--limit` | `MetricLimit` | No |  |
| `--descending` | `Descending` | No |  |

#### `agentmail inboxes metrics query-usage`

Cumulative usage series for the inbox. Each point is the running total
of the usage type at that timestamp, not the change within the bucket.
Inbox-scoped queries carry `storage_bytes`, `message_count`, and
`thread_count`; requested types that don't apply to the scope are
ignored. Defaults to the last 24 hours; `start` must be within the
last 90 days, and a future `end` is clamped to now. The range divided
by `period` must not exceed 1000 buckets.

`GET /v0/inboxes/{inbox_id}/metrics/usage`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--inbox-id` | `inboxesInboxId` | Yes |  |
| `--usage-types` | `UsageTypes` | No |  |
| `--start` | `Start` | No |  |
| `--end` | `End` | No |  |
| `--period` | `Period` | No |  |
| `--limit` | `MetricLimit` | No |  |
| `--descending` | `Descending` | No |  |

---

### `agentmail inboxes threads`

#### `agentmail inboxes threads delete`

Permanently deletes a thread and all of its messages.

**CLI:**
```bash
agentmail inboxes threads delete --inbox-id <inbox_id> --thread-id <thread_id>
```

`DELETE /v0/inboxes/{inbox_id}/threads/{thread_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--inbox-id` | `inboxesInboxId` | Yes |  |
| `--thread-id` | `ThreadId` | Yes |  |

#### `agentmail inboxes threads get`

**CLI:**
```bash
agentmail inboxes threads get --inbox-id <inbox_id> --thread-id <thread_id>
```

`GET /v0/inboxes/{inbox_id}/threads/{thread_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--inbox-id` | `inboxesInboxId` | Yes |  |
| `--thread-id` | `ThreadId` | Yes |  |

#### `agentmail inboxes threads get-attachment`

**CLI:**
```bash
agentmail inboxes threads get-attachment --inbox-id <inbox_id> --thread-id <thread_id> --attachment-id <attachment_id>
```

`GET /v0/inboxes/{inbox_id}/threads/{thread_id}/attachments/{attachment_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--inbox-id` | `inboxesInboxId` | Yes |  |
| `--thread-id` | `ThreadId` | Yes |  |
| `--attachment-id` | `AttachmentId` | Yes |  |

#### `agentmail inboxes threads list`

Lists threads in the inbox, most recent first. Pass `senders`,
`recipients`, or `subject` to filter by substring. Filtered requests are
served by search, which caps `limit` at 100. For relevance-ranked
full-text search, use `Search Threads`.

**CLI:**
```bash
agentmail inboxes threads list --inbox-id <inbox_id>
```

`GET /v0/inboxes/{inbox_id}/threads`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--inbox-id` | `inboxesInboxId` | Yes |  |
| `--limit` | `Limit` | No |  |
| `--page-token` | `PageToken` | No |  |
| `--labels` | `Labels` | No |  |
| `--before` | `Before` | No |  |
| `--after` | `After` | No |  |
| `--ascending` | `Ascending` | No |  |
| `--include-spam` | `IncludeSpam` | No |  |
| `--include-blocked` | `IncludeBlocked` | No |  |
| `--include-unauthenticated` | `IncludeUnauthenticated` | No |  |
| `--include-trash` | `IncludeTrash` | No |  |
| `--senders` | `string[]` | No | Filter to threads whose senders contain this value (substring match). Repeatable; all values must match. |
| `--recipients` | `string[]` | No | Filter to threads whose recipients contain this value (substring match). Repeatable; all values must match. |
| `--subject` | `string[]` | No | Filter to threads whose subject contains this value (substring match). Repeatable; all values must match. |

#### `agentmail inboxes threads search`

Full-text search across threads in the inbox, ranked by relevance. The
query is matched against senders, recipients, and subject (substring)
and the message body (tokenized full text). Spam, trash, blocked, and
unauthenticated threads are always excluded. `limit` cannot exceed 100.

`GET /v0/inboxes/{inbox_id}/threads/search`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--inbox-id` | `inboxesInboxId` | Yes |  |
| `--q` | `Query` | Yes |  |
| `--limit` | `Limit` | No |  |
| `--page-token` | `PageToken` | No |  |
| `--before` | `Before` | No |  |
| `--after` | `After` | No |  |

#### `agentmail inboxes threads update`

Updates thread labels. Cannot add or remove system labels (sent, received, bounced, etc.). Rejects requests with a `422` for threads with 100 or more messages.

`PATCH /v0/inboxes/{inbox_id}/threads/{thread_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--inbox-id` | `inboxesInboxId` | Yes |  |
| `--thread-id` | `ThreadId` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `agentmail inboxes webhooks`

#### `agentmail inboxes webhooks create`

Create a webhook scoped to this inbox.

**CLI:**
```bash
agentmail inboxes webhooks create --inbox-id <inbox_id> --url https://example.com/webhook --event-types message.received
```

`POST /v0/inboxes/{inbox_id}/webhooks`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--inbox-id` | `inboxesInboxId` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `agentmail inboxes webhooks delete`

**CLI:**
```bash
agentmail inboxes webhooks delete --inbox-id <inbox_id> --webhook-id <webhook_id>
```

`DELETE /v0/inboxes/{inbox_id}/webhooks/{webhook_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--inbox-id` | `inboxesInboxId` | Yes |  |
| `--webhook-id` | `webhooksWebhookId` | Yes |  |

#### `agentmail inboxes webhooks get`

**CLI:**
```bash
agentmail inboxes webhooks get --inbox-id <inbox_id> --webhook-id <webhook_id>
```

`GET /v0/inboxes/{inbox_id}/webhooks/{webhook_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--inbox-id` | `inboxesInboxId` | Yes |  |
| `--webhook-id` | `webhooksWebhookId` | Yes |  |

#### `agentmail inboxes webhooks get-headers`

List the names of custom HTTP headers included with deliveries to this inbox-scoped webhook.
Header values are write-only and are never returned.

`GET /v0/inboxes/{inbox_id}/webhooks/{webhook_id}/headers`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--inbox-id` | `inboxesInboxId` | Yes |  |
| `--webhook-id` | `webhooksWebhookId` | Yes |  |

#### `agentmail inboxes webhooks list`

**CLI:**
```bash
agentmail inboxes webhooks list --inbox-id <inbox_id>
```

`GET /v0/inboxes/{inbox_id}/webhooks`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--inbox-id` | `inboxesInboxId` | Yes |  |
| `--limit` | `Limit` | No |  |
| `--page-token` | `PageToken` | No |  |
| `--ascending` | `Ascending` | No |  |

#### `agentmail inboxes webhooks update`

**CLI:**
```bash
agentmail inboxes webhooks update --inbox-id <inbox_id> --webhook-id <webhook_id> --event-types message.received
```

`PATCH /v0/inboxes/{inbox_id}/webhooks/{webhook_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--inbox-id` | `inboxesInboxId` | Yes |  |
| `--webhook-id` | `webhooksWebhookId` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `agentmail inboxes webhooks update-headers`

Atomically set, replace, or remove custom HTTP headers included with deliveries to this
inbox-scoped webhook. Header values remain write-only.

`PATCH /v0/inboxes/{inbox_id}/webhooks/{webhook_id}/headers`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--inbox-id` | `inboxesInboxId` | Yes |  |
| `--webhook-id` | `webhooksWebhookId` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `agentmail lists`

#### `agentmail lists create`

**CLI:**
```bash
agentmail lists create --direction <direction> --type <type> --entry user@example.com
```

`POST /v0/lists/{direction}/{type}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--direction` | `Direction` | Yes |  |
| `--type` | `ListType` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `agentmail lists delete`

**CLI:**
```bash
agentmail lists delete --direction <direction> --type <type> --entry <entry>
```

`DELETE /v0/lists/{direction}/{type}/{entry}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--direction` | `Direction` | Yes |  |
| `--type` | `ListType` | Yes |  |
| `--entry` | `string` | Yes | Email address or domain. |

#### `agentmail lists get`

**CLI:**
```bash
agentmail lists get --direction <direction> --type <type> --entry <entry>
```

`GET /v0/lists/{direction}/{type}/{entry}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--direction` | `Direction` | Yes |  |
| `--type` | `ListType` | Yes |  |
| `--entry` | `string` | Yes | Email address or domain. |

#### `agentmail lists list`

**CLI:**
```bash
agentmail lists list --direction <direction> --type <type>
```

`GET /v0/lists/{direction}/{type}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--direction` | `Direction` | Yes |  |
| `--type` | `ListType` | Yes |  |
| `--limit` | `Limit` | No |  |
| `--page-token` | `PageToken` | No |  |

---

### `agentmail metrics`

#### `agentmail metrics query-events`

Counts of email events (sent, delivered, bounced, etc.) over time for
the organization. Defaults to the last 24 hours; `start` must be within
the last 90 days, and a future `end` is clamped to now. Omit `period`
for individual event counts, or set it to sum counts into buckets of
that many seconds.

**CLI:**
```bash
agentmail metrics query-events
```

`GET /v0/metrics/events`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--event-types` | `MetricEventTypes` | No |  |
| `--start` | `Start` | No |  |
| `--end` | `End` | No |  |
| `--period` | `Period` | No |  |
| `--limit` | `MetricLimit` | No |  |
| `--descending` | `Descending` | No |  |

#### `agentmail metrics query-usage`

Cumulative usage series for the organization. Each point is the running
total of the usage type at that timestamp, not the change within the
bucket. Defaults to the last 24 hours; `start` must be within the last
90 days, and a future `end` is clamped to now. The range divided by
`period` must not exceed 1000 buckets.

`GET /v0/metrics/usage`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--usage-types` | `UsageTypes` | No |  |
| `--start` | `Start` | No |  |
| `--end` | `End` | No |  |
| `--period` | `Period` | No |  |
| `--limit` | `MetricLimit` | No |  |
| `--descending` | `Descending` | No |  |

---

### `agentmail organizations`

#### `agentmail organizations get`

Returns the organization for the authenticated API key (usage limits, counts, and billing metadata).

**CLI:**
```bash
agentmail organizations get
```

`GET /v0/organizations`

---

### `agentmail pods`

#### `agentmail pods create`

**CLI:**
```bash
agentmail pods create --client-id my-pod
```

`POST /v0/pods`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `agentmail pods delete`

**CLI:**
```bash
agentmail pods delete --pod-id <pod_id>
```

`DELETE /v0/pods/{pod_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pod-id` | `podsPodId` | Yes |  |

#### `agentmail pods get`

**CLI:**
```bash
agentmail pods get --pod-id <pod_id>
```

`GET /v0/pods/{pod_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pod-id` | `podsPodId` | Yes |  |

#### `agentmail pods list`

**CLI:**
```bash
agentmail pods list
```

`GET /v0/pods`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--limit` | `Limit` | No |  |
| `--page-token` | `PageToken` | No |  |
| `--ascending` | `Ascending` | No |  |

---

### `agentmail pods api-keys`

#### `agentmail pods api-keys create`

**CLI:**
```bash
agentmail pods api-keys create --pod-id <pod_id> --name "My Key"
```

`POST /v0/pods/{pod_id}/api-keys`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pod-id` | `podsPodId` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `agentmail pods api-keys delete`

**CLI:**
```bash
agentmail pods api-keys delete --pod-id <pod_id> --api-key-id <api_key_id>
```

`DELETE /v0/pods/{pod_id}/api-keys/{api_key_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pod-id` | `podsPodId` | Yes |  |
| `--api-key-id` | `ApiKeyId` | Yes |  |

#### `agentmail pods api-keys list`

**CLI:**
```bash
agentmail pods api-keys list --pod-id <pod_id>
```

`GET /v0/pods/{pod_id}/api-keys`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pod-id` | `podsPodId` | Yes |  |
| `--limit` | `Limit` | No |  |
| `--page-token` | `PageToken` | No |  |

---

### `agentmail pods domains`

#### `agentmail pods domains create`

**CLI:**
```bash
agentmail pods domains create --pod-id <pod_id> --domain example.com
```

`POST /v0/pods/{pod_id}/domains`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pod-id` | `podsPodId` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `agentmail pods domains delete`

**CLI:**
```bash
agentmail pods domains delete --pod-id <pod_id> --domain-id <domain_id>
```

`DELETE /v0/pods/{pod_id}/domains/{domain_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pod-id` | `podsPodId` | Yes |  |
| `--domain-id` | `DomainId` | Yes |  |

#### `agentmail pods domains get`

**CLI:**
```bash
agentmail pods domains get --pod-id <pod_id> --domain-id <domain_id>
```

`GET /v0/pods/{pod_id}/domains/{domain_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pod-id` | `podsPodId` | Yes |  |
| `--domain-id` | `DomainId` | Yes |  |

#### `agentmail pods domains get-zone-file`

**CLI:**
```bash
agentmail pods domains get-zone-file --pod-id <pod_id> --domain-id <domain_id>
```

`GET /v0/pods/{pod_id}/domains/{domain_id}/zone-file`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pod-id` | `podsPodId` | Yes |  |
| `--domain-id` | `DomainId` | Yes |  |

#### `agentmail pods domains list`

**CLI:**
```bash
agentmail pods domains list --pod-id <pod_id>
```

`GET /v0/pods/{pod_id}/domains`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pod-id` | `podsPodId` | Yes |  |
| `--limit` | `Limit` | No |  |
| `--page-token` | `PageToken` | No |  |
| `--ascending` | `Ascending` | No |  |

#### `agentmail pods domains update`

**CLI:**
```bash
agentmail pods domains update --pod-id <pod_id> --domain-id <domain_id>
```

`PATCH /v0/pods/{pod_id}/domains/{domain_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pod-id` | `podsPodId` | Yes |  |
| `--domain-id` | `DomainId` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `agentmail pods domains verify`

**CLI:**
```bash
agentmail pods domains verify --pod-id <pod_id> --domain-id <domain_id>
```

`POST /v0/pods/{pod_id}/domains/{domain_id}/verify`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pod-id` | `podsPodId` | Yes |  |
| `--domain-id` | `DomainId` | Yes |  |

---

### `agentmail pods drafts`

#### `agentmail pods drafts get`

**CLI:**
```bash
agentmail pods drafts get --pod-id <pod_id> --draft-id <draft_id>
```

`GET /v0/pods/{pod_id}/drafts/{draft_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pod-id` | `podsPodId` | Yes |  |
| `--draft-id` | `DraftId` | Yes |  |

#### `agentmail pods drafts get-attachment`

**CLI:**
```bash
agentmail pods drafts get-attachment --pod-id <pod_id> --draft-id <draft_id> --attachment-id <attachment_id>
```

`GET /v0/pods/{pod_id}/drafts/{draft_id}/attachments/{attachment_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pod-id` | `podsPodId` | Yes |  |
| `--draft-id` | `DraftId` | Yes |  |
| `--attachment-id` | `AttachmentId` | Yes |  |

#### `agentmail pods drafts list`

**CLI:**
```bash
agentmail pods drafts list --pod-id <pod_id>
```

`GET /v0/pods/{pod_id}/drafts`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pod-id` | `podsPodId` | Yes |  |
| `--limit` | `Limit` | No |  |
| `--page-token` | `PageToken` | No |  |
| `--labels` | `Labels` | No |  |
| `--before` | `Before` | No |  |
| `--after` | `After` | No |  |
| `--ascending` | `Ascending` | No |  |

---

### `agentmail pods inboxes`

#### `agentmail pods inboxes create`

**CLI:**
```bash
agentmail pods inboxes create --pod-id <pod_id> --username myagent --domain example.com
```

`POST /v0/pods/{pod_id}/inboxes`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pod-id` | `podsPodId` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `agentmail pods inboxes delete`

**CLI:**
```bash
agentmail pods inboxes delete --pod-id <pod_id> --inbox-id <inbox_id>
```

`DELETE /v0/pods/{pod_id}/inboxes/{inbox_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pod-id` | `podsPodId` | Yes |  |
| `--inbox-id` | `inboxesInboxId` | Yes |  |

#### `agentmail pods inboxes get`

**CLI:**
```bash
agentmail pods inboxes get --pod-id <pod_id> --inbox-id <inbox_id>
```

`GET /v0/pods/{pod_id}/inboxes/{inbox_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pod-id` | `podsPodId` | Yes |  |
| `--inbox-id` | `inboxesInboxId` | Yes |  |

#### `agentmail pods inboxes list`

**CLI:**
```bash
agentmail pods inboxes list --pod-id <pod_id>
```

`GET /v0/pods/{pod_id}/inboxes`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pod-id` | `podsPodId` | Yes |  |
| `--limit` | `Limit` | No |  |
| `--page-token` | `PageToken` | No |  |
| `--ascending` | `Ascending` | No |  |

#### `agentmail pods inboxes update`

**CLI:**
```bash
agentmail pods inboxes update --pod-id <pod_id> --inbox-id <inbox_id>
```

`PATCH /v0/pods/{pod_id}/inboxes/{inbox_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pod-id` | `podsPodId` | Yes |  |
| `--inbox-id` | `inboxesInboxId` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `agentmail pods lists`

#### `agentmail pods lists create`

**CLI:**
```bash
agentmail pods lists create --pod-id <pod_id> --direction <direction> --type <type> --entry user@example.com
```

`POST /v0/pods/{pod_id}/lists/{direction}/{type}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pod-id` | `podsPodId` | Yes |  |
| `--direction` | `Direction` | Yes |  |
| `--type` | `ListType` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `agentmail pods lists delete`

**CLI:**
```bash
agentmail pods lists delete --pod-id <pod_id> --direction <direction> --type <type> --entry <entry>
```

`DELETE /v0/pods/{pod_id}/lists/{direction}/{type}/{entry}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pod-id` | `podsPodId` | Yes |  |
| `--direction` | `Direction` | Yes |  |
| `--type` | `ListType` | Yes |  |
| `--entry` | `string` | Yes | Email address or domain. |

#### `agentmail pods lists get`

**CLI:**
```bash
agentmail pods lists get --pod-id <pod_id> --direction <direction> --type <type> --entry <entry>
```

`GET /v0/pods/{pod_id}/lists/{direction}/{type}/{entry}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pod-id` | `podsPodId` | Yes |  |
| `--direction` | `Direction` | Yes |  |
| `--type` | `ListType` | Yes |  |
| `--entry` | `string` | Yes | Email address or domain. |

#### `agentmail pods lists list`

**CLI:**
```bash
agentmail pods lists list --pod-id <pod_id> --direction <direction> --type <type>
```

`GET /v0/pods/{pod_id}/lists/{direction}/{type}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pod-id` | `podsPodId` | Yes |  |
| `--direction` | `Direction` | Yes |  |
| `--type` | `ListType` | Yes |  |
| `--limit` | `Limit` | No |  |
| `--page-token` | `PageToken` | No |  |

---

### `agentmail pods metrics`

#### `agentmail pods metrics query-events`

Counts of email events (sent, delivered, bounced, etc.) over time for
the pod. Defaults to the last 24 hours; `start` must be within the last
90 days, and a future `end` is clamped to now. Omit `period` for
individual event counts, or set it to sum counts into buckets of that
many seconds.

**CLI:**
```bash
agentmail pods metrics query-events --pod-id <pod_id>
```

`GET /v0/pods/{pod_id}/metrics/events`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pod-id` | `podsPodId` | Yes |  |
| `--event-types` | `MetricEventTypes` | No |  |
| `--start` | `Start` | No |  |
| `--end` | `End` | No |  |
| `--period` | `Period` | No |  |
| `--limit` | `MetricLimit` | No |  |
| `--descending` | `Descending` | No |  |

#### `agentmail pods metrics query-usage`

Cumulative usage series for the pod. Each point is the running total of
the usage type at that timestamp, not the change within the bucket.
Pod-scoped queries carry every usage type except `pod_count`; requested
types that don't apply to the scope are ignored. Defaults to the last
24 hours; `start` must be within the last 90 days, and a future `end`
is clamped to now. The range divided by `period` must not exceed 1000
buckets.

`GET /v0/pods/{pod_id}/metrics/usage`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pod-id` | `podsPodId` | Yes |  |
| `--usage-types` | `UsageTypes` | No |  |
| `--start` | `Start` | No |  |
| `--end` | `End` | No |  |
| `--period` | `Period` | No |  |
| `--limit` | `MetricLimit` | No |  |
| `--descending` | `Descending` | No |  |

---

### `agentmail pods threads`

#### `agentmail pods threads delete`

Permanently deletes a thread and all of its messages.

**CLI:**
```bash
agentmail pods threads delete --pod-id <pod_id> --thread-id <thread_id>
```

`DELETE /v0/pods/{pod_id}/threads/{thread_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pod-id` | `podsPodId` | Yes |  |
| `--thread-id` | `ThreadId` | Yes |  |

#### `agentmail pods threads get`

**CLI:**
```bash
agentmail pods threads get --pod-id <pod_id> --thread-id <thread_id>
```

`GET /v0/pods/{pod_id}/threads/{thread_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pod-id` | `podsPodId` | Yes |  |
| `--thread-id` | `ThreadId` | Yes |  |

#### `agentmail pods threads get-attachment`

**CLI:**
```bash
agentmail pods threads get-attachment --pod-id <pod_id> --thread-id <thread_id> --attachment-id <attachment_id>
```

`GET /v0/pods/{pod_id}/threads/{thread_id}/attachments/{attachment_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pod-id` | `podsPodId` | Yes |  |
| `--thread-id` | `ThreadId` | Yes |  |
| `--attachment-id` | `AttachmentId` | Yes |  |

#### `agentmail pods threads list`

Lists threads in the pod, most recent first. Pass `senders`,
`recipients`, or `subject` to filter by substring. Filtered requests are
served by search, which caps `limit` at 100. For relevance-ranked
full-text search, use `Search Threads`.

**CLI:**
```bash
agentmail pods threads list --pod-id <pod_id>
```

`GET /v0/pods/{pod_id}/threads`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pod-id` | `podsPodId` | Yes |  |
| `--limit` | `Limit` | No |  |
| `--page-token` | `PageToken` | No |  |
| `--labels` | `Labels` | No |  |
| `--before` | `Before` | No |  |
| `--after` | `After` | No |  |
| `--ascending` | `Ascending` | No |  |
| `--include-spam` | `IncludeSpam` | No |  |
| `--include-blocked` | `IncludeBlocked` | No |  |
| `--include-unauthenticated` | `IncludeUnauthenticated` | No |  |
| `--include-trash` | `IncludeTrash` | No |  |
| `--senders` | `string[]` | No | Filter to threads whose senders contain this value (substring match). Repeatable; all values must match. |
| `--recipients` | `string[]` | No | Filter to threads whose recipients contain this value (substring match). Repeatable; all values must match. |
| `--subject` | `string[]` | No | Filter to threads whose subject contains this value (substring match). Repeatable; all values must match. |

#### `agentmail pods threads search`

Full-text search across threads in the pod, ranked by relevance. The
query is matched against senders, recipients, and subject (substring)
and the message body (tokenized full text). Spam, trash, blocked, and
unauthenticated threads are always excluded. `limit` cannot exceed 100.

`GET /v0/pods/{pod_id}/threads/search`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pod-id` | `podsPodId` | Yes |  |
| `--q` | `Query` | Yes |  |
| `--limit` | `Limit` | No |  |
| `--page-token` | `PageToken` | No |  |
| `--before` | `Before` | No |  |
| `--after` | `After` | No |  |

#### `agentmail pods threads update`

Updates thread labels. Cannot add or remove system labels (sent, received, bounced, etc.). Rejects requests with a `422` for threads with 100 or more messages.

`PATCH /v0/pods/{pod_id}/threads/{thread_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pod-id` | `podsPodId` | Yes |  |
| `--thread-id` | `ThreadId` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `agentmail pods webhooks`

#### `agentmail pods webhooks create`

Create a webhook scoped to this pod.

**CLI:**
```bash
agentmail pods webhooks create --pod-id <pod_id> --url https://example.com/webhook --event-types message.received
```

`POST /v0/pods/{pod_id}/webhooks`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pod-id` | `podsPodId` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `agentmail pods webhooks delete`

**CLI:**
```bash
agentmail pods webhooks delete --pod-id <pod_id> --webhook-id <webhook_id>
```

`DELETE /v0/pods/{pod_id}/webhooks/{webhook_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pod-id` | `podsPodId` | Yes |  |
| `--webhook-id` | `webhooksWebhookId` | Yes |  |

#### `agentmail pods webhooks get`

**CLI:**
```bash
agentmail pods webhooks get --pod-id <pod_id> --webhook-id <webhook_id>
```

`GET /v0/pods/{pod_id}/webhooks/{webhook_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pod-id` | `podsPodId` | Yes |  |
| `--webhook-id` | `webhooksWebhookId` | Yes |  |

#### `agentmail pods webhooks get-headers`

List the names of custom HTTP headers included with deliveries to this pod-scoped webhook.
Header values are write-only and are never returned.

`GET /v0/pods/{pod_id}/webhooks/{webhook_id}/headers`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pod-id` | `podsPodId` | Yes |  |
| `--webhook-id` | `webhooksWebhookId` | Yes |  |

#### `agentmail pods webhooks list`

**CLI:**
```bash
agentmail pods webhooks list --pod-id <pod_id>
```

`GET /v0/pods/{pod_id}/webhooks`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pod-id` | `podsPodId` | Yes |  |
| `--limit` | `Limit` | No |  |
| `--page-token` | `PageToken` | No |  |
| `--ascending` | `Ascending` | No |  |

#### `agentmail pods webhooks update`

**CLI:**
```bash
agentmail pods webhooks update --pod-id <pod_id> --webhook-id <webhook_id> --add-inbox-ids <inbox_id>
```

`PATCH /v0/pods/{pod_id}/webhooks/{webhook_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pod-id` | `podsPodId` | Yes |  |
| `--webhook-id` | `webhooksWebhookId` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `agentmail pods webhooks update-headers`

Atomically set, replace, or remove custom HTTP headers included with deliveries to this
pod-scoped webhook. Header values remain write-only.

`PATCH /v0/pods/{pod_id}/webhooks/{webhook_id}/headers`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pod-id` | `podsPodId` | Yes |  |
| `--webhook-id` | `webhooksWebhookId` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `agentmail threads`

#### `agentmail threads delete`

Permanently deletes a thread and all of its messages.

**CLI:**
```bash
agentmail threads delete --thread-id <thread_id>
```

`DELETE /v0/threads/{thread_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--thread-id` | `ThreadId` | Yes |  |

#### `agentmail threads get`

**CLI:**
```bash
agentmail threads get --thread-id <thread_id>
```

`GET /v0/threads/{thread_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--thread-id` | `ThreadId` | Yes |  |

#### `agentmail threads get-attachment`

**CLI:**
```bash
agentmail threads get-attachment --thread-id <thread_id> --attachment-id <attachment_id>
```

`GET /v0/threads/{thread_id}/attachments/{attachment_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--thread-id` | `ThreadId` | Yes |  |
| `--attachment-id` | `AttachmentId` | Yes |  |

#### `agentmail threads list`

Lists threads, most recent first. Pass `senders`, `recipients`, or
`subject` to filter by substring. Filtered requests are served by
search, which caps `limit` at 100. For relevance-ranked full-text
search across senders, recipients, subject, and message body, use
`Search Threads`.

**CLI:**
```bash
agentmail threads list
```

`GET /v0/threads`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--limit` | `Limit` | No |  |
| `--page-token` | `PageToken` | No |  |
| `--labels` | `Labels` | No |  |
| `--before` | `Before` | No |  |
| `--after` | `After` | No |  |
| `--ascending` | `Ascending` | No |  |
| `--include-spam` | `IncludeSpam` | No |  |
| `--include-blocked` | `IncludeBlocked` | No |  |
| `--include-unauthenticated` | `IncludeUnauthenticated` | No |  |
| `--include-trash` | `IncludeTrash` | No |  |
| `--senders` | `string[]` | No | Filter to threads whose senders contain this value (substring match). Repeatable; all values must match. |
| `--recipients` | `string[]` | No | Filter to threads whose recipients contain this value (substring match). Repeatable; all values must match. |
| `--subject` | `string[]` | No | Filter to threads whose subject contains this value (substring match). Repeatable; all values must match. |

#### `agentmail threads search`

Full-text search across threads in the organization, ranked by
relevance. The query is matched against senders, recipients, and
subject (substring) and the message body (tokenized full text). Spam,
trash, blocked, and unauthenticated threads are always excluded.
`limit` cannot exceed 100.

`GET /v0/threads/search`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--q` | `Query` | Yes |  |
| `--limit` | `Limit` | No |  |
| `--page-token` | `PageToken` | No |  |
| `--before` | `Before` | No |  |
| `--after` | `After` | No |  |

#### `agentmail threads update`

Updates thread labels. Cannot add or remove system labels (sent, received, bounced, etc.). Rejects requests with a `422` for threads with 100 or more messages.

`PATCH /v0/threads/{thread_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--thread-id` | `ThreadId` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `agentmail webhooks`

#### `agentmail webhooks create`

**CLI:**
```bash
agentmail webhooks create --url https://example.com/webhook --event-types message.received
```

`POST /v0/webhooks`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `agentmail webhooks delete`

**CLI:**
```bash
agentmail webhooks delete --webhook-id <webhook_id>
```

`DELETE /v0/webhooks/{webhook_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--webhook-id` | `webhooksWebhookId` | Yes |  |

#### `agentmail webhooks get`

**CLI:**
```bash
agentmail webhooks get --webhook-id <webhook_id>
```

`GET /v0/webhooks/{webhook_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--webhook-id` | `webhooksWebhookId` | Yes |  |

#### `agentmail webhooks get-headers`

List the names of custom HTTP headers included with deliveries to this webhook. Header values are
write-only and are never returned.

`GET /v0/webhooks/{webhook_id}/headers`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--webhook-id` | `webhooksWebhookId` | Yes |  |

#### `agentmail webhooks list`

**CLI:**
```bash
agentmail webhooks list
```

`GET /v0/webhooks`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--limit` | `Limit` | No |  |
| `--page-token` | `PageToken` | No |  |
| `--ascending` | `Ascending` | No |  |

#### `agentmail webhooks update`

Update inbox or pod subscriptions, or replace the webhook's `event_types` in full when you pass a
non-empty `event_types` array (see request field docs). Inbox and pod changes use add/remove lists.

**CLI:**
```bash
agentmail webhooks update --webhook-id <webhook_id> --add-inbox-ids <inbox_id>
```

`PATCH /v0/webhooks/{webhook_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--webhook-id` | `webhooksWebhookId` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `agentmail webhooks update-headers`

Atomically set, replace, or remove custom HTTP headers included with deliveries to this webhook.
Header values remain write-only.

`PATCH /v0/webhooks/{webhook_id}/headers`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--webhook-id` | `webhooksWebhookId` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

## Global flags

These flags are available on every command:

| Flag | Description |
|------|-------------|
| `--dry-run` | Print the HTTP request without sending it |
| `--json <JSON\|->` | Supply the request body as JSON (or `-` for stdin) |
| `--params <JSON>` | Merge extra parameters as JSON |
| `--format <json\|table\|yaml\|csv>` | Output format (default: `json`) |
| `--output <PATH>` | Write binary responses to a file |
| `--base-url <URL>` | Override the API base URL |
| `--page-all` | Auto-paginate and stream all results |
| `--page-limit <N>` | Max pages to fetch (default: `10`) |
| `-q, --quiet` | Suppress stdout on success |
| `-h, --help` | Print help |
| `-V, --version` | Print version |

