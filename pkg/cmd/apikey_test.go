// File generated from our OpenAPI spec by Stainless. See CONTRIBUTING.md for details.

package cmd

import (
	"testing"

	"github.com/stainless-sdks/agentmail-cli/internal/mocktest"
)

func TestAPIKeysCreate(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"api-keys", "create",
		"--api-key", "string",
		"--name", "name",
	)
}

func TestAPIKeysList(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"api-keys", "list",
		"--api-key", "string",
		"--limit", "0",
		"--page-token", "page_token",
	)
}

func TestAPIKeysDelete(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"api-keys", "delete",
		"--api-key", "string",
		"--api-key", "api_key",
	)
}
