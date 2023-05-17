package corepack

import (
	"errors"
	"io"
	"log"
	"os"
	"os/exec"
	"strings"
)

func CorepackStatus() bool {
	binary, err := exec.LookPath("yarn")

	if err != nil && !errors.Is(err, exec.ErrNotFound) {
		log.Fatal(err)
	} else if err != nil {
		return false
	}

	binaryF, err := os.Open(binary)
	if err != nil {
		return false
	}

	binaryBytes, err := io.ReadAll(binaryF)
	if err != nil {
		return false
	}
	binaryString := string(binaryBytes)

	return strings.Contains(binaryString, "corepack")
}
