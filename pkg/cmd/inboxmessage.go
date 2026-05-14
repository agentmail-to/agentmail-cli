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

var inboxesMessagesUpdate = cli.Command{
	Name:    "update",
	Usage:   "**CLI:**",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:      "inbox-id",
			Usage:     "The ID of the inbox.",
			Required:  true,
			PathParam: "inbox_id",
		},
		&requestflag.Flag[string]{
			Name:      "message-id",
			Usage:     "ID of message.",
			Required:  true,
			PathParam: "message_id",
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
	Action:          handleInboxesMessagesUpdate,
	HideHelpCommand: true,
}

var inboxesMessagesList = cli.Command{
	Name:    "list",
	Usage:   "**CLI:**",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:      "inbox-id",
			Usage:     "The ID of the inbox.",
			Required:  true,
			PathParam: "inbox_id",
		},
		&requestflag.Flag[any]{
			Name:      "after",
			Usage:     "Timestamp after which to filter by.",
			QueryPath: "after",
		},
		&requestflag.Flag[*bool]{
			Name:      "ascending",
			Usage:     "Sort in ascending temporal order.",
			QueryPath: "ascending",
		},
		&requestflag.Flag[any]{
			Name:      "before",
			Usage:     "Timestamp before which to filter by.",
			QueryPath: "before",
		},
		&requestflag.Flag[*bool]{
			Name:      "include-blocked",
			Usage:     "Include blocked in results.",
			QueryPath: "include_blocked",
		},
		&requestflag.Flag[*bool]{
			Name:      "include-spam",
			Usage:     "Include spam in results.",
			QueryPath: "include_spam",
		},
		&requestflag.Flag[*bool]{
			Name:      "include-trash",
			Usage:     "Include trash in results.",
			QueryPath: "include_trash",
		},
		&requestflag.Flag[*bool]{
			Name:      "include-unauthenticated",
			Usage:     "Include unauthenticated in results.",
			QueryPath: "include_unauthenticated",
		},
		&requestflag.Flag[any]{
			Name:      "label",
			Usage:     "Labels to filter by.",
			QueryPath: "labels",
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
	Action:          handleInboxesMessagesList,
	HideHelpCommand: true,
}

var inboxesMessagesForward = requestflag.WithInnerFlags(cli.Command{
	Name:    "forward",
	Usage:   "**CLI:**",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:      "inbox-id",
			Usage:     "The ID of the inbox.",
			Required:  true,
			PathParam: "inbox_id",
		},
		&requestflag.Flag[string]{
			Name:      "message-id",
			Usage:     "ID of message.",
			Required:  true,
			PathParam: "message_id",
		},
		&requestflag.Flag[any]{
			Name:     "attachment",
			Usage:    "Attachments to include in message.",
			BodyPath: "attachments",
		},
		&requestflag.Flag[any]{
			Name:     "bcc",
			BodyPath: "bcc",
		},
		&requestflag.Flag[any]{
			Name:     "cc",
			BodyPath: "cc",
		},
		&requestflag.Flag[map[string]any]{
			Name:     "headers",
			Usage:    "Headers to include in message.",
			BodyPath: "headers",
		},
		&requestflag.Flag[*string]{
			Name:     "html",
			Usage:    "HTML body of message.",
			BodyPath: "html",
		},
		&requestflag.Flag[any]{
			Name:     "label",
			Usage:    "Labels of message.",
			BodyPath: "labels",
		},
		&requestflag.Flag[any]{
			Name:     "reply-to",
			BodyPath: "reply_to",
		},
		&requestflag.Flag[*string]{
			Name:     "subject",
			Usage:    "Subject of message.",
			BodyPath: "subject",
		},
		&requestflag.Flag[*string]{
			Name:     "text",
			Usage:    "Plain text body of message.",
			BodyPath: "text",
		},
		&requestflag.Flag[any]{
			Name:     "to",
			BodyPath: "to",
		},
	},
	Action:          handleInboxesMessagesForward,
	HideHelpCommand: true,
}, map[string][]requestflag.HasOuterFlag{
	"attachment": {
		&requestflag.InnerFlag[*string]{
			Name:                  "attachment.content",
			Usage:                 "Base64 encoded content of attachment.",
			InnerField:            "content",
			OuterIsArrayOfObjects: true,
		},
		&requestflag.InnerFlag[*string]{
			Name:                  "attachment.content-disposition",
			Usage:                 "Content disposition of attachment.",
			InnerField:            "content_disposition",
			OuterIsArrayOfObjects: true,
		},
		&requestflag.InnerFlag[*string]{
			Name:                  "attachment.content-id",
			Usage:                 "Content ID of attachment.",
			InnerField:            "content_id",
			OuterIsArrayOfObjects: true,
		},
		&requestflag.InnerFlag[*string]{
			Name:                  "attachment.content-type",
			Usage:                 "Content type of attachment.",
			InnerField:            "content_type",
			OuterIsArrayOfObjects: true,
		},
		&requestflag.InnerFlag[*string]{
			Name:                  "attachment.filename",
			Usage:                 "Filename of attachment.",
			InnerField:            "filename",
			OuterIsArrayOfObjects: true,
		},
		&requestflag.InnerFlag[*string]{
			Name:                  "attachment.url",
			Usage:                 "URL to the attachment.",
			InnerField:            "url",
			OuterIsArrayOfObjects: true,
		},
	},
})

