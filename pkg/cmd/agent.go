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

var agentSignUp = cli.Command{
	Name:    "sign-up",
	Usage:   "Create a new agent organization with an inbox and API key. This endpoint is for\nsigning up for the first time. If you've already signed up, you're all set —\njust use your existing API key.",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:     "human-email",
			Usage:    "Email address of the human who owns the agent. A 6-digit OTP will be sent to this address.",
			Required: true,
			BodyPath: "human_email",
		},
		&requestflag.Flag[string]{
			Name:     "username",
			Usage:    `Username for the auto-created inbox (e.g. "my-agent" creates my-agent@agentmail.to).`,
			Required: true,
			BodyPath: "username",
		},
	},
	Action:          handleAgentSignUp,
	HideHelpCommand: true,
}

var agentVerify = cli.Command{
	Name:    "verify",
	Usage:   "Verify an agent organization using the 6-digit OTP sent to the human's email\nduring sign-up.",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:     "otp-code",
			Usage:    "6-digit verification code sent to the human's email address.",
			Required: true,
			BodyPath: "otp_code",
		},
	},
	Action:          handleAgentVerify,
	HideHelpCommand: true,
}

func handleAgentSignUp(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()

	if len(unusedArgs) > 0 {
		return fmt.Errorf("Unexpected extra arguments: %v", unusedArgs)
	}

	params := agentmail.AgentSignUpParams{}

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
	_, err = client.Agent.SignUp(ctx, params, options...)
	if err != nil {
		return err
	}

	obj := gjson.ParseBytes(res)
	format := cmd.Root().String("format")
	transform := cmd.Root().String("transform")
	return ShowJSON(os.Stdout, "agent sign-up", obj, format, transform)
}

func handleAgentVerify(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()

	if len(unusedArgs) > 0 {
		return fmt.Errorf("Unexpected extra arguments: %v", unusedArgs)
	}

	params := agentmail.AgentVerifyParams{}

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
	_, err = client.Agent.Verify(ctx, params, options...)
	if err != nil {
		return err
	}

	obj := gjson.ParseBytes(res)
	format := cmd.Root().String("format")
	transform := cmd.Root().String("transform")
	return ShowJSON(os.Stdout, "agent verify", obj, format, transform)
}
