package helpers_test

import (
	"testing"

	"github.com/jtbaker/adventofcode/2021/day3/helpers"
)

func TestPart1(t *testing.T) {
	result := helpers.Part1("../input.txt")
	if result != 198 {
		t.Errorf("Result was %v instead of 198\n", result)
	}
}
