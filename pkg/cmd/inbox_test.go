// File generated from our OpenAPI spec by Stainless. See CONTRIBUTING.md for details.

package cmd

import (
	"testing"

	"github.com/agentmail-to/agentmail-cli/internal/mocktest"
)

func TestInboxesCreate(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"inboxes", "create",
		"--api-key", "string",
		"--client-id", "client_id",
		"--display-name", "display_name",
		"--domain", "domain",
		"--username", "username",
	)
}

func TestInboxesRetrieve(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"inboxes", "retrieve",
		"--api-key", "string",
		"--inbox-id", "inbox_id",
	)
}

func TestInboxesUpdate(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"inboxes", "update",
		"--api-key", "string",
		"--inbox-id", "inbox_id",
		"--display-name", "display_name",
	)
}

func TestInboxesList(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"inboxes", "list",
		"--api-key", "string",
		"--ascending=true",
		"--limit", "0",
		"--page-token", "page_token",
	)
}

func TestInboxesDelete(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"inboxes", "delete",
		"--api-key", "string",
		"--inbox-id", "inbox_id",
	)
}

func TestInboxesListMetrics(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"inboxes", "list-metrics",
		"--api-key", "string",
		"--inbox-id", "inbox_id",
		"--end-timestamp", "'2019-12-27T18:11:19.117Z'",
		"--start-timestamp", "'2019-12-27T18:11:19.117Z'",
		"--event-type", "[message.sent]",
	)
}
