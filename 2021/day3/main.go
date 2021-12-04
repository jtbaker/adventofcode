package main

import (
	"flag"
	"fmt"

	"github.com/jtbaker/adventofcode/2021/day3/helpers"
)

func main() {
	filename := flag.String("file", "./input.txt", "Filename for data input")
	flag.Parse()
	part1Result := helpers.Part1(*filename)

	fmt.Printf("Part 1: %v\n", part1Result)
}
