package main

import (
	utils "day-04"
	"fmt"
	"slices"
	"strconv"
	"strings"
)

type Card struct {
	id             int
	matchingAmount int
}

func getNumbers(line string) ([]int, []int) {
	content := strings.Split(strings.Split(line, ": ")[1], " | ")
	firstPart := strings.Split(content[0], " ")
	secondPart := strings.Split(content[1], " ")
	var winning []int
	var cardNumbers []int

	for _, element := range firstPart {
		if strings.Trim(element, " ") != "" {
			num, _ := strconv.Atoi(element)
			winning = append(winning, num)
		}
	}
	for _, element := range secondPart {
		if strings.Trim(element, " ") != "" {
			num, _ := strconv.Atoi(element)
			cardNumbers = append(cardNumbers, num)
		}
	}

	return winning, cardNumbers
}

func getCards(lines []string) []Card {
	var cards []Card

	for i, line := range lines {
		winning, cardNumbers := getNumbers(line)

		var matching []int
		for _, num := range cardNumbers {
			if slices.Contains(winning, num) {
				matching = append(matching, num)
			}
		}

		cards = append(cards, Card{id: i + 1, matchingAmount: len(matching)})

	}

	return cards
}

func getCardsNumFromCardId(id int, cards []Card, cacheMap map[int]int) int {
	if cachedAmount, ok := cacheMap[id]; ok {
		return cachedAmount
	}

	card := cards[id-1]

	amount := 1

	// fmt.Println("id:", id, ", to search:")
	for i := id + 1; i < id+1+card.matchingAmount; i++ {
		amount += getCardsNumFromCardId(i, cards, cacheMap)
	}

	cacheMap[id] = amount

	return amount
}

func getTotalScratchcards(lines []string) int {
	cards := getCards(lines)
	// utils.PrintArray(cards)
	cacheMap := make(map[int]int)

	var total int
	for _, card := range cards {
		amount := getCardsNumFromCardId(card.id, cards, cacheMap)
		// fmt.Println("id:", card.id, ", cards:", amount)
		total += amount
	}

	return total
}

func solve(lines []string) int {
	return getTotalScratchcards(lines)
}

func main() {
	lines := utils.InputToLines("input.txt")
	// fmt.Println("lines:")
	// utils.PrintArray(lines)
	// fmt.Println()
	fmt.Println("solved:", solve(lines))
}
