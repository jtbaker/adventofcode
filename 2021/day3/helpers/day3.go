package helpers

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"sort"
	"strconv"
	"sync"
)

func Handle(err error) {
	log.Fatal(err)
}

type NArray [][]int

func ParseInput(filename string) (NArray, error) {
	var res = *(new(NArray))
	file, err := os.Open(filename)

	if err != nil {
		return res, err
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		text := scanner.Text()
		for i := 0; i < len(text); i++ {
			bitstring := text[i : i+1]
			bit, err := strconv.Atoi(bitstring)
			if err != nil {
				fmt.Printf("Error parsing %s to integer", bitstring)
			} else {
				if len(res) > i {
					res[i] = append(res[i], bit)
				} else {
					res = append(res, []int{bit})
				}
			}
		}
	}
	return res, nil
}

func Tokenize(input []int, idx int, collector *[]map[int]int, wg *sync.WaitGroup) {
	defer wg.Done()
	var tokens = make(map[int]int, len(input))
	for _, v := range input {
		// var match int
		if match, ok := tokens[v]; ok {
			match += 1
			tokens[v] = match
		} else {
			match = 1

			tokens[v] = match
		}
	}
	(*collector)[idx] = tokens
}

func (arr *NArray) Counts() []map[int]int {

	var collector []map[int]int
	faker := make(map[int]int)
	for i := 0; i < len(*arr); i++ {
		collector = append(collector, faker)
	}
	// var collector = []map[int]int{faker, faker, faker, faker}
	var wg sync.WaitGroup
	for i, v := range *arr {
		wg.Add(1)
		go Tokenize(v, i, &collector, &wg)
	}

	wg.Wait()

	return collector
}

type Group struct {
	key   int
	count int
}

func Part1(filename string) int {
	array, err := ParseInput(filename)
	if err != nil {
		log.Fatalf("Error parsing: %s", err)
	}

	counts := array.Counts()
	ints := []int{4, 5, 6}
	sort.Ints(ints)

	var rows = *new([][]Group)

	for idx := range counts {
		var groups []Group
		for key, val := range counts[idx] {
			groups = append(groups, Group{key: key, count: val})
		}
		sort.Slice(groups, func(i, j int) bool {
			return groups[i].count < groups[j].count
		})
		rows = append(rows, groups)
	}
	var gamma, epsilon string

	for _, row := range rows {
		gamma += fmt.Sprint(row[0].key)
		epsilon += fmt.Sprint(row[len(row)-1].key)
	}

	gammaNumeric, err := strconv.ParseInt(gamma, 2, 64)
	if err != nil {
		log.Fatalf("Error parsing gamma: %v", err)
	}
	epsilonNumeric, err := strconv.ParseInt(epsilon, 2, 64)
	if err != nil {
		log.Fatalf("Error parsing epsilon: %v", err)
	}

	fmt.Printf("Gamma:%v\nEpsilon:%v\n", gammaNumeric, epsilonNumeric)

	return int(gammaNumeric) * int(epsilonNumeric)

}
