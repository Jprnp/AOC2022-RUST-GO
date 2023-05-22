package main

import (
	"fmt"
	"log"
	"os"
	"strings"
)

const WIN = 'Z'
const DRAW = 'Y'
const LOSS = 'X'

const ROCK = 'A'
const PAPER = 'B'
const SCISSORS = 'C'

var pointsByMove = map[rune]int{
	ROCK:     1,
	PAPER:    2,
	SCISSORS: 3,
}

var pointsByResult = map[rune]int{
	WIN:  6,
	DRAW: 3,
	LOSS: 0,
}

func main() {
	totalPoints := 0

	bytes, err := os.ReadFile("src/aoc/aoc2/input")
	if err != nil {
		log.Fatal(err)
	}
	content := string(bytes[:])
	lines := strings.Split(content, "\n")

	for _, line := range lines {
		line = strings.Replace(line, "-", "", 1)
		line = strings.ReplaceAll(line, " ", "")
		inputs := []rune(line)
		if len(inputs) == 3 {
			totalPoints += determinePoints(inputs)
		}
	}

	fmt.Println(totalPoints)
}

func determineMove(theirMove rune, expectedResult rune) rune {
	if expectedResult == DRAW {
		return theirMove
	}

	if expectedResult == WIN {
		switch theirMove {
		case ROCK:
			return PAPER
		case PAPER:
			return SCISSORS
		case SCISSORS:
			return ROCK
		}
	} else {
		switch theirMove {
		case ROCK:
			return SCISSORS
		case PAPER:
			return ROCK
		case SCISSORS:
			return PAPER
		}
	}

	return ROCK
}

func determinePoints(input []rune) int {
	theirMove := input[0]
	expectedResult := input[1]
	myMove := determineMove(theirMove, expectedResult)

	return pointsByMove[myMove] + pointsByResult[expectedResult]
}
