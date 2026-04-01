// File generated from our OpenAPI spec by Stainless. See CONTRIBUTING.md for details.

package cmd

import (
	"testing"

	"github.com/agentmail-to/agentmail-cli/internal/mocktest"
)

func TestInboxesListsCreate(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"inboxes:lists", "create",
			"--inbox-id", "inbox_id",
			"--direction", "send",
			"--type", "allow",
			"--entry", "entry",
			"--reason", "reason",
		)
	})

	t.Run("piping data", func(t *testing.T) {
		// Test piping YAML data over stdin
		pipeData := []byte("" +
			"entry: entry\n" +
			"reason: reason\n")
		mocktest.TestRunMockTestWithPipeAndFlags(
			t, pipeData,
			"--api-key", "string",
			"inboxes:lists", "create",
			"--inbox-id", "inbox_id",
			"--direction", "send",
			"--type", "allow",
		)
	})
}

func TestInboxesListsRetrieve(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"inboxes:lists", "retrieve",
			"--inbox-id", "inbox_id",
			"--direction", "send",
			"--type", "allow",
			"--entry", "entry",
		)
	})
}

func TestInboxesListsList(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"inboxes:lists", "list",
			"--inbox-id", "inbox_id",
			"--direction", "send",
			"--type", "allow",
			"--limit", "0",
			"--page-token", "page_token",
		)
	})
}

func TestInboxesListsDelete(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"inboxes:lists", "delete",
			"--inbox-id", "inbox_id",
			"--direction", "send",
			"--type", "allow",
			"--entry", "entry",
		)
	})
}
