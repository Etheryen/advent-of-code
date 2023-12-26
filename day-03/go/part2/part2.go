package main

import (
	utils "day-03"
	"fmt"
	"slices"
	"strconv"
)

type Coordinates struct {
	y int
	x int
}

func getGearsCoordinates(lines []string) []Coordinates {
	var coords []Coordinates

	for y, line := range lines {
		for x, char := range line {
			if string(char) == "*" {
				coords = append(coords, Coordinates{
					y, x,
				})
			}
		}
	}

	return coords
}

func getNumbersAround(lines []string, coords Coordinates) []int {
	var found []int
	var buffer string
	var isGearFound bool

	allGearCoords := []Coordinates{
		{y: coords.y - 1, x: coords.x - 1},
		{y: coords.y - 1, x: coords.x},
		{y: coords.y - 1, x: coords.x + 1},
		{y: coords.y, x: coords.x - 1},
		{y: coords.y, x: coords.x},
		{y: coords.y, x: coords.x + 1},
		{y: coords.y + 1, x: coords.x - 1},
		{y: coords.y + 1, x: coords.x},
		{y: coords.y + 1, x: coords.x + 1},
	}

	for y, line := range lines {
		for x, char := range line {
			_, parseError := strconv.Atoi(string(char))
			if parseError == nil {
				buffer += string(char)
				if slices.Contains(allGearCoords, Coordinates{y, x}) {
					// fmt.Println("Found symbol at y:", y, "x:", x)
					isGearFound = true
					if x == len(line)-1 {
						foundNum, _ := strconv.Atoi(buffer)
						found = append(found, foundNum)
						buffer = ""
						isGearFound = false
					}
				}
				continue
			}
			if parseError != nil {
				if !isGearFound {
					buffer = ""
					continue
				}
				if len(buffer) > 0 {
					foundNum, _ := strconv.Atoi(buffer)
					found = append(found, foundNum)
					buffer = ""
					isGearFound = false
				}
			}
		}
	}

	return found
}

func getGearPairs(lines []string) [][]int {
	gears := getGearsCoordinates(lines)
	// utils.PrintArray(gears)

	var pairs [][]int

	for _, gear := range gears {
		numbersAround := getNumbersAround(lines, gear)
		if len(numbersAround) == 2 {
			pairs = append(pairs, numbersAround)
		}
	}

	return pairs
}

func solve(lines []string) int {
	gearParis := getGearPairs(lines)
	// utils.PrintArray(gearParis)

	var sum int
	for _, pair := range gearParis {
		sum += pair[0] * pair[1]
	}
	return sum
}

func main() {
	lines := utils.InputToLines("input.txt")
	// fmt.Println("lines:")
	// utils.PrintArray(lines)
	// fmt.Println()
	fmt.Println("\nsolved:", solve(lines))
}
