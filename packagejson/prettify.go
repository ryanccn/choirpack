package packagejson

import (
	"errors"
	"log"
	"os"
	"os/exec"
	"path/filepath"

	"github.com/ryanccn/choirpack/utils"
)

func Prettify() bool {
	cwd, err := os.Getwd()
	if err != nil {
		log.Fatal(err)
	}

	prettierPath, err := exec.LookPath("prettier")

	if err != nil && !errors.Is(err, exec.ErrNotFound) {
		log.Fatal(err)
	} else if err != nil {
		localPrettier := filepath.Join(cwd, "node_modules", ".bin", "prettier")

		if utils.Exists(localPrettier) {
			prettierPath = localPrettier
		} else {
			return false
		}
	}

	prettierCmd := exec.Command(prettierPath, "--write", filepath.Join(cwd, "package.json"))

	if err := prettierCmd.Run(); err != nil {
		log.Fatal(err)
	}

	return true
}
