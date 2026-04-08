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

var inboxesDraftsCreate = requestflag.WithInnerFlags(cli.Command{
	Name:    "create",
	Usage:   "**CLI:**",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:     "inbox-id",
			Usage:    "The ID of the inbox.",
			Required: true,
		},
		&requestflag.Flag[any]{
			Name:     "attachment",
			Usage:    "Attachments to include in draft.",
			BodyPath: "attachments",
		},
		&requestflag.Flag[any]{
			Name:     "bcc",
			Usage:    "Addresses of BCC recipients. In format `username@domain.com` or `Display Name <username@domain.com>`.",
			BodyPath: "bcc",
		},
		&requestflag.Flag[any]{
			Name:     "cc",
			Usage:    "Addresses of CC recipients. In format `username@domain.com` or `Display Name <username@domain.com>`.",
			BodyPath: "cc",
		},
		&requestflag.Flag[any]{
			Name:     "client-id",
			Usage:    "Client ID of draft.",
			BodyPath: "client_id",
		},
		&requestflag.Flag[any]{
			Name:     "html",
			Usage:    "HTML body of draft.",
			BodyPath: "html",
		},
		&requestflag.Flag[any]{
			Name:     "in-reply-to",
			Usage:    "ID of message being replied to.",
			BodyPath: "in_reply_to",
		},
		&requestflag.Flag[any]{
			Name:     "label",
			Usage:    "Labels of draft.",
			BodyPath: "labels",
		},
		&requestflag.Flag[any]{
			Name:     "reply-to",
			Usage:    "Reply-to addresses. In format `username@domain.com` or `Display Name <username@domain.com>`.",
			BodyPath: "reply_to",
		},
		&requestflag.Flag[any]{
			Name:     "send-at",
			Usage:    "Time at which to schedule send draft.",
			BodyPath: "send_at",
		},
		&requestflag.Flag[any]{
			Name:     "subject",
			Usage:    "Subject of draft.",
			BodyPath: "subject",
		},
		&requestflag.Flag[any]{
			Name:     "text",
			Usage:    "Plain text body of draft.",
			BodyPath: "text",
		},
		&requestflag.Flag[any]{
			Name:     "to",
			Usage:    "Addresses of recipients. In format `username@domain.com` or `Display Name <username@domain.com>`.",
			BodyPath: "to",
		},
	},
	Action:          handleInboxesDraftsCreate,
	HideHelpCommand: true,
}, map[string][]requestflag.HasOuterFlag{
	"attachment": {
		&requestflag.InnerFlag[any]{
			Name:       "attachment.content",
			Usage:      "Base64 encoded content of attachment.",
			InnerField: "content",
		},
		&requestflag.InnerFlag[string]{
			Name:       "attachment.content-disposition",
			Usage:      "Content disposition of attachment.",
			InnerField: "content_disposition",
		},
		&requestflag.InnerFlag[any]{
			Name:       "attachment.content-id",
			Usage:      "Content ID of attachment.",
			InnerField: "content_id",
		},
		&requestflag.InnerFlag[any]{
			Name:       "attachment.content-type",
			Usage:      "Content type of attachment.",
			InnerField: "content_type",
		},
		&requestflag.InnerFlag[any]{
			Name:       "attachment.filename",
			Usage:      "Filename of attachment.",
			InnerField: "filename",
		},
		&requestflag.InnerFlag[any]{
			Name:       "attachment.url",
			Usage:      "URL to the attachment.",
			InnerField: "url",
		},
	},
})

var inboxesDraftsRetrieve = cli.Command{
	Name:    "retrieve",
	Usage:   "**CLI:**",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:     "inbox-id",
			Usage:    "The ID of the inbox.",
			Required: true,
		},
		&requestflag.Flag[string]{
			Name:     "draft-id",
			Usage:    "ID of draft.",
			Required: true,
		},
	},
	Action:          handleInboxesDraftsRetrieve,
	HideHelpCommand: true,
}

