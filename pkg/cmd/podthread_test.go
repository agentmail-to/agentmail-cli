// File generated from our OpenAPI spec by Stainless. See CONTRIBUTING.md for details.

package cmd

import (
	"testing"

	"github.com/stainless-sdks/agentmail-cli/internal/mocktest"
)

func TestPodsThreadsRetrieve(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"pods:threads", "retrieve",
		"--api-key", "string",
		"--pod-id", "pod_id",
		"--thread-id", "thread_id",
	)
}

func TestPodsThreadsList(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"pods:threads", "list",
		"--api-key", "string",
		"--pod-id", "pod_id",
		"--after", "'2019-12-27T18:11:19.117Z'",
		"--ascending=true",
		"--before", "'2019-12-27T18:11:19.117Z'",
		"--include-spam=true",
		"--label", "[string]",
		"--limit", "0",
		"--page-token", "page_token",
	)
}

func TestPodsThreadsGetAttachment(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"pods:threads", "get-attachment",
		"--api-key", "string",
		"--pod-id", "pod_id",
		"--thread-id", "thread_id",
		"--attachment-id", "attachment_id",
	)
}
