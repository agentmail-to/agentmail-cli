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

var inboxesThreadsRetrieve = cli.Command{
	Name:    "retrieve",
	Usage:   "**CLI:**",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:     "inbox-id",
			Usage:    "The ID of the inbox.",
			Required: true,
		},
		&requestflag.Flag[string]{
			Name:     "thread-id",
			Usage:    "ID of thread.",
			Required: true,
		},
	},
	Action:          handleInboxesThreadsRetrieve,
	HideHelpCommand: true,
}

var inboxesThreadsList = cli.Command{
	Name:    "list",
	Usage:   "**CLI:**",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:     "inbox-id",
			Usage:    "The ID of the inbox.",
			Required: true,
		},
		&requestflag.Flag[any]{
			Name:      "after",
			Usage:     "Timestamp after which to filter by.",
			QueryPath: "after",
		},
		&requestflag.Flag[any]{
			Name:      "ascending",
			Usage:     "Sort in ascending temporal order.",
			QueryPath: "ascending",
		},
		&requestflag.Flag[any]{
			Name:      "before",
			Usage:     "Timestamp before which to filter by.",
			QueryPath: "before",
		},
		&requestflag.Flag[any]{
			Name:      "include-blocked",
			Usage:     "Include blocked in results.",
			QueryPath: "include_blocked",
		},
		&requestflag.Flag[any]{
			Name:      "include-spam",
			Usage:     "Include spam in results.",
			QueryPath: "include_spam",
		},
		&requestflag.Flag[any]{
			Name:      "include-trash",
			Usage:     "Include trash in results.",
			QueryPath: "include_trash",
		},
		&requestflag.Flag[any]{
			Name:      "label",
			Usage:     "Labels to filter by.",
			QueryPath: "labels",
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
	Action:          handleInboxesThreadsList,
	HideHelpCommand: true,
}

var inboxesThreadsDelete = cli.Command{
	Name:    "delete",
	Usage:   "Moves the thread to trash by adding a trash label to all messages. If the thread\nis already in trash, it will be permanently deleted. Use `permanent=true` to\nforce permanent deletion.",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:     "inbox-id",
			Usage:    "The ID of the inbox.",
			Required: true,
		},
		&requestflag.Flag[string]{
			Name:     "thread-id",
			Usage:    "ID of thread.",
			Required: true,
		},
		&requestflag.Flag[any]{
			Name:      "permanent",
			Usage:     "If true, permanently delete the thread instead of moving to trash.",
			QueryPath: "permanent",
		},
	},
	Action:          handleInboxesThreadsDelete,
	HideHelpCommand: true,
}

var inboxesThreadsGetAttachment = cli.Command{
	Name:    "get-attachment",
	Usage:   "**CLI:**",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:     "inbox-id",
			Usage:    "The ID of the inbox.",
			Required: true,
		},
		&requestflag.Flag[string]{
			Name:     "thread-id",
			Usage:    "ID of thread.",
			Required: true,
		},
		&requestflag.Flag[string]{
			Name:     "attachment-id",
			Usage:    "ID of attachment.",
			Required: true,
		},
	},
	Action:          handleInboxesThreadsGetAttachment,
	HideHelpCommand: true,
}

func handleInboxesThreadsRetrieve(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("thread-id") && len(unusedArgs) > 0 {
		cmd.Set("thread-id", unusedArgs[0])
		unusedArgs = unusedArgs[1:]
	}
	if len(unusedArgs) > 0 {
		return fmt.Errorf("Unexpected extra arguments: %v", unusedArgs)
	}

	params := agentmail.InboxThreadGetParams{
		InboxID: cmd.Value("inbox-id").(string),
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

	var res []byte
	options = append(options, option.WithResponseBodyInto(&res))
	_, err = client.Inboxes.Threads.Get(
		ctx,
		cmd.Value("thread-id").(string),
		params,
		options...,
	)
	if err != nil {
		return err
	}

	obj := gjson.ParseBytes(res)
	format := cmd.Root().String("format")
	transform := cmd.Root().String("transform")
	return ShowJSON(os.Stdout, "inboxes:threads retrieve", obj, format, transform)
}

func handleInboxesThreadsList(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("inbox-id") && len(unusedArgs) > 0 {
		cmd.Set("inbox-id", unusedArgs[0])
		unusedArgs = unusedArgs[1:]
	}
	if len(unusedArgs) > 0 {
		return fmt.Errorf("Unexpected extra arguments: %v", unusedArgs)
	}

	params := agentmail.InboxThreadListParams{}

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
	_, err = client.Inboxes.Threads.List(
		ctx,
		cmd.Value("inbox-id").(string),
		params,
		options...,
	)
	if err != nil {
		return err
	}

	obj := gjson.ParseBytes(res)
	format := cmd.Root().String("format")
	transform := cmd.Root().String("transform")
	return ShowJSON(os.Stdout, "inboxes:threads list", obj, format, transform)
}

func handleInboxesThreadsDelete(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("thread-id") && len(unusedArgs) > 0 {
		cmd.Set("thread-id", unusedArgs[0])
		unusedArgs = unusedArgs[1:]
	}
	if len(unusedArgs) > 0 {
		return fmt.Errorf("Unexpected extra arguments: %v", unusedArgs)
	}

	params := agentmail.InboxThreadDeleteParams{
		InboxID: cmd.Value("inbox-id").(string),
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

	return client.Inboxes.Threads.Delete(
		ctx,
		cmd.Value("thread-id").(string),
		params,
		options...,
	)
}

func handleInboxesThreadsGetAttachment(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("attachment-id") && len(unusedArgs) > 0 {
		cmd.Set("attachment-id", unusedArgs[0])
		unusedArgs = unusedArgs[1:]
	}
	if len(unusedArgs) > 0 {
		return fmt.Errorf("Unexpected extra arguments: %v", unusedArgs)
	}

	params := agentmail.InboxThreadGetAttachmentParams{
		InboxID:  cmd.Value("inbox-id").(string),
		ThreadID: cmd.Value("thread-id").(string),
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

	var res []byte
	options = append(options, option.WithResponseBodyInto(&res))
	_, err = client.Inboxes.Threads.GetAttachment(
		ctx,
		cmd.Value("attachment-id").(string),
		params,
		options...,
	)
	if err != nil {
		return err
	}

	obj := gjson.ParseBytes(res)
	format := cmd.Root().String("format")
	transform := cmd.Root().String("transform")
	return ShowJSON(os.Stdout, "inboxes:threads get-attachment", obj, format, transform)
}
