// File generated from our OpenAPI spec by Stainless. See CONTRIBUTING.md for details.

package cmd

import (
	"testing"

	"github.com/stainless-sdks/agentmail-cli/internal/mocktest"
)

func TestMetricsList(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"metrics", "list",
		"--api-key", "string",
		"--end-timestamp", "'2019-12-27T18:11:19.117Z'",
		"--start-timestamp", "'2019-12-27T18:11:19.117Z'",
		"--event-type", "[message.sent]",
	)
}
