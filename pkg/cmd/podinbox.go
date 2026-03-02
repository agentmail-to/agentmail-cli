// File generated from our OpenAPI spec by Stainless. See CONTRIBUTING.md for details.

package cmd

import (
	"context"
	"fmt"
	"os"

	"github.com/stainless-sdks/agentmail-cli/internal/apiquery"
	"github.com/stainless-sdks/agentmail-cli/internal/requestflag"
	"github.com/stainless-sdks/agentmail-go"
	"github.com/stainless-sdks/agentmail-go/option"
	"github.com/tidwall/gjson"
	"github.com/urfave/cli/v3"
)

var podsInboxesCreate = cli.Command{
	Name:    "create",
	Usage:   "Create Inbox",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:     "pod-id",
			Usage:    "ID of pod.",
			Required: true,
		},
		&requestflag.Flag[any]{
			Name:     "client-id",
			Usage:    "Client ID of inbox.",
			BodyPath: "client_id",
		},
		&requestflag.Flag[any]{
			Name:     "display-name",
			Usage:    "Display name: `Display Name <username@domain.com>`.",
			BodyPath: "display_name",
		},
		&requestflag.Flag[any]{
			Name:     "domain",
			Usage:    "Domain of address. Must be verified domain. Defaults to `agentmail.to`.",
			BodyPath: "domain",
		},
		&requestflag.Flag[any]{
			Name:     "username",
			Usage:    "Username of address. Randomly generated if not specified.",
			BodyPath: "username",
		},
	},
	Action:          handlePodsInboxesCreate,
	HideHelpCommand: true,
}

var podsInboxesRetrieve = cli.Command{
	Name:    "retrieve",
	Usage:   "Get Inbox",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:     "pod-id",
			Usage:    "ID of pod.",
			Required: true,
		},
		&requestflag.Flag[string]{
			Name:     "inbox-id",
			Usage:    "ID of inbox.",
			Required: true,
		},
	},
	Action:          handlePodsInboxesRetrieve,
	HideHelpCommand: true,
}

var podsInboxesList = cli.Command{
	Name:    "list",
	Usage:   "List Inboxes",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:     "pod-id",
			Usage:    "ID of pod.",
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
	Action:          handlePodsInboxesList,
	HideHelpCommand: true,
}

var podsInboxesDelete = cli.Command{
	Name:    "delete",
	Usage:   "Delete Inbox",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:     "pod-id",
			Usage:    "ID of pod.",
			Required: true,
		},
		&requestflag.Flag[string]{
			Name:     "inbox-id",
			Usage:    "ID of inbox.",
			Required: true,
		},
	},
	Action:          handlePodsInboxesDelete,
	HideHelpCommand: true,
}

func handlePodsInboxesCreate(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("pod-id") && len(unusedArgs) > 0 {
		cmd.Set("pod-id", unusedArgs[0])
		unusedArgs = unusedArgs[1:]
	}
	if len(unusedArgs) > 0 {
		return fmt.Errorf("Unexpected extra arguments: %v", unusedArgs)
	}

	params := agentmail.PodInboxNewParams{}

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
	_, err = client.Pods.Inboxes.New(
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
	return ShowJSON(os.Stdout, "pods:inboxes create", obj, format, transform)
}

func handlePodsInboxesRetrieve(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("inbox-id") && len(unusedArgs) > 0 {
		cmd.Set("inbox-id", unusedArgs[0])
		unusedArgs = unusedArgs[1:]
	}
	if len(unusedArgs) > 0 {
		return fmt.Errorf("Unexpected extra arguments: %v", unusedArgs)
	}

	params := agentmail.PodInboxGetParams{
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

	var res []byte
	options = append(options, option.WithResponseBodyInto(&res))
	_, err = client.Pods.Inboxes.Get(
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
	return ShowJSON(os.Stdout, "pods:inboxes retrieve", obj, format, transform)
}

func handlePodsInboxesList(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("pod-id") && len(unusedArgs) > 0 {
		cmd.Set("pod-id", unusedArgs[0])
		unusedArgs = unusedArgs[1:]
	}
	if len(unusedArgs) > 0 {
		return fmt.Errorf("Unexpected extra arguments: %v", unusedArgs)
	}

	params := agentmail.PodInboxListParams{}

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
	_, err = client.Pods.Inboxes.List(
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
	return ShowJSON(os.Stdout, "pods:inboxes list", obj, format, transform)
}

func handlePodsInboxesDelete(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("inbox-id") && len(unusedArgs) > 0 {
		cmd.Set("inbox-id", unusedArgs[0])
		unusedArgs = unusedArgs[1:]
	}
	if len(unusedArgs) > 0 {
		return fmt.Errorf("Unexpected extra arguments: %v", unusedArgs)
	}

	params := agentmail.PodInboxDeleteParams{
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

	return client.Pods.Inboxes.Delete(
		ctx,
		cmd.Value("inbox-id").(string),
		params,
		options...,
	)
}
