// File generated from our OpenAPI spec by Stainless. See CONTRIBUTING.md for details.

package cmd

import (
	"testing"

	"github.com/agentmail-to/agentmail-cli/internal/mocktest"
)

func TestPodsInboxesCreate(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t, "pods:inboxes", "create",
			"--api-key", "string",
			"--pod-id", "pod_id",
			"--client-id", "client_id",
			"--display-name", "display_name",
			"--domain", "domain",
			"--username", "username",
		)
	})

	t.Run("piping data", func(t *testing.T) {
		// Test piping YAML data over stdin
		pipeData := []byte("" +
			"client_id: client_id\n" +
			"display_name: display_name\n" +
			"domain: domain\n" +
			"username: username\n")
		mocktest.TestRunMockTestWithPipeAndFlags(
			t, pipeData, "pods:inboxes", "create",
			"--api-key", "string",
			"--pod-id", "pod_id",
		)
	})
}

func TestPodsInboxesRetrieve(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t, "pods:inboxes", "retrieve",
			"--api-key", "string",
			"--pod-id", "pod_id",
			"--inbox-id", "inbox_id",
		)
	})
}

func TestPodsInboxesList(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t, "pods:inboxes", "list",
			"--api-key", "string",
			"--pod-id", "pod_id",
			"--ascending=true",
			"--limit", "0",
			"--page-token", "page_token",
		)
	})
}

func TestPodsInboxesDelete(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t, "pods:inboxes", "delete",
			"--api-key", "string",
			"--pod-id", "pod_id",
			"--inbox-id", "inbox_id",
		)
	})
}
