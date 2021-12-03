package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

func main() {
	file, err := os.Open("./input.txt")
	if err != nil {
		log.Fatal("Error opening the file.")
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	var inputs, result []float64

	for scanner.Scan() {
		if i, err := strconv.ParseFloat(scanner.Text(), 64); err == nil {
			inputs = append(inputs, i)
		}
	}

	for i := 1; i < len(inputs); i++ {
		if inputs[i] > inputs[i-1] {
			result = append(result, inputs[i])
		}
	}

	fmt.Printf("Number of records: %d\n", len(result))
}
