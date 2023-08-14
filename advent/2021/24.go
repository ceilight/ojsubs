// This is a solution based on my puzzle input, so the data from the input is hard-coded instead

package main

import "fmt"

// On observing the puzzle input, I found that all sets of instructions for 14 digits are partially
// the same, except the 5th, 6th and 16th instructions, the difference being the right hand values.
// Also I noticed that 5th instruction can only be either "div z 1" or "div z 26", and that in
// all sets where the 5th instruction is "div z 26, the right hand value of the 6th instruction is
// non-positive.
// So the data needed is the right hand values of 6th and 16th instructions of each instruction set
var param1 = [14]int{13, 13, 10, 15, -8, -10, 11, -3, 14, -4, 14, -5, -8, -11}
var param2 = [14]int{15, 16, 4, 14, 1, 5, 1, 3, 3, 7, 5, 13, 3, 10}

// x, y are assigned to 0 before being used in the operation, z is still used so let's pass it
func runInstructionSet(p1, p2, w int, z int64) int64 {
	// mul x 0, add x z, mod x 26
	x := int(z % 26)
	// div z 26 if p1 <= 0, otherwise div z 1
	if p1 <= 0 {
		z /= 26
	}
	if x+p1 != w {
		z *= 26
		z += int64(w) + int64(p2)
	}
	return z
}

func findModelNumber(idx int, z, modelNum int64, findMax bool) (int64, bool) {
	if idx == 14 {
		if z == 0 {
			return modelNum, true
		} else {
			return -1, false
		}
	}

	p1, p2 := param1[idx], param2[idx]
	if findMax {
		for w := 9; w >= 1; w-- {
			// The goal to make z able to reach 0 after the all digits are checked
			// Since z can only be divided by 26, multiply 26 and be added with 2 positive value
			// w (digit 1-9) and p2 (all positive on my input), it cannot be negative
			// The value z can be divided by 26 in instruction set where p1 > 0, and the rest
			// of instruction only makes z increase
			// So when p1 > 0, z can only increase -> must make z decrease when p1 <= 0
			// -> only accept p1 that do not satisfy the second if clause
			if p1 > 0 || (p1 <= 0 && int(z%26)+p1 == w) {
				nextZ := runInstructionSet(p1, p2, w, z)
				nextModelNum := 10*modelNum + int64(w)
				if res, found := findModelNumber(idx+1, nextZ, nextModelNum, findMax); found {
					return res, true
				}
			}
		}
	} else {
		for w := 1; w <= 9; w++ {
			if p1 > 0 || (p1 <= 0 && int(z%26)+p1 == w) {
				nextZ := runInstructionSet(p1, p2, w, z)
				nextModelNum := 10*modelNum + int64(w)
				if res, found := findModelNumber(idx+1, nextZ, nextModelNum, findMax); found {
					return res, true
				}
			}
		}
	}
	return -1, false
}

func main() {
	maxModelNum, _ := findModelNumber(0, 0, 0, true)
	minModelNum, _ := findModelNumber(0, 0, 0, false)
	fmt.Printf("Part 1: %d\nPart 2: %d\n", maxModelNum, minModelNum)
}
