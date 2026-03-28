// File generated from our OpenAPI spec by Stainless. See CONTRIBUTING.md for details.

package cmd

import (
	"testing"

	"github.com/agentmail-to/agentmail-cli/internal/mocktest"
)

func TestMetricsList(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"metrics", "list",
			"--end-timestamp", "'2019-12-27T18:11:19.117Z'",
			"--start-timestamp", "'2019-12-27T18:11:19.117Z'",
			"--event-type", "[message.sent]",
		)
	})
}
