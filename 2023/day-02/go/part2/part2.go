package main

import (
	utils "day-02"
	"fmt"
	"strconv"
	"strings"
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

func getPower(line string) int {
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

	max_red := 0
	max_green := 0
	max_blue := 0

	for _, amountColor := range amountsColors {
		switch amountColor.color {
		case Red:
			if amountColor.amount > max_red {
				max_red = amountColor.amount
			}
		case Green:
			if amountColor.amount > max_green {
				max_green = amountColor.amount
			}
		case Blue:
			if amountColor.amount > max_blue {
				max_blue = amountColor.amount
			}
		}
	}

	// utils.PrintArray(amountsColors)

	return max_red * max_green * max_blue
}

func solve(lines []string) int {
	var powers []int
	for _, line := range lines {
		powers = append(powers, getPower(line))
		// fmt.Println()
	}
	sum := 0
	for _, power := range powers {
		sum += power
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
