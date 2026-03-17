// File generated from our OpenAPI spec by Stainless. See CONTRIBUTING.md for details.

package cmd

import (
	"testing"

	"github.com/agentmail-to/agentmail-cli/internal/mocktest"
	"github.com/agentmail-to/agentmail-cli/internal/requestflag"
)

func TestInboxesMessagesRetrieve(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"inboxes:messages", "retrieve",
			"--inbox-id", "inbox_id",
			"--message-id", "message_id",
		)
	})
}

func TestInboxesMessagesUpdate(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"inboxes:messages", "update",
			"--inbox-id", "inbox_id",
			"--message-id", "message_id",
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
			"inboxes:messages", "update",
			"--inbox-id", "inbox_id",
			"--message-id", "message_id",
		)
	})
}

func TestInboxesMessagesList(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"inboxes:messages", "list",
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
	})
}

func TestInboxesMessagesForward(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"inboxes:messages", "forward",
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
	})

	t.Run("inner flags", func(t *testing.T) {
		// Check that inner flags have been set up correctly
		requestflag.CheckInnerFlags(inboxesMessagesForward)

		// Alternative argument passing style using inner flags
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
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
	})

	t.Run("piping data", func(t *testing.T) {
		// Test piping YAML data over stdin
		pipeData := []byte("" +
			"attachments:\n" +
			"  - content: content\n" +
			"    content_disposition: inline\n" +
			"    content_id: content_id\n" +
			"    content_type: content_type\n" +
			"    filename: filename\n" +
			"    url: url\n" +
			"bcc: string\n" +
			"cc: string\n" +
			"headers:\n" +
			"  foo: string\n" +
			"html: html\n" +
			"labels:\n" +
			"  - string\n" +
			"reply_to: string\n" +
			"subject: subject\n" +
			"text: text\n" +
			"to: string\n")
		mocktest.TestRunMockTestWithPipeAndFlags(
			t, pipeData,
			"--api-key", "string",
			"inboxes:messages", "forward",
			"--inbox-id", "inbox_id",
			"--message-id", "message_id",
		)
	})
}

func TestInboxesMessagesGetAttachment(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"inboxes:messages", "get-attachment",
			"--inbox-id", "inbox_id",
			"--message-id", "message_id",
			"--attachment-id", "attachment_id",
		)
	})
}

func TestInboxesMessagesGetRaw(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"inboxes:messages", "get-raw",
			"--inbox-id", "inbox_id",
			"--message-id", "message_id",
		)
	})
}

func TestInboxesMessagesReply(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"inboxes:messages", "reply",
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
	})

	t.Run("inner flags", func(t *testing.T) {
		// Check that inner flags have been set up correctly
		requestflag.CheckInnerFlags(inboxesMessagesReply)

		// Alternative argument passing style using inner flags
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
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
	})

	t.Run("piping data", func(t *testing.T) {
		// Test piping YAML data over stdin
		pipeData := []byte("" +
			"attachments:\n" +
			"  - content: content\n" +
			"    content_disposition: inline\n" +
			"    content_id: content_id\n" +
			"    content_type: content_type\n" +
			"    filename: filename\n" +
			"    url: url\n" +
			"bcc: string\n" +
			"cc: string\n" +
			"headers:\n" +
			"  foo: string\n" +
			"html: html\n" +
			"labels:\n" +
			"  - string\n" +
			"reply_all: true\n" +
			"reply_to: string\n" +
			"text: text\n" +
			"to: string\n")
		mocktest.TestRunMockTestWithPipeAndFlags(
			t, pipeData,
			"--api-key", "string",
			"inboxes:messages", "reply",
			"--inbox-id", "inbox_id",
			"--message-id", "message_id",
		)
	})
}

func TestInboxesMessagesReplyAll(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"inboxes:messages", "reply-all",
			"--inbox-id", "inbox_id",
			"--message-id", "message_id",
			"--attachment", "[{content: content, content_disposition: inline, content_id: content_id, content_type: content_type, filename: filename, url: url}]",
			"--headers", "{foo: string}",
			"--html", "html",
			"--label", "[string]",
			"--reply-to", "string",
			"--text", "text",
		)
	})

	t.Run("inner flags", func(t *testing.T) {
		// Check that inner flags have been set up correctly
		requestflag.CheckInnerFlags(inboxesMessagesReplyAll)

		// Alternative argument passing style using inner flags
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
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
	})

	t.Run("piping data", func(t *testing.T) {
		// Test piping YAML data over stdin
		pipeData := []byte("" +
			"attachments:\n" +
			"  - content: content\n" +
			"    content_disposition: inline\n" +
			"    content_id: content_id\n" +
			"    content_type: content_type\n" +
			"    filename: filename\n" +
			"    url: url\n" +
			"headers:\n" +
			"  foo: string\n" +
			"html: html\n" +
			"labels:\n" +
			"  - string\n" +
			"reply_to: string\n" +
			"text: text\n")
		mocktest.TestRunMockTestWithPipeAndFlags(
			t, pipeData,
			"--api-key", "string",
			"inboxes:messages", "reply-all",
			"--inbox-id", "inbox_id",
			"--message-id", "message_id",
		)
	})
}

func TestInboxesMessagesSend(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"inboxes:messages", "send",
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
	})

	t.Run("inner flags", func(t *testing.T) {
		// Check that inner flags have been set up correctly
		requestflag.CheckInnerFlags(inboxesMessagesSend)

		// Alternative argument passing style using inner flags
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
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
	})

	t.Run("piping data", func(t *testing.T) {
		// Test piping YAML data over stdin
		pipeData := []byte("" +
			"attachments:\n" +
			"  - content: content\n" +
			"    content_disposition: inline\n" +
			"    content_id: content_id\n" +
			"    content_type: content_type\n" +
			"    filename: filename\n" +
			"    url: url\n" +
			"bcc: string\n" +
			"cc: string\n" +
			"headers:\n" +
			"  foo: string\n" +
			"html: html\n" +
			"labels:\n" +
			"  - string\n" +
			"reply_to: string\n" +
			"subject: subject\n" +
			"text: text\n" +
			"to: string\n")
		mocktest.TestRunMockTestWithPipeAndFlags(
			t, pipeData,
			"--api-key", "string",
			"inboxes:messages", "send",
			"--inbox-id", "inbox_id",
		)
	})
}