var inboxesMessagesGet = cli.Command{
	Name:    "get",
	Usage:   "**CLI:**",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:      "inbox-id",
			Usage:     "The ID of the inbox.",
			Required:  true,
			PathParam: "inbox_id",
		},
		&requestflag.Flag[string]{
			Name:      "message-id",
			Usage:     "ID of message.",
			Required:  true,
			PathParam: "message_id",
		},
	},
	Action:          handleInboxesMessagesGet,
	HideHelpCommand: true,
}

var inboxesMessagesGetAttachment = cli.Command{
	Name:    "get-attachment",
	Usage:   "**CLI:**",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:      "inbox-id",
			Usage:     "The ID of the inbox.",
			Required:  true,
			PathParam: "inbox_id",
		},
		&requestflag.Flag[string]{
			Name:      "message-id",
			Usage:     "ID of message.",
			Required:  true,
			PathParam: "message_id",
		},
		&requestflag.Flag[string]{
			Name:      "attachment-id",
			Usage:     "ID of attachment.",
			Required:  true,
			PathParam: "attachment_id",
		},
	},
	Action:          handleInboxesMessagesGetAttachment,
	HideHelpCommand: true,
}

var inboxesMessagesGetRaw = cli.Command{
	Name:    "get-raw",
	Usage:   "**CLI:**",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:      "inbox-id",
			Usage:     "The ID of the inbox.",
			Required:  true,
			PathParam: "inbox_id",
		},
		&requestflag.Flag[string]{
			Name:      "message-id",
			Usage:     "ID of message.",
			Required:  true,
			PathParam: "message_id",
		},
	},
	Action:          handleInboxesMessagesGetRaw,
	HideHelpCommand: true,
}

