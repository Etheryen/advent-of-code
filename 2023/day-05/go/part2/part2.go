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

func (ar *AlmanacRange) diff() int {
	return ar.destinationStart - ar.sourceStart
}

func (ar *AlmanacRange) sourceInclusiveEnd() int {
	return ar.sourceStart + ar.length - 1
}

type SeedRange struct {
	start  int
	length int
}

func (sr *SeedRange) inclusiveEnd() int {
	return sr.start + sr.length - 1
}

func transformSeedRangeIntoRanges(
	seedRange SeedRange,
	almanacRange AlmanacRange,
) []SeedRange {
	// 1/5 If seeedRange is either entirely excluded from alamanacRange
	if seedRange.inclusiveEnd() < almanacRange.sourceStart ||
		seedRange.start > almanacRange.sourceInclusiveEnd() {
		return []SeedRange{seedRange}
	}

	// 2/5 If seeedRange is included entirely included in alamanacRange
	if seedRange.start >= almanacRange.sourceStart &&
		seedRange.inclusiveEnd() <= almanacRange.sourceInclusiveEnd() {
		return []SeedRange{
			{
				start:  seedRange.start + almanacRange.diff(),
				length: seedRange.length,
			},
		}
	}

	// 3/5 If seedRange is in a left union with alamancRange
	if seedRange.start < almanacRange.sourceStart &&
		seedRange.inclusiveEnd() >= almanacRange.sourceStart &&
		seedRange.inclusiveEnd() <= almanacRange.sourceInclusiveEnd() {
		fmt.Println("\tLEFT UNION")
		return []SeedRange{}
	}

	// 4/5 If seedRange is in a right union with alamancRange
	if seedRange.inclusiveEnd() > almanacRange.sourceInclusiveEnd() &&
		seedRange.start <= almanacRange.sourceInclusiveEnd() &&
		seedRange.start >= almanacRange.sourceStart {
		fmt.Println("\tRIGHT UNION")
		return []SeedRange{}
	}

	// 5/5 If alamancRange is included entirely in seedRange
	if seedRange.start < almanacRange.sourceStart &&
		seedRange.inclusiveEnd() > almanacRange.sourceInclusiveEnd() {
		fmt.Println("\tDIFFICULT INCLUSION")
		return []SeedRange{}
	}

	panic("Unhandled range intersection")

	var result []SeedRange

	if seedRange.start < almanacRange.sourceStart {
		result = append(result, SeedRange{
			start:  seedRange.start,
			length: almanacRange.sourceStart - seedRange.start,
		})
	}

	newLength := min(
		seedRange.inclusiveEnd()-almanacRange.sourceStart,
		almanacRange.sourceInclusiveEnd()-seedRange.start,
	)

	result = append(result, SeedRange{
		start:  almanacRange.sourceStart + almanacRange.diff(),
		length: newLength,
	})

	if seedRange.inclusiveEnd() > almanacRange.sourceInclusiveEnd() {
		start := almanacRange.sourceInclusiveEnd() + 1
		result = append(result, SeedRange{
			start:  start,
			length: min(seedRange.inclusiveEnd()-start, seedRange.length),
		})
	}

	return result
}

func getTransformedRanges(
	seedRanges []SeedRange,
	almanacRanges [][]AlmanacRange,
) []SeedRange {
	fmt.Println("seedRanges:")
	utils.PrintArray(seedRanges)
	fmt.Println("almanacRanges:")
	utils.PrintArray(almanacRanges)

	var transformedRanges []SeedRange

	// TODO: fix not using transformed ranges in next iterations
	for _, almanacRangeSet := range almanacRanges {
		for _, seedRange := range seedRanges {
			for _, almanacRange := range almanacRangeSet {
				fmt.Println(
					"seedRange:",
					seedRange,
					"almanacRange:",
					almanacRange,
				)

				newRanges := transformSeedRangeIntoRanges(
					seedRange,
					almanacRange,
				)

				fmt.Println(
					"ranges from transformation:", newRanges)

				transformedRanges = append(transformedRanges, newRanges...)
			}
		}
		fmt.Println("RANGES:", seedRanges)
		seedRanges = withoutDuplicates(transformedRanges)
		transformedRanges = []SeedRange{}
	}

	return seedRanges
}

func withoutDuplicates(ranges []SeedRange) []SeedRange {
	var result []SeedRange

	for _, r := range ranges {
		if !slices.ContainsFunc(result, func(testRange SeedRange) bool {
			return r.start == testRange.start && r.length == testRange.length
		}) {
			result = append(result, r)
		}
	}

	return result
}

func lowestLocation(ranges []SeedRange) int {
	result := ranges[0].start

	for _, r := range ranges {
		if r.start < result {
			result = r.start
		}
	}

	return result
}

func solve(lines []string) int {
	seeds := getSeeds(lines[0])
	seedRanges := getSeedRanges(seeds)
	almanacMaps := getAlmanacMaps(lines)
	almanacRanges := getAlmanacRanges(almanacMaps)

	transformedRanges := getTransformedRanges(seedRanges, almanacRanges)

	fmt.Println("transformedRanges:")
	utils.PrintArray(transformedRanges)

	// TODO: find smallest number in final ranges

	return lowestLocation(transformedRanges)
}

func main() {
	lines := utils.InputToLines("input.txt")
	// fmt.Println("lines:")
	// utils.PrintArray(lines)
	fmt.Println("\nsolved:", solve(lines))
}

// -- PARSING --

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
	currMap := -1

	for _, line := range linesWithoutSeeds {
		if strings.Contains(line, "map") {
			currMap++
			maps = append(maps, [][]int{})
			continue
		}

		numStrings := strings.Split(line, " ")
		var lineNums []int

		for _, numStr := range numStrings {
			numInt, _ := strconv.Atoi(numStr)
			lineNums = append(lineNums, numInt)
		}
		maps[currMap] = append(
			maps[currMap],
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

func getSeedRanges(seeds []int) []SeedRange {
	var seedRanges []SeedRange

	for i := 0; i < len(seeds); i += 2 {
		seedRanges = append(
			seedRanges,
			SeedRange{start: seeds[i], length: seeds[i+1]},
		)
	}

	return seedRanges
}
