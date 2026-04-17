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

var podsDomainsCreate = cli.Command{
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
			Name:     "domain",
			Usage:    "The name of the domain (e.g., `example.com`).",
			Required: true,
			BodyPath: "domain",
		},
		&requestflag.Flag[bool]{
			Name:     "feedback-enabled",
			Usage:    "Bounce and complaint notifications are sent to your inboxes.",
			Required: true,
			BodyPath: "feedback_enabled",
		},
	},
	Action:          handlePodsDomainsCreate,
	HideHelpCommand: true,
}

var podsDomainsUpdate = cli.Command{
	Name:    "update",
	Usage:   "**CLI:**",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:     "pod-id",
			Usage:    "ID of pod.",
			Required: true,
		},
		&requestflag.Flag[string]{
			Name:     "domain-id",
			Usage:    "The ID of the domain.",
			Required: true,
		},
		&requestflag.Flag[any]{
			Name:     "feedback-enabled",
			Usage:    "Bounce and complaint notifications are sent to your inboxes.",
			BodyPath: "feedback_enabled",
		},
	},
	Action:          handlePodsDomainsUpdate,
	HideHelpCommand: true,
}

var podsDomainsList = cli.Command{
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
	Action:          handlePodsDomainsList,
	HideHelpCommand: true,
}

var podsDomainsDelete = cli.Command{
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
			Name:     "domain-id",
			Usage:    "The ID of the domain.",
			Required: true,
		},
	},
	Action:          handlePodsDomainsDelete,
	HideHelpCommand: true,
}

var podsDomainsGet = cli.Command{
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
			Name:     "domain-id",
			Usage:    "The ID of the domain.",
			Required: true,
		},
	},
	Action:          handlePodsDomainsGet,
	HideHelpCommand: true,
}

var podsDomainsGetZoneFile = cli.Command{
	Name:    "get-zone-file",
	Usage:   "**CLI:**",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:     "pod-id",
			Usage:    "ID of pod.",
			Required: true,
		},
		&requestflag.Flag[string]{
			Name:     "domain-id",
			Usage:    "The ID of the domain.",
			Required: true,
		},
	},
	Action:          handlePodsDomainsGetZoneFile,
	HideHelpCommand: true,
}

var podsDomainsVerify = cli.Command{
	Name:    "verify",
	Usage:   "**CLI:**",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:     "pod-id",
			Usage:    "ID of pod.",
			Required: true,
		},
		&requestflag.Flag[string]{
			Name:     "domain-id",
			Usage:    "The ID of the domain.",
			Required: true,
		},
	},
	Action:          handlePodsDomainsVerify,
	HideHelpCommand: true,
}

func handlePodsDomainsCreate(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("pod-id") && len(unusedArgs) > 0 {
		cmd.Set("pod-id", unusedArgs[0])
		unusedArgs = unusedArgs[1:]
	}
	if len(unusedArgs) > 0 {
		return fmt.Errorf("Unexpected extra arguments: %v", unusedArgs)
	}

	params := agentmail.PodDomainNewParams{}

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
	_, err = client.Pods.Domains.New(
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
	explicitFormat := cmd.Root().IsSet("format")
	transform := cmd.Root().String("transform")
	return ShowJSON(obj, ShowJSONOpts{
		ExplicitFormat: explicitFormat,
		Format:         format,
		RawOutput:      cmd.Root().Bool("raw-output"),
		Title:          "pods:domains create",
		Transform:      transform,
	})
}

func handlePodsDomainsUpdate(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("domain-id") && len(unusedArgs) > 0 {
		cmd.Set("domain-id", unusedArgs[0])
		unusedArgs = unusedArgs[1:]
	}
	if len(unusedArgs) > 0 {
		return fmt.Errorf("Unexpected extra arguments: %v", unusedArgs)
	}

	params := agentmail.PodDomainUpdateParams{
		PodID: cmd.Value("pod-id").(string),
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
	_, err = client.Pods.Domains.Update(
		ctx,
		cmd.Value("domain-id").(string),
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
		Title:          "pods:domains update",
		Transform:      transform,
	})
}

func handlePodsDomainsList(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("pod-id") && len(unusedArgs) > 0 {
		cmd.Set("pod-id", unusedArgs[0])
		unusedArgs = unusedArgs[1:]
	}
	if len(unusedArgs) > 0 {
		return fmt.Errorf("Unexpected extra arguments: %v", unusedArgs)
	}

	params := agentmail.PodDomainListParams{}

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
	_, err = client.Pods.Domains.List(
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
	explicitFormat := cmd.Root().IsSet("format")
	transform := cmd.Root().String("transform")
	return ShowJSON(obj, ShowJSONOpts{
		ExplicitFormat: explicitFormat,
		Format:         format,
		RawOutput:      cmd.Root().Bool("raw-output"),
		Title:          "pods:domains list",
		Transform:      transform,
	})
}

func handlePodsDomainsDelete(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("domain-id") && len(unusedArgs) > 0 {
		cmd.Set("domain-id", unusedArgs[0])
		unusedArgs = unusedArgs[1:]
	}
	if len(unusedArgs) > 0 {
		return fmt.Errorf("Unexpected extra arguments: %v", unusedArgs)
	}

	params := agentmail.PodDomainDeleteParams{
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

	return client.Pods.Domains.Delete(
		ctx,
		cmd.Value("domain-id").(string),
		params,
		options...,
	)
}

func handlePodsDomainsGet(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("domain-id") && len(unusedArgs) > 0 {
		cmd.Set("domain-id", unusedArgs[0])
		unusedArgs = unusedArgs[1:]
	}
	if len(unusedArgs) > 0 {
		return fmt.Errorf("Unexpected extra arguments: %v", unusedArgs)
	}

	params := agentmail.PodDomainGetParams{
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
	_, err = client.Pods.Domains.Get(
		ctx,
		cmd.Value("domain-id").(string),
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
		Title:          "pods:domains get",
		Transform:      transform,
	})
}

func handlePodsDomainsGetZoneFile(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("domain-id") && len(unusedArgs) > 0 {
		cmd.Set("domain-id", unusedArgs[0])
		unusedArgs = unusedArgs[1:]
	}
	if len(unusedArgs) > 0 {
		return fmt.Errorf("Unexpected extra arguments: %v", unusedArgs)
	}

	params := agentmail.PodDomainGetZoneFileParams{
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

	return client.Pods.Domains.GetZoneFile(
		ctx,
		cmd.Value("domain-id").(string),
		params,
		options...,
	)
}

func handlePodsDomainsVerify(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("domain-id") && len(unusedArgs) > 0 {
		cmd.Set("domain-id", unusedArgs[0])
		unusedArgs = unusedArgs[1:]
	}
	if len(unusedArgs) > 0 {
		return fmt.Errorf("Unexpected extra arguments: %v", unusedArgs)
	}

	params := agentmail.PodDomainVerifyParams{
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

	return client.Pods.Domains.Verify(
		ctx,
		cmd.Value("domain-id").(string),
		params,
		options...,
	)
}
