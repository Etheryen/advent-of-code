package main

import (
	"day-05/utils"
	"fmt"
	"slices"
	"sort"
	"strconv"
	"strings"
)

type AlmanacRange struct {
	destinationStart int
	sourceStart      int
	length           int
}

type SeedRange struct {
	start int
	end   int
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

func getAlmanacMaps(lines []string) [][][]int {
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

func getAlmanacRanges(maps [][][]int) [][]AlmanacRange {
	var mapsRanges [][]AlmanacRange

	for i, mapEl := range maps {
		mapsRanges = append(mapsRanges, []AlmanacRange{})
		for _, row := range mapEl {
			mapsRanges[i] = append(
				mapsRanges[i],
				AlmanacRange{
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

func getTransformedNumbers(seeds []int, mapsRanges [][]AlmanacRange) []int {
	cache := make(map[int]int)
	var transformedNumbers []int

	for _, seed := range seeds {
		if _, ok := cache[seed]; ok {
			fmt.Println("cache hit on seed:", seed)
			continue
		}
		originalSeed := seed
		// fmt.Println("original seed:", seed)
		for _, rangeArr := range mapsRanges {
			// fmt.Println("    seed:", seed)
			var correctRange AlmanacRange

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
		cache[originalSeed] = seed
	}

	return transformedNumbers
}

func getAllPossibleSeeds(seeds []int) []int {
	var allSeedsLength int

	for i := 1; i < len(seeds); i += 2 {
		allSeedsLength += seeds[i]
	}

	allPossibleSeeds := make([]int, 0, allSeedsLength)

	seedStart := -1
	for _, seed := range seeds {
		// fmt.Println(
		// 	"seed:",
		// 	seed,
		// 	"start:",
		// 	seedStart,
		// )
		if seedStart == -1 {
			seedStart = seed
			continue
		}
		for i := seedStart; i < seedStart+seed; i++ {
			// fmt.Println("i:", i)
			if slices.Contains(allPossibleSeeds, i) {
				// fmt.Println("cache")
				continue
			}
			allPossibleSeeds = append(allPossibleSeeds, i)
		}
		seedStart = -1
	}

	return allPossibleSeeds
}

func getSeedRanges(seeds []int) []SeedRange {
	var seedRanges []SeedRange

	for i := 0; i < len(seeds); i += 2 {
		seedRanges = append(
			seedRanges,
			SeedRange{start: seeds[i], end: seeds[i] + seeds[i+1]},
		)
	}

	return seedRanges
}

func getTransformedSeedRangeIntoRanges(
	seedRange SeedRange,
	almanacRange AlmanacRange,
) []SeedRange {
	if seedRange.start > almanacRange.sourceStart+almanacRange.length ||
		seedRange.end < almanacRange.sourceStart {
		return []SeedRange{seedRange}
	}

	var result []SeedRange

	currNum := seedRange.start
	for currNum <= seedRange.end {
		diff := almanacRange.destinationStart - almanacRange.sourceStart

		end := min(
			almanacRange.destinationStart+almanacRange.length,
			seedRange.end+diff,
		)

		result = append(result, SeedRange{
			start: currNum + diff,
			end:   end,
		})

		fmt.Println("result:", result)

		fmt.Println("diff:", diff)
		break
	}

	return result
}

func getTransformedRanges(
	seedRanges []SeedRange,
	almanacRanges [][]AlmanacRange,
) []SeedRange {
	var transformedRanges []SeedRange

	fmt.Println("seedRanges:")
	utils.PrintArray(seedRanges)
	fmt.Println("almanacRanges:")
	utils.PrintArray(almanacRanges)

	for _, seedRange := range seedRanges {
		for _, almanacRangeSet := range almanacRanges {
			for _, almanacRange := range almanacRangeSet {
				fmt.Println(
					"seedRange:",
					seedRange,
					"almanacRange:",
					almanacRange,
				)
				fmt.Println(
					"ranges from transformation:",
					getTransformedSeedRangeIntoRanges(seedRange, almanacRange),
				)
			}
			break
		}
		break
	}

	return transformedRanges
}

func solve(lines []string) int {
	seeds := getSeeds(lines[0])
	allPossibleSeeds := getAllPossibleSeeds(seeds)
	// fmt.Println("allPossibleSeeds:", allPossibleSeeds)

	// seedRanges := getSeedRanges(seeds)
	almanacMaps := getAlmanacMaps(lines)
	almanacRanges := getAlmanacRanges(almanacMaps)

	transformedNumbers := getTransformedNumbers(allPossibleSeeds, almanacRanges)
	// transformedRanges := getTransformedRanges(seedRanges, almanacRanges)
	// fmt.Println("transformedRanges:")
	// utils.PrintArray(transformedRanges)

	lowest := transformedNumbers[0]
	for _, num := range transformedNumbers {
		if num < lowest {
			lowest = num
		}
	}

	return lowest
}

func main() {
	lines := utils.InputToLines("input.txt")
	// fmt.Println("lines:")
	// utils.PrintArray(lines)
	fmt.Println("\nsolved:", solve(lines))
}
