package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	var board [][]byte

	for scanner.Scan() {
		line := scanner.Text()
		board = append(board, []byte(line))
	}

	var row, col = len(board), len(board[0])
	var startX, startY int

FindStart:
	for i, row := range board {
		for j, c := range row {
			if c == 'S' {
				startX, startY = i, j
				break FindStart
			}
		}
	}

	// find pipes that are connected to the one at starting position
	var startDiff [][2]int
	if x, y := startX-1, startY; x >= 0 && (board[x][y] == '|' || board[x][y] == 'F' || board[x][y] == '7') {
		startDiff = append(startDiff, [2]int{-1, 0})
	}
	if x, y := startX+1, startY; x < row && (board[x][y] == '|' || board[x][y] == 'L' || board[x][y] == 'J') {
		startDiff = append(startDiff, [2]int{1, 0})
	}
	if x, y := startX, startY-1; y >= 0 && (board[x][y] == '-' || board[x][y] == 'L' || board[x][y] == 'F') {
		startDiff = append(startDiff, [2]int{0, -1})
	}
	if x, y := startX, startY+1; y < col && (board[x][y] == '-' || board[x][y] == 'J' || board[x][y] == '7') {
		startDiff = append(startDiff, [2]int{0, 1})
	}

	// determine the type of the pipe at starting position
	diffX, diffY := startDiff[1][0]+startDiff[0][0], startDiff[1][1]+startDiff[0][1]
	if diffX == 0 && diffY == 0 {
		if startDiff[0][0] == 0 {
			board[startX][startY] = '-'
		} else {
			board[startX][startY] = '|'
		}
	} else if diffX == -1 {
		if diffY == -1 {
			board[startX][startY] = 'J'
		} else {
			board[startX][startY] = 'L'
		}
	} else if diffX == 1 {
		if diffY == -1 {
			board[startX][startY] = '7'
		} else {
			board[startX][startY] = 'F'
		}
	}

	// part 1
	// do a round trip from the starting position and insert pipes that belong to the loop
	x, y := startX, startY
	dx, dy := startDiff[0][0], startDiff[0][1]

	// tfw no hash set in golang
	loop := map[int]struct{}{}
	loop[x*col+y] = struct{}{}

	for {
		x += dx
		y += dy
		if x < 0 || x >= row || y < 0 || y >= col {
			panic("bro...")
		}
		if x == startX && y == startY {
			break
		}

		loop[x*col+y] = struct{}{}

		// pipe '|', '-' retain the traversal direction
		switch board[x][y] {
		case 'F':
			if dx == -1 && dy == 0 {
				dx, dy = 0, 1
			} else {
				dx, dy = 1, 0
			}
		case '7':
			if dx == -1 && dy == 0 {
				dx, dy = 0, -1
			} else {
				dx, dy = 1, 0
			}
		case 'J':
			if dx == 0 && dy == 1 {
				dx, dy = -1, 0
			} else {
				dx, dy = 0, -1
			}
		case 'L':
			if dx == 0 && dy == -1 {
				dx, dy = -1, 0
			} else {
				dx, dy = 0, 1
			}
		}
	}
	// pipe with the furthest distance is equal to floor value of half the loop length
	fmt.Println("Part 1:", len(loop)/2)

	// part 2
	// number of tiles enclosed by the loop
	boBurnhamCount := 0

	// traverse row by row while keeping track on whether the current tile is inside
	// or not every time we cross past any pipe in loop
	// !!be careful handling the case where we traverse along the loop
	for i, row := range board {
		isInside := false
		isOnEdge := false
		firstCornerOnEdge := byte(0)

		for j, c := range row {
			if _, ok := loop[i*col+j]; ok {
				switch c {
				case '|':
					isInside = !isInside
				case 'L', 'F':
					if isOnEdge {
						panic("edge exists right before opening corner")
					} else {
						isOnEdge = true
						firstCornerOnEdge = c
					}
				case 'J':
					if isOnEdge {
						if firstCornerOnEdge == 'F' {
							isInside = !isInside
						}
						isOnEdge = false
					}
				case '7':
					if isOnEdge {
						if firstCornerOnEdge == 'L' {
							isInside = !isInside
						}
						isOnEdge = false
					}
				}
			} else {
				if isInside {
					boBurnhamCount++
				}
			}
		}
	}
	fmt.Println("Part 2:", boBurnhamCount)
}
