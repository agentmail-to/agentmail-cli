// File generated from our OpenAPI spec by Stainless. See CONTRIBUTING.md for details.

package cmd

import (
	"testing"

	"github.com/agentmail-to/agentmail-cli/internal/mocktest"
	"github.com/agentmail-to/agentmail-cli/internal/requestflag"
)

func TestAPIKeysCreate(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"api-keys", "create",
			"--name", "name",
			"--permissions", "{api_key_create: true, api_key_delete: true, api_key_read: true, domain_create: true, domain_delete: true, domain_read: true, domain_update: true, draft_create: true, draft_delete: true, draft_read: true, draft_send: true, draft_update: true, inbox_create: true, inbox_delete: true, inbox_read: true, inbox_update: true, label_blocked_read: true, label_spam_read: true, label_trash_read: true, list_entry_create: true, list_entry_delete: true, list_entry_read: true, message_read: true, message_send: true, message_update: true, metrics_read: true, pod_create: true, pod_delete: true, pod_read: true, thread_delete: true, thread_read: true, webhook_create: true, webhook_delete: true, webhook_read: true, webhook_update: true}",
		)
	})

	t.Run("inner flags", func(t *testing.T) {
		// Check that inner flags have been set up correctly
		requestflag.CheckInnerFlags(apiKeysCreate)

		// Alternative argument passing style using inner flags
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"api-keys", "create",
			"--name", "name",
			"--permissions.api-key-create=true",
			"--permissions.api-key-delete=true",
			"--permissions.api-key-read=true",
			"--permissions.domain-create=true",
			"--permissions.domain-delete=true",
			"--permissions.domain-read=true",
			"--permissions.domain-update=true",
			"--permissions.draft-create=true",
			"--permissions.draft-delete=true",
			"--permissions.draft-read=true",
			"--permissions.draft-send=true",
			"--permissions.draft-update=true",
			"--permissions.inbox-create=true",
			"--permissions.inbox-delete=true",
			"--permissions.inbox-read=true",
			"--permissions.inbox-update=true",
			"--permissions.label-blocked-read=true",
			"--permissions.label-spam-read=true",
			"--permissions.label-trash-read=true",
			"--permissions.list-entry-create=true",
			"--permissions.list-entry-delete=true",
			"--permissions.list-entry-read=true",
			"--permissions.message-read=true",
			"--permissions.message-send=true",
			"--permissions.message-update=true",
			"--permissions.metrics-read=true",
			"--permissions.pod-create=true",
			"--permissions.pod-delete=true",
			"--permissions.pod-read=true",
			"--permissions.thread-delete=true",
			"--permissions.thread-read=true",
			"--permissions.webhook-create=true",
			"--permissions.webhook-delete=true",
			"--permissions.webhook-read=true",
			"--permissions.webhook-update=true",
		)
	})

	t.Run("piping data", func(t *testing.T) {
		// Test piping YAML data over stdin
		pipeData := []byte("" +
			"name: name\n" +
			"permissions:\n" +
			"  api_key_create: true\n" +
			"  api_key_delete: true\n" +
			"  api_key_read: true\n" +
			"  domain_create: true\n" +
			"  domain_delete: true\n" +
			"  domain_read: true\n" +
			"  domain_update: true\n" +
			"  draft_create: true\n" +
			"  draft_delete: true\n" +
			"  draft_read: true\n" +
			"  draft_send: true\n" +
			"  draft_update: true\n" +
			"  inbox_create: true\n" +
			"  inbox_delete: true\n" +
			"  inbox_read: true\n" +
			"  inbox_update: true\n" +
			"  label_blocked_read: true\n" +
			"  label_spam_read: true\n" +
			"  label_trash_read: true\n" +
			"  list_entry_create: true\n" +
			"  list_entry_delete: true\n" +
			"  list_entry_read: true\n" +
			"  message_read: true\n" +
			"  message_send: true\n" +
			"  message_update: true\n" +
			"  metrics_read: true\n" +
			"  pod_create: true\n" +
			"  pod_delete: true\n" +
			"  pod_read: true\n" +
			"  thread_delete: true\n" +
			"  thread_read: true\n" +
			"  webhook_create: true\n" +
			"  webhook_delete: true\n" +
			"  webhook_read: true\n" +
			"  webhook_update: true\n")
		mocktest.TestRunMockTestWithPipeAndFlags(
			t, pipeData,
			"--api-key", "string",
			"api-keys", "create",
		)
	})
}

func TestAPIKeysList(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"api-keys", "list",
			"--ascending=true",
			"--limit", "0",
			"--page-token", "page_token",
		)
	})
}
