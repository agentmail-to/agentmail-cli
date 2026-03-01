// File generated from our OpenAPI spec by Stainless. See CONTRIBUTING.md for details.

package cmd

import (
	"testing"

	"github.com/stainless-sdks/agentmail-cli/internal/mocktest"
)

func TestPodsDomainsCreate(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"pods:domains", "create",
		"--api-key", "string",
		"--pod-id", "pod_id",
		"--domain", "domain",
		"--feedback-enabled=true",
	)
}

func TestPodsDomainsList(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"pods:domains", "list",
		"--api-key", "string",
		"--pod-id", "pod_id",
		"--limit", "0",
		"--page-token", "page_token",
	)
}

func TestPodsDomainsDelete(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"pods:domains", "delete",
		"--api-key", "string",
		"--pod-id", "pod_id",
		"--domain-id", "domain_id",
	)
}
