// File generated from our OpenAPI spec by Stainless. See CONTRIBUTING.md for details.

package cmd

import (
	"context"
	"fmt"

	"github.com/agentmail-to/agentmail-cli/internal/apiquery"
	"github.com/agentmail-to/agentmail-cli/internal/requestflag"
	"github.com/agentmail-to/agentmail-go"
	"github.com/agentmail-to/agentmail-go/option"
	"github.com/tidwall/gjson"
	"github.com/urfave/cli/v3"
)

var threadsList = cli.Command{
	Name:    "list",
	Usage:   "Lists threads, most recent first. Pass `senders`, `recipients`, or `subject` to\nfilter by substring. Filtered requests are served by search, which caps `limit`\nat 100. For relevance-ranked full-text search across senders, recipients,\nsubject, and message body, use `Search Threads`.",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[any]{
			Name:      "after",
			Usage:     "Timestamp after which to filter by.",
			QueryPath: "after",
		},
		&requestflag.Flag[*bool]{
			Name:      "ascending",
			Usage:     "Sort in ascending temporal order.",
			QueryPath: "ascending",
		},
		&requestflag.Flag[any]{
			Name:      "before",
			Usage:     "Timestamp before which to filter by.",
			QueryPath: "before",
		},
		&requestflag.Flag[*bool]{
			Name:      "include-blocked",
			Usage:     "Include blocked in results.",
			QueryPath: "include_blocked",
		},
		&requestflag.Flag[*bool]{
			Name:      "include-spam",
			Usage:     "Include spam in results.",
			QueryPath: "include_spam",
		},
		&requestflag.Flag[*bool]{
			Name:      "include-trash",
			Usage:     "Include trash in results.",
			QueryPath: "include_trash",
		},
		&requestflag.Flag[*bool]{
			Name:      "include-unauthenticated",
			Usage:     "Include unauthenticated in results.",
			QueryPath: "include_unauthenticated",
		},
		&requestflag.Flag[any]{
			Name:      "label",
			Usage:     "Labels to filter by.",
			QueryPath: "labels",
		},
		&requestflag.Flag[*int64]{
			Name:      "limit",
			Usage:     "Limit of number of items returned.",
			QueryPath: "limit",
		},
		&requestflag.Flag[*string]{
			Name:      "page-token",
			Usage:     "Page token for pagination.",
			QueryPath: "page_token",
		},
		&requestflag.Flag[any]{
			Name:      "recipient",
			Usage:     "Filter to threads whose recipients contain this value (substring match). Repeatable; all values must match.",
			QueryPath: "recipients",
		},
		&requestflag.Flag[any]{
			Name:      "sender",
			Usage:     "Filter to threads whose senders contain this value (substring match). Repeatable; all values must match.",
			QueryPath: "senders",
		},
		&requestflag.Flag[any]{
			Name:      "subject",
			Usage:     "Filter to threads whose subject contains this value (substring match). Repeatable; all values must match.",
			QueryPath: "subject",
		},
	},
	Action:          handleThreadsList,
	HideHelpCommand: true,
}

var threadsDelete = cli.Command{
	Name:    "delete",
	Usage:   "Permanently deletes a thread and all of its messages.",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:      "thread-id",
			Usage:     "ID of thread.",
			Required:  true,
			PathParam: "thread_id",
		},
	},
	Action:          handleThreadsDelete,
	HideHelpCommand: true,
}

var threadsGet = cli.Command{
	Name:    "get",
	Usage:   "**CLI:**",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:      "thread-id",
			Usage:     "ID of thread.",
			Required:  true,
			PathParam: "thread_id",
		},
	},
	Action:          handleThreadsGet,
	HideHelpCommand: true,
}

var threadsGetAttachment = cli.Command{
	Name:    "get-attachment",
	Usage:   "**CLI:**",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:      "thread-id",
			Usage:     "ID of thread.",
			Required:  true,
			PathParam: "thread_id",
		},
		&requestflag.Flag[string]{
			Name:      "attachment-id",
			Usage:     "ID of attachment.",
			Required:  true,
			PathParam: "attachment_id",
		},
	},
	Action:          handleThreadsGetAttachment,
	HideHelpCommand: true,
}

func handleThreadsList(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()

	if len(unusedArgs) > 0 {
		return fmt.Errorf("Unexpected extra arguments: %v", unusedArgs)
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

	params := agentmail.ThreadListParams{}

	var res []byte
	options = append(options, option.WithResponseBodyInto(&res))
	_, err = client.Threads.List(ctx, params, options...)
	if err != nil {
		return err
	}

	obj := gjson.ParseBytes(res)
	format := cmd.Root().String("format")
	explicitFormat := cmd.Root().IsSet("format")
	transform := cmd.Root().String("transform")
	return ShowJSON(obj, ShowJSONOpts{
		ExplicitFormat: explicitFormat,
		Format:         format,
		RawOutput:      cmd.Root().Bool("raw-output"),
		Title:          "threads list",
		Transform:      transform,
	})
}

func handleThreadsDelete(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("thread-id") && len(unusedArgs) > 0 {
		cmd.Set("thread-id", unusedArgs[0])
		unusedArgs = unusedArgs[1:]
	}
	if len(unusedArgs) > 0 {
		return fmt.Errorf("Unexpected extra arguments: %v", unusedArgs)
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

	return client.Threads.Delete(ctx, cmd.Value("thread-id").(string), options...)
}

func handleThreadsGet(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("thread-id") && len(unusedArgs) > 0 {
		cmd.Set("thread-id", unusedArgs[0])
		unusedArgs = unusedArgs[1:]
	}
	if len(unusedArgs) > 0 {
		return fmt.Errorf("Unexpected extra arguments: %v", unusedArgs)
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
	_, err = client.Threads.Get(ctx, cmd.Value("thread-id").(string), options...)
	if err != nil {
		return err
	}

	obj := gjson.ParseBytes(res)
	format := cmd.Root().String("format")
	explicitFormat := cmd.Root().IsSet("format")
	transform := cmd.Root().String("transform")
	return ShowJSON(obj, ShowJSONOpts{
		ExplicitFormat: explicitFormat,
		Format:         format,
		RawOutput:      cmd.Root().Bool("raw-output"),
		Title:          "threads get",
		Transform:      transform,
	})
}

func handleThreadsGetAttachment(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("attachment-id") && len(unusedArgs) > 0 {
		cmd.Set("attachment-id", unusedArgs[0])
		unusedArgs = unusedArgs[1:]
	}
	if len(unusedArgs) > 0 {
		return fmt.Errorf("Unexpected extra arguments: %v", unusedArgs)
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

	params := agentmail.ThreadGetAttachmentParams{
		ThreadID: cmd.Value("thread-id").(string),
	}

	var res []byte
	options = append(options, option.WithResponseBodyInto(&res))
	_, err = client.Threads.GetAttachment(
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
	explicitFormat := cmd.Root().IsSet("format")
	transform := cmd.Root().String("transform")
	return ShowJSON(obj, ShowJSONOpts{
		ExplicitFormat: explicitFormat,
		Format:         format,
		RawOutput:      cmd.Root().Bool("raw-output"),
		Title:          "threads get-attachment",
		Transform:      transform,
	})
}
