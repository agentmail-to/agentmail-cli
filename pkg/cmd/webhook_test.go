// File generated from our OpenAPI spec by Stainless. See CONTRIBUTING.md for details.

package cmd

import (
	"testing"

	"github.com/stainless-sdks/agentmail-cli/internal/mocktest"
)

func TestWebhooksCreate(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"webhooks", "create",
		"--api-key", "string",
		"--event-type", "message.received",
		"--url", "url",
		"--client-id", "client_id",
		"--inbox-id", "[string]",
		"--pod-id", "[string]",
	)
}

func TestWebhooksRetrieve(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"webhooks", "retrieve",
		"--api-key", "string",
		"--webhook-id", "webhook_id",
	)
}

func TestWebhooksUpdate(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"webhooks", "update",
		"--api-key", "string",
		"--webhook-id", "webhook_id",
		"--add-inbox-id", "[string]",
		"--add-pod-id", "[string]",
		"--remove-inbox-id", "[string]",
		"--remove-pod-id", "[string]",
	)
}

func TestWebhooksList(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"webhooks", "list",
		"--api-key", "string",
		"--limit", "0",
		"--page-token", "page_token",
	)
}

func TestWebhooksDelete(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"webhooks", "delete",
		"--api-key", "string",
		"--webhook-id", "webhook_id",
	)
}
