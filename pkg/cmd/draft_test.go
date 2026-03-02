// File generated from our OpenAPI spec by Stainless. See CONTRIBUTING.md for details.

package cmd

import (
	"testing"

	"github.com/stainless-sdks/agentmail-cli/internal/mocktest"
)

func TestDraftsRetrieve(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"drafts", "retrieve",
		"--api-key", "string",
		"--draft-id", "draft_id",
	)
}

func TestDraftsList(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"drafts", "list",
		"--api-key", "string",
		"--after", "'2019-12-27T18:11:19.117Z'",
		"--ascending=true",
		"--before", "'2019-12-27T18:11:19.117Z'",
		"--label", "[string]",
		"--limit", "0",
		"--page-token", "page_token",
	)
}
