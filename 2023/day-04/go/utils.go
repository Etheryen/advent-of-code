package utils

import (
	"fmt"
	"os"
	"strings"
)

func InputToLines(file string) []string {
	data, _ := os.ReadFile(file)
	lines := strings.Split(string(data), "\n")
	var notEmptyLines []string

	for _, line := range lines {
		if line != "" {
			notEmptyLines = append(notEmptyLines, line)
		}
	}

	return notEmptyLines
}

func PrintArray[T any](arr []T) {
	fmt.Println("[")
	for _, element := range arr {
		fmt.Println("  ", element, ",")
	}
	fmt.Println("]")
}
