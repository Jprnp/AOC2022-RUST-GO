package main

import (
	"log"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	var calories []int
	current := 0
	bytes, err := os.ReadFile("src/aoc/aoc1/input")
	if err != nil {
		log.Fatal(err)
	}

	content := string(bytes[:])
	lines := strings.Split(content, "\n")

	for _, line := range lines {
		value, err := strconv.Atoi(line)
		if err != nil {
			calories = append(calories, current)
			current = 0
			continue
		}

		current += value
	}
	calories = append(calories, current)
	sort.Sort(sort.Reverse(sort.IntSlice(calories)))

	println(calories[0] + calories[1] + calories[2])
}
