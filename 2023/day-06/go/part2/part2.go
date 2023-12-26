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

func getRace(lines []string) Race {
	timesStrings := strings.Split(strings.Split(lines[0], ":")[1], " ")
	var totalTimeString string

	for _, timeStr := range timesStrings {
		_, err := strconv.Atoi(strings.Trim(timeStr, " "))
		if err != nil {
			continue
		}
		totalTimeString += timeStr
	}

	distancesStrings := strings.Split(strings.Split(lines[1], ":")[1], " ")
	var totalDistanceStr string

	for _, distanceStr := range distancesStrings {
		_, err := strconv.Atoi(strings.Trim(distanceStr, " "))
		if err != nil {
			continue
		}
		totalDistanceStr += distanceStr
	}

	timeNum, _ := strconv.Atoi(totalTimeString)
	distnceNum, _ := strconv.Atoi(totalDistanceStr)

	return Race{time: timeNum, distance: distnceNum}
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

func solve(lines []string) int {
	race := getRace(lines)
	allWays := getRacePossibleWays(race)

	return allWays
}

func main() {
	lines := utils.InputToLines("input.txt")
	fmt.Println("lines:")
	utils.PrintArray(lines)
	fmt.Println("\nsolved:", solve(lines))
}
