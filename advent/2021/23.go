package main

import (
	"bufio"
	"container/heap"
	"fmt"
	"os"
	"time"
)

// There are 23 valid cells, so let's get all of them indexed
var cellsRow = [23]int{0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4}
var cellsCol = [23]int{1, 2, 4, 6, 8, 10, 11, 3, 3, 3, 3, 5, 5, 5, 5, 7, 7, 7, 7, 9, 9, 9, 9}

type State struct {
	// Each element represents the state of a cell.
	// A, B, C, D indicate the amphipods while the period mark indicates empty cell
	cells []byte
	// Total cost to reach this state from initial state
	cost int
}

type Move struct {
	// Indices of cells where amphipod stays before and after moving
	orig, dest int
	// Cost of a single move
	cost int
}

func newState(grid [][]byte) State {
	maxRow := len(grid) - 2
	cells := make([]byte, 23)
	for i := range cells {
		x, y := cellsRow[i], cellsCol[i]
		if x > 0 && x <= maxRow {
			cells[i] = grid[x][y]
		} else {
			cells[i] = '.'
		}
	}
	return State{cells, 0}
}

// Weighted costs by amphipod type
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
func (s *State) getMove(index, destRow, destCol int) Move {
	cell := s.cells[index]
	x, y := cellsRow[index], cellsCol[index]
	dist := abs(x-destRow) + abs(y-destCol)
	cost := dist * costs[cell]
	destIndex := getCellIndex(destRow, destCol)
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
func (s *State) isHallwayOpen(col, destCol, index int) bool {
	lCol, rCol := col, destCol
	if lCol > rCol {
		lCol, rCol = rCol, lCol
	}
	for i, cell := range s.cells {
		x, y := cellsRow[i], cellsCol[i]
		if i != index && x == 0 && y >= lCol && y <= rCol && cell != '.' {
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

// Since the cell state can only be one of 5 values (A, B, C, D, empty), the cells
// attribute can be represented in a base 5 system, of which the largest value
// (5 pow 23 - 1 ~ 1.192e16) is within int64 range
func (s *State) cellsHash() int64 {
	var key int64
	for _, cell := range s.cells {
		key = 5*key + digits[cell]
	}
	return key
}

func getCellIndex(x, y int) int {
	for i := 0; i < 23; i++ {
		if cellsRow[i] == x && cellsCol[i] == y {
			return i
		}
	}
	return -1
}

// Returns a cell at given coordinate, if a cell is out of range, `\0` is returned
func (s *State) getCell(x, y int) byte {
	index := getCellIndex(x, y)
	if index == -1 {
		return 0
	}
	return s.cells[index]
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

var destRoomCol = map[byte]int{
	'A': 3,
	'B': 5,
	'C': 7,
	'D': 9,
}

func (s *State) availableMoves() []Move {
	moves := []Move{}
	maxRow := s.amphipodCount() / 4

nextCell:
	for i, cell := range s.cells {
		// ignore empty cells
		if cell == '.' {
			continue
		}
		col, row := cellsCol[i], cellsRow[i]
		destCol := destRoomCol[cell]

		if row == 0 {
			// Hallway to target room transition:
			// Iterate over the target room from the bottom up to find the target cell.
			// Check if there are amphipods of other type inside the room
			// so they won't get stuck when the amphipod moves in target room
			destRow := 0
			for x := maxRow; x >= 1; x-- {
				c := s.getCell(x, destCol)
				if c == '.' {
					destRow = x
					break
				} else if c != cell {
					continue nextCell
				}
			}
			// Also, make sure the hallway segment between the amphipod and the
			// target room is empty before moving
			if s.isHallwayOpen(col, destCol, i) {
				moves = append(moves, s.getMove(i, destRow, destCol))
			}
		} else {
			// Room to hallway transition:
			// Amphipod can move to the hallway when there are no amphipods overhead
			for x := 1; x < row; x++ {
				if s.getCell(x, col) != '.' {
					continue nextCell
				}
			}
			// If it's already in the target room, it only moves out when it has
			// to make way for amphipods underneath that are in the wrong room to
			// move to their target rooms
			if col == destCol {
				isSet := true
				for x := row + 1; x <= maxRow; x++ {
					if s.getCell(x, col) != cell {
						isSet = false
						break
					}
				}
				if isSet {
					continue
				}
			}
			// Move to all reachable cells in the hallway
			for _, hallCol := range []int{1, 2, 4, 6, 8, 10, 11} {
				if s.isHallwayOpen(col, hallCol, i) {
					moves = append(moves, s.getMove(i, 0, hallCol))
				}
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
type PQueue []*State

func (h PQueue) Len() int           { return len(h) }
func (h PQueue) Less(i, j int) bool { return h[i].cost < h[j].cost }
func (h PQueue) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *PQueue) Push(x any)        { *h = append(*h, x.(*State)) }

func (h *PQueue) Pop() any {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}

func dijkstra(init, end State) (int, []*State) {
	costs := make(map[int64]int)
	prevs := make(map[int64]*State)
	pq := PQueue{&init}
	heap.Init(&pq)

	for len(pq) > 0 {
		state := heap.Pop(&pq).(*State)
		// If end state is reached, trace state path and return total cost
		if state.isEqualTo(end) {
			path := []*State{}
			cur := state
			for cur.isEqualTo(init) {
				path = append(path, cur)
				cur = prevs[cur.cellsHash()]
			}
			path = append(path, &init)
			for i, j := 0, len(path)-1; i < j; i, j = i+1, j-1 {
				path[i], path[j] = path[j], path[i]
			}
			return state.cost, path
		}

		for _, move := range state.availableMoves() {
			next := state.applyMove(move)
			nextKey := next.cellsHash()
			if c, ok := costs[nextKey]; !ok || next.cost < c {
				prevs[nextKey] = state
				costs[nextKey] = next.cost
				heap.Push(&pq, &next)
			}
		}
	}
	return -1, nil
}

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	scanner.Scan()

	grid := [][]byte{}
	for scanner.Scan() {
		row := []byte(scanner.Text())
		grid = append(grid, row)
	}
	init1 := newState(grid)
	end1 := State{
		cells: []byte(".......AA..BB..CC..DD.."),
		cost:  0,
	}

	// Add 2 more lines in the input for part 2
	grid = append(grid[:4], grid[2:]...)
	grid[2] = []byte("  #D#C#B#A#")
	grid[3] = []byte("  #D#B#A#C#")
	init2 := newState(grid)
	end2 := State{
		cells: []byte(".......AAAABBBBCCCCDDDD"),
		cost:  0,
	}

	startTime := time.Now()
	p1Cost, _ := dijkstra(init1, end1)
	p1Time := fmt.Sprint(time.Since(startTime))

	startTime = time.Now()
	p2Cost, _ := dijkstra(init2, end2)
	p2Time := fmt.Sprint(time.Since(startTime))

	fmt.Printf("Part 1: %d (%s elapsed)\n", p1Cost, p1Time)
	fmt.Printf("Part 2: %d (%s elapsed)\n", p2Cost, p2Time)
}
