// File generated from our OpenAPI spec by Stainless. See CONTRIBUTING.md for details.

package cmd

import (
	"testing"

	"github.com/agentmail-to/agentmail-cli/internal/mocktest"
	"github.com/agentmail-to/agentmail-cli/internal/requestflag"
)

func TestAPIKeysCreate(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"api-keys", "create",
			"--name", "name",
			"--permissions", "{create_api_key: true, create_domain: true, create_draft: true, create_inbox: true, create_list_entry: true, create_pod: true, create_webhook: true, delete_api_key: true, delete_domain: true, delete_draft: true, delete_inbox: true, delete_list_entry: true, delete_pod: true, delete_thread: true, delete_webhook: true, read_api_key: true, read_blocked: true, read_domain: true, read_draft: true, read_inbox: true, read_list_entry: true, read_message: true, read_metrics: true, read_pod: true, read_spam: true, read_thread: true, read_trash: true, read_webhook: true, send_draft: true, send_message: true, update_domain: true, update_draft: true, update_inbox: true, update_message: true, update_webhook: true}",
		)
	})

	t.Run("inner flags", func(t *testing.T) {
		// Check that inner flags have been set up correctly
		requestflag.CheckInnerFlags(apiKeysCreate)

		// Alternative argument passing style using inner flags
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"api-keys", "create",
			"--name", "name",
			"--permissions.create-api-key=true",
			"--permissions.create-domain=true",
			"--permissions.create-draft=true",
			"--permissions.create-inbox=true",
			"--permissions.create-list-entry=true",
			"--permissions.create-pod=true",
			"--permissions.create-webhook=true",
			"--permissions.delete-api-key=true",
			"--permissions.delete-domain=true",
			"--permissions.delete-draft=true",
			"--permissions.delete-inbox=true",
			"--permissions.delete-list-entry=true",
			"--permissions.delete-pod=true",
			"--permissions.delete-thread=true",
			"--permissions.delete-webhook=true",
			"--permissions.read-api-key=true",
			"--permissions.read-blocked=true",
			"--permissions.read-domain=true",
			"--permissions.read-draft=true",
			"--permissions.read-inbox=true",
			"--permissions.read-list-entry=true",
			"--permissions.read-message=true",
			"--permissions.read-metrics=true",
			"--permissions.read-pod=true",
			"--permissions.read-spam=true",
			"--permissions.read-thread=true",
			"--permissions.read-trash=true",
			"--permissions.read-webhook=true",
			"--permissions.send-draft=true",
			"--permissions.send-message=true",
			"--permissions.update-domain=true",
			"--permissions.update-draft=true",
			"--permissions.update-inbox=true",
			"--permissions.update-message=true",
			"--permissions.update-webhook=true",
		)
	})

	t.Run("piping data", func(t *testing.T) {
		// Test piping YAML data over stdin
		pipeData := []byte("" +
			"name: name\n" +
			"permissions:\n" +
			"  create_api_key: true\n" +
			"  create_domain: true\n" +
			"  create_draft: true\n" +
			"  create_inbox: true\n" +
			"  create_list_entry: true\n" +
			"  create_pod: true\n" +
			"  create_webhook: true\n" +
			"  delete_api_key: true\n" +
			"  delete_domain: true\n" +
			"  delete_draft: true\n" +
			"  delete_inbox: true\n" +
			"  delete_list_entry: true\n" +
			"  delete_pod: true\n" +
			"  delete_thread: true\n" +
			"  delete_webhook: true\n" +
			"  read_api_key: true\n" +
			"  read_blocked: true\n" +
			"  read_domain: true\n" +
			"  read_draft: true\n" +
			"  read_inbox: true\n" +
			"  read_list_entry: true\n" +
			"  read_message: true\n" +
			"  read_metrics: true\n" +
			"  read_pod: true\n" +
			"  read_spam: true\n" +
			"  read_thread: true\n" +
			"  read_trash: true\n" +
			"  read_webhook: true\n" +
			"  send_draft: true\n" +
			"  send_message: true\n" +
			"  update_domain: true\n" +
			"  update_draft: true\n" +
			"  update_inbox: true\n" +
			"  update_message: true\n" +
			"  update_webhook: true\n")
		mocktest.TestRunMockTestWithPipeAndFlags(
			t, pipeData,
			"--api-key", "string",
			"api-keys", "create",
		)
	})
}

func TestAPIKeysList(t *testing.T) {
	t.Skip("Mock server tests are disabled")
	t.Run("regular flags", func(t *testing.T) {
		mocktest.TestRunMockTestWithFlags(
			t,
			"--api-key", "string",
			"api-keys", "list",
			"--ascending=true",
			"--limit", "0",
			"--page-token", "page_token",
		)
	})
}
