package helpers_test

import (
	"testing"

	"github.com/jtbaker/adventofcode/2021/day2/helpers"
)

func TestPart1(t *testing.T) {
	result := helpers.Part1("../input.txt")
	if result != 150 {
		t.Errorf("Result was %v, not 150", result)
	}
}

func TestPart2(t *testing.T) {
	result := helpers.Part2("../input.txt")
	if result != 900 {
		t.Errorf("Result was %v, not 900", result)
	}
}
