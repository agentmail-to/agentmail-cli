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

var inboxesMessagesRetrieve = cli.Command{
	Name:    "retrieve",
	Usage:   "Get Message",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:     "inbox-id",
			Usage:    "The ID of the inbox.",
			Required: true,
		},
		&requestflag.Flag[string]{
			Name:     "message-id",
			Usage:    "ID of message.",
			Required: true,
		},
	},
	Action:          handleInboxesMessagesRetrieve,
	HideHelpCommand: true,
}

var inboxesMessagesUpdate = cli.Command{
	Name:    "update",
	Usage:   "Update Message",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:     "inbox-id",
			Usage:    "The ID of the inbox.",
			Required: true,
		},
		&requestflag.Flag[string]{
			Name:     "message-id",
			Usage:    "ID of message.",
			Required: true,
		},
		&requestflag.Flag[any]{
			Name:     "add-label",
			Usage:    "Labels to add to message.",
			BodyPath: "add_labels",
		},
		&requestflag.Flag[any]{
			Name:     "remove-label",
			Usage:    "Labels to remove from message.",
			BodyPath: "remove_labels",
		},
	},
	Action:          handleInboxesMessagesUpdate,
	HideHelpCommand: true,
}

var inboxesMessagesList = cli.Command{
	Name:    "list",
	Usage:   "List Messages",
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
			Name:      "include-blocked",
			Usage:     "Include blocked in results.",
			QueryPath: "include_blocked",
		},
		&requestflag.Flag[any]{
			Name:      "include-spam",
			Usage:     "Include spam in results.",
			QueryPath: "include_spam",
		},
		&requestflag.Flag[any]{
			Name:      "include-trash",
			Usage:     "Include trash in results.",
			QueryPath: "include_trash",
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
	Action:          handleInboxesMessagesList,
	HideHelpCommand: true,
}

var inboxesMessagesForward = requestflag.WithInnerFlags(cli.Command{
	Name:    "forward",
	Usage:   "Forward Message",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:     "inbox-id",
			Usage:    "The ID of the inbox.",
			Required: true,
		},
		&requestflag.Flag[string]{
			Name:     "message-id",
			Usage:    "ID of message.",
			Required: true,
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
		&requestflag.Flag[any]{
			Name:     "headers",
			Usage:    "Headers to include in message.",
			BodyPath: "headers",
		},
		&requestflag.Flag[any]{
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
		&requestflag.Flag[any]{
			Name:     "subject",
			Usage:    "Subject of message.",
			BodyPath: "subject",
		},
		&requestflag.Flag[any]{
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

var inboxesMessagesGetAttachment = cli.Command{
	Name:    "get-attachment",
	Usage:   "Get Attachment",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:     "inbox-id",
			Usage:    "The ID of the inbox.",
			Required: true,
		},
		&requestflag.Flag[string]{
			Name:     "message-id",
			Usage:    "ID of message.",
			Required: true,
		},
		&requestflag.Flag[string]{
			Name:     "attachment-id",
			Usage:    "ID of attachment.",
			Required: true,
		},
	},
	Action:          handleInboxesMessagesGetAttachment,
	HideHelpCommand: true,
}

var inboxesMessagesGetRaw = cli.Command{
	Name:    "get-raw",
	Usage:   "Get Raw Message",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:     "inbox-id",
			Usage:    "The ID of the inbox.",
			Required: true,
		},
		&requestflag.Flag[string]{
			Name:     "message-id",
			Usage:    "ID of message.",
			Required: true,
		},
	},
	Action:          handleInboxesMessagesGetRaw,
	HideHelpCommand: true,
}

