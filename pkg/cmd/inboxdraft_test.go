// File generated from our OpenAPI spec by Stainless. See CONTRIBUTING.md for details.

package cmd

import (
	"testing"

	"github.com/agentmail-to/agentmail-cli/internal/mocktest"
)

func TestInboxesDraftsCreate(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"inboxes:drafts", "create",
			"--inbox-id", "inbox_id",
			"--bcc", "[string]",
			"--cc", "[string]",
			"--client-id", "client_id",
			"--html", "html",
			"--in-reply-to", "in_reply_to",
			"--label", "[string]",
			"--reply-to", "[string]",
			"--send-at", "'2019-12-27T18:11:19.117Z'",
			"--subject", "subject",
			"--text", "text",
			"--to", "[string]",
		)
	})

	t.Run("piping data", func(t *testing.T) {
		// Test piping YAML data over stdin
		pipeData := []byte("" +
			"bcc:\n" +
			"  - string\n" +
			"cc:\n" +
			"  - string\n" +
			"client_id: client_id\n" +
			"html: html\n" +
			"in_reply_to: in_reply_to\n" +
			"labels:\n" +
			"  - string\n" +
			"reply_to:\n" +
			"  - string\n" +
			"send_at: '2019-12-27T18:11:19.117Z'\n" +
			"subject: subject\n" +
			"text: text\n" +
			"to:\n" +
			"  - string\n")
		mocktest.TestRunMockTestWithPipeAndFlags(
			t, pipeData,
			"--api-key", "string",
			"inboxes:drafts", "create",
			"--inbox-id", "inbox_id",
		)
	})
}

func TestInboxesDraftsRetrieve(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"inboxes:drafts", "retrieve",
			"--inbox-id", "inbox_id",
			"--draft-id", "draft_id",
		)
	})
}

func TestInboxesDraftsUpdate(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"inboxes:drafts", "update",
			"--inbox-id", "inbox_id",
			"--draft-id", "draft_id",
			"--bcc", "[string]",
			"--cc", "[string]",
			"--html", "html",
			"--reply-to", "[string]",
			"--send-at", "'2019-12-27T18:11:19.117Z'",
			"--subject", "subject",
			"--text", "text",
			"--to", "[string]",
		)
	})

	t.Run("piping data", func(t *testing.T) {
		// Test piping YAML data over stdin
		pipeData := []byte("" +
			"bcc:\n" +
			"  - string\n" +
			"cc:\n" +
			"  - string\n" +
			"html: html\n" +
			"reply_to:\n" +
			"  - string\n" +
			"send_at: '2019-12-27T18:11:19.117Z'\n" +
			"subject: subject\n" +
			"text: text\n" +
			"to:\n" +
			"  - string\n")
		mocktest.TestRunMockTestWithPipeAndFlags(
			t, pipeData,
			"--api-key", "string",
			"inboxes:drafts", "update",
			"--inbox-id", "inbox_id",
			"--draft-id", "draft_id",
		)
	})
}

func TestInboxesDraftsList(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"inboxes:drafts", "list",
			"--inbox-id", "inbox_id",
			"--after", "'2019-12-27T18:11:19.117Z'",
			"--ascending=true",
			"--before", "'2019-12-27T18:11:19.117Z'",
			"--label", "[string]",
			"--limit", "0",
			"--page-token", "page_token",
		)
	})
}

func TestInboxesDraftsDelete(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"inboxes:drafts", "delete",
			"--inbox-id", "inbox_id",
			"--draft-id", "draft_id",
		)
	})
}

func TestInboxesDraftsSend(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"inboxes:drafts", "send",
			"--inbox-id", "inbox_id",
			"--draft-id", "draft_id",
			"--add-label", "[string]",
			"--remove-label", "[string]",
		)
	})

	t.Run("piping data", func(t *testing.T) {
		// Test piping YAML data over stdin
		pipeData := []byte("" +
			"add_labels:\n" +
			"  - string\n" +
			"remove_labels:\n" +
			"  - string\n")
		mocktest.TestRunMockTestWithPipeAndFlags(
			t, pipeData,
			"--api-key", "string",
			"inboxes:drafts", "send",
			"--inbox-id", "inbox_id",
			"--draft-id", "draft_id",
		)
	})
}
