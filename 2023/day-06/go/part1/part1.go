package main

import (
	"day-06/utils"
	"fmt"
	"strconv"
	"strings"
)

type Race struct {
	time     int
	distance int
}

func getRaces(lines []string) []Race {
	var races []Race

	timesStrings := strings.Split(strings.Split(lines[0], ":")[1], " ")
	var timesNums []int

	for _, timeStr := range timesStrings {
		timeNum, err := strconv.Atoi(strings.Trim(timeStr, " "))
		if err != nil {
			continue
		}
		timesNums = append(timesNums, timeNum)
	}

	distancesStrings := strings.Split(strings.Split(lines[1], ":")[1], " ")
	var distancesNums []int

	for _, distanceStr := range distancesStrings {
		distanceNum, err := strconv.Atoi(strings.Trim(distanceStr, " "))
		if err != nil {
			continue
		}
		distancesNums = append(distancesNums, distanceNum)
	}

	for i := 0; i < len(timesNums); i++ {
		races = append(races, Race{
			time:     timesNums[i],
			distance: distancesNums[i],
		})
	}

	return races
}

func getRacePossibleWays(race Race) int {
	var waysAmount int

	for timeHeld := 1; timeHeld < race.time; timeHeld++ {
		if timeHeld*(race.time-timeHeld) > race.distance {
			waysAmount++
		}
	}

	fmt.Println("race:", race, "ways:", waysAmount)

	return waysAmount
}

func getAllPossibleWays(races []Race) []int {
	var allWays []int

	for _, race := range races {
		allWays = append(allWays, getRacePossibleWays(race))
	}

	return allWays
}

func solve(lines []string) int {
	races := getRaces(lines)
	allWays := getAllPossibleWays(races)

	product := 1
	for _, way := range allWays {
		product *= way
	}
	return product
}

func main() {
	lines := utils.InputToLines("input.txt")
	fmt.Println("lines:")
	utils.PrintArray(lines)
	fmt.Println("\nsolved:", solve(lines))
}
