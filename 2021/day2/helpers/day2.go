package helpers

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

type Direction string

const (
	up      Direction = "up"
	forward Direction = "forward"
	down    Direction = "down"
)

type Move struct {
	direction Direction
	distance  int
}

func parseMove(in string) (Move, error) {
	pieces := strings.Split(in, " ")[:2]
	var move Move
	distance, err := strconv.Atoi(pieces[1])
	if err != nil {
		return move, err
	}
	move.direction = Direction(pieces[0])
	move.distance = distance
	return move, nil
}

type Position struct {
	Horizontal int
	Depth      int
}

func (p *Position) Move(move Move) {
	switch move.direction {
	case up:
		p.Depth -= move.distance
	case down:
		p.Depth += move.distance
	case forward:
		p.Horizontal += move.distance
	}
}

func InitializePosition() *Position {
	return &Position{
		Horizontal: 0,
		Depth:      0,
	}
}

func ParseInput(filename string) []Move {
	file, err := os.Open(filename)

	if err != nil {
		log.Fatal(err)
	}

	scanner := bufio.NewScanner(file)

	var moves []Move

	for scanner.Scan() {
		line := scanner.Text()
		move, err := parseMove(line)
		if err != nil {
			fmt.Printf("Error parsing line: %s. Error: %s", line, err)
		} else {
			moves = append(moves, move)
		}
	}

	return moves
}

func Part1(in string) int {
	moves := ParseInput(in)
	position := InitializePosition()

	for _, move := range moves {
		position.Move(move)
	}

	var product = position.Horizontal * position.Depth
	return product
}
