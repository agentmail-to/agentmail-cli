// File generated from our OpenAPI spec by Stainless. See CONTRIBUTING.md for details.

package cmd

import (
	"testing"

	"github.com/agentmail-to/agentmail-cli/internal/mocktest"
)

func TestThreadsList(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"threads", "list",
			"--after", "'2019-12-27T18:11:19.117Z'",
			"--ascending=true",
			"--before", "'2019-12-27T18:11:19.117Z'",
			"--include-blocked=true",
			"--include-spam=true",
			"--include-trash=true",
			"--include-unauthenticated=true",
			"--label", "[string]",
			"--limit", "0",
			"--page-token", "page_token",
		)
	})
}

func TestThreadsDelete(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"threads", "delete",
			"--thread-id", "thread_id",
			"--permanent=true",
		)
	})
}

func TestThreadsGet(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"threads", "get",
			"--thread-id", "thread_id",
		)
	})
}

func TestThreadsGetAttachment(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"threads", "get-attachment",
			"--thread-id", "thread_id",
			"--attachment-id", "attachment_id",
		)
	})
}
