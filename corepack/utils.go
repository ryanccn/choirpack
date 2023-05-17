package corepack

import (
	"log"
	"os"
	"path"
	"runtime"
)

func getCorepackCacheDir() string {
	homeDir, err := os.UserHomeDir()

	if err != nil {
		log.Fatal(err)
	}

	xdgCacheHome, xdgCacheHomeExists := os.LookupEnv("XDG_CACHE_HOME")
	localAppData, localAppDataExists := os.LookupEnv("LOCALAPPDATA")

	cacheDir := path.Join(homeDir, ".cache")
	if runtime.GOOS == "windows" {
		cacheDir = path.Join(homeDir, "AppData/Local")
	}
	if xdgCacheHomeExists {
		cacheDir = xdgCacheHome
	} else if localAppDataExists {
		cacheDir = localAppData
	}

	return path.Join(cacheDir, "node", "corepack")
}
