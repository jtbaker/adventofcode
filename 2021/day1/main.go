package main

import (
	"flag"
	"fmt"

	"github.com/jtbaker/adventofcode/2021/day1/helpers"
)

func main() {
	filename := flag.String("file", "./input.txt", "A filename to use as input")
	flag.Parse()
	result := helpers.Solution(*filename)
	fmt.Printf("Number of records: %d\n", result)
}
