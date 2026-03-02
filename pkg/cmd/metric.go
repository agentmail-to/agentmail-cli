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
	Usage:   "List Metrics",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[any]{
			Name:      "end-timestamp",
			Usage:     "End timestamp for the metrics query range.",
			Required:  true,
			QueryPath: "end_timestamp",
		},
		&requestflag.Flag[any]{
			Name:      "start-timestamp",
			Usage:     "Start timestamp for the metrics query range.",
			Required:  true,
			QueryPath: "start_timestamp",
		},
		&requestflag.Flag[any]{
			Name:      "event-type",
			Usage:     "List of metric event types to filter by.",
			QueryPath: "event_types",
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
