// File generated from our OpenAPI spec by Stainless. See CONTRIBUTING.md for details.

package cmd

import (
	"testing"

	"github.com/agentmail-to/agentmail-cli/internal/mocktest"
)

func TestWebhooksCreate(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"webhooks", "create",
			"--event-type", "message.received",
			"--url", "url",
			"--client-id", "client_id",
			"--inbox-id", "[string]",
			"--pod-id", "[string]",
		)
	})

	t.Run("piping data", func(t *testing.T) {
		// Test piping YAML data over stdin
		pipeData := []byte("" +
			"event_types:\n" +
			"  - message.received\n" +
			"url: url\n" +
			"client_id: client_id\n" +
			"inbox_ids:\n" +
			"  - string\n" +
			"pod_ids:\n" +
			"  - string\n")
		mocktest.TestRunMockTestWithPipeAndFlags(
			t, pipeData,
			"--api-key", "string",
			"webhooks", "create",
		)
	})
}

func TestWebhooksUpdate(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"webhooks", "update",
			"--webhook-id", "webhook_id",
			"--add-inbox-id", "[string]",
			"--add-pod-id", "[string]",
			"--remove-inbox-id", "[string]",
			"--remove-pod-id", "[string]",
		)
	})

	t.Run("piping data", func(t *testing.T) {
		// Test piping YAML data over stdin
		pipeData := []byte("" +
			"add_inbox_ids:\n" +
			"  - string\n" +
			"add_pod_ids:\n" +
			"  - string\n" +
			"remove_inbox_ids:\n" +
			"  - string\n" +
			"remove_pod_ids:\n" +
			"  - string\n")
		mocktest.TestRunMockTestWithPipeAndFlags(
			t, pipeData,
			"--api-key", "string",
			"webhooks", "update",
			"--webhook-id", "webhook_id",
		)
	})
}

func TestWebhooksList(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"webhooks", "list",
			"--ascending=true",
			"--limit", "0",
			"--page-token", "page_token",
		)
	})
}

func TestWebhooksDelete(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"webhooks", "delete",
			"--webhook-id", "webhook_id",
		)
	})
}

func TestWebhooksGet(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"webhooks", "get",
			"--webhook-id", "webhook_id",
		)
	})
}
