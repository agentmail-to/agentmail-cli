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
				Validator: func(baseURL string) error {
					return ValidateBaseURL(baseURL, "--base-url")
				},
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
					&inboxesUpdate,
					&inboxesList,
					&inboxesDelete,
					&inboxesGet,
					&inboxesListMetrics,
				},
			},
			{
				Name:     "inboxes:drafts",
				Category: "API RESOURCE",
				Suggest:  true,
				Commands: []*cli.Command{
					&inboxesDraftsCreate,
					&inboxesDraftsUpdate,
					&inboxesDraftsList,
					&inboxesDraftsDelete,
					&inboxesDraftsGet,
					&inboxesDraftsGetAttachment,
					&inboxesDraftsSend,
				},
			},
			{
				Name:     "inboxes:messages",
				Category: "API RESOURCE",
				Suggest:  true,
				Commands: []*cli.Command{
					&inboxesMessagesUpdate,
					&inboxesMessagesList,
					&inboxesMessagesForward,
					&inboxesMessagesGet,
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
					&inboxesThreadsList,
					&inboxesThreadsDelete,
					&inboxesThreadsGet,
					&inboxesThreadsGetAttachment,
				},
			},
			{
				Name:     "inboxes:lists",
				Category: "API RESOURCE",
				Suggest:  true,
				Commands: []*cli.Command{
					&inboxesListsCreate,
					&inboxesListsList,
					&inboxesListsDelete,
					&inboxesListsGet,
				},
			},
			{
				Name:     "inboxes:api-keys",
				Category: "API RESOURCE",
				Suggest:  true,
				Commands: []*cli.Command{
					&inboxesAPIKeysCreate,
					&inboxesAPIKeysList,
					&inboxesAPIKeysDelete,
				},
			},
			{
				Name:     "pods",
				Category: "API RESOURCE",
				Suggest:  true,
				Commands: []*cli.Command{
					&podsCreate,
					&podsList,
					&podsDelete,
					&podsGet,
				},
			},
			{
				Name:     "pods:domains",
				Category: "API RESOURCE",
				Suggest:  true,
				Commands: []*cli.Command{
					&podsDomainsCreate,
					&podsDomainsUpdate,
					&podsDomainsList,
					&podsDomainsDelete,
					&podsDomainsGet,
					&podsDomainsGetZoneFile,
					&podsDomainsVerify,
				},
			},
			{
				Name:     "pods:drafts",
				Category: "API RESOURCE",
				Suggest:  true,
				Commands: []*cli.Command{
					&podsDraftsList,
					&podsDraftsGet,
					&podsDraftsGetAttachment,
				},
			},
			{
				Name:     "pods:inboxes",
				Category: "API RESOURCE",
				Suggest:  true,
				Commands: []*cli.Command{
					&podsInboxesCreate,
					&podsInboxesUpdate,
					&podsInboxesList,
					&podsInboxesDelete,
					&podsInboxesGet,
				},
			},
			{
				Name:     "pods:threads",
				Category: "API RESOURCE",
				Suggest:  true,
				Commands: []*cli.Command{
					&podsThreadsList,
					&podsThreadsDelete,
					&podsThreadsGet,
					&podsThreadsGetAttachment,
				},
			},
			{
				Name:     "pods:lists",
				Category: "API RESOURCE",
				Suggest:  true,
				Commands: []*cli.Command{
					&podsListsCreate,
					&podsListsList,
					&podsListsDelete,
					&podsListsGet,
				},
			},
			{
				Name:     "pods:api-keys",
				Category: "API RESOURCE",
				Suggest:  true,
				Commands: []*cli.Command{
					&podsAPIKeysCreate,
					&podsAPIKeysList,
					&podsAPIKeysDelete,
				},
			},
			{
				Name:     "pods:metrics",
				Category: "API RESOURCE",
				Suggest:  true,
				Commands: []*cli.Command{
					&podsMetricsQuery,
				},
			},
			{
				Name:     "webhooks",
				Category: "API RESOURCE",
				Suggest:  true,
				Commands: []*cli.Command{
					&webhooksCreate,
					&webhooksUpdate,
					&webhooksList,
					&webhooksDelete,
					&webhooksGet,
				},
			},
			{
				Name:     "api-keys",
				Category: "API RESOURCE",
				Suggest:  true,
				Commands: []*cli.Command{
					&apiKeysCreate,
					&apiKeysList,
					&apiKeysDelete,
				},
			},
			{
				Name:     "domains",
				Category: "API RESOURCE",
				Suggest:  true,
				Commands: []*cli.Command{
					&domainsCreate,
					&domainsUpdate,
					&domainsList,
					&domainsDelete,
					&domainsGet,
					&domainsGetZoneFile,
					&domainsVerify,
				},
			},
			{
				Name:     "drafts",
				Category: "API RESOURCE",
				Suggest:  true,
				Commands: []*cli.Command{
					&draftsList,
					&draftsGet,
					&draftsGetAttachment,
				},
			},
			{
				Name:     "lists",
				Category: "API RESOURCE",
				Suggest:  true,
				Commands: []*cli.Command{
					&listsCreate,
					&listsList,
					&listsDelete,
					&listsGet,
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
					&organizationsGet,
				},
			},
			{
				Name:     "threads",
				Category: "API RESOURCE",
				Suggest:  true,
				Commands: []*cli.Command{
					&threadsList,
					&threadsDelete,
					&threadsGet,
					&threadsGetAttachment,
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
