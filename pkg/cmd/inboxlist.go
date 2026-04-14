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

var inboxesListsCreate = cli.Command{
	Name:    "create",
	Usage:   "**CLI:**",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:     "inbox-id",
			Usage:    "The ID of the inbox.",
			Required: true,
		},
		&requestflag.Flag[string]{
			Name:     "direction",
			Usage:    "Direction of list entry.",
			Required: true,
		},
		&requestflag.Flag[string]{
			Name:     "type",
			Usage:    "Type of list entry.",
			Required: true,
		},
		&requestflag.Flag[string]{
			Name:     "entry",
			Usage:    "Email address or domain to add.",
			Required: true,
			BodyPath: "entry",
		},
		&requestflag.Flag[any]{
			Name:     "reason",
			Usage:    "Reason for adding the entry.",
			BodyPath: "reason",
		},
	},
	Action:          handleInboxesListsCreate,
	HideHelpCommand: true,
}

var inboxesListsList = cli.Command{
	Name:    "list",
	Usage:   "**CLI:**",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:     "inbox-id",
			Usage:    "The ID of the inbox.",
			Required: true,
		},
		&requestflag.Flag[string]{
			Name:     "direction",
			Usage:    "Direction of list entry.",
			Required: true,
		},
		&requestflag.Flag[string]{
			Name:     "type",
			Usage:    "Type of list entry.",
			Required: true,
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
	Action:          handleInboxesListsList,
	HideHelpCommand: true,
}

var inboxesListsDelete = cli.Command{
	Name:    "delete",
	Usage:   "**CLI:**",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:     "inbox-id",
			Usage:    "The ID of the inbox.",
			Required: true,
		},
		&requestflag.Flag[string]{
			Name:     "direction",
			Usage:    "Direction of list entry.",
			Required: true,
		},
		&requestflag.Flag[string]{
			Name:     "type",
			Usage:    "Type of list entry.",
			Required: true,
		},
		&requestflag.Flag[string]{
			Name:     "entry",
			Required: true,
		},
	},
	Action:          handleInboxesListsDelete,
	HideHelpCommand: true,
}

var inboxesListsGet = cli.Command{
	Name:    "get",
	Usage:   "**CLI:**",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:     "inbox-id",
			Usage:    "The ID of the inbox.",
			Required: true,
		},
		&requestflag.Flag[string]{
			Name:     "direction",
			Usage:    "Direction of list entry.",
			Required: true,
		},
		&requestflag.Flag[string]{
			Name:     "type",
			Usage:    "Type of list entry.",
			Required: true,
		},
		&requestflag.Flag[string]{
			Name:     "entry",
			Required: true,
		},
	},
	Action:          handleInboxesListsGet,
	HideHelpCommand: true,
}

func handleInboxesListsCreate(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("type") && len(unusedArgs) > 0 {
		cmd.Set("type", unusedArgs[0])
		unusedArgs = unusedArgs[1:]
	}
	if len(unusedArgs) > 0 {
		return fmt.Errorf("Unexpected extra arguments: %v", unusedArgs)
	}

	params := agentmail.InboxListNewParams{
		InboxID:   cmd.Value("inbox-id").(string),
		Direction: agentmail.InboxListNewParamsDirection(cmd.Value("direction").(string)),
	}

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
	_, err = client.Inboxes.Lists.New(
		ctx,
		agentmail.InboxListNewParamsType(cmd.Value("type").(string)),
		params,
		options...,
	)
	if err != nil {
		return err
	}

	obj := gjson.ParseBytes(res)
	format := cmd.Root().String("format")
	transform := cmd.Root().String("transform")
	return ShowJSON(os.Stdout, "inboxes:lists create", obj, format, transform)
}

func handleInboxesListsList(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("type") && len(unusedArgs) > 0 {
		cmd.Set("type", unusedArgs[0])
		unusedArgs = unusedArgs[1:]
	}
	if len(unusedArgs) > 0 {
		return fmt.Errorf("Unexpected extra arguments: %v", unusedArgs)
	}

	params := agentmail.InboxListListParams{
		InboxID:   cmd.Value("inbox-id").(string),
		Direction: agentmail.InboxListListParamsDirection(cmd.Value("direction").(string)),
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
	_, err = client.Inboxes.Lists.List(
		ctx,
		agentmail.InboxListListParamsType(cmd.Value("type").(string)),
		params,
		options...,
	)
	if err != nil {
		return err
	}

	obj := gjson.ParseBytes(res)
	format := cmd.Root().String("format")
	transform := cmd.Root().String("transform")
	return ShowJSON(os.Stdout, "inboxes:lists list", obj, format, transform)
}

func handleInboxesListsDelete(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("entry") && len(unusedArgs) > 0 {
		cmd.Set("entry", unusedArgs[0])
		unusedArgs = unusedArgs[1:]
	}
	if len(unusedArgs) > 0 {
		return fmt.Errorf("Unexpected extra arguments: %v", unusedArgs)
	}

	params := agentmail.InboxListDeleteParams{
		InboxID:   cmd.Value("inbox-id").(string),
		Direction: agentmail.InboxListDeleteParamsDirection(cmd.Value("direction").(string)),
		Type:      agentmail.InboxListDeleteParamsType(cmd.Value("type").(string)),
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

	return client.Inboxes.Lists.Delete(
		ctx,
		cmd.Value("entry").(string),
		params,
		options...,
	)
}

func handleInboxesListsGet(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("entry") && len(unusedArgs) > 0 {
		cmd.Set("entry", unusedArgs[0])
		unusedArgs = unusedArgs[1:]
	}
	if len(unusedArgs) > 0 {
		return fmt.Errorf("Unexpected extra arguments: %v", unusedArgs)
	}

	params := agentmail.InboxListGetParams{
		InboxID:   cmd.Value("inbox-id").(string),
		Direction: agentmail.InboxListGetParamsDirection(cmd.Value("direction").(string)),
		Type:      agentmail.InboxListGetParamsType(cmd.Value("type").(string)),
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
	_, err = client.Inboxes.Lists.Get(
		ctx,
		cmd.Value("entry").(string),
		params,
		options...,
	)
	if err != nil {
		return err
	}

	obj := gjson.ParseBytes(res)
	format := cmd.Root().String("format")
	transform := cmd.Root().String("transform")
	return ShowJSON(os.Stdout, "inboxes:lists get", obj, format, transform)
}
