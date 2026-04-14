// File generated from our OpenAPI spec by Stainless. See CONTRIBUTING.md for details.

package cmd

import (
	"testing"

	"github.com/agentmail-to/agentmail-cli/internal/mocktest"
)

func TestListsCreate(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"lists", "create",
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
			"lists", "create",
			"--direction", "send",
			"--type", "allow",
		)
	})
}

func TestListsList(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"lists", "list",
			"--direction", "send",
			"--type", "allow",
			"--limit", "0",
			"--page-token", "page_token",
		)
	})
}

func TestListsDelete(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"lists", "delete",
			"--direction", "send",
			"--type", "allow",
			"--entry", "entry",
		)
	})
}

func TestListsGet(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"lists", "get",
			"--direction", "send",
			"--type", "allow",
			"--entry", "entry",
		)
	})
}
