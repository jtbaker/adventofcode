package helpers_test

import (
	"testing"

	"github.com/jtbaker/adventofcode/2021/day1/helpers"
)

func TestResults(t *testing.T) {
	res := helpers.Solution("../input.txt")

	if res != 7 {
		t.Errorf("The results should have equaled 7")
	}
}
