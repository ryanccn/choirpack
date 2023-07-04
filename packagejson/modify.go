package packagejson

import (
	"fmt"
	"log"
	"os"
	"path/filepath"

	"github.com/buger/jsonparser"
)

func Modify(packageManager string, version string) {
	cwd, err := os.Getwd()
	if err != nil {
		log.Fatal(err)
	}

	data, err := os.ReadFile(filepath.Join(cwd, "package.json"))
	if err != nil {
		log.Fatal(err)
	}

	packageManagerData := fmt.Sprintf("\"%v@%v\"", packageManager, version)

	data, err = jsonparser.Set(data, []byte(packageManagerData), "packageManager")
	if err != nil {
		log.Fatal(err)
	}

	os.WriteFile(filepath.Join(cwd, "package.json"), data, 0644)
}
