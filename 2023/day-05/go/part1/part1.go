package main

import (
	"day-05/utils"
	"fmt"
	"sort"
	"strconv"
	"strings"
)

type Range struct {
	destinationStart int
	sourceStart      int
	length           int
}

func getSeeds(line string) []int {
	var seeds []int

	numbers := strings.Split(strings.Split(line, ": ")[1], " ")
	for _, numStr := range numbers {
		numInt, _ := strconv.Atoi(numStr)
		seeds = append(seeds, numInt)
	}

	return seeds
}

func getMaps(lines []string) [][][]int {
	var maps [][][]int

	var linesWithoutSeeds []string
	for i, line := range lines {
		if i != 0 {
			linesWithoutSeeds = append(linesWithoutSeeds, line)
		}
	}

	currSegment := -1

	for _, line := range linesWithoutSeeds {
		// fmt.Println("maps:", maps)
		// fmt.Println("line:", line, "currSegment:", currSegment)
		if strings.Contains(line, "map") {
			currSegment++
			maps = append(maps, [][]int{})
			continue
		}

		numStrings := strings.Split(line, " ")
		var lineNums []int

		for _, numStr := range numStrings {
			numInt, _ := strconv.Atoi(numStr)
			// fmt.Println("numInt:", numInt)
			lineNums = append(lineNums, numInt)
		}
		maps[currSegment] = append(
			maps[currSegment],
			lineNums,
		)
	}

	return maps
}

func getMapsRanges(maps [][][]int) [][]Range {
	var mapsRanges [][]Range

	for i, mapEl := range maps {
		mapsRanges = append(mapsRanges, []Range{})
		for _, row := range mapEl {
			mapsRanges[i] = append(
				mapsRanges[i],
				Range{
					destinationStart: row[0],
					sourceStart:      row[1],
					length:           row[2],
				},
			)
		}
	}

	for _, rangeArr := range mapsRanges {
		sort.Slice(rangeArr, func(i, j int) bool {
			return rangeArr[i].sourceStart < rangeArr[j].sourceStart
		})
	}

	return mapsRanges
}

func getTransformedNumbers(seeds []int, mapsRanges [][]Range) []int {
	var transformedNumbers []int

	for _, seed := range seeds {
		// fmt.Println("original seed:", seed)
		for _, rangeArr := range mapsRanges {
			// fmt.Println("    seed:", seed)
			var correctRange Range

			for _, mapRange := range rangeArr {
				if mapRange.sourceStart <= seed &&
					mapRange.sourceStart+mapRange.length > seed {
					correctRange = mapRange
				}
			}
			// fmt.Println("    correctRange:", correctRange)

			if correctRange.length == 0 {
				continue
			}

			seed = correctRange.destinationStart + seed - correctRange.sourceStart
		}
		transformedNumbers = append(transformedNumbers, seed)
	}

	return transformedNumbers
}

func solve(lines []string) int {
	seeds := getSeeds(lines[0])
	maps := getMaps(lines)
	mapsRanges := getMapsRanges(maps)
	transformedNumbers := getTransformedNumbers(seeds, mapsRanges)

	// fmt.Println("seeds:", seeds)
	// fmt.Println("transformedNumbers:")
	// utils.PrintArray(transformedNumbers)

	lowest := transformedNumbers[0]
	for _, num := range transformedNumbers {
		if num < lowest {
			lowest = num
		}
	}

	return lowest
}

func main() {
	lines := utils.InputToLines("input0.txt")
	// fmt.Println("lines:")
	// utils.PrintArray(lines)
	fmt.Println("\nsolved:", solve(lines))
}
