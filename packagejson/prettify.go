package packagejson

import (
	"encoding/json"
	"log"
	"strings"
)

func prettifyJSON(data []byte) []byte {
	var rawData interface{}
	err := json.Unmarshal(data, &rawData)
	if err != nil {
		log.Fatal(err)
	}

	stringData := string(data[:])

	indentChar := "  "
	if strings.Contains(stringData, "    ") {
		indentChar = "    "
	} else if strings.Contains(stringData, "	") {
		indentChar = "	"
	}

	pretty, err := json.MarshalIndent(rawData, "", indentChar)
	if err != nil {
		log.Fatal(err)
	}

	return append(pretty, "\n"...)
}