var inboxesMessagesReply = requestflag.WithInnerFlags(cli.Command{
	Name:    "reply",
	Usage:   "**CLI:**",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:      "inbox-id",
			Usage:     "The ID of the inbox.",
			Required:  true,
			PathParam: "inbox_id",
		},
		&requestflag.Flag[string]{
			Name:      "message-id",
			Usage:     "ID of message.",
			Required:  true,
			PathParam: "message_id",
		},
		&requestflag.Flag[any]{
			Name:     "attachment",
			Usage:    "Attachments to include in message.",
			BodyPath: "attachments",
		},
		&requestflag.Flag[any]{
			Name:     "bcc",
			BodyPath: "bcc",
		},
		&requestflag.Flag[any]{
			Name:     "cc",
			BodyPath: "cc",
		},
		&requestflag.Flag[map[string]any]{
			Name:     "headers",
			Usage:    "Headers to include in message.",
			BodyPath: "headers",
		},
		&requestflag.Flag[*string]{
			Name:     "html",
			Usage:    "HTML body of message.",
			BodyPath: "html",
		},
		&requestflag.Flag[any]{
			Name:     "label",
			Usage:    "Labels of message.",
			BodyPath: "labels",
		},
		&requestflag.Flag[*bool]{
			Name:     "reply-all",
			Usage:    "Reply to all recipients of the original message.",
			BodyPath: "reply_all",
		},
		&requestflag.Flag[any]{
			Name:     "reply-to",
			BodyPath: "reply_to",
		},
		&requestflag.Flag[*string]{
			Name:     "text",
			Usage:    "Plain text body of message.",
			BodyPath: "text",
		},
		&requestflag.Flag[any]{
			Name:     "to",
			BodyPath: "to",
		},
	},
	Action:          handleInboxesMessagesReply,
	HideHelpCommand: true,
}, map[string][]requestflag.HasOuterFlag{
	"attachment": {
		&requestflag.InnerFlag[*string]{
			Name:                  "attachment.content",
			Usage:                 "Base64 encoded content of attachment.",
			InnerField:            "content",
			OuterIsArrayOfObjects: true,
		},
		&requestflag.InnerFlag[*string]{
			Name:                  "attachment.content-disposition",
			Usage:                 "Content disposition of attachment.",
			InnerField:            "content_disposition",
			OuterIsArrayOfObjects: true,
		},
		&requestflag.InnerFlag[*string]{
			Name:                  "attachment.content-id",
			Usage:                 "Content ID of attachment.",
			InnerField:            "content_id",
			OuterIsArrayOfObjects: true,
		},
		&requestflag.InnerFlag[*string]{
			Name:                  "attachment.content-type",
			Usage:                 "Content type of attachment.",
			InnerField:            "content_type",
			OuterIsArrayOfObjects: true,
		},
		&requestflag.InnerFlag[*string]{
			Name:                  "attachment.filename",
			Usage:                 "Filename of attachment.",
			InnerField:            "filename",
			OuterIsArrayOfObjects: true,
		},
		&requestflag.InnerFlag[*string]{
			Name:                  "attachment.url",
			Usage:                 "URL to the attachment.",
			InnerField:            "url",
			OuterIsArrayOfObjects: true,
		},
	},
})

var inboxesMessagesReplyAll = requestflag.WithInnerFlags(cli.Command{
	Name:    "reply-all",
	Usage:   "**CLI:**",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:      "inbox-id",
			Usage:     "The ID of the inbox.",
			Required:  true,
			PathParam: "inbox_id",
		},
		&requestflag.Flag[string]{
			Name:      "message-id",
			Usage:     "ID of message.",
			Required:  true,
			PathParam: "message_id",
		},
		&requestflag.Flag[any]{
			Name:     "attachment",
			Usage:    "Attachments to include in message.",
			BodyPath: "attachments",
		},
		&requestflag.Flag[map[string]any]{
			Name:     "headers",
			Usage:    "Headers to include in message.",
			BodyPath: "headers",
		},
		&requestflag.Flag[*string]{
			Name:     "html",
			Usage:    "HTML body of message.",
			BodyPath: "html",
		},
		&requestflag.Flag[any]{
			Name:     "label",
			Usage:    "Labels of message.",
			BodyPath: "labels",
		},
		&requestflag.Flag[any]{
			Name:     "reply-to",
			BodyPath: "reply_to",
		},
		&requestflag.Flag[*string]{
			Name:     "text",
			Usage:    "Plain text body of message.",
			BodyPath: "text",
		},
	},
	Action:          handleInboxesMessagesReplyAll,
	HideHelpCommand: true,
}, map[string][]requestflag.HasOuterFlag{
	"attachment": {
		&requestflag.InnerFlag[*string]{
			Name:                  "attachment.content",
			Usage:                 "Base64 encoded content of attachment.",
			InnerField:            "content",
			OuterIsArrayOfObjects: true,
		},
		&requestflag.InnerFlag[*string]{
			Name:                  "attachment.content-disposition",
			Usage:                 "Content disposition of attachment.",
			InnerField:            "content_disposition",
			OuterIsArrayOfObjects: true,
		},
		&requestflag.InnerFlag[*string]{
			Name:                  "attachment.content-id",
			Usage:                 "Content ID of attachment.",
			InnerField:            "content_id",
			OuterIsArrayOfObjects: true,
		},
		&requestflag.InnerFlag[*string]{
			Name:                  "attachment.content-type",
			Usage:                 "Content type of attachment.",
			InnerField:            "content_type",
			OuterIsArrayOfObjects: true,
		},
		&requestflag.InnerFlag[*string]{
			Name:                  "attachment.filename",
			Usage:                 "Filename of attachment.",
			InnerField:            "filename",
			OuterIsArrayOfObjects: true,
		},
		&requestflag.InnerFlag[*string]{
			Name:                  "attachment.url",
			Usage:                 "URL to the attachment.",
			InnerField:            "url",
			OuterIsArrayOfObjects: true,
		},
	},
})

