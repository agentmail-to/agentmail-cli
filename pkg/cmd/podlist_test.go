// File generated from our OpenAPI spec by Stainless. See CONTRIBUTING.md for details.

package cmd

import (
	"testing"

	"github.com/agentmail-to/agentmail-cli/internal/mocktest"
)

func TestPodsListsCreate(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"pods:lists", "create",
			"--pod-id", "pod_id",
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
			"pods:lists", "create",
			"--pod-id", "pod_id",
			"--direction", "send",
			"--type", "allow",
		)
	})
}

func TestPodsListsList(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"pods:lists", "list",
			"--pod-id", "pod_id",
			"--direction", "send",
			"--type", "allow",
			"--limit", "0",
			"--page-token", "page_token",
		)
	})
}

func TestPodsListsDelete(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"pods:lists", "delete",
			"--pod-id", "pod_id",
			"--direction", "send",
			"--type", "allow",
			"--entry", "entry",
		)
	})
}

func TestPodsListsGet(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"pods:lists", "get",
			"--pod-id", "pod_id",
			"--direction", "send",
			"--type", "allow",
			"--entry", "entry",
		)
	})
}
