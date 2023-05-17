package corepack

import (
	"encoding/json"
	"log"
	"os"
	"path"
)

type CorepackVersion struct {
	Npm  string
	Pnpm string
	Yarn string
}

func GetCorepackVersions() CorepackVersion {
	fileLoc := path.Join(getCorepackCacheDir(), "lastKnownGood.json")
	file, err := os.ReadFile(fileLoc)

	if err != nil {
		log.Fatal(err)
	}

	var data CorepackVersion
	err = json.Unmarshal(file, &data)

	if err != nil {
		log.Fatal(err)
	}

	return data
}
