package main

import (
	utils "day-04"
	"fmt"
	"slices"
	"strconv"
	"strings"
)

func getNumbers(line string) ([]int, []int) {
	content := strings.Split(strings.Split(line, ": ")[1], " | ")
	firstPart := strings.Split(content[0], " ")
	secondPart := strings.Split(content[1], " ")
	var winning []int
	var cardNumbers []int

	for _, element := range firstPart {
		if strings.Trim(element, " ") != "" {
			num, _ := strconv.Atoi(element)
			winning = append(winning, num)
		}
	}
	for _, element := range secondPart {
		if strings.Trim(element, " ") != "" {
			num, _ := strconv.Atoi(element)
			cardNumbers = append(cardNumbers, num)
		}
	}

	return winning, cardNumbers
}

func getPointsArray(lines []string) []int {
	var points []int

	for _, line := range lines {
		winning, cardNumbers := getNumbers(line)

		var matching []int
		for _, num := range cardNumbers {
			if slices.Contains(winning, num) {
				matching = append(matching, num)
			}
		}

		// fmt.Println(matching)

		if len(matching) > 0 {
			points = append(points, 1<<(len(matching)-1))
		}
	}

	return points
}

func solve(lines []string) int {
	pointsArray := getPointsArray(lines)

	var sum int
	for _, point := range pointsArray {
		sum += point
	}
	return sum
}

func main() {
	lines := utils.InputToLines("input.txt")
	// fmt.Println("lines:")
	// utils.PrintArray(lines)
	// fmt.Println()
	fmt.Println("solved:", solve(lines))
}
