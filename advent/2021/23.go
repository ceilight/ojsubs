package main

import (
	"bufio"
	"container/heap"
	"fmt"
	"os"
	"time"
)

// There are 23 valid cells, so let's get all of them indexed
var cellPosX = [23]int{0, 1, 3, 5, 7, 9, 10, 2, 2, 2, 2, 4, 4, 4, 4, 6, 6, 6, 6, 8, 8, 8, 8}
var cellPosY = [23]int{0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4}

type State struct {
	// Each element represents the state of a cell. A, B, C, D indicate the amphipods
	// while the period mark indicates empty cell.
	cells []byte
	// Total cost to reach this state from initial state
	cost int
}

// Info about one amphipod's move
type Move struct {
	// Indices of cells where amphipod stays before and after moving
	orig, dest int
	// Cost of a single move (not to be confused with total cost in State.cost)
	cost int
}

// Weighted costs by amphipod's kind
var costs = map[byte]int{
	'A': 1,
	'B': 10,
	'C': 100,
	'D': 1000,
}

func abs(a int) int {
	if a < 0 {
		return -a
	}
	return a
}

// Returns information about one amphipod's move
func (s *State) getMove(index, destX, destY int) Move {
	kind := s.cells[index]
	x, y := cellPosX[index], cellPosY[index]
	distance := abs(x-destX) + abs(y-destY)
	cost := distance * costs[kind]
	destIndex := getCellIndex(destX, destY)
	return Move{index, destIndex, cost}
}

// Creates the resulting state after applying one move on original state and returns it
func (s *State) applyMove(m Move) State {
	cellsTemp := make([]byte, len(s.cells))
	copy(cellsTemp, s.cells)
	cellsTemp[m.dest] = cellsTemp[m.orig]
	cellsTemp[m.orig] = '.'
	return State{cellsTemp, s.cost + m.cost}
}

// Checks if amphipod can move along the hallway starting from column x to destX
func (s *State) isHallwayOpen(x, destX, index int) bool {
	xLeft, xRight := x, destX
	if destX < x {
		xLeft, xRight = destX, x
	}
	for i, cell := range s.cells {
		x, y := cellPosX[i], cellPosY[i]
		if i != index && y == 0 && x >= xLeft && x <= xRight && cell != '.' {
			return false
		}
	}
	return true
}

var digits = map[byte]int64{
	'.': 0,
	'A': 1,
	'B': 2,
	'C': 3,
	'D': 4,
}

// Returns the representation of cells attribute in hashable data type
// Since the cell state can only be one of 5 values (A, B, C, D, empty cell), the cells attribute
// can be represented in a base 5 system, of which the largest value (5 pow 23 - 1 ~ 1.192e16) is
// within int64 range
func (s *State) getCellsKey() int64 {
	var key int64
	for _, cell := range s.cells {
		key = 5*key + digits[cell]
	}
	return key
}

func getCellIndex(x, y int) int {
	for i := 0; i < 23; i++ {
		if cellPosX[i] == x && cellPosY[i] == y {
			return i
		}
	}
	return -1
}

// Returns a cell kind from coordinate, if a cell is invalid or out of range, `false` is returned
func (s *State) getCell(x, y int) (byte, bool) {
	index := getCellIndex(x, y)
	if index == -1 {
		return 0, false
	}
	return s.cells[index], true
}

func (s *State) amphipodCount() int {
	count := 0
	for _, cell := range s.cells {
		if cell != '.' {
			count++
		}
	}
	return count
}

var destRoom = map[byte]int{
	'A': 2,
	'B': 4,
	'C': 6,
	'D': 8,
}

func (s *State) availableMoves() []Move {
	moves := []Move{}

	maxY := 4
	if s.amphipodCount() == 8 {
		maxY = 2
	}

	for i, cell := range s.cells {
		// ignore empty cells
		if cell == '.' {
			continue
		}

		curX, curY := cellPosX[i], cellPosY[i]
		destX := destRoom[cell]

		// already in its right place
		if curX == destX && curY == maxY {
			continue
		}

		// hallway to room transition (destY is the lowest empty cell in the room)
		if curY == 0 {
			destY := -1
			for y := maxY; y >= 1; y-- {
				kind, _ := s.getCell(destX, y)
				if kind == '.' {
					destY = y
					break
				}
				if kind != cell {
					break
				}
			}
			if destY == -1 {
				continue
			}

			if s.isHallwayOpen(curX, destX, i) {
				moves = append(moves, s.getMove(i, destX, destY))
			}
			continue
		}

		// amphipod is in either of the 4 side rooms
		// check if there's any amphipod above them
		isStuck := false
		for y := 1; y < curY; y++ {
			kind, _ := s.getCell(curX, y)
			if kind != '.' {
				isStuck = true
				break
			}
		}
		if isStuck {
			continue
		}

		// suppose current amphipods is in its destination room
		// check if there's any amphipod under it that is not in its correct room
		if curX == destX {
			isSet := true
			for y := curY + 1; y <= maxY; y++ {
				kind, _ := s.getCell(curX, y)
				if kind != cell {
					isSet = false
					break
				}
			}
			if isSet {
				continue
			}
		}

		// room to hallway transition
		for _, hallX := range []int{0, 1, 3, 5, 7, 9, 10} {
			if s.isHallwayOpen(curX, hallX, i) {
				moves = append(moves, s.getMove(i, hallX, 0))
			}
		}
	}
	return moves
}

