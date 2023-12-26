package main

import "testing"

func TestHighNumbers(t *testing.T) {
	result := getTransformedSeedRangeIntoRanges(
		SeedRange{start: 200, end: 202},
		AlmanacRange{destinationStart: 52, sourceStart: 50, length: 48},
	)

	if len(result) != 1 || result[0].start != 200 || result[0].end != 202 {
		t.Fatalf(
			"SeedRange{start: 200, end: 202} must equal []SeedRange{SeedRange{start: 200, end: 202}}",
		)
	}
}

func TestLowNumbers(t *testing.T) {
	result := getTransformedSeedRangeIntoRanges(
		SeedRange{start: 2, end: 4},
		AlmanacRange{destinationStart: 52, sourceStart: 50, length: 48},
	)

	if len(result) != 1 || result[0].start != 2 || result[0].end != 4 {
		t.Fatalf(
			"SeedRange{start: 2, end: 4} must equal []SeedRange{SeedRange{start: 2, end: 4}}",
		)
	}
}
