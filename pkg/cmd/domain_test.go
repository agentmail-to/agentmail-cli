// File generated from our OpenAPI spec by Stainless. See CONTRIBUTING.md for details.

package cmd

import (
	"testing"

	"github.com/agentmail-to/agentmail-cli/internal/mocktest"
)

func TestDomainsCreate(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t, "domains", "create",
			"--api-key", "string",
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
			t, pipeData, "domains", "create",
			"--api-key", "string",
		)
	})
}

func TestDomainsRetrieve(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t, "domains", "retrieve",
			"--api-key", "string",
			"--domain-id", "domain_id",
		)
	})
}

func TestDomainsList(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t, "domains", "list",
			"--api-key", "string",
			"--ascending=true",
			"--limit", "0",
			"--page-token", "page_token",
		)
	})
}

func TestDomainsDelete(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t, "domains", "delete",
			"--api-key", "string",
			"--domain-id", "domain_id",
		)
	})
}

func TestDomainsGetZoneFile(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t, "domains", "get-zone-file",
			"--api-key", "string",
			"--domain-id", "domain_id",
		)
	})
}

func TestDomainsVerify(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t, "domains", "verify",
			"--api-key", "string",
			"--domain-id", "domain_id",
		)
	})
}
