// File generated from our OpenAPI spec by Stainless. See CONTRIBUTING.md for details.

package cmd

import (
	"testing"

	"github.com/agentmail-to/agentmail-cli/internal/mocktest"
	"github.com/agentmail-to/agentmail-cli/internal/requestflag"
)

func TestInboxesMessagesRetrieve(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"inboxes:messages", "retrieve",
		"--api-key", "string",
		"--inbox-id", "inbox_id",
		"--message-id", "message_id",
	)
}

func TestInboxesMessagesUpdate(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"inboxes:messages", "update",
		"--api-key", "string",
		"--inbox-id", "inbox_id",
		"--message-id", "message_id",
		"--add-label", "[string]",
		"--remove-label", "[string]",
	)
}

func TestInboxesMessagesList(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"inboxes:messages", "list",
		"--api-key", "string",
		"--inbox-id", "inbox_id",
		"--after", "'2019-12-27T18:11:19.117Z'",
		"--ascending=true",
		"--before", "'2019-12-27T18:11:19.117Z'",
		"--include-blocked=true",
		"--include-spam=true",
		"--include-trash=true",
		"--label", "[string]",
		"--limit", "0",
		"--page-token", "page_token",
	)
}

func TestInboxesMessagesForward(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"inboxes:messages", "forward",
		"--api-key", "string",
		"--inbox-id", "inbox_id",
		"--message-id", "message_id",
		"--attachment", "[{content: content, content_disposition: inline, content_id: content_id, content_type: content_type, filename: filename, url: url}]",
		"--bcc", "string",
		"--cc", "string",
		"--headers", "{foo: string}",
		"--html", "html",
		"--label", "[string]",
		"--reply-to", "string",
		"--subject", "subject",
		"--text", "text",
		"--to", "string",
	)

	// Check that inner flags have been set up correctly
	requestflag.CheckInnerFlags(inboxesMessagesForward)

	// Alternative argument passing style using inner flags
	mocktest.TestRunMockTestWithFlags(
		t,
		"inboxes:messages", "forward",
		"--inbox-id", "inbox_id",
		"--message-id", "message_id",
		"--attachment.content", "content",
		"--attachment.content-disposition", "inline",
		"--attachment.content-id", "content_id",
		"--attachment.content-type", "content_type",
		"--attachment.filename", "filename",
		"--attachment.url", "url",
		"--bcc", "string",
		"--cc", "string",
		"--headers", "{foo: string}",
		"--html", "html",
		"--label", "[string]",
		"--reply-to", "string",
		"--subject", "subject",
		"--text", "text",
		"--to", "string",
	)
}

func TestInboxesMessagesGetAttachment(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"inboxes:messages", "get-attachment",
		"--api-key", "string",
		"--inbox-id", "inbox_id",
		"--message-id", "message_id",
		"--attachment-id", "attachment_id",
	)
}

func TestInboxesMessagesGetRaw(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"inboxes:messages", "get-raw",
		"--api-key", "string",
		"--inbox-id", "inbox_id",
		"--message-id", "message_id",
	)
}

func TestInboxesMessagesReply(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"inboxes:messages", "reply",
		"--api-key", "string",
		"--inbox-id", "inbox_id",
		"--message-id", "message_id",
		"--attachment", "[{content: content, content_disposition: inline, content_id: content_id, content_type: content_type, filename: filename, url: url}]",
		"--bcc", "string",
		"--cc", "string",
		"--headers", "{foo: string}",
		"--html", "html",
		"--label", "[string]",
		"--reply-all=true",
		"--reply-to", "string",
		"--text", "text",
		"--to", "string",
	)

	// Check that inner flags have been set up correctly
	requestflag.CheckInnerFlags(inboxesMessagesReply)

	// Alternative argument passing style using inner flags
	mocktest.TestRunMockTestWithFlags(
		t,
		"inboxes:messages", "reply",
		"--inbox-id", "inbox_id",
		"--message-id", "message_id",
		"--attachment.content", "content",
		"--attachment.content-disposition", "inline",
		"--attachment.content-id", "content_id",
		"--attachment.content-type", "content_type",
		"--attachment.filename", "filename",
		"--attachment.url", "url",
		"--bcc", "string",
		"--cc", "string",
		"--headers", "{foo: string}",
		"--html", "html",
		"--label", "[string]",
		"--reply-all=true",
		"--reply-to", "string",
		"--text", "text",
		"--to", "string",
	)
}

func TestInboxesMessagesReplyAll(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"inboxes:messages", "reply-all",
		"--api-key", "string",
		"--inbox-id", "inbox_id",
		"--message-id", "message_id",
		"--attachment", "[{content: content, content_disposition: inline, content_id: content_id, content_type: content_type, filename: filename, url: url}]",
		"--headers", "{foo: string}",
		"--html", "html",
		"--label", "[string]",
		"--reply-to", "string",
		"--text", "text",
	)

	// Check that inner flags have been set up correctly
	requestflag.CheckInnerFlags(inboxesMessagesReplyAll)

	// Alternative argument passing style using inner flags
	mocktest.TestRunMockTestWithFlags(
		t,
		"inboxes:messages", "reply-all",
		"--inbox-id", "inbox_id",
		"--message-id", "message_id",
		"--attachment.content", "content",
		"--attachment.content-disposition", "inline",
		"--attachment.content-id", "content_id",
		"--attachment.content-type", "content_type",
		"--attachment.filename", "filename",
		"--attachment.url", "url",
		"--headers", "{foo: string}",
		"--html", "html",
		"--label", "[string]",
		"--reply-to", "string",
		"--text", "text",
	)
}

func TestInboxesMessagesSend(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	mocktest.TestRunMockTestWithFlags(
		t,
		"inboxes:messages", "send",
		"--api-key", "string",
		"--inbox-id", "inbox_id",
		"--attachment", "[{content: content, content_disposition: inline, content_id: content_id, content_type: content_type, filename: filename, url: url}]",
		"--bcc", "string",
		"--cc", "string",
		"--headers", "{foo: string}",
		"--html", "html",
		"--label", "[string]",
		"--reply-to", "string",
		"--subject", "subject",
		"--text", "text",
		"--to", "string",
	)

	// Check that inner flags have been set up correctly
	requestflag.CheckInnerFlags(inboxesMessagesSend)

	// Alternative argument passing style using inner flags
	mocktest.TestRunMockTestWithFlags(
		t,
		"inboxes:messages", "send",
		"--inbox-id", "inbox_id",
		"--attachment.content", "content",
		"--attachment.content-disposition", "inline",
		"--attachment.content-id", "content_id",
		"--attachment.content-type", "content_type",
		"--attachment.filename", "filename",
		"--attachment.url", "url",
		"--bcc", "string",
		"--cc", "string",
		"--headers", "{foo: string}",
		"--html", "html",
		"--label", "[string]",
		"--reply-to", "string",
		"--subject", "subject",
		"--text", "text",
		"--to", "string",
	)
}
