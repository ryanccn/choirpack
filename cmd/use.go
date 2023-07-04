package cmd

import (
	"fmt"
	"log"
	"os"
	"os/exec"

	"github.com/fatih/color"
	"github.com/ryanccn/choirpack/corepack"
	"github.com/ryanccn/choirpack/packagejson"
	"github.com/ryanccn/choirpack/utils"
	"github.com/spf13/cobra"
)

var configPaths = []string{".yarnrc.yml", ".yarnrc", ".npmrc", "package-lock.json", "yarn.lock", "pnpm-lock.yaml"}
var installPaths = []string{"node_modules", ".yarn", ".pnp.cjs"}

var UseCmd = &cobra.Command{
	Use:   "use <name>",
	Short: "Use a package manager in the current project",
	Args:  cobra.MatchAll(cobra.ExactArgs(1), cobra.OnlyValidArgs),

	Run: func(cmd *cobra.Command, args []string) {
		errorLog := color.New(color.FgRed).SetWriter(os.Stderr)
		if !utils.Exists("package.json") {
			errorLog.Println("Working directory doesn't seem to be a Node.js package.")
			os.Exit(1)
		}

		if !corepack.CorepackStatus() {
			errorLog.Println("Corepack is not enabled or misconfigured. Try running `corepack enable`.")
			os.Exit(1)
		}

		chosenPackageManager := args[0]

		for _, path := range configPaths {
			err := os.RemoveAll(path)
			if err != nil {
				log.Fatal(err)
			}
		}
		fmt.Printf("%v old configuration files\n", color.New(color.FgYellow).Sprint("Cleared"))

		versions := corepack.GetCorepackVersions()

		var chosenVersion string
		if chosenPackageManager == "npm" {
			chosenVersion = versions.Npm
		} else if chosenPackageManager == "yarn" {
			chosenVersion = versions.Yarn
		} else if chosenPackageManager == "pnpm" {
			chosenVersion = versions.Pnpm
		}

		packagejson.Modify(chosenPackageManager, chosenVersion)
		fmt.Printf("%v package.json to use %v@%v\n", color.New(color.FgCyan).Sprint("Modified"), chosenPackageManager, chosenVersion)

		prettified := packagejson.Prettify()
		if prettified {
			fmt.Printf("%v package.json with Prettier\n", color.New(color.FgCyan).Sprint("Formatted"))
		}

		for _, path := range installPaths {
			err := os.RemoveAll(path)
			if err != nil {
				log.Fatal(err)
			}
		}

		fmt.Printf("%v dependencies\n", color.New(color.FgGreen).Sprint("Reinstalling"))

		installCommand := exec.Command(chosenPackageManager, "install")
		installCommand.Stdout = os.Stdout
		installCommand.Stderr = os.Stderr

		if err := installCommand.Run(); err != nil {
			log.Fatal(err)
		}
	},
}
