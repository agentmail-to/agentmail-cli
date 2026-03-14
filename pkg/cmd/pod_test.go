// File generated from our OpenAPI spec by Stainless. See CONTRIBUTING.md for details.

package cmd

import (
	"testing"

	"github.com/agentmail-to/agentmail-cli/internal/mocktest"
)

func TestPodsCreate(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t, "pods", "create",
			"--api-key", "string",
			"--client-id", "client_id",
			"--name", "name",
		)
	})

	t.Run("piping data", func(t *testing.T) {
		// Test piping YAML data over stdin
		pipeData := []byte("" +
			"client_id: client_id\n" +
			"name: name\n")
		mocktest.TestRunMockTestWithPipeAndFlags(
			t, pipeData, "pods", "create",
			"--api-key", "string",
		)
	})
}

func TestPodsRetrieve(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t, "pods", "retrieve",
			"--api-key", "string",
			"--pod-id", "pod_id",
		)
	})
}

func TestPodsList(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t, "pods", "list",
			"--api-key", "string",
			"--ascending=true",
			"--limit", "0",
			"--page-token", "page_token",
		)
	})
}

func TestPodsDelete(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t, "pods", "delete",
			"--api-key", "string",
			"--pod-id", "pod_id",
		)
	})
}
