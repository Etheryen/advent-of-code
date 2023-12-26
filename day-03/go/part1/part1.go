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

func getSymbolCoordinates(lines []string) []Coordinates {
	var coords []Coordinates

	for y, line := range lines {
		for x, char := range line {
			if _, err := strconv.Atoi(string(char)); err != nil &&
				string(char) != "." {
				coords = append(coords, Coordinates{
					y: y - 1, x: x - 1,
				})
				coords = append(coords, Coordinates{
					y: y - 1, x: x,
				})
				coords = append(coords, Coordinates{
					y: y - 1, x: x + 1,
				})
				coords = append(coords, Coordinates{
					y: y, x: x - 1,
				})
				coords = append(coords, Coordinates{
					y, x,
				})
				coords = append(coords, Coordinates{
					y: y, x: x + 1,
				})
				coords = append(coords, Coordinates{
					y: y + 1, x: x - 1,
				})
				coords = append(coords, Coordinates{
					y: y + 1, x: x,
				})
				coords = append(coords, Coordinates{
					y: y + 1, x: x + 1,
				})
			}
		}
	}

	return coords
}

func getPartNumbers(lines []string) []int {
	symbolCoordinates := getSymbolCoordinates(lines)
	// utils.PrintArray(symbolCoordinates)
	var buffer string
	var foundPartNumbers []int
	var isSymbolFound bool

	for y, line := range lines {
		for x, char := range line {
			_, parseError := strconv.Atoi(string(char))
			if parseError == nil {
				buffer += string(char)
				if slices.Contains(symbolCoordinates, Coordinates{y, x}) {
					// fmt.Println("Found symbol at y:", y, "x:", x)
					isSymbolFound = true
					if x == len(line)-1 {
						foundNum, _ := strconv.Atoi(buffer)
						foundPartNumbers = append(foundPartNumbers, foundNum)
						buffer = ""
						isSymbolFound = false
					}
				}
				continue
			}
			if parseError != nil {
				if !isSymbolFound {
					buffer = ""
					continue
				}
				if len(buffer) > 0 {
					foundNum, _ := strconv.Atoi(buffer)
					foundPartNumbers = append(foundPartNumbers, foundNum)
					buffer = ""
					isSymbolFound = false
				}
			}
		}
	}

	return foundPartNumbers
}

func solve(lines []string) int {
	partNumbers := getPartNumbers(lines)
	utils.PrintArray(partNumbers)

	var sum int
	for _, num := range partNumbers {
		sum += num
	}
	return sum
}

func main() {
	lines := utils.InputToLines("input.txt")
	fmt.Println("lines:")
	utils.PrintArray(lines)
	fmt.Println()
	fmt.Println("\nsolved:", solve(lines))
}