var inboxesMessagesSend = requestflag.WithInnerFlags(cli.Command{
	Name:    "send",
	Usage:   "**CLI:**",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:      "inbox-id",
			Usage:     "The ID of the inbox.",
			Required:  true,
			PathParam: "inbox_id",
		},
		&requestflag.Flag[any]{
			Name:     "attachment",
			Usage:    "Attachments to include in message.",
			BodyPath: "attachments",
		},
		&requestflag.Flag[any]{
			Name:     "bcc",
			BodyPath: "bcc",
		},
		&requestflag.Flag[any]{
			Name:     "cc",
			BodyPath: "cc",
		},
		&requestflag.Flag[map[string]any]{
			Name:     "headers",
			Usage:    "Headers to include in message.",
			BodyPath: "headers",
		},
		&requestflag.Flag[*string]{
			Name:     "html",
			Usage:    "HTML body of message.",
			BodyPath: "html",
		},
		&requestflag.Flag[any]{
			Name:     "label",
			Usage:    "Labels of message.",
			BodyPath: "labels",
		},
		&requestflag.Flag[any]{
			Name:     "reply-to",
			BodyPath: "reply_to",
		},
		&requestflag.Flag[*string]{
			Name:     "subject",
			Usage:    "Subject of message.",
			BodyPath: "subject",
		},
		&requestflag.Flag[*string]{
			Name:     "text",
			Usage:    "Plain text body of message.",
			BodyPath: "text",
		},
		&requestflag.Flag[any]{
			Name:     "to",
			BodyPath: "to",
		},
	},
	Action:          handleInboxesMessagesSend,
	HideHelpCommand: true,
}, map[string][]requestflag.HasOuterFlag{
	"attachment": {
		&requestflag.InnerFlag[*string]{
			Name:                  "attachment.content",
			Usage:                 "Base64 encoded content of attachment.",
			InnerField:            "content",
			OuterIsArrayOfObjects: true,
		},
		&requestflag.InnerFlag[*string]{
			Name:                  "attachment.content-disposition",
			Usage:                 "Content disposition of attachment.",
			InnerField:            "content_disposition",
			OuterIsArrayOfObjects: true,
		},
		&requestflag.InnerFlag[*string]{
			Name:                  "attachment.content-id",
			Usage:                 "Content ID of attachment.",
			InnerField:            "content_id",
			OuterIsArrayOfObjects: true,
		},
		&requestflag.InnerFlag[*string]{
			Name:                  "attachment.content-type",
			Usage:                 "Content type of attachment.",
			InnerField:            "content_type",
			OuterIsArrayOfObjects: true,
		},
		&requestflag.InnerFlag[*string]{
			Name:                  "attachment.filename",
			Usage:                 "Filename of attachment.",
			InnerField:            "filename",
			OuterIsArrayOfObjects: true,
		},
		&requestflag.InnerFlag[*string]{
			Name:                  "attachment.url",
			Usage:                 "URL to the attachment.",
			InnerField:            "url",
			OuterIsArrayOfObjects: true,
		},
	},
})

func handleInboxesMessagesUpdate(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("message-id") && len(unusedArgs) > 0 {
		cmd.Set("message-id", unusedArgs[0])
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

	params := agentmail.InboxMessageUpdateParams{
		InboxID: cmd.Value("inbox-id").(string),
	}

	var res []byte
	options = append(options, option.WithResponseBodyInto(&res))
	_, err = client.Inboxes.Messages.Update(
		ctx,
		cmd.Value("message-id").(string),
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
		Title:          "inboxes:messages update",
		Transform:      transform,
	})
}

