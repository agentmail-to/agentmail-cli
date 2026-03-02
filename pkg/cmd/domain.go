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

var domainsCreate = cli.Command{
	Name:    "create",
	Usage:   "Create Domain",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:     "domain",
			Usage:    `The name of the domain. (e.g., "example.com")`,
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
	Action:          handleDomainsCreate,
	HideHelpCommand: true,
}

var domainsRetrieve = cli.Command{
	Name:    "retrieve",
	Usage:   "Get Domain",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:     "domain-id",
			Usage:    `The name of the domain. (e.g., " your-domain.com")`,
			Required: true,
		},
	},
	Action:          handleDomainsRetrieve,
	HideHelpCommand: true,
}

var domainsList = cli.Command{
	Name:    "list",
	Usage:   "List Domains",
	Suggest: true,
	Flags: []cli.Flag{
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
	Action:          handleDomainsList,
	HideHelpCommand: true,
}

var domainsDelete = cli.Command{
	Name:    "delete",
	Usage:   "Delete Domain",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:     "domain-id",
			Usage:    `The name of the domain. (e.g., " your-domain.com")`,
			Required: true,
		},
	},
	Action:          handleDomainsDelete,
	HideHelpCommand: true,
}

var domainsGetZoneFile = cli.Command{
	Name:    "get-zone-file",
	Usage:   "Get Zone File",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:     "domain-id",
			Usage:    `The name of the domain. (e.g., " your-domain.com")`,
			Required: true,
		},
	},
	Action:          handleDomainsGetZoneFile,
	HideHelpCommand: true,
}

var domainsVerify = cli.Command{
	Name:    "verify",
	Usage:   "Verify Domain",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:     "domain-id",
			Usage:    `The name of the domain. (e.g., " your-domain.com")`,
			Required: true,
		},
	},
	Action:          handleDomainsVerify,
	HideHelpCommand: true,
}

func handleDomainsCreate(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()

	if len(unusedArgs) > 0 {
		return fmt.Errorf("Unexpected extra arguments: %v", unusedArgs)
	}

	params := agentmail.DomainNewParams{}

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
	_, err = client.Domains.New(ctx, params, options...)
	if err != nil {
		return err
	}

	obj := gjson.ParseBytes(res)
	format := cmd.Root().String("format")
	transform := cmd.Root().String("transform")
	return ShowJSON(os.Stdout, "domains create", obj, format, transform)
}

func handleDomainsRetrieve(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("domain-id") && len(unusedArgs) > 0 {
		cmd.Set("domain-id", unusedArgs[0])
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
	_, err = client.Domains.Get(ctx, cmd.Value("domain-id").(string), options...)
	if err != nil {
		return err
	}

	obj := gjson.ParseBytes(res)
	format := cmd.Root().String("format")
	transform := cmd.Root().String("transform")
	return ShowJSON(os.Stdout, "domains retrieve", obj, format, transform)
}

func handleDomainsList(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()

	if len(unusedArgs) > 0 {
		return fmt.Errorf("Unexpected extra arguments: %v", unusedArgs)
	}

	params := agentmail.DomainListParams{}

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
	_, err = client.Domains.List(ctx, params, options...)
	if err != nil {
		return err
	}

	obj := gjson.ParseBytes(res)
	format := cmd.Root().String("format")
	transform := cmd.Root().String("transform")
	return ShowJSON(os.Stdout, "domains list", obj, format, transform)
}

func handleDomainsDelete(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("domain-id") && len(unusedArgs) > 0 {
		cmd.Set("domain-id", unusedArgs[0])
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

	return client.Domains.Delete(ctx, cmd.Value("domain-id").(string), options...)
}

func handleDomainsGetZoneFile(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("domain-id") && len(unusedArgs) > 0 {
		cmd.Set("domain-id", unusedArgs[0])
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

	return client.Domains.GetZoneFile(ctx, cmd.Value("domain-id").(string), options...)
}

func handleDomainsVerify(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("domain-id") && len(unusedArgs) > 0 {
		cmd.Set("domain-id", unusedArgs[0])
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

	return client.Domains.Verify(ctx, cmd.Value("domain-id").(string), options...)
}
