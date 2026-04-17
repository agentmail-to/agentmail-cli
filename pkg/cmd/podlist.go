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

var podsListsCreate = cli.Command{
	Name:    "create",
	Usage:   "**CLI:**",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:     "pod-id",
			Usage:    "ID of pod.",
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
	Action:          handlePodsListsCreate,
	HideHelpCommand: true,
}

var podsListsList = cli.Command{
	Name:    "list",
	Usage:   "**CLI:**",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:     "pod-id",
			Usage:    "ID of pod.",
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
	Action:          handlePodsListsList,
	HideHelpCommand: true,
}

var podsListsDelete = cli.Command{
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
	Action:          handlePodsListsDelete,
	HideHelpCommand: true,
}

var podsListsGet = cli.Command{
	Name:    "get",
	Usage:   "**CLI:**",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:     "pod-id",
			Usage:    "ID of pod.",
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
	Action:          handlePodsListsGet,
	HideHelpCommand: true,
}

func handlePodsListsCreate(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("type") && len(unusedArgs) > 0 {
		cmd.Set("type", unusedArgs[0])
		unusedArgs = unusedArgs[1:]
	}
	if len(unusedArgs) > 0 {
		return fmt.Errorf("Unexpected extra arguments: %v", unusedArgs)
	}

	params := agentmail.PodListNewParams{
		PodID:     cmd.Value("pod-id").(string),
		Direction: agentmail.PodListNewParamsDirection(cmd.Value("direction").(string)),
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
	_, err = client.Pods.Lists.New(
		ctx,
		agentmail.PodListNewParamsType(cmd.Value("type").(string)),
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
		Title:          "pods:lists create",
		Transform:      transform,
	})
}

func handlePodsListsList(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("type") && len(unusedArgs) > 0 {
		cmd.Set("type", unusedArgs[0])
		unusedArgs = unusedArgs[1:]
	}
	if len(unusedArgs) > 0 {
		return fmt.Errorf("Unexpected extra arguments: %v", unusedArgs)
	}

	params := agentmail.PodListListParams{
		PodID:     cmd.Value("pod-id").(string),
		Direction: agentmail.PodListListParamsDirection(cmd.Value("direction").(string)),
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
	_, err = client.Pods.Lists.List(
		ctx,
		agentmail.PodListListParamsType(cmd.Value("type").(string)),
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
		Title:          "pods:lists list",
		Transform:      transform,
	})
}

func handlePodsListsDelete(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("entry") && len(unusedArgs) > 0 {
		cmd.Set("entry", unusedArgs[0])
		unusedArgs = unusedArgs[1:]
	}
	if len(unusedArgs) > 0 {
		return fmt.Errorf("Unexpected extra arguments: %v", unusedArgs)
	}

	params := agentmail.PodListDeleteParams{
		PodID:     cmd.Value("pod-id").(string),
		Direction: agentmail.PodListDeleteParamsDirection(cmd.Value("direction").(string)),
		Type:      agentmail.PodListDeleteParamsType(cmd.Value("type").(string)),
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

	return client.Pods.Lists.Delete(
		ctx,
		cmd.Value("entry").(string),
		params,
		options...,
	)
}

func handlePodsListsGet(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("entry") && len(unusedArgs) > 0 {
		cmd.Set("entry", unusedArgs[0])
		unusedArgs = unusedArgs[1:]
	}
	if len(unusedArgs) > 0 {
		return fmt.Errorf("Unexpected extra arguments: %v", unusedArgs)
	}

	params := agentmail.PodListGetParams{
		PodID:     cmd.Value("pod-id").(string),
		Direction: agentmail.PodListGetParamsDirection(cmd.Value("direction").(string)),
		Type:      agentmail.PodListGetParamsType(cmd.Value("type").(string)),
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
	_, err = client.Pods.Lists.Get(
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
	explicitFormat := cmd.Root().IsSet("format")
	transform := cmd.Root().String("transform")
	return ShowJSON(obj, ShowJSONOpts{
		ExplicitFormat: explicitFormat,
		Format:         format,
		RawOutput:      cmd.Root().Bool("raw-output"),
		Title:          "pods:lists get",
		Transform:      transform,
	})
}
