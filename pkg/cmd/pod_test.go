// File generated from our OpenAPI spec by Stainless. See CONTRIBUTING.md for details.

package cmd

import (
	"testing"

	"github.com/stainless-sdks/agentmail-cli/internal/mocktest"
)

func TestPodsCreate(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"pods", "create",
		"--api-key", "string",
		"--client-id", "client_id",
		"--name", "name",
	)
}

func TestPodsRetrieve(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"pods", "retrieve",
		"--api-key", "string",
		"--pod-id", "pod_id",
	)
}

func TestPodsList(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"pods", "list",
		"--api-key", "string",
		"--limit", "0",
		"--page-token", "page_token",
	)
}

func TestPodsDelete(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"pods", "delete",
		"--api-key", "string",
		"--pod-id", "pod_id",
	)
}
