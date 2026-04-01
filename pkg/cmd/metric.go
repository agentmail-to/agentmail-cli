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

var metricsList = cli.Command{
	Name:    "list",
	Usage:   "**CLI:**",
	Suggest: true,
	Flags: []cli.Flag{
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
	Action:          handleMetricsList,
	HideHelpCommand: true,
}

func handleMetricsList(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()

	if len(unusedArgs) > 0 {
		return fmt.Errorf("Unexpected extra arguments: %v", unusedArgs)
	}

	params := agentmail.MetricListParams{}

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
	_, err = client.Metrics.List(ctx, params, options...)
	if err != nil {
		return err
	}

	obj := gjson.ParseBytes(res)
	format := cmd.Root().String("format")
	transform := cmd.Root().String("transform")
	return ShowJSON(os.Stdout, "metrics list", obj, format, transform)
}
