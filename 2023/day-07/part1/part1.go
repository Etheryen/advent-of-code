package main

import (
	"day-07/utils"
	"fmt"
	"slices"
	"sort"
	"strconv"
	"strings"
)

type handBid struct {
	hand string
	bid  int
}

type handType int

const (
	highCard handType = iota
	onePair
	twoPair
	threeOfAKind
	fullHouse
	fourOfAKind
	fiveOfAKind
)

func isOfAKind(hand string, kindType int) bool {
	var occurancesMax int

	for _, card := range hand {
		occurances := cardOccurs(hand, card)
		if occurances > occurancesMax {
			occurancesMax = occurances
		}
	}

	return occurancesMax == kindType
}

func cardSet(hand string) []rune {
	var result []rune

	for _, card := range hand {
		if !slices.Contains(result, card) {
			result = append(result, card)
		}
	}

	return result
}

func cardOccurs(hand string, card rune) int {
	var result int

	for _, c := range hand {
		if c == card {
			result++
		}
	}

	return result
}

func isFullHouse(hand string) bool {
	cardTypes := cardSet(hand)

	if len(cardTypes) != 2 {
		return false
	}

	occursFirst := cardOccurs(hand, cardTypes[0])
	occursSecond := cardOccurs(hand, cardTypes[1])

	occurances := []int{occursFirst, occursSecond}
	sort.Ints(occurances)

	return slices.Equal(occurances, []int{2, 3})
}

func isTwoPair(hand string) bool {
	cardTypes := cardSet(hand)

	if len(cardTypes) != 3 {
		return false
	}

	occursFirst := cardOccurs(hand, cardTypes[0])
	occursSecond := cardOccurs(hand, cardTypes[1])
	occursThird := cardOccurs(hand, cardTypes[2])

	occurances := []int{occursFirst, occursSecond, occursThird}
	sort.Ints(occurances)

	return slices.Equal(occurances, []int{1, 2, 2})
}

func getHandType(hand string) handType {
	switch {
	case isOfAKind(hand, 5):
		return fiveOfAKind
	case isOfAKind(hand, 4):
		return fourOfAKind
	case isFullHouse(hand):
		return fullHouse
	case isOfAKind(hand, 3):
		return threeOfAKind
	case isTwoPair(hand):
		return twoPair
	case isOfAKind(hand, 2):
		return onePair
	default:
		return highCard
	}
}

func cmpCards(a, b byte) int {
	if a == b {
		return 0
	}

	strengths := []byte{
		'A', 'K', 'Q', 'J', 'T',
		'9', '8', '7', '6', '5', '4', '3', '2',
	}

	ia := slices.Index(strengths, a)
	ib := slices.Index(strengths, b)

	if ia < ib {
		return 1
	}
	return -1
}

func sortHands(hands []handBid) {
	slices.SortFunc(hands, func(a, b handBid) int {
		ta := getHandType(a.hand)
		tb := getHandType(b.hand)

		if ta > tb {
			return 1
		}
		if ta < tb {
			return -1
		}

		for i := range a.hand {
			cmpResult := cmpCards(a.hand[i], b.hand[i])
			if cmpResult != 0 {
				return cmpResult
			}
		}

		panic("equality")
	})
}

func winnings(handBids []handBid) int {
	var result int

	for i, hb := range handBids {
		result += (i + 1) * hb.bid
	}

	return result
}

func solve(lines []string) int {
	handBids := toHandBids(lines)
	utils.PrintArray(handBids)
	sortHands(handBids)
	utils.PrintArray(handBids)

	return winnings(handBids)
}

func main() {
	lines := utils.InputToLines("input.txt")
	fmt.Println("solved:", solve(lines))
}

// -- PARSING --

func toHandBids(lines []string) []handBid {
	var result []handBid

	for _, l := range lines {
		hand := strings.Split(l, " ")[0]
		bid, _ := strconv.Atoi(strings.Split(l, " ")[1])
		result = append(result, handBid{
			hand: hand,
			bid:  bid,
		})
	}

	return result
}
