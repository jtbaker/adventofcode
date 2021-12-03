package helpers

import (
	"bufio"
	"log"
	"os"
	"strconv"
)

func LoadLines(filename string) []float64 {
	file, err := os.Open(filename)
	if err != nil {
		log.Fatal("Error opening the file.")
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	var inputs []float64

	for scanner.Scan() {
		if i, err := strconv.ParseFloat(scanner.Text(), 64); err == nil {
			inputs = append(inputs, i)
		}
	}

	return inputs
}

func FindLinesGreaterThanLast(inputs []float64) []float64 {
	var result []float64
	for i := 1; i < len(inputs); i++ {
		if inputs[i] > inputs[i-1] {
			result = append(result, inputs[i])
		}
	}
	return result
}

func Solution(file string) int {
	var inputs, results []float64

	inputs = LoadLines(file)

	results = FindLinesGreaterThanLast(inputs)

	return len(results)
}