var inboxesMessagesReply = requestflag.WithInnerFlags(cli.Command{
	Name:    "reply",
	Usage:   "Reply To Message",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:     "inbox-id",
			Usage:    "The ID of the inbox.",
			Required: true,
		},
		&requestflag.Flag[string]{
			Name:     "message-id",
			Usage:    "ID of message.",
			Required: true,
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
		&requestflag.Flag[any]{
			Name:     "headers",
			Usage:    "Headers to include in message.",
			BodyPath: "headers",
		},
		&requestflag.Flag[any]{
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
			Name:     "reply-all",
			Usage:    "Reply to all recipients of the original message.",
			BodyPath: "reply_all",
		},
		&requestflag.Flag[any]{
			Name:     "reply-to",
			BodyPath: "reply_to",
		},
		&requestflag.Flag[any]{
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

var inboxesMessagesReplyAll = requestflag.WithInnerFlags(cli.Command{
	Name:    "reply-all",
	Usage:   "Reply All Message",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:     "inbox-id",
			Usage:    "The ID of the inbox.",
			Required: true,
		},
		&requestflag.Flag[string]{
			Name:     "message-id",
			Usage:    "ID of message.",
			Required: true,
		},
		&requestflag.Flag[any]{
			Name:     "attachment",
			Usage:    "Attachments to include in message.",
			BodyPath: "attachments",
		},
		&requestflag.Flag[any]{
			Name:     "headers",
			Usage:    "Headers to include in message.",
			BodyPath: "headers",
		},
		&requestflag.Flag[any]{
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
		&requestflag.Flag[any]{
			Name:     "text",
			Usage:    "Plain text body of message.",
			BodyPath: "text",
		},
	},
	Action:          handleInboxesMessagesReplyAll,
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

var inboxesMessagesSend = requestflag.WithInnerFlags(cli.Command{
	Name:    "send",
	Usage:   "Send Message",
	Suggest: true,
	Flags: []cli.Flag{
		&requestflag.Flag[string]{
			Name:     "inbox-id",
			Usage:    "The ID of the inbox.",
			Required: true,
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
		&requestflag.Flag[any]{
			Name:     "headers",
			Usage:    "Headers to include in message.",
			BodyPath: "headers",
		},
		&requestflag.Flag[any]{
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
		&requestflag.Flag[any]{
			Name:     "subject",
			Usage:    "Subject of message.",
			BodyPath: "subject",
		},
		&requestflag.Flag[any]{
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

func handleInboxesMessagesRetrieve(ctx context.Context, cmd *cli.Command) error {
	client := agentmail.NewClient(getDefaultRequestOptions(cmd)...)
	unusedArgs := cmd.Args().Slice()
	if !cmd.IsSet("message-id") && len(unusedArgs) > 0 {
		cmd.Set("message-id", unusedArgs[0])
		unusedArgs = unusedArgs[1:]
	}
	if len(unusedArgs) > 0 {
		return fmt.Errorf("Unexpected extra arguments: %v", unusedArgs)
	}

	params := agentmail.InboxMessageGetParams{
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
	transform := cmd.Root().String("transform")
	return ShowJSON(os.Stdout, "inboxes:messages retrieve", obj, format, transform)
}

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

	params := agentmail.InboxMessageUpdateParams{
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
	transform := cmd.Root().String("transform")
	return ShowJSON(os.Stdout, "inboxes:messages update", obj, format, transform)
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

	params := agentmail.InboxMessageListParams{}

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
	transform := cmd.Root().String("transform")
	return ShowJSON(os.Stdout, "inboxes:messages list", obj, format, transform)
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

	params := agentmail.InboxMessageForwardParams{
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
	transform := cmd.Root().String("transform")
	return ShowJSON(os.Stdout, "inboxes:messages forward", obj, format, transform)
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

	params := agentmail.InboxMessageGetAttachmentParams{
		InboxID:   cmd.Value("inbox-id").(string),
		MessageID: cmd.Value("message-id").(string),
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
	transform := cmd.Root().String("transform")
	return ShowJSON(os.Stdout, "inboxes:messages get-attachment", obj, format, transform)
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

	params := agentmail.InboxMessageGetRawParams{
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

	return client.Inboxes.Messages.GetRaw(
		ctx,
		cmd.Value("message-id").(string),
		params,
		options...,
	)
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

	params := agentmail.InboxMessageReplyParams{
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
	transform := cmd.Root().String("transform")
	return ShowJSON(os.Stdout, "inboxes:messages reply", obj, format, transform)
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

	params := agentmail.InboxMessageReplyAllParams{
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
	transform := cmd.Root().String("transform")
	return ShowJSON(os.Stdout, "inboxes:messages reply-all", obj, format, transform)
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

	params := agentmail.InboxMessageSendParams{}

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
	transform := cmd.Root().String("transform")
	return ShowJSON(os.Stdout, "inboxes:messages send", obj, format, transform)
}
