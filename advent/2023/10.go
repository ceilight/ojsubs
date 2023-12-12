// a more math-y solution
// significantly reduces execution time on much larger input (2900ms down to ~100ms on 15002x1002 grid)
// (input: https://files.catbox.moe/hsq72a.txt) Part 1: 5016002, Part 2: 5000000

package main

import (
	"bufio"
	"fmt"
	"os"
)

type Pos struct{ x, y int }

var (
	North = Pos{-1, 0}
	South = Pos{1, 0}
	West  = Pos{0, -1}
	East  = Pos{0, 1}
)

func (p *Pos) add(rhs Pos) {
	p.x += rhs.x
	p.y += rhs.y
}

func nextDir(dir Pos, tileTile byte) Pos {
	switch {
	case tileTile == 'F' && dir == North:
		return East
	case tileTile == 'F' && dir == West:
		return South
	case tileTile == '7' && dir == North:
		return West
	case tileTile == '7' && dir == East:
		return South
	case tileTile == 'L' && dir == South:
		return East
	case tileTile == 'L' && dir == West:
		return North
	case tileTile == 'J' && dir == South:
		return West
	case tileTile == 'J' && dir == East:
		return North
	default:
		panic("this tile is not supposed to be in the loop")
	}
}

func abs(n int) int {
	if n < 0 {
		return -n
	}
	return n
}

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	var board [][]byte

	for scanner.Scan() {
		line := scanner.Text()
		board = append(board, []byte(line))
	}

	var row, col = len(board), len(board[0])
	var tile Pos

FindStart:
	for i, row := range board {
		for j, c := range row {
			if c == 'S' {
				tile = Pos{i, j}
				break FindStart
			}
		}
	}

	// find tiles that are connected to the one at starting position
	var dir Pos
	if x, y := tile.x-1, tile.y; x >= 0 && (board[x][y] == '|' || board[x][y] == 'F' || board[x][y] == '7') {
		dir = North
	} else if x, y := tile.x+1, tile.y; x < row && (board[x][y] == '|' || board[x][y] == 'L' || board[x][y] == 'J') {
		dir = South
	} else if x, y := tile.x, tile.y-1; y >= 0 && (board[x][y] == '-' || board[x][y] == 'L' || board[x][y] == 'F') {
		dir = West
	} else if x, y := tile.x, tile.y+1; y < col && (board[x][y] == '-' || board[x][y] == 'J' || board[x][y] == '7') {
		dir = East
	}

	// coordinates of vertices ('S', F', '7', 'J', 'L' tiles) in a loop
	// (had to flatten the position because golang doesn't seem to have tuples)
	var vertices []int
	vertices = append(vertices, tile.x*col+tile.y)

	// do a round trip from the starting position
FindLoop:
	for {
		tile.add(dir)
		if tile.x < 0 || tile.x >= row || tile.y < 0 || tile.y >= col {
			panic("current tile is out of the grid")
		}

		switch board[tile.x][tile.y] {
		case '.':
			panic("out of the loop")
		case 'S':
			break FindLoop
		case '|', '-':
			continue
		case 'F', '7', 'L', 'J':
			vertices = append(vertices, tile.x*col+tile.y)
			dir = nextDir(dir, board[tile.x][tile.y])
		default:
			panic("invalid character")
		}
	}

	// the furthest point from start position has a distance of half the loop's perimeter
	// as for the number of tiles enclosed by the loop, calculate the area of the loop
	// including the edges using the shoelace formula, increment the result by 1 then
	// subtract it with half the perimeter (as per the pick's theorem)
	loopPeri := 0
	loopArea := 0
	verticesCount := len(vertices)

	for i := 0; i < verticesCount; i++ {
		vert := vertices[i]
		next := vertices[(i+1)%verticesCount]
		vertX, vertY, nextX, nextY := vert/col, vert%col, next/col, next%col

		if vertX != nextX && vertY != nextY {
			panic("adjacent vertices must be connected by vertical or horizontal edges")
		}
		loopPeri += abs(nextX-vertX) + abs(nextY-vertY)
		loopArea += vertX*nextY - vertY*nextX
	}
	loopArea /= 2

	fmt.Println("Part 1:", loopPeri/2)
	fmt.Println("Part 2:", abs(loopArea)-loopPeri/2+1)
}
