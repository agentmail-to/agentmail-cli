// File generated from our OpenAPI spec by Stainless. See CONTRIBUTING.md for details.

package cmd

import (
	"context"
	"fmt"
	"os"

	"github.com/agentmail-to/agentmail-cli/internal/apiquery"
	"github.com/agentmail-to/agentmail-cli/internal/requestflag"
	"github.com/agentmail-to/agentmail-go"
	"github.com/agentmail-to/agentmail-go/option"
	"github.com/tidwall/gjson"
	"github.com/urfave/cli/v3"
)

var apiKeysCreate = requestflag.WithInnerFlags(cli.Command{
	Name:    "create",
	Usage:   "Create API Key",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:     "name",
			Usage:    "Name of api key.",
			Required: true,
			BodyPath: "name",
		},
		&requestflag.Flag[any]{
			Name:     "permissions",
			Usage:    "Granular permissions for the API key. When ommitted all permissions are granted. Otherwise, only permissions set to true are granted.",
			BodyPath: "permissions",
		},
	},
	Action:          handleAPIKeysCreate,
	HideHelpCommand: true,
}, map[string][]requestflag.HasOuterFlag{
	"permissions": {
		&requestflag.InnerFlag[any]{
			Name:       "permissions.create-api-key",
			Usage:      "Create API keys.",
			InnerField: "create_api_key",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.create-domain",
			Usage:      "Create domains.",
			InnerField: "create_domain",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.create-draft",
			Usage:      "Create drafts.",
			InnerField: "create_draft",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.create-inbox",
			Usage:      "Create new inboxes.",
			InnerField: "create_inbox",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.create-list-entry",
			Usage:      "Create list entries.",
			InnerField: "create_list_entry",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.create-pod",
			Usage:      "Create pods.",
			InnerField: "create_pod",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.create-webhook",
			Usage:      "Create webhooks.",
			InnerField: "create_webhook",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.delete-api-key",
			Usage:      "Delete API keys.",
			InnerField: "delete_api_key",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.delete-domain",
			Usage:      "Delete domains.",
			InnerField: "delete_domain",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.delete-draft",
			Usage:      "Delete drafts.",
			InnerField: "delete_draft",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.delete-inbox",
			Usage:      "Delete inboxes.",
			InnerField: "delete_inbox",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.delete-list-entry",
			Usage:      "Delete list entries.",
			InnerField: "delete_list_entry",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.delete-pod",
			Usage:      "Delete pods.",
			InnerField: "delete_pod",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.delete-thread",
			Usage:      "Delete threads.",
			InnerField: "delete_thread",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.delete-webhook",
			Usage:      "Delete webhooks.",
			InnerField: "delete_webhook",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.read-api-key",
			Usage:      "Read API keys.",
			InnerField: "read_api_key",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.read-blocked",
			Usage:      "Access messages labeled blocked.",
			InnerField: "read_blocked",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.read-domain",
			Usage:      "Read domain details.",
			InnerField: "read_domain",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.read-draft",
			Usage:      "Read drafts.",
			InnerField: "read_draft",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.read-inbox",
			Usage:      "Read inbox details.",
			InnerField: "read_inbox",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.read-list-entry",
			Usage:      "Read list entries.",
			InnerField: "read_list_entry",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.read-message",
			Usage:      "Read messages.",
			InnerField: "read_message",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.read-metrics",
			Usage:      "Read metrics.",
			InnerField: "read_metrics",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.read-pod",
			Usage:      "Read pods.",
			InnerField: "read_pod",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.read-spam",
			Usage:      "Access messages labeled spam.",
			InnerField: "read_spam",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.read-thread",
			Usage:      "Read threads.",
			InnerField: "read_thread",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.read-trash",
			Usage:      "Access messages labeled trash.",
			InnerField: "read_trash",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.read-webhook",
			Usage:      "Read webhook configurations.",
			InnerField: "read_webhook",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.send-draft",
			Usage:      "Send drafts.",
			InnerField: "send_draft",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.send-message",
			Usage:      "Send messages.",
			InnerField: "send_message",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.update-domain",
			Usage:      "Update domains.",
			InnerField: "update_domain",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.update-draft",
			Usage:      "Update drafts.",
			InnerField: "update_draft",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.update-inbox",
			Usage:      "Update inbox settings.",
			InnerField: "update_inbox",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.update-message",
			Usage:      "Update message labels.",
			InnerField: "update_message",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.update-webhook",
			Usage:      "Update webhooks.",
			InnerField: "update_webhook",
		},
	},
})

var apiKeysList = cli.Command{
	Name:    "list",
	Usage:   "List API Keys",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[any]{
			Name:      "ascending",
			Usage:     "Sort in ascending temporal order.",
			QueryPath: "ascending",
		},
		&requestflag.Flag[any]{
			Name:      "limit",
			Usage:     "Limit of number of items returned.",
			QueryPath: "limit",
		},
		&requestflag.Flag[any]{
			Name:      "page-token",
			Usage:     "Page token for pagination.",
			QueryPath: "page_token",
		},
	},
	Action:          handleAPIKeysList,
	HideHelpCommand: true,
}

func handleAPIKeysCreate(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()

	if len(unusedArgs) > 0 {
		return fmt.Errorf("Unexpected extra arguments: %v", unusedArgs)
	}

	params := agentmail.APIKeyNewParams{}

	options, err := flagOptions(
		cmd,
		apiquery.NestedQueryFormatBrackets,
		apiquery.ArrayQueryFormatComma,
		ApplicationJSON,
		false,
	)
	if err != nil {
		return err
	}

	var res []byte
	options = append(options, option.WithResponseBodyInto(&res))
	_, err = client.APIKeys.New(ctx, params, options...)
	if err != nil {
		return err
	}

	obj := gjson.ParseBytes(res)
	format := cmd.Root().String("format")
	transform := cmd.Root().String("transform")
	return ShowJSON(os.Stdout, "api-keys create", obj, format, transform)
}

func handleAPIKeysList(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()

	if len(unusedArgs) > 0 {
		return fmt.Errorf("Unexpected extra arguments: %v", unusedArgs)
	}

	params := agentmail.APIKeyListParams{}

	options, err := flagOptions(
		cmd,
		apiquery.NestedQueryFormatBrackets,
		apiquery.ArrayQueryFormatComma,
		EmptyBody,
		false,
	)
	if err != nil {
		return err
	}

	var res []byte
	options = append(options, option.WithResponseBodyInto(&res))
	_, err = client.APIKeys.List(ctx, params, options...)
	if err != nil {
		return err
	}

	obj := gjson.ParseBytes(res)
	format := cmd.Root().String("format")
	transform := cmd.Root().String("transform")
	return ShowJSON(os.Stdout, "api-keys list", obj, format, transform)
}
