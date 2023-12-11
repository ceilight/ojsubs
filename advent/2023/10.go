// a more math-y solution
// significantly reduces execution time on much larger input (2900ms down to ~100ms on 15002x1002 grid)
// input: https://files.catbox.moe/hsq72a.txt) Part 1: 5016002, Part 2: 5000000

package main

import (
	"bufio"
	"fmt"
	"os"
)

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
	var x, y int

FindStart:
	for i, row := range board {
		for j, c := range row {
			if c == 'S' {
				x, y = i, j
				break FindStart
			}
		}
	}

	// find pipes that are connected to the one at starting position
	var dx, dy int
	if x-1 >= 0 && (board[x-1][y] == '|' || board[x-1][y] == 'F' || board[x-1][y] == '7') {
		dx, dy = -1, 0
	} else if x+1 < row && (board[x+1][y] == '|' || board[x+1][y] == 'L' || board[x+1][y] == 'J') {
		dx, dy = 1, 0
	} else if y-1 >= 0 && (board[x][y-1] == '-' || board[x][y-1] == 'L' || board[x][y-1] == 'F') {
		dx, dy = 0, -1
	} else if y+1 < col && (board[x][y+1] == '-' || board[x][y+1] == 'J' || board[x][y+1] == '7') {
		dx, dy = 0, 1
	}

	// part 1
	// coordinates of vertices ('S', F', '7', 'J', 'L' tiles) in a loop
	// (had to flatten the position because golang doesn't seem to have tuples)
	var vertices []int
	vertices = append(vertices, x*col+y)

	// do a round trip from the starting position
	for {
		x += dx
		y += dy
		if x < 0 || x >= row || y < 0 || y >= col {
			panic("current tile is out of the grid")
		}
		if board[x][y] == 'S' {
			break
		}

		switch board[x][y] {
		case 'F':
			vertices = append(vertices, x*col+y)
			if dx == -1 && dy == 0 {
				dx, dy = 0, 1
			} else {
				dx, dy = 1, 0
			}
		case '7':
			vertices = append(vertices, x*col+y)
			if dx == -1 && dy == 0 {
				dx, dy = 0, -1
			} else {
				dx, dy = 1, 0
			}
		case 'J':
			vertices = append(vertices, x*col+y)
			if dx == 0 && dy == 1 {
				dx, dy = -1, 0
			} else {
				dx, dy = 0, -1
			}
		case 'L':
			vertices = append(vertices, x*col+y)
			if dx == 0 && dy == -1 {
				dx, dy = -1, 0
			} else {
				dx, dy = 0, 1
			}
		}
	}

	loopLen := 0 // number of tiles in the loop
	loopArea := 0
	verticesCount := len(vertices)

	// the length of loop is the sum of the distances of all pairs of adjacent vertices
	// as for the number of tiles enclosed by the loop, calculate the area of the loop
	// including the edges using the shoelace formula, then subtract the result with
	// perimeter lattices (or half the loop length)
	for i := 0; i < verticesCount; i++ {
		curr := vertices[i]
		next := vertices[(i+1)%verticesCount]
		currX, currY, nextX, nextY := curr/col, curr%col, next/col, next%col

		if currX != nextX && currY != nextY {
			panic("adjacent vertices must be connected by vertical or horizontal edges")
		}
		// for convenience, only one vertex is included in the edge
		loopLen += abs(nextX-currX) + abs(nextY-currY)
		loopArea += currY*nextX - currX*nextY
	}

	fmt.Println("Part 1:", loopLen/2)
	fmt.Println("Part 2:", (abs(loopArea)-loopLen)/2+1)
}
