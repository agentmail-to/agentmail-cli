// File generated from our OpenAPI spec by Stainless. See CONTRIBUTING.md for details.

package cmd

import (
	"testing"

	"github.com/agentmail-to/agentmail-cli/internal/mocktest"
)

func TestInboxesThreadsRetrieve(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"inboxes:threads", "retrieve",
			"--inbox-id", "inbox_id",
			"--thread-id", "thread_id",
		)
	})
}

func TestInboxesThreadsList(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"inboxes:threads", "list",
			"--inbox-id", "inbox_id",
			"--after", "'2019-12-27T18:11:19.117Z'",
			"--ascending=true",
			"--before", "'2019-12-27T18:11:19.117Z'",
			"--include-blocked=true",
			"--include-spam=true",
			"--include-trash=true",
			"--label", "[string]",
			"--limit", "0",
			"--page-token", "page_token",
		)
	})
}

func TestInboxesThreadsDelete(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"inboxes:threads", "delete",
			"--inbox-id", "inbox_id",
			"--thread-id", "thread_id",
			"--permanent=true",
		)
	})
}

func TestInboxesThreadsGetAttachment(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"inboxes:threads", "get-attachment",
			"--inbox-id", "inbox_id",
			"--thread-id", "thread_id",
			"--attachment-id", "attachment_id",
		)
	})
}