// Two states are considered equal when the cell states of both are the same
func (s *State) isEqualTo(other State) bool {
	if len(s.cells) != len(other.cells) {
		return false
	}
	for i := 0; i < len(s.cells); i++ {
		if s.cells[i] != other.cells[i] {
			return false
		}
	}
	return true
}

// Priority queue implementation
type PQueue []State

func (h PQueue) Len() int           { return len(h) }
func (h PQueue) Less(i, j int) bool { return h[i].cost < h[j].cost }
func (h PQueue) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *PQueue) Push(x any)        { *h = append(*h, x.(State)) }

func (h *PQueue) Pop() any {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}

func dijkstra(init, end []byte) (int, []State) {
	costs := make(map[int64]int)
	prev := make(map[int64]State)

	initState := State{init, 0}
	endState := State{end, 0} // a dummy one to compare with other states

	pq := PQueue{initState}
	heap.Init(&pq)

	for len(pq) > 0 {
		cur := heap.Pop(&pq).(State)
		cost := cur.cost

		if cur.isEqualTo(endState) {
			path := []State{}
			for {
				path = append(path, cur)
				if cur.isEqualTo(initState) {
					break
				}
				curKey := cur.getCellsKey()
				prev, found := prev[curKey]
				if !found {
					panic(curKey)
				}
				cur = prev
			}
			for i, j := 0, len(path)-1; i < j; i, j = i+1, j-1 {
				path[i], path[j] = path[j], path[i]
			}
			return cost, path
		}

		for _, move := range cur.availableMoves() {
			next := cur.applyMove(move)
			nextKey := next.getCellsKey()
			if c, ok := costs[nextKey]; !ok || next.cost < c {
				prev[nextKey] = cur
				costs[nextKey] = next.cost
				heap.Push(&pq, next)
			}
		}
	}
	return -1, nil
}

var final1 = []byte(".......AA..BB..CC..DD..")
var final2 = []byte(".......AAAABBBBCCCCDDDD")

func parseInput(scanner *bufio.Scanner) []byte {
	lines := []string{}
	for scanner.Scan() {
		line := scanner.Text()
		lines = append(lines, line)
	}
	if err := scanner.Err(); err != nil {
		panic(err)
	}

	cells := make([]byte, 23)
	for i := range cells {
		cells[i] = '.'
	}
	i := 7
	for _, col := range []int{3, 5, 7, 9} {
		cells[i] = lines[2][col]
		cells[i+1] = lines[3][col]
		i += 4
	}
	return cells
}

func getInputForPart2(cells []byte) []byte {
	cellsTemp := make([]byte, len(cells))
	copy(cellsTemp, cells)
	addition := [4][2]byte{{'D', 'D'}, {'C', 'B'}, {'B', 'A'}, {'A', 'C'}}
	for i, start := range []int{7, 11, 15, 19} {
		cellsTemp[start+3] = cellsTemp[start+1]
		cellsTemp[start+1] = addition[i][0]
		cellsTemp[start+2] = addition[i][1]
	}
	return cellsTemp
}

func main() {
	start1 := parseInput(bufio.NewScanner(os.Stdin))
	start2 := getInputForPart2(start1)

	startTime := time.Now()
	p1Cost, _ := dijkstra(start1, final1)
	p1Time := fmt.Sprint(time.Since(startTime))

	startTime = time.Now()
	p2Cost, _ := dijkstra(start2, final2)
	p2Time := fmt.Sprint(time.Since(startTime))

	fmt.Printf("Part 1: %d (%s elapsed)\n", p1Cost, p1Time)
	fmt.Printf("Part 2: %d (%s elapsed)\n", p2Cost, p2Time)
}
