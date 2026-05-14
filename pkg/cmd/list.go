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

var listsCreate = cli.Command{
	Name:    "create",
	Usage:   "**CLI:**",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:      "direction",
			Usage:     "Direction of list entry.",
			Required:  true,
			PathParam: "direction",
		},
		&requestflag.Flag[string]{
			Name:      "type",
			Usage:     "Type of list entry.",
			Required:  true,
			PathParam: "type",
		},
		&requestflag.Flag[string]{
			Name:     "entry",
			Usage:    "Email address or domain to add.",
			Required: true,
			BodyPath: "entry",
		},
		&requestflag.Flag[*string]{
			Name:     "reason",
			Usage:    "Reason for adding the entry.",
			BodyPath: "reason",
		},
	},
	Action:          handleListsCreate,
	HideHelpCommand: true,
}

var listsList = cli.Command{
	Name:    "list",
	Usage:   "**CLI:**",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:      "direction",
			Usage:     "Direction of list entry.",
			Required:  true,
			PathParam: "direction",
		},
		&requestflag.Flag[string]{
			Name:      "type",
			Usage:     "Type of list entry.",
			Required:  true,
			PathParam: "type",
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
	},
	Action:          handleListsList,
	HideHelpCommand: true,
}

var listsDelete = cli.Command{
	Name:    "delete",
	Usage:   "**CLI:**",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:      "direction",
			Usage:     "Direction of list entry.",
			Required:  true,
			PathParam: "direction",
		},
		&requestflag.Flag[string]{
			Name:      "type",
			Usage:     "Type of list entry.",
			Required:  true,
			PathParam: "type",
		},
		&requestflag.Flag[string]{
			Name:      "entry",
			Required:  true,
			PathParam: "entry",
		},
	},
	Action:          handleListsDelete,
	HideHelpCommand: true,
}

var listsGet = cli.Command{
	Name:    "get",
	Usage:   "**CLI:**",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:      "direction",
			Usage:     "Direction of list entry.",
			Required:  true,
			PathParam: "direction",
		},
		&requestflag.Flag[string]{
			Name:      "type",
			Usage:     "Type of list entry.",
			Required:  true,
			PathParam: "type",
		},
		&requestflag.Flag[string]{
			Name:      "entry",
			Required:  true,
			PathParam: "entry",
		},
	},
	Action:          handleListsGet,
	HideHelpCommand: true,
}

func handleListsCreate(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("type") && len(unusedArgs) > 0 {
		cmd.Set("type", unusedArgs[0])
		unusedArgs = unusedArgs[1:]
	}
	if len(unusedArgs) > 0 {
		return fmt.Errorf("Unexpected extra arguments: %v", unusedArgs)
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

	params := agentmail.ListNewParams{
		Direction: agentmail.ListNewParamsDirection(cmd.Value("direction").(string)),
	}

	var res []byte
	options = append(options, option.WithResponseBodyInto(&res))
	_, err = client.Lists.New(
		ctx,
		agentmail.ListNewParamsType(cmd.Value("type").(string)),
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
		Title:          "lists create",
		Transform:      transform,
	})
}

func handleListsList(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("type") && len(unusedArgs) > 0 {
		cmd.Set("type", unusedArgs[0])
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

	params := agentmail.ListListParams{
		Direction: agentmail.ListListParamsDirection(cmd.Value("direction").(string)),
	}

	var res []byte
	options = append(options, option.WithResponseBodyInto(&res))
	_, err = client.Lists.List(
		ctx,
		agentmail.ListListParamsType(cmd.Value("type").(string)),
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
		Title:          "lists list",
		Transform:      transform,
	})
}

func handleListsDelete(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("entry") && len(unusedArgs) > 0 {
		cmd.Set("entry", unusedArgs[0])
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

	params := agentmail.ListDeleteParams{
		Direction: agentmail.ListDeleteParamsDirection(cmd.Value("direction").(string)),
		Type:      agentmail.ListDeleteParamsType(cmd.Value("type").(string)),
	}

	return client.Lists.Delete(
		ctx,
		cmd.Value("entry").(string),
		params,
		options...,
	)
}

func handleListsGet(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("entry") && len(unusedArgs) > 0 {
		cmd.Set("entry", unusedArgs[0])
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

	params := agentmail.ListGetParams{
		Direction: agentmail.ListGetParamsDirection(cmd.Value("direction").(string)),
		Type:      agentmail.ListGetParamsType(cmd.Value("type").(string)),
	}

	var res []byte
	options = append(options, option.WithResponseBodyInto(&res))
	_, err = client.Lists.Get(
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
		Title:          "lists get",
		Transform:      transform,
	})
}
