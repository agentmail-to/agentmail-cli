// File generated from our OpenAPI spec by Stainless. See CONTRIBUTING.md for details.

package cmd

import (
	"testing"

	"github.com/agentmail-to/agentmail-cli/internal/mocktest"
)

func TestPodsDomainsCreate(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"pods:domains", "create",
			"--pod-id", "pod_id",
			"--domain", "domain",
			"--feedback-enabled=true",
		)
	})

	t.Run("piping data", func(t *testing.T) {
		// Test piping YAML data over stdin
		pipeData := []byte("" +
			"domain: domain\n" +
			"feedback_enabled: true\n")
		mocktest.TestRunMockTestWithPipeAndFlags(
			t, pipeData,
			"--api-key", "string",
			"pods:domains", "create",
			"--pod-id", "pod_id",
		)
	})
}

func TestPodsDomainsList(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"pods:domains", "list",
			"--pod-id", "pod_id",
			"--ascending=true",
			"--limit", "0",
			"--page-token", "page_token",
		)
	})
}

func TestPodsDomainsDelete(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"pods:domains", "delete",
			"--pod-id", "pod_id",
			"--domain-id", "domain_id",
		)
	})
}
