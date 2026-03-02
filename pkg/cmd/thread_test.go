// File generated from our OpenAPI spec by Stainless. See CONTRIBUTING.md for details.

package cmd

import (
	"testing"

	"github.com/agentmail-to/agentmail-cli/internal/mocktest"
)

func TestThreadsRetrieve(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"threads", "retrieve",
		"--api-key", "string",
		"--thread-id", "thread_id",
	)
}

func TestThreadsList(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"threads", "list",
		"--api-key", "string",
		"--after", "'2019-12-27T18:11:19.117Z'",
		"--ascending=true",
		"--before", "'2019-12-27T18:11:19.117Z'",
		"--include-spam=true",
		"--label", "[string]",
		"--limit", "0",
		"--page-token", "page_token",
	)
}

func TestThreadsRetrieveAttachment(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"threads", "retrieve-attachment",
		"--api-key", "string",
		"--thread-id", "thread_id",
		"--attachment-id", "attachment_id",
	)
}
