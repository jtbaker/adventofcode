package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"path/filepath"
	"strconv"
)

func main() {
	ex, err := os.Executable()
	if err != nil {
		log.Fatal("Problem finding the executable")
	}
	dir := filepath.Dir(ex)
	path := fmt.Sprintf("%s/input.txt", dir)
	file, err := os.Open(path)
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