var inboxesDraftsUpdate = cli.Command{
	Name:    "update",
	Usage:   "**CLI:**",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:     "inbox-id",
			Usage:    "The ID of the inbox.",
			Required: true,
		},
		&requestflag.Flag[string]{
			Name:     "draft-id",
			Usage:    "ID of draft.",
			Required: true,
		},
		&requestflag.Flag[any]{
			Name:     "bcc",
			Usage:    "Addresses of BCC recipients. In format `username@domain.com` or `Display Name <username@domain.com>`.",
			BodyPath: "bcc",
		},
		&requestflag.Flag[any]{
			Name:     "cc",
			Usage:    "Addresses of CC recipients. In format `username@domain.com` or `Display Name <username@domain.com>`.",
			BodyPath: "cc",
		},
		&requestflag.Flag[any]{
			Name:     "html",
			Usage:    "HTML body of draft.",
			BodyPath: "html",
		},
		&requestflag.Flag[any]{
			Name:     "reply-to",
			Usage:    "Reply-to addresses. In format `username@domain.com` or `Display Name <username@domain.com>`.",
			BodyPath: "reply_to",
		},
		&requestflag.Flag[any]{
			Name:     "send-at",
			Usage:    "Time at which to schedule send draft.",
			BodyPath: "send_at",
		},
		&requestflag.Flag[any]{
			Name:     "subject",
			Usage:    "Subject of draft.",
			BodyPath: "subject",
		},
		&requestflag.Flag[any]{
			Name:     "text",
			Usage:    "Plain text body of draft.",
			BodyPath: "text",
		},
		&requestflag.Flag[any]{
			Name:     "to",
			Usage:    "Addresses of recipients. In format `username@domain.com` or `Display Name <username@domain.com>`.",
			BodyPath: "to",
		},
	},
	Action:          handleInboxesDraftsUpdate,
	HideHelpCommand: true,
}

var inboxesDraftsList = cli.Command{
	Name:    "list",
	Usage:   "**CLI:**",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:     "inbox-id",
			Usage:    "The ID of the inbox.",
			Required: true,
		},
		&requestflag.Flag[any]{
			Name:      "after",
			Usage:     "Timestamp after which to filter by.",
			QueryPath: "after",
		},
		&requestflag.Flag[any]{
			Name:      "ascending",
			Usage:     "Sort in ascending temporal order.",
			QueryPath: "ascending",
		},
		&requestflag.Flag[any]{
			Name:      "before",
			Usage:     "Timestamp before which to filter by.",
			QueryPath: "before",
		},
		&requestflag.Flag[any]{
			Name:      "label",
			Usage:     "Labels to filter by.",
			QueryPath: "labels",
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
	Action:          handleInboxesDraftsList,
	HideHelpCommand: true,
}

var inboxesDraftsDelete = cli.Command{
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
			Name:     "draft-id",
			Usage:    "ID of draft.",
			Required: true,
		},
	},
	Action:          handleInboxesDraftsDelete,
	HideHelpCommand: true,
}

var inboxesDraftsGetAttachment = cli.Command{
	Name:    "get-attachment",
	Usage:   "**CLI:**",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:     "inbox-id",
			Usage:    "The ID of the inbox.",
			Required: true,
		},
		&requestflag.Flag[string]{
			Name:     "draft-id",
			Usage:    "ID of draft.",
			Required: true,
		},
		&requestflag.Flag[string]{
			Name:     "attachment-id",
			Usage:    "ID of attachment.",
			Required: true,
		},
	},
	Action:          handleInboxesDraftsGetAttachment,
	HideHelpCommand: true,
}

var inboxesDraftsSend = cli.Command{
	Name:    "send",
	Usage:   "**CLI:**",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:     "inbox-id",
			Usage:    "The ID of the inbox.",
			Required: true,
		},
		&requestflag.Flag[string]{
			Name:     "draft-id",
			Usage:    "ID of draft.",
			Required: true,
		},
		&requestflag.Flag[any]{
			Name:     "add-labels",
			Usage:    "Label or labels to add to message.",
			BodyPath: "add_labels",
		},
		&requestflag.Flag[any]{
			Name:     "remove-labels",
			Usage:    "Label or labels to remove from message.",
			BodyPath: "remove_labels",
		},
	},
	Action:          handleInboxesDraftsSend,
	HideHelpCommand: true,
}

