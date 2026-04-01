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

var podsMetricsQuery = cli.Command{
	Name:    "query",
	Usage:   "**CLI:**",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:     "pod-id",
			Usage:    "ID of pod.",
			Required: true,
		},
		&requestflag.Flag[any]{
			Name:      "descending",
			Usage:     "Sort in descending order.",
			QueryPath: "descending",
		},
		&requestflag.Flag[any]{
			Name:      "end",
			Usage:     "End timestamp for the query.",
			QueryPath: "end",
		},
		&requestflag.Flag[any]{
			Name:      "event-type",
			Usage:     "List of metric event types to query.",
			QueryPath: "event_types",
		},
		&requestflag.Flag[any]{
			Name:      "limit",
			Usage:     "Limit on number of buckets to return.",
			QueryPath: "limit",
		},
		&requestflag.Flag[any]{
			Name:      "period",
			Usage:     "Period in number of seconds for the query.",
			QueryPath: "period",
		},
		&requestflag.Flag[any]{
			Name:      "start",
			Usage:     "Start timestamp for the query.",
			QueryPath: "start",
		},
	},
	Action:          handlePodsMetricsQuery,
	HideHelpCommand: true,
}

func handlePodsMetricsQuery(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("pod-id") && len(unusedArgs) > 0 {
		cmd.Set("pod-id", unusedArgs[0])
		unusedArgs = unusedArgs[1:]
	}
	if len(unusedArgs) > 0 {
		return fmt.Errorf("Unexpected extra arguments: %v", unusedArgs)
	}

	params := agentmail.PodMetricQueryParams{}

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
	_, err = client.Pods.Metrics.Query(
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
	return ShowJSON(os.Stdout, "pods:metrics query", obj, format, transform)
}
