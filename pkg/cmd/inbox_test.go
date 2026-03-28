// File generated from our OpenAPI spec by Stainless. See CONTRIBUTING.md for details.

package cmd

import (
	"testing"

	"github.com/agentmail-to/agentmail-cli/internal/mocktest"
)

func TestInboxesCreate(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"inboxes", "create",
			"--client-id", "client_id",
			"--display-name", "display_name",
			"--domain", "domain",
			"--username", "username",
		)
	})

	t.Run("piping data", func(t *testing.T) {
		// Test piping YAML data over stdin
		pipeData := []byte("" +
			"client_id: client_id\n" +
			"display_name: display_name\n" +
			"domain: domain\n" +
			"username: username\n")
		mocktest.TestRunMockTestWithPipeAndFlags(
			t, pipeData,
			"--api-key", "string",
			"inboxes", "create",
		)
	})
}

func TestInboxesRetrieve(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"inboxes", "retrieve",
			"--inbox-id", "inbox_id",
		)
	})
}

func TestInboxesUpdate(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"inboxes", "update",
			"--inbox-id", "inbox_id",
			"--display-name", "display_name",
		)
	})

	t.Run("piping data", func(t *testing.T) {
		// Test piping YAML data over stdin
		pipeData := []byte("display_name: display_name")
		mocktest.TestRunMockTestWithPipeAndFlags(
			t, pipeData,
			"--api-key", "string",
			"inboxes", "update",
			"--inbox-id", "inbox_id",
		)
	})
}

func TestInboxesList(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"inboxes", "list",
			"--limit", "0",
			"--page-token", "page_token",
		)
	})
}

func TestInboxesDelete(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"inboxes", "delete",
			"--inbox-id", "inbox_id",
		)
	})
}

func TestInboxesListMetrics(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"inboxes", "list-metrics",
			"--inbox-id", "inbox_id",
			"--end-timestamp", "'2019-12-27T18:11:19.117Z'",
			"--start-timestamp", "'2019-12-27T18:11:19.117Z'",
			"--event-type", "[message.sent]",
		)
	})
}
