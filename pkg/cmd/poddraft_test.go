// File generated from our OpenAPI spec by Stainless. See CONTRIBUTING.md for details.

package cmd

import (
	"testing"

	"github.com/agentmail-to/agentmail-cli/internal/mocktest"
)

func TestPodsDraftsRetrieve(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"pods:drafts", "retrieve",
			"--pod-id", "pod_id",
			"--draft-id", "draft_id",
		)
	})
}

func TestPodsDraftsList(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"pods:drafts", "list",
			"--pod-id", "pod_id",
			"--after", "'2019-12-27T18:11:19.117Z'",
			"--ascending=true",
			"--before", "'2019-12-27T18:11:19.117Z'",
			"--label", "[string]",
			"--limit", "0",
			"--page-token", "page_token",
		)
	})
}

func TestPodsDraftsGetAttachment(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"pods:drafts", "get-attachment",
			"--pod-id", "pod_id",
			"--draft-id", "draft_id",
			"--attachment-id", "attachment_id",
		)
	})
}
