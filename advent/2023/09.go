package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	record := parseInput()
	p1, p2 := int64(0), int64(0)

	for _, h := range record {
		left, right := predict(h)
		p1 += right
		p2 += left
	}

	fmt.Println("part 1: ", p1)
	fmt.Println("part 2: ", p2)
}

func parseInput() [][]int64 {
	scanner := bufio.NewScanner(os.Stdin)
	var record [][]int64

	for scanner.Scan() {
		line := scanner.Text()
		var row []int64

		for _, str := range strings.Fields(line) {
			num, err := strconv.ParseInt(str, 10, 64)
			if err != nil {
				panic(err)
			}
			row = append(row, num)
		}
		record = append(record, row)
	}
	return record
}

func predict(diff []int64) (int64, int64) {
	var leftTrace []int64
	var rightTrace []int64

Loop:
	for i := len(diff) - 1; i >= 0; i-- {
		leftTrace = append(leftTrace, diff[0])
		rightTrace = append(rightTrace, diff[i])

		allZero := true
		for j := 0; j < i; j++ {
			diff[j] = diff[j+1] - diff[j]
			if diff[j] != 0 {
				allZero = false
			}
		}
		if allZero {
			break Loop
		}
	}

	leftPred, rightPred := int64(0), int64(0)
	for i := len(leftTrace) - 1; i >= 0; i-- {
		leftPred = leftTrace[i] - leftPred
	}
	for _, x := range rightTrace {
		rightPred += x
	}

	return leftPred, rightPred
}
