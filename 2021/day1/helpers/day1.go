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

func Part1(file string) int {
	var inputs, results []float64

	inputs = LoadLines(file)

	results = FindLinesGreaterThanLast(inputs)

	return len(results)
}

func Sum(slice []float64) float64 {
	sum := float64(0)
	for _, v := range slice {
		sum += v
	}
	return sum
}

func Part2(file string) int {
	var inputs []float64
	var sums []float64

	inputs = LoadLines(file)
	for i := 3; i <= len(inputs); i++ {
		s := Sum(inputs[i-3 : i])
		sums = append(sums, s)
	}

	res := FindLinesGreaterThanLast(sums)

	return len(res)

}
