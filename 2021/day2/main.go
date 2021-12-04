package main

import (
	"flag"
	"fmt"

	"github.com/jtbaker/adventofcode/2021/day2/helpers"
)

func main() {
	filename := flag.String("file", "./input.txt", "Filename to use for input")

	flag.Parse()

	part1result := helpers.Part1(*filename)

	fmt.Printf("Part 1: %v\n", part1result)

	part2result := helpers.Part2(*filename)

	fmt.Printf("Part 2: %v\n", part2result)
}
