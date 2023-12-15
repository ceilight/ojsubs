package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	var grid [][]byte
	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
		line := scanner.Text()
		grid = append(grid, []byte(line))
	}
	fmt.Println("Part 1:", part1(grid))
	fmt.Println("Part 2:", part2(grid))
}

func part1(grid [][]byte) int {
	return getLoad(tiltUp(grid))
}

func part2(grid [][]byte) int {
	var load []int
	memo := make(map[string]int)
	start, curr := 0, 0
	for {
		key := gridHashKey(grid)
		if x, found := memo[key]; found {
			start = x
			break
		}
		memo[key] = curr
		load = append(load, getLoad(grid))
		grid = cycle(grid)
		curr++
	}
	remaining := (1000000000 - start) % (curr - start)
	return load[start+remaining]
}

func min(a int, b int) int {
	if a < b {
		return a
	}
	return b
}

func tiltUp(grid [][]byte) [][]byte {
	row, col := len(grid), len(grid[0])
	var newGrid [][]byte
	for _, r := range grid {
		newGrid = append(newGrid, r)
	}

	minRowAtCol := make([]int, col)
	for j := 0; j < col; j++ {
		minRowAtCol[j] = 0
	}
	for i := 0; i < row; i++ {
		for j := 0; j < col; j++ {
			switch grid[i][j] {
			case 'O':
				k := min(i, minRowAtCol[j])
				minRowAtCol[j] = k + 1
				if k < i {
					newGrid[k][j], newGrid[i][j] = 'O', '.'
				}
			case '#':
				minRowAtCol[j] = i + 1
			}
		}
	}
	return newGrid
}

func rotateRight(grid [][]byte) [][]byte {
	row, col := len(grid), len(grid[0])
	var newGrid [][]byte
	for j := 0; j < col; j++ {
		temp := make([]byte, row)
		for i := 0; i < row; i++ {
			temp[i] = grid[row-i-1][j]
		}
		newGrid = append(newGrid, temp)
	}
	return newGrid
}

func cycle(grid [][]byte) [][]byte {
	for i := 0; i < 4; i++ {
		grid = rotateRight(tiltUp(grid))
	}
	return grid
}

func getLoad(grid [][]byte) int {
	totalLoad := 0
	row := len(grid)
	for i, r := range grid {
		for _, c := range r {
			if c == 'O' {
				totalLoad += row - i
			}
		}
	}
	return totalLoad
}

func gridHashKey(grid [][]byte) string {
	var key string
	for _, x := range grid {
		key += fmt.Sprintf("%s", x)
	}
	return key
}
