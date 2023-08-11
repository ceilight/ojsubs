package main

import (
	"bufio"
	"container/heap"
	"fmt"
	"os"
	"time"
)

var costs = map[rune]int{
	'A': 1,
	'B': 10,
	'C': 100,
	'D': 1000,
}

var destX = map[rune]int{
	'A': 2,
	'B': 4,
	'C': 6,
	'D': 8,
}

// there are 23 valid cells, so each cell gets indexed
var cellX = [23]int{0, 1, 3, 5, 7, 9, 10, 2, 2, 2, 2, 4, 4, 4, 4, 6, 6, 6, 6, 8, 8, 8, 8}
var cellY = [23]int{0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4}

type State struct {
	// keeps track on the state of the 23 cells
	// for each char indexed by a corresponding cell order, A, B, C, D indicate the amphipods
	// while the period mark indicates empty cell
	cells string
	// the cost of a new state after one amphipod's move
	cost int
}

func abs(a int) int {
	if a < 0 {
		return -a
	}
	return a
}

// returns a new state after an amphipod moves
func (s *State) move(index int, destX, destY int) State {
	cellsTemp := []rune(s.cells)
	kind := cellsTemp[index]

	x, y := cellX[index], cellY[index]
	distance := abs(x-destX) + abs(y-destY)
	cost := distance * costs[kind]

	destIndex := getRoomIndex(destX, destY)
	cellsTemp[destIndex] = kind
	cellsTemp[index] = '.'

	state := State{
		cells: string(cellsTemp),
		cost:  cost,
	}
	return state
}

func (s *State) isHallwayOpen(start, stop int, selfIdx int) bool {
	var xl, xh int
	if stop < start {
		xl, xh = stop, start
	} else {
		xl, xh = start, stop
	}
	for i, r := range s.cells {
		if r != '.' {
			x, y := cellX[i], cellY[i]
			if i != selfIdx && y == 0 && x >= xl && x <= xh {
				return false
			}
		}
	}
	return true
}

func getRoomIndex(x, y int) int {
	for i := 0; i < 23; i++ {
		if cellX[i] == x && cellY[i] == y {
			return i
		}
	}
	return -1
}

func (s *State) amphipodKind(x, y int) (rune, bool) {
	index := getRoomIndex(x, y)
	if index == -1 {
		return '#', false
	}
	return []rune(s.cells)[index], true
}

func (s *State) amphipodCount() int {
	count := 0
	for _, r := range s.cells {
		if r != '.' {
			count++
		}
	}
	return count
}

func (s *State) nextStates() []State {
	next := []State{}

	maxY := 4
	if s.amphipodCount() == 8 {
		maxY = 2
	}

	for i, r := range s.cells {
		// ignore empty cells
		if r == '.' {
			continue
		}

		ax, ay := cellX[i], cellY[i]
		dx := destX[r]
		// already in its right place
		if ay == maxY && ax == dx {
			continue
		}

		if ay == 0 {
			dy := -1
			for y := maxY; y >= 1; y-- {
				kind, found := s.amphipodKind(dx, y)
				if !found || kind == '.' {
					dy = y
					break
				}
				if kind != r {
					break
				}
			}
			if dy == -1 {
				continue
			}

			if s.isHallwayOpen(ax, dx, i) {
				next = append(next, s.move(i, dx, dy))
			}
			continue
		}

		isStuck := false
		for y := 1; y < ay; y++ {
			kind, found := s.amphipodKind(ax, y)
			if found && kind != '.' {
				isStuck = true
				break
			}
		}
		if isStuck {
			continue
		}

		if ax == dx {
			isSet := true
			for y := ay + 1; y <= maxY; y++ {
				kind, found := s.amphipodKind(ax, y)
				if !found || kind != r {
					isSet = false
					break
				}
			}
			if isSet {
				continue
			}
		}

		// hallway transition
		for _, x := range []int{0, 1, 3, 5, 7, 9, 10} {
			if s.isHallwayOpen(ax, x, i) {
				next = append(next, s.move(i, x, 0))
			}
		}
	}
	return next
}

// binary heap implementation
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

// dijkstra's algorithm
func dijkstra(start, stop string) (int, []State) {
	costs := make(map[string]int)
	prev := make(map[string]string)

	pq := PQueue{State{start, 0}}
	heap.Init(&pq)

	for len(pq) > 0 {
		cur := heap.Pop(&pq).(State)
		cells, cost := cur.cells, cur.cost

		if cells == stop {
			path := []State{{cells, cost}}
			for path[len(path)-1].cells != start {
				prevRooms, found := prev[cells]
				if !found {
					panic(cells)
				}
				cells = prevRooms
				path = append(path, State{cells, costs[cells]})
			}
			for i, j := 0, len(path)-1; i < j; i, j = i+1, j-1 {
				path[i], path[j] = path[j], path[i]
			}
			return cost, path
		}

		for _, n := range cur.nextStates() {
			next, edgeCost := n.cells, n.cost
			nextCost := cost + edgeCost
			if c, ok := costs[next]; !ok || nextCost < c {
				prev[next] = cells
				costs[next] = nextCost
				nextNode := State{next, nextCost}
				heap.Push(&pq, nextNode)
			}
		}
	}
	return -1, nil
}

var final1 = ".......AA..BB..CC..DD.."
var final2 = ".......AAAABBBBCCCCDDDD"

func parseInput(scanner *bufio.Scanner) string {
	lines := []string{}
	for scanner.Scan() {
		line := scanner.Text()
		lines = append(lines, line)
	}
	if err := scanner.Err(); err != nil {
		panic(err)
	}

	cells := make([]rune, 23)
	for i := range cells {
		cells[i] = '.'
	}
	i := 7
	for _, col := range []int{3, 5, 7, 9} {
		cells[i] = []rune(lines[2])[col]
		cells[i+1] = []rune(lines[3])[col]
		i += 4
	}
	return string(cells)
}

func changeInputForPart2(cells string) string {
	cellsMk2 := []rune(cells)
	addition := [4]string{"DD", "CB", "BA", "AC"}
	for i, start := range []int{7, 11, 15, 19} {
		cellsMk2[start+1] = []rune(addition[i])[0]
		cellsMk2[start+2] = []rune(addition[i])[1]
		cellsMk2[start+3] = []rune(cells)[start+1]
	}
	return string(cellsMk2)
}

func main() {
	start1 := parseInput(bufio.NewScanner(os.Stdin))
	start2 := changeInputForPart2(start1)

	startTime := time.Now()
	p1Cost, _ := dijkstra(start1, final1)
	p1Time := fmt.Sprint(time.Since(startTime))

	startTime = time.Now()
	p2Cost, _ := dijkstra(start2, final2)
	p2Time := fmt.Sprint(time.Since(startTime))

	fmt.Printf("Part 1: %d (%s elapsed)\n", p1Cost, p1Time)
	fmt.Printf("Part 2: %d (%s elapsed)\n", p2Cost, p2Time)
}
