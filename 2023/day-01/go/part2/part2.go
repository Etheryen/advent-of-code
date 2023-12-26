package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

var spelledDigits = [...]string{
	"one",
	"two",
	"three",
	"four",
	"five",
	"six",
	"seven",
	"eight",
	"nine",
}

func getLineNumber(line string) int {
	num1 := -1
	i1 := -1
	num2 := -1
	i2 := -1

	for i, char := range line {
		parsedNum, err := strconv.Atoi(string(char))
		if err != nil {
			continue
		}
		if num1 == -1 {
			num1 = parsedNum
			i1 = i
			continue
		}
		num2 = parsedNum
		i2 = i
	}

	if num2 == -1 {
		num2 = num1
		i2 = i1
	}

	firstWordDigit := -1
	firstWordIndex := -1
	lastWordDigit := -1
	lastWordIndex := -1

	for i, digit := range spelledDigits {
		foundFirstDigitIndex := strings.Index(line, digit)
		foundLastDigitIndex := strings.LastIndex(line, digit)

		if foundFirstDigitIndex != -1 && (firstWordDigit == -1 ||
			firstWordIndex == -1 ||
			foundFirstDigitIndex < firstWordIndex) {
			firstWordIndex = foundFirstDigitIndex
			firstWordDigit = i + 1
		}

		if foundLastDigitIndex != -1 && (lastWordDigit == -1 ||
			lastWordIndex == -1 ||
			foundLastDigitIndex > lastWordIndex) {
			lastWordIndex = foundLastDigitIndex
			lastWordDigit = i + 1
		}

	}

	// fmt.Println("num1:", num1, "i1:", i1)
	// fmt.Println("num2:", num2, "i2:", i2)
	// fmt.Println(
	// 	"firstWordDigit:",
	// 	firstWordDigit,
	// 	"firstWordIndex:",
	// 	firstWordIndex,
	// )
	// fmt.Println(
	// 	"lastWordDigit:",
	// 	lastWordDigit,
	// 	"lastWordIndex:",
	// 	lastWordIndex,
	// )

	var finalFirstDigit int
	var finalLastDigit int

	if i1 != -1 && (i1 < firstWordIndex || firstWordDigit == -1) {
		finalFirstDigit = num1
	} else {
		finalFirstDigit = firstWordDigit
	}

	if i2 != -1 && (i2 > lastWordIndex || lastWordDigit == -1) {
		finalLastDigit = num2
	} else {
		finalLastDigit = lastWordDigit
	}

	resultNum := fmt.Sprintf(
		"%s%s",
		strconv.Itoa(finalFirstDigit),
		strconv.Itoa(finalLastDigit),
	)

	parsedResultNum, _ := strconv.Atoi(resultNum)

	return parsedResultNum
}

func solve(lines []string) int {
	var result []int

	for _, line := range lines {
		// fmt.Println("Line :", line)
		result = append(result, getLineNumber(line))
		// fmt.Println("result:", result[i])
		// fmt.Println()
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

	// fmt.Println("lines:", notEmptyLines)
	fmt.Println("solve:", solve(notEmptyLines))
}
