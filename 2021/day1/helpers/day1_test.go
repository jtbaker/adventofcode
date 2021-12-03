package helpers_test

import (
	"testing"

	"github.com/jtbaker/adventofcode/2021/day1/helpers"
)

func TestLinesGreaterThanLast(t *testing.T) {
	inputs := []float64{4.5, 8.0, 1.2, 9.4, 5.7}
	res := helpers.FindLinesGreaterThanLast(inputs)

	if len(res) != 2 {
		t.Errorf("The results should have equaled 2")
	}
}

func TestSum(t *testing.T) {
	sum := helpers.Sum([]float64{1.3, 3.0, 9.1})

	if target := float32(float64(13.4)); float32(sum) != target {
		t.Errorf("Sum should have equaled %f, instead it was %f", target, sum)
	}
}
