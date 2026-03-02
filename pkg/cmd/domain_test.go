// File generated from our OpenAPI spec by Stainless. See CONTRIBUTING.md for details.

package cmd

import (
	"testing"

	"github.com/stainless-sdks/agentmail-cli/internal/mocktest"
)

func TestDomainsCreate(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"domains", "create",
		"--api-key", "string",
		"--domain", "domain",
		"--feedback-enabled=true",
	)
}

func TestDomainsRetrieve(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"domains", "retrieve",
		"--api-key", "string",
		"--domain-id", "domain_id",
	)
}

func TestDomainsList(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"domains", "list",
		"--api-key", "string",
		"--limit", "0",
		"--page-token", "page_token",
	)
}

func TestDomainsDelete(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"domains", "delete",
		"--api-key", "string",
		"--domain-id", "domain_id",
	)
}

func TestDomainsGetZoneFile(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"domains", "get-zone-file",
		"--api-key", "string",
		"--domain-id", "domain_id",
	)
}

func TestDomainsVerify(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"domains", "verify",
		"--api-key", "string",
		"--domain-id", "domain_id",
	)
}
