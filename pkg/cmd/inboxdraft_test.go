// File generated from our OpenAPI spec by Stainless. See CONTRIBUTING.md for details.

package cmd

import (
	"testing"

	"github.com/agentmail-to/agentmail-cli/internal/mocktest"
	"github.com/agentmail-to/agentmail-cli/internal/requestflag"
)

func TestInboxesDraftsCreate(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"inboxes:drafts", "create",
		"--api-key", "string",
		"--inbox-id", "inbox_id",
		"--attachment", "[{content: content, content_disposition: inline, content_id: content_id, content_type: content_type, filename: filename, url: url}]",
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

	// Check that inner flags have been set up correctly
	requestflag.CheckInnerFlags(inboxesDraftsCreate)

	// Alternative argument passing style using inner flags
	mocktest.TestRunMockTestWithFlags(
		t,
		"inboxes:drafts", "create",
		"--inbox-id", "inbox_id",
		"--attachment.content", "content",
		"--attachment.content-disposition", "inline",
		"--attachment.content-id", "content_id",
		"--attachment.content-type", "content_type",
		"--attachment.filename", "filename",
		"--attachment.url", "url",
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
}

func TestInboxesDraftsRetrieve(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"inboxes:drafts", "retrieve",
		"--api-key", "string",
		"--inbox-id", "inbox_id",
		"--draft-id", "draft_id",
	)
}

func TestInboxesDraftsUpdate(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"inboxes:drafts", "update",
		"--api-key", "string",
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
}

func TestInboxesDraftsList(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"inboxes:drafts", "list",
		"--api-key", "string",
		"--inbox-id", "inbox_id",
		"--after", "'2019-12-27T18:11:19.117Z'",
		"--ascending=true",
		"--before", "'2019-12-27T18:11:19.117Z'",
		"--label", "[string]",
		"--limit", "0",
		"--page-token", "page_token",
	)
}

func TestInboxesDraftsDelete(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"inboxes:drafts", "delete",
		"--api-key", "string",
		"--inbox-id", "inbox_id",
		"--draft-id", "draft_id",
	)
}

func TestInboxesDraftsSend(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"inboxes:drafts", "send",
		"--api-key", "string",
		"--inbox-id", "inbox_id",
		"--draft-id", "draft_id",
		"--add-label", "[string]",
		"--remove-label", "[string]",
	)
}
