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

var podsAPIKeysCreate = requestflag.WithInnerFlags(cli.Command{
	Name:    "create",
	Usage:   "**CLI:**",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:     "pod-id",
			Usage:    "ID of pod.",
			Required: true,
		},
		&requestflag.Flag[any]{
			Name:     "name",
			Usage:    "Name of api key.",
			BodyPath: "name",
		},
		&requestflag.Flag[any]{
			Name:     "permissions",
			Usage:    "Granular permissions for the API key. When ommitted all permissions are granted. Otherwise, only permissions set to true are granted.",
			BodyPath: "permissions",
		},
	},
	Action:          handlePodsAPIKeysCreate,
	HideHelpCommand: true,
}, map[string][]requestflag.HasOuterFlag{
	"permissions": {
		&requestflag.InnerFlag[any]{
			Name:       "permissions.api-key-create",
			Usage:      "Create API keys.",
			InnerField: "api_key_create",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.api-key-delete",
			Usage:      "Delete API keys.",
			InnerField: "api_key_delete",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.api-key-read",
			Usage:      "Read API keys.",
			InnerField: "api_key_read",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.domain-create",
			Usage:      "Create domains.",
			InnerField: "domain_create",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.domain-delete",
			Usage:      "Delete domains.",
			InnerField: "domain_delete",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.domain-read",
			Usage:      "Read domain details.",
			InnerField: "domain_read",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.domain-update",
			Usage:      "Update domains.",
			InnerField: "domain_update",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.draft-create",
			Usage:      "Create drafts.",
			InnerField: "draft_create",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.draft-delete",
			Usage:      "Delete drafts.",
			InnerField: "draft_delete",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.draft-read",
			Usage:      "Read drafts.",
			InnerField: "draft_read",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.draft-send",
			Usage:      "Send drafts.",
			InnerField: "draft_send",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.draft-update",
			Usage:      "Update drafts.",
			InnerField: "draft_update",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.inbox-create",
			Usage:      "Create new inboxes.",
			InnerField: "inbox_create",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.inbox-delete",
			Usage:      "Delete inboxes.",
			InnerField: "inbox_delete",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.inbox-read",
			Usage:      "Read inbox details.",
			InnerField: "inbox_read",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.inbox-update",
			Usage:      "Update inbox settings.",
			InnerField: "inbox_update",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.label-blocked-read",
			Usage:      "Access messages labeled blocked.",
			InnerField: "label_blocked_read",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.label-spam-read",
			Usage:      "Access messages labeled spam.",
			InnerField: "label_spam_read",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.label-trash-read",
			Usage:      "Access messages labeled trash.",
			InnerField: "label_trash_read",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.list-entry-create",
			Usage:      "Create list entries.",
			InnerField: "list_entry_create",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.list-entry-delete",
			Usage:      "Delete list entries.",
			InnerField: "list_entry_delete",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.list-entry-read",
			Usage:      "Read list entries.",
			InnerField: "list_entry_read",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.message-read",
			Usage:      "Read messages.",
			InnerField: "message_read",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.message-send",
			Usage:      "Send messages.",
			InnerField: "message_send",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.message-update",
			Usage:      "Update message labels.",
			InnerField: "message_update",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.metrics-read",
			Usage:      "Read metrics.",
			InnerField: "metrics_read",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.pod-create",
			Usage:      "Create pods.",
			InnerField: "pod_create",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.pod-delete",
			Usage:      "Delete pods.",
			InnerField: "pod_delete",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.pod-read",
			Usage:      "Read pods.",
			InnerField: "pod_read",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.thread-delete",
			Usage:      "Delete threads.",
			InnerField: "thread_delete",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.thread-read",
			Usage:      "Read threads.",
			InnerField: "thread_read",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.webhook-create",
			Usage:      "Create webhooks.",
			InnerField: "webhook_create",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.webhook-delete",
			Usage:      "Delete webhooks.",
			InnerField: "webhook_delete",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.webhook-read",
			Usage:      "Read webhook configurations.",
			InnerField: "webhook_read",
		},
		&requestflag.InnerFlag[any]{
			Name:       "permissions.webhook-update",
			Usage:      "Update webhooks.",
			InnerField: "webhook_update",
		},
	},
})

var podsAPIKeysList = cli.Command{
	Name:    "list",
	Usage:   "**CLI:**",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:     "pod-id",
			Usage:    "ID of pod.",
			Required: true,
		},
		&requestflag.Flag[any]{
			Name:      "limit",
			Usage:     "Maximum number of items to return in a single page.",
			QueryPath: "limit",
		},
		&requestflag.Flag[any]{
			Name:      "page-token",
			Usage:     "Page token for pagination.",
			QueryPath: "page_token",
		},
	},
	Action:          handlePodsAPIKeysList,
	HideHelpCommand: true,
}

var podsAPIKeysDelete = cli.Command{
	Name:    "delete",
	Usage:   "**CLI:**",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:     "pod-id",
			Usage:    "ID of pod.",
			Required: true,
		},
		&requestflag.Flag[string]{
			Name:     "api-key-id",
			Usage:    "ID of api key.",
			Required: true,
		},
	},
	Action:          handlePodsAPIKeysDelete,
	HideHelpCommand: true,
}

func handlePodsAPIKeysCreate(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("pod-id") && len(unusedArgs) > 0 {
		cmd.Set("pod-id", unusedArgs[0])
		unusedArgs = unusedArgs[1:]
	}
	if len(unusedArgs) > 0 {
		return fmt.Errorf("Unexpected extra arguments: %v", unusedArgs)
	}

	params := agentmail.PodAPIKeyNewParams{}

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
	_, err = client.Pods.APIKeys.New(
		ctx,
		cmd.Value("pod-id").(string),
		params,
		options...,
	)
	if err != nil {
		return err
	}

	obj := gjson.ParseBytes(res)
	format := cmd.Root().String("format")
	transform := cmd.Root().String("transform")
	return ShowJSON(os.Stdout, "pods:api-keys create", obj, format, transform)
}

func handlePodsAPIKeysList(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("pod-id") && len(unusedArgs) > 0 {
		cmd.Set("pod-id", unusedArgs[0])
		unusedArgs = unusedArgs[1:]
	}
	if len(unusedArgs) > 0 {
		return fmt.Errorf("Unexpected extra arguments: %v", unusedArgs)
	}

	params := agentmail.PodAPIKeyListParams{}

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
	_, err = client.Pods.APIKeys.List(
		ctx,
		cmd.Value("pod-id").(string),
		params,
		options...,
	)
	if err != nil {
		return err
	}

	obj := gjson.ParseBytes(res)
	format := cmd.Root().String("format")
	transform := cmd.Root().String("transform")
	return ShowJSON(os.Stdout, "pods:api-keys list", obj, format, transform)
}

func handlePodsAPIKeysDelete(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("api-key-id") && len(unusedArgs) > 0 {
		cmd.Set("api-key-id", unusedArgs[0])
		unusedArgs = unusedArgs[1:]
	}
	if len(unusedArgs) > 0 {
		return fmt.Errorf("Unexpected extra arguments: %v", unusedArgs)
	}

	params := agentmail.PodAPIKeyDeleteParams{
		PodID: cmd.Value("pod-id").(string),
	}

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

	return client.Pods.APIKeys.Delete(
		ctx,
		cmd.Value("api-key-id").(string),
		params,
		options...,
	)
}
