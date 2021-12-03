package main

import (
	"flag"
	"fmt"

	"github.com/jtbaker/adventofcode/2021/day1/helpers"
)

func main() {
	filename := flag.String("file", "./input.txt", "A filename to use as input")
	flag.Parse()
	part1result := helpers.Part1(*filename)
	fmt.Printf("Part 1: Number of records: %d\n", part1result)

	part2result := helpers.Part2(*filename)
	fmt.Printf("Part 2: Number of records: %d\n", part2result)
}
