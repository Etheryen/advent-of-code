package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func getLineNumber(line string) int {
	num1 := -1
	num2 := -1

	for _, char := range line {
		parsedNum, err := strconv.Atoi(string(char))
		if err != nil {
			continue
		}
		fmt.Println(parsedNum)
		if num1 == -1 {
			num1 = parsedNum
			continue
		}
		num2 = parsedNum
	}

	if num2 == -1 {
		num2 = num1
	}

	resultNum := fmt.Sprintf("%s%s", strconv.Itoa(num1), strconv.Itoa(num2))

	parsedResultNum, _ := strconv.Atoi(resultNum)

	return parsedResultNum
}

func solve(lines []string) int {
	var result []int

	for _, line := range lines {
		result = append(result, getLineNumber(line))
	}

	sum := 0

	for _, num := range result {
		sum += num
	}

	return sum
}

func main() {
	data, _ := os.ReadFile("input1.txt")
	lines := strings.Split(string(data), "\n")
	var notEmptyLines []string

	for _, line := range lines {
		if line != "" {
			notEmptyLines = append(notEmptyLines, line)
		}
	}

	fmt.Println("lines:", notEmptyLines)
	fmt.Println("solve:", solve(notEmptyLines))
}
