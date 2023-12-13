package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	var data [][][]byte
	var grid [][]byte
	scanner := bufio.NewScanner(os.Stdin)

	for scanner.Scan() {
		line := scanner.Text()
		if len(line) == 0 {
			data = append(data, grid)
			grid = nil
		} else {
			grid = append(grid, []byte(line))
		}
	}
	data = append(data, grid)

	fmt.Println("Part 1:", part1(data))
	fmt.Println("Part 2:", part2(data))
}

func part1(data [][][]byte) int {
	res := 0
	for _, grid := range data {
		if row, ok := findMirrorRow(grid, 0); ok {
			res += (row + 1) * 100
		} else if col, ok := findMirrorCol(grid, 0); ok {
			res += col + 1
		}
	}
	return res
}

func part2(data [][][]byte) int {
	res := 0
	for _, grid := range data {
		if row, ok := findMirrorRow(grid, 1); ok {
			res += (row + 1) * 100
		} else if col, ok := findMirrorCol(grid, 1); ok {
			res += col + 1
		}
	}
	return res
}

func findMirrorRow(grid [][]byte, diffLimit int) (int, bool) {
	row, col := len(grid), len(grid[0])
NextRow:
	for i := 0; i < row-1; i++ {
		diff := 0
		for lo, hi := i, i+1; lo >= 0 && hi < row; lo, hi = lo-1, hi+1 {
			for j := 0; j < col; j++ {
				if grid[lo][j] != grid[hi][j] {
					diff++
					if diff > diffLimit {
						continue NextRow
					}
				}
			}
		}
		if diff == diffLimit {
			return i, true
		}
	}
	return 0, false
}

func findMirrorCol(grid [][]byte, diffLimit int) (int, bool) {
	row, col := len(grid), len(grid[0])
NextCol:
	for j := 0; j < col-1; j++ {
		diff := 0
		for lo, hi := j, j+1; lo >= 0 && hi < col; lo, hi = lo-1, hi+1 {
			for i := 0; i < row; i++ {
				if grid[i][lo] != grid[i][hi] {
					diff++
					if diff > diffLimit {
						continue NextCol
					}
				}
			}
		}
		if diff == diffLimit {
			return j, true
		}
	}
	return 0, false
}
