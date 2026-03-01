// File generated from our OpenAPI spec by Stainless. See CONTRIBUTING.md for details.

package cmd

import (
	"testing"

	"github.com/stainless-sdks/agentmail-cli/internal/mocktest"
)

func TestPodsInboxesCreate(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"pods:inboxes", "create",
		"--api-key", "string",
		"--pod-id", "pod_id",
		"--client-id", "client_id",
		"--display-name", "display_name",
		"--domain", "domain",
		"--username", "username",
	)
}

func TestPodsInboxesRetrieve(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"pods:inboxes", "retrieve",
		"--api-key", "string",
		"--pod-id", "pod_id",
		"--inbox-id", "inbox_id",
	)
}

func TestPodsInboxesList(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"pods:inboxes", "list",
		"--api-key", "string",
		"--pod-id", "pod_id",
		"--limit", "0",
		"--page-token", "page_token",
	)
}

func TestPodsInboxesDelete(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"pods:inboxes", "delete",
		"--api-key", "string",
		"--pod-id", "pod_id",
		"--inbox-id", "inbox_id",
	)
}