func handleInboxesMessagesList(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("inbox-id") && len(unusedArgs) > 0 {
		cmd.Set("inbox-id", unusedArgs[0])
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

	params := agentmail.InboxMessageListParams{}

	var res []byte
	options = append(options, option.WithResponseBodyInto(&res))
	_, err = client.Inboxes.Messages.List(
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
	explicitFormat := cmd.Root().IsSet("format")
	transform := cmd.Root().String("transform")
	return ShowJSON(obj, ShowJSONOpts{
		ExplicitFormat: explicitFormat,
		Format:         format,
		RawOutput:      cmd.Root().Bool("raw-output"),
		Title:          "inboxes:messages list",
		Transform:      transform,
	})
}

func handleInboxesMessagesForward(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("message-id") && len(unusedArgs) > 0 {
		cmd.Set("message-id", unusedArgs[0])
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

	params := agentmail.InboxMessageForwardParams{
		InboxID: cmd.Value("inbox-id").(string),
	}

	var res []byte
	options = append(options, option.WithResponseBodyInto(&res))
	_, err = client.Inboxes.Messages.Forward(
		ctx,
		cmd.Value("message-id").(string),
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
		Title:          "inboxes:messages forward",
		Transform:      transform,
	})
}

func handleInboxesMessagesGet(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("message-id") && len(unusedArgs) > 0 {
		cmd.Set("message-id", unusedArgs[0])
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

	params := agentmail.InboxMessageGetParams{
		InboxID: cmd.Value("inbox-id").(string),
	}

	var res []byte
	options = append(options, option.WithResponseBodyInto(&res))
	_, err = client.Inboxes.Messages.Get(
		ctx,
		cmd.Value("message-id").(string),
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
		Title:          "inboxes:messages get",
		Transform:      transform,
	})
}

func handleInboxesMessagesGetAttachment(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("attachment-id") && len(unusedArgs) > 0 {
		cmd.Set("attachment-id", unusedArgs[0])
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

	params := agentmail.InboxMessageGetAttachmentParams{
		InboxID:   cmd.Value("inbox-id").(string),
		MessageID: cmd.Value("message-id").(string),
	}

	var res []byte
	options = append(options, option.WithResponseBodyInto(&res))
	_, err = client.Inboxes.Messages.GetAttachment(
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
	explicitFormat := cmd.Root().IsSet("format")
	transform := cmd.Root().String("transform")
	return ShowJSON(obj, ShowJSONOpts{
		ExplicitFormat: explicitFormat,
		Format:         format,
		RawOutput:      cmd.Root().Bool("raw-output"),
		Title:          "inboxes:messages get-attachment",
		Transform:      transform,
	})
}

func handleInboxesMessagesGetRaw(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("message-id") && len(unusedArgs) > 0 {
		cmd.Set("message-id", unusedArgs[0])
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

	params := agentmail.InboxMessageGetRawParams{
		InboxID: cmd.Value("inbox-id").(string),
	}

	var res []byte
	options = append(options, option.WithResponseBodyInto(&res))
	_, err = client.Inboxes.Messages.GetRaw(
		ctx,
		cmd.Value("message-id").(string),
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
		Title:          "inboxes:messages get-raw",
		Transform:      transform,
	})
}

func handleInboxesMessagesReply(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("message-id") && len(unusedArgs) > 0 {
		cmd.Set("message-id", unusedArgs[0])
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

	params := agentmail.InboxMessageReplyParams{
		InboxID: cmd.Value("inbox-id").(string),
	}

	var res []byte
	options = append(options, option.WithResponseBodyInto(&res))
	_, err = client.Inboxes.Messages.Reply(
		ctx,
		cmd.Value("message-id").(string),
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
		Title:          "inboxes:messages reply",
		Transform:      transform,
	})
}

func handleInboxesMessagesReplyAll(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("message-id") && len(unusedArgs) > 0 {
		cmd.Set("message-id", unusedArgs[0])
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

	params := agentmail.InboxMessageReplyAllParams{
		InboxID: cmd.Value("inbox-id").(string),
	}

	var res []byte
	options = append(options, option.WithResponseBodyInto(&res))
	_, err = client.Inboxes.Messages.ReplyAll(
		ctx,
		cmd.Value("message-id").(string),
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
		Title:          "inboxes:messages reply-all",
		Transform:      transform,
	})
}

func handleInboxesMessagesSend(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("inbox-id") && len(unusedArgs) > 0 {
		cmd.Set("inbox-id", unusedArgs[0])
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

	params := agentmail.InboxMessageSendParams{}

	var res []byte
	options = append(options, option.WithResponseBodyInto(&res))
	_, err = client.Inboxes.Messages.Send(
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
	explicitFormat := cmd.Root().IsSet("format")
	transform := cmd.Root().String("transform")
	return ShowJSON(obj, ShowJSONOpts{
		ExplicitFormat: explicitFormat,
		Format:         format,
		RawOutput:      cmd.Root().Bool("raw-output"),
		Title:          "inboxes:messages send",
		Transform:      transform,
	})
}
