package main

import (
	"bufio"
	"fmt"
	"os"
)

func parseInput() (pos []int) {
	scanner := bufio.NewScanner(os.Stdin)
	for {
		scanner.Scan()
		line := scanner.Text()
		if len(line) == 0 {
			break
		}
		var n, p int
		fmt.Sscanf(line, "Player %d starting position: %d", &n, &p)
		pos = append(pos, p-1)
	}
	return pos
}

func part1(pos []int) int {
	dice := 1
	rolls := 0
	score := [2]int{}
game:
	for {
		for p := 0; p < 2; p++ {
			for i := 0; i < 3; i++ {
				pos[p] += dice
				dice++
				if dice == 101 {
					dice = 1
				}
			}
			pos[p] %= 10
			score[p] += pos[p] + 1
			rolls += 3
			if score[p] >= 1000 {
				break game
			}
		}
	}

	loser := score[0]
	if loser >= 1000 {
		loser = score[1]
	}
	return loser * rolls
}

// Distribution of the sum of dice values after 3 rolls (only indices 3-9 are used)
var rollSumDistribution = [10]int64{0, 0, 0, 1, 3, 6, 7, 6, 3, 1}

// First two attributes of key is the state of current player
func getKey(pos, score [2]int, isPlayer1Turn bool) string {
	if isPlayer1Turn {
		return fmt.Sprint(pos[0], score[0], pos[1], score[1])
	} else {
		return fmt.Sprint(pos[1], score[1], pos[0], score[0])
	}
}

// Note: play() returns the numbers of outcomes where player 1 and player 2 wins respectively
// while memo stores values with the number of outcomes where current player wins first
func play(pos, score [2]int, isPlayer1Turn bool, memo map[string][2]int64) (int64, int64) {
	key := getKey(pos, score, isPlayer1Turn)
	if res, found := memo[key]; found {
		if isPlayer1Turn {
			return res[0], res[1]
		} else {
			return res[1], res[0]
		}
	}

	wins := [2]int64{}
	p := 1
	if isPlayer1Turn {
		p = 0
	}
	for i := 3; i < 10; i++ {
		distribution := rollSumDistribution[i]
		curPos := (pos[p] + i) % 10
		curScore := score[p] + curPos + 1
		if curScore >= 21 {
			wins[p] += distribution
		} else {
			nextPos := [2]int{pos[0], pos[1]}
			nextScore := [2]int{score[0], score[1]}
			nextPos[p] = curPos
			nextScore[p] = curScore
			nextPlayer1Wins, nextPlayer2Wins := play(nextPos, nextScore, !isPlayer1Turn, memo)
			wins[0] += nextPlayer1Wins * distribution
			wins[1] += nextPlayer2Wins * distribution
		}
	}

	if isPlayer1Turn {
		memo[key] = wins
	} else {
		memo[key] = [2]int64{wins[1], wins[0]}
	}
	return wins[0], wins[1]
}

func part2(pos []int) int64 {
	player1Wins, player2Wins := play([2]int{pos[0], pos[1]}, [2]int{}, true, map[string][2]int64{})
	if player1Wins > player2Wins {
		return player1Wins
	}
	return player2Wins
}

func main() {
	pos := parseInput()
	posTemp := []int{pos[0], pos[1]}
	part1Ans := part1(pos)
	fmt.Println("Part 1:", part1Ans)
	pos = posTemp
	part2Ans := part2(pos)
	fmt.Println("Part 2:", part2Ans)
}