func handleInboxesDraftsCreate(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("inbox-id") && len(unusedArgs) > 0 {
		cmd.Set("inbox-id", unusedArgs[0])
		unusedArgs = unusedArgs[1:]
	}
	if len(unusedArgs) > 0 {
		return fmt.Errorf("Unexpected extra arguments: %v", unusedArgs)
	}

	params := agentmail.InboxDraftNewParams{}

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
	_, err = client.Inboxes.Drafts.New(
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
	return ShowJSON(os.Stdout, "inboxes:drafts create", obj, format, transform)
}

func handleInboxesDraftsRetrieve(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("draft-id") && len(unusedArgs) > 0 {
		cmd.Set("draft-id", unusedArgs[0])
		unusedArgs = unusedArgs[1:]
	}
	if len(unusedArgs) > 0 {
		return fmt.Errorf("Unexpected extra arguments: %v", unusedArgs)
	}

	params := agentmail.InboxDraftGetParams{
		InboxID: cmd.Value("inbox-id").(string),
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
	_, err = client.Inboxes.Drafts.Get(
		ctx,
		cmd.Value("draft-id").(string),
		params,
		options...,
	)
	if err != nil {
		return err
	}

	obj := gjson.ParseBytes(res)
	format := cmd.Root().String("format")
	transform := cmd.Root().String("transform")
	return ShowJSON(os.Stdout, "inboxes:drafts retrieve", obj, format, transform)
}

func handleInboxesDraftsUpdate(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("draft-id") && len(unusedArgs) > 0 {
		cmd.Set("draft-id", unusedArgs[0])
		unusedArgs = unusedArgs[1:]
	}
	if len(unusedArgs) > 0 {
		return fmt.Errorf("Unexpected extra arguments: %v", unusedArgs)
	}

	params := agentmail.InboxDraftUpdateParams{
		InboxID: cmd.Value("inbox-id").(string),
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
	_, err = client.Inboxes.Drafts.Update(
		ctx,
		cmd.Value("draft-id").(string),
		params,
		options...,
	)
	if err != nil {
		return err
	}

	obj := gjson.ParseBytes(res)
	format := cmd.Root().String("format")
	transform := cmd.Root().String("transform")
	return ShowJSON(os.Stdout, "inboxes:drafts update", obj, format, transform)
}

func handleInboxesDraftsList(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("inbox-id") && len(unusedArgs) > 0 {
		cmd.Set("inbox-id", unusedArgs[0])
		unusedArgs = unusedArgs[1:]
	}
	if len(unusedArgs) > 0 {
		return fmt.Errorf("Unexpected extra arguments: %v", unusedArgs)
	}

	params := agentmail.InboxDraftListParams{}

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
	_, err = client.Inboxes.Drafts.List(
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
	return ShowJSON(os.Stdout, "inboxes:drafts list", obj, format, transform)
}

func handleInboxesDraftsDelete(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("draft-id") && len(unusedArgs) > 0 {
		cmd.Set("draft-id", unusedArgs[0])
		unusedArgs = unusedArgs[1:]
	}
	if len(unusedArgs) > 0 {
		return fmt.Errorf("Unexpected extra arguments: %v", unusedArgs)
	}

	params := agentmail.InboxDraftDeleteParams{
		InboxID: cmd.Value("inbox-id").(string),
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

	return client.Inboxes.Drafts.Delete(
		ctx,
		cmd.Value("draft-id").(string),
		params,
		options...,
	)
}

func handleInboxesDraftsGetAttachment(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("attachment-id") && len(unusedArgs) > 0 {
		cmd.Set("attachment-id", unusedArgs[0])
		unusedArgs = unusedArgs[1:]
	}
	if len(unusedArgs) > 0 {
		return fmt.Errorf("Unexpected extra arguments: %v", unusedArgs)
	}

	params := agentmail.InboxDraftGetAttachmentParams{
		InboxID: cmd.Value("inbox-id").(string),
		DraftID: cmd.Value("draft-id").(string),
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
	_, err = client.Inboxes.Drafts.GetAttachment(
		ctx,
		cmd.Value("attachment-id").(string),
		params,
		options...,
	)
	if err != nil {
		return err
	}

	obj := gjson.ParseBytes(res)
	format := cmd.Root().String("format")
	transform := cmd.Root().String("transform")
	return ShowJSON(os.Stdout, "inboxes:drafts get-attachment", obj, format, transform)
}

func handleInboxesDraftsSend(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("draft-id") && len(unusedArgs) > 0 {
		cmd.Set("draft-id", unusedArgs[0])
		unusedArgs = unusedArgs[1:]
	}
	if len(unusedArgs) > 0 {
		return fmt.Errorf("Unexpected extra arguments: %v", unusedArgs)
	}

	params := agentmail.InboxDraftSendParams{
		InboxID: cmd.Value("inbox-id").(string),
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
	_, err = client.Inboxes.Drafts.Send(
		ctx,
		cmd.Value("draft-id").(string),
		params,
		options...,
	)
	if err != nil {
		return err
	}

	obj := gjson.ParseBytes(res)
	format := cmd.Root().String("format")
	transform := cmd.Root().String("transform")
	return ShowJSON(os.Stdout, "inboxes:drafts send", obj, format, transform)
}
