// File generated from our OpenAPI spec by Stainless. See CONTRIBUTING.md for details.

package cmd

import (
	"bytes"
	"compress/gzip"
	"context"
	"fmt"
	"os"
	"path/filepath"
	"slices"
	"strings"

	"github.com/agentmail-to/agentmail-cli/internal/autocomplete"
	"github.com/agentmail-to/agentmail-cli/internal/requestflag"
	docs "github.com/urfave/cli-docs/v3"
	"github.com/urfave/cli/v3"
)

var (
	Command            *cli.Command
	CommandErrorBuffer bytes.Buffer
)

func init() {
	Command = &cli.Command{
		Name:      "agentmail",
		Usage:     "CLI for the agentmail API",
		Suggest:   true,
		Version:   Version,
		ErrWriter: &CommandErrorBuffer,
		Flags: []cli.Flag{
			&cli.BoolFlag{
				Name:  "debug",
				Usage: "Enable debug logging",
			},
			&cli.StringFlag{
				Name:        "base-url",
				DefaultText: "url",
				Usage:       "Override the base URL for API requests",
			},
			&cli.StringFlag{
				Name:  "format",
				Usage: "The format for displaying response data (one of: " + strings.Join(OutputFormats, ", ") + ")",
				Value: "auto",
				Validator: func(format string) error {
					if !slices.Contains(OutputFormats, strings.ToLower(format)) {
						return fmt.Errorf("format must be one of: %s", strings.Join(OutputFormats, ", "))
					}
					return nil
				},
			},
			&cli.StringFlag{
				Name:  "format-error",
				Usage: "The format for displaying error data (one of: " + strings.Join(OutputFormats, ", ") + ")",
				Value: "auto",
				Validator: func(format string) error {
					if !slices.Contains(OutputFormats, strings.ToLower(format)) {
						return fmt.Errorf("format must be one of: %s", strings.Join(OutputFormats, ", "))
					}
					return nil
				},
			},
			&cli.StringFlag{
				Name:  "transform",
				Usage: "The GJSON transformation for data output.",
			},
			&cli.StringFlag{
				Name:  "transform-error",
				Usage: "The GJSON transformation for errors.",
			},
			&requestflag.Flag[string]{
				Name:    "api-key",
				Sources: cli.EnvVars("AGENTMAIL_API_KEY"),
			},
			&cli.StringFlag{
				Name:  "environment",
				Usage: "Set the environment for API requests",
			},
		},
		Commands: []*cli.Command{
			{
				Name:     "agent",
				Category: "API RESOURCE",
				Suggest:  true,
				Commands: []*cli.Command{
					&agentSignUp,
					&agentVerify,
				},
			},
			{
				Name:     "inboxes",
				Category: "API RESOURCE",
				Suggest:  true,
				Commands: []*cli.Command{
					&inboxesCreate,
					&inboxesRetrieve,
					&inboxesUpdate,
					&inboxesList,
					&inboxesDelete,
					&inboxesListMetrics,
				},
			},
			{
				Name:     "inboxes:drafts",
				Category: "API RESOURCE",
				Suggest:  true,
				Commands: []*cli.Command{
					&inboxesDraftsCreate,
					&inboxesDraftsRetrieve,
					&inboxesDraftsUpdate,
					&inboxesDraftsList,
					&inboxesDraftsDelete,
					&inboxesDraftsSend,
				},
			},
			{
				Name:     "inboxes:messages",
				Category: "API RESOURCE",
				Suggest:  true,
				Commands: []*cli.Command{
					&inboxesMessagesRetrieve,
					&inboxesMessagesUpdate,
					&inboxesMessagesList,
					&inboxesMessagesForward,
					&inboxesMessagesGetAttachment,
					&inboxesMessagesGetRaw,
					&inboxesMessagesReply,
					&inboxesMessagesReplyAll,
					&inboxesMessagesSend,
				},
			},
			{
				Name:     "inboxes:threads",
				Category: "API RESOURCE",
				Suggest:  true,
				Commands: []*cli.Command{
					&inboxesThreadsRetrieve,
					&inboxesThreadsList,
					&inboxesThreadsDelete,
					&inboxesThreadsGetAttachment,
				},
			},
			{
				Name:     "pods",
				Category: "API RESOURCE",
				Suggest:  true,
				Commands: []*cli.Command{
					&podsCreate,
					&podsRetrieve,
					&podsList,
					&podsDelete,
				},
			},
			{
				Name:     "pods:domains",
				Category: "API RESOURCE",
				Suggest:  true,
				Commands: []*cli.Command{
					&podsDomainsCreate,
					&podsDomainsList,
					&podsDomainsDelete,
				},
			},
			{
				Name:     "pods:drafts",
				Category: "API RESOURCE",
				Suggest:  true,
				Commands: []*cli.Command{
					&podsDraftsRetrieve,
					&podsDraftsList,
				},
			},
			{
				Name:     "pods:inboxes",
				Category: "API RESOURCE",
				Suggest:  true,
				Commands: []*cli.Command{
					&podsInboxesCreate,
					&podsInboxesRetrieve,
					&podsInboxesList,
					&podsInboxesDelete,
				},
			},
			{
				Name:     "pods:threads",
				Category: "API RESOURCE",
				Suggest:  true,
				Commands: []*cli.Command{
					&podsThreadsRetrieve,
					&podsThreadsList,
					&podsThreadsGetAttachment,
				},
			},
			{
				Name:     "webhooks",
				Category: "API RESOURCE",
				Suggest:  true,
				Commands: []*cli.Command{
					&webhooksCreate,
					&webhooksRetrieve,
					&webhooksUpdate,
					&webhooksList,
					&webhooksDelete,
				},
			},
			{
				Name:     "api-keys",
				Category: "API RESOURCE",
				Suggest:  true,
				Commands: []*cli.Command{
					&apiKeysCreate,
					&apiKeysList,
				},
			},
			{
				Name:     "domains",
				Category: "API RESOURCE",
				Suggest:  true,
				Commands: []*cli.Command{
					&domainsCreate,
					&domainsRetrieve,
					&domainsList,
					&domainsDelete,
					&domainsGetZoneFile,
					&domainsVerify,
				},
			},
			{
				Name:     "drafts",
				Category: "API RESOURCE",
				Suggest:  true,
				Commands: []*cli.Command{
					&draftsRetrieve,
					&draftsList,
				},
			},
			{
				Name:     "metrics",
				Category: "API RESOURCE",
				Suggest:  true,
				Commands: []*cli.Command{
					&metricsList,
				},
			},
			{
				Name:     "organizations",
				Category: "API RESOURCE",
				Suggest:  true,
				Commands: []*cli.Command{
					&organizationsRetrieve,
				},
			},
			{
				Name:     "threads",
				Category: "API RESOURCE",
				Suggest:  true,
				Commands: []*cli.Command{
					&threadsRetrieve,
					&threadsList,
					&threadsRetrieveAttachment,
				},
			},
			{
				Name:            "@manpages",
				Usage:           "Generate documentation for 'man'",
				UsageText:       "agentmail @manpages [-o agentmail.1] [--gzip]",
				Hidden:          true,
				Action:          generateManpages,
				HideHelpCommand: true,
				Flags: []cli.Flag{
					&cli.StringFlag{
						Name:    "output",
						Aliases: []string{"o"},
						Usage:   "write manpages to the given folder",
						Value:   "man",
					},
					&cli.BoolFlag{
						Name:    "gzip",
						Aliases: []string{"z"},
						Usage:   "output gzipped manpage files to .gz",
						Value:   true,
					},
					&cli.BoolFlag{
						Name:    "text",
						Aliases: []string{"z"},
						Usage:   "output uncompressed text files",
						Value:   false,
					},
				},
			},
			{
				Name:            "__complete",
				Hidden:          true,
				HideHelpCommand: true,
				Action:          autocomplete.ExecuteShellCompletion,
			},
			{
				Name:            "@completion",
				Hidden:          true,
				HideHelpCommand: true,
				Action:          autocomplete.OutputCompletionScript,
			},
		},
		HideHelpCommand: true,
	}
}

func generateManpages(ctx context.Context, c *cli.Command) error {
	manpage, err := docs.ToManWithSection(Command, 1)
	if err != nil {
		return err
	}
	dir := c.String("output")
	err = os.MkdirAll(filepath.Join(dir, "man1"), 0755)
	if err != nil {
		// handle error
	}
	if c.Bool("text") {
		file, err := os.Create(filepath.Join(dir, "man1", "agentmail.1"))
		if err != nil {
			return err
		}
		defer file.Close()
		if _, err := file.WriteString(manpage); err != nil {
			return err
		}
	}
	if c.Bool("gzip") {
		file, err := os.Create(filepath.Join(dir, "man1", "agentmail.1.gz"))
		if err != nil {
			return err
		}
		defer file.Close()
		gzWriter := gzip.NewWriter(file)
		defer gzWriter.Close()
		_, err = gzWriter.Write([]byte(manpage))
		if err != nil {
			return err
		}
	}
	fmt.Printf("Wrote manpages to %s\n", dir)
	return nil
}
