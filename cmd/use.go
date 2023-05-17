package cmd

import (
	"fmt"
	"log"
	"os"
	"os/exec"

	"github.com/ryanccn/choirpack/corepack"
	"github.com/ryanccn/choirpack/packagejson"
	"github.com/ryanccn/choirpack/utils"
	"github.com/spf13/cobra"
)

var cleanPaths = []string{"node_modules", ".yarnrc.yml", ".yarnrc", ".npmrc", "package-lock.json", "yarn.lock", "pnpm-lock.yaml"}

var UseCmd = &cobra.Command{
	Use:   "use <name>",
	Short: "Use a package manager in the current project",
	Args:  cobra.MatchAll(cobra.ExactArgs(1), cobra.OnlyValidArgs),

	Run: func(cmd *cobra.Command, args []string) {
		if !utils.Exists("package.json") {
			log.Fatalln("Working directory doesn't seem to be a Node.js package.")
		}

		if !corepack.CorepackStatus() {
			log.Fatalln("Corepack is not enabled or misconfigured. Try running `corepack enable`.")
		}

		chosenPackageManager := args[0]

		for _, path := range cleanPaths {
			err := os.RemoveAll(path)
			if err != nil {
				log.Fatal(err)
			}
		}
		fmt.Println("> Cleared stale files")

		versions := corepack.GetCorepackVersions()

		var chosenVersion string
		if chosenPackageManager == "npm" {
			chosenVersion = versions.Npm
		} else if chosenPackageManager == "yarn" {
			chosenVersion = versions.Yarn
		} else if chosenPackageManager == "pnpm" {
			chosenVersion = versions.Pnpm
		}

		packagejson.ModifyPackageJson(chosenPackageManager, chosenVersion)

		fmt.Printf("> Modified package.json to use %v@%v\n", chosenPackageManager, chosenVersion)
		fmt.Println("> Reinstalling dependencies")

		installCommand := exec.Command(chosenPackageManager, "install")
		installCommand.Stdout = os.Stdout
		installCommand.Stderr = os.Stderr

		if err := installCommand.Run(); err != nil {
			log.Fatal(err)
		}
	},
}
