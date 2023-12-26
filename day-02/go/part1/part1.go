package main

import (
	utils "day-02"
	"fmt"
	"strconv"
	"strings"
)

const (
	MAX_RED   = 12
	MAX_GREEN = 13
	MAX_BLUE  = 14
)

type Color string

const (
	Red   Color = "red"
	Green Color = "green"
	Blue  Color = "blue"
)

type AmountColor struct {
	amount int
	color  Color
}

func getGameId(line string) int {
	id := strings.Split(strings.Split(line, "Game ")[1], ":")[0]
	result, _ := strconv.Atoi(id)
	return result
}

func isGamePossible(line string) bool {
	sets := strings.Split(strings.Split(line, ":")[1], ";")
	var draws []string
	for _, set := range sets {
		setDraws := strings.Split(set, ",")
		for _, setDraw := range setDraws {
			draws = append(draws, strings.Trim(setDraw, " "))
		}
	}

	var amountsColors []AmountColor

	for _, draw := range draws {
		amountColorArray := strings.Split(draw, " ")
		amount, _ := strconv.Atoi(amountColorArray[0])
		color := amountColorArray[1]

		var foundColor Color
		switch color {
		case "red":
			foundColor = Red
		case "green":
			foundColor = Green
		case "blue":
			foundColor = Blue
		}

		amountsColors = append(
			amountsColors,
			AmountColor{amount: amount, color: foundColor},
		)
	}

	for _, amountColor := range amountsColors {
		switch amountColor.color {
		case Red:
			if amountColor.amount > MAX_RED {
				return false
			}
		case Green:
			if amountColor.amount > MAX_GREEN {
				return false
			}
		case Blue:
			if amountColor.amount > MAX_BLUE {
				return false
			}
		}
	}

	// utils.PrintArray(amountsColors)

	return true
}

func solve(lines []string) int {
	var possibleGameIds []int
	for _, line := range lines {
		if isGamePossible(line) {
			possibleGameIds = append(possibleGameIds, getGameId(line))
			// fmt.Println()
		}
	}
	sum := 0
	for _, possibleGameId := range possibleGameIds {
		sum += possibleGameId
	}
	return sum
}

func main() {
	lines := utils.InputToLines("input.txt")
	// fmt.Println("lines:", lines)
	// fmt.Println()
	fmt.Println("solved:", solve(lines))
}
