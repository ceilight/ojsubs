package main

import (
	"bufio"
	"container/heap"
	"fmt"
	"os"
)

type Amphipod struct {
	roomIndex int
	kind      rune
}

type State struct {
	amphipods [16]Amphipod
	cost      int
	count     int
}

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

func (s *State) key() string {
	rooms := make([]rune, 23)
	for i := range rooms {
		rooms[i] = '.'
	}
	for _, a := range s.amphipods[:s.count] {
		rooms[a.roomIndex] = a.kind
	}
	return string(rooms)
}

func abs(a int) int {
	if a < 0 {
		return -a
	}
	return a
}

func (s State) move(idx int, destX, destY int) *State {
	a := s.amphipods[idx]
	x, y := roomX[a.roomIndex], roomY[a.roomIndex]
	moveD := abs(x-destX) + abs(y-destY)
	moveCost := moveD * costs[a.kind]

	s.cost = moveCost
	s.amphipods[idx].roomIndex = getRoomIndex(destX, destY)
	return &s
}

func (s *State) isHallwayOpen(start, stop int, selfIdx int) bool {
	var xl, xh int
	if stop < start {
		xl, xh = stop, start
	} else {
		xl, xh = start, stop
	}
	for i, a := range s.amphipods[:s.count] {
		x, y := roomX[a.roomIndex], roomY[a.roomIndex]
		if i != selfIdx && y == 0 && x >= xl && x <= xh {
			return false
		}
	}
	return true
}

func parseState(key string) State {
	if len(key) != 23 {
		panic("invalid state key, must have 23 chars")
	}
	s := State{}
	i := 0
	for roomIndex, a := range key {
		if a != '.' {
			s.amphipods[i] = Amphipod{roomIndex, a}
			i++
		}
	}
	s.count = i
	return s
}

var roomX = [23]int{0, 1, 3, 5, 7, 9, 10, 2, 2, 2, 2, 4, 4, 4, 4, 6, 6, 6, 6, 8, 8, 8, 8}
var roomY = [23]int{0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4}

func getRoomIndex(x, y int) int {
	for i := 0; i < 23; i++ {
		if roomX[i] == x && roomY[i] == y {
			return i
		}
	}
	return -1
}

func getKind(key string, x, y int) (rune, bool) {
	index := getRoomIndex(x, y)
	if index == -1 {
		return '.', false
	}
	return []rune(key)[index], true
}

func (s *State) nextStates() []*State {
	var next []*State
	key := s.key()

	maxY := 4
	if s.count == 8 {
		maxY = 2
	}

	for i, a := range s.amphipods[:s.count] {
		ax, ay := roomX[a.roomIndex], roomY[a.roomIndex]
		dx := destX[a.kind]
		if ay == maxY && ax == dx {
			continue
		}

		if ay == 0 {
			dy := -1
			for y := maxY; y >= 1; y-- {
				kind, found := getKind(key, dx, y)
				if !found || kind == '.' {
					dy = y
					break
				}
				if kind != a.kind {
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
			kind, found := getKind(key, ax, y)
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
				kind, found := getKind(key, ax, y)
				if !found || kind != a.kind {
					isSet = false
					break
				}
			}
			if isSet {
				continue
			}
		}

		for _, x := range []int{0, 1, 3, 5, 7, 9, 10} {
			if s.isHallwayOpen(ax, x, i) {
				next = append(next, s.move(i, x, 0))
			}
		}
	}
	return next
}

type NodeWithCost struct {
	node string
	cost int
}

type PQueue []NodeWithCost

func (h PQueue) Len() int           { return len(h) }
func (h PQueue) Less(i, j int) bool { return h[i].cost < h[j].cost }
func (h PQueue) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }

func (h *PQueue) Push(x any) {
	*h = append(*h, x.(NodeWithCost))
}

func (h *PQueue) Pop() any {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}

func getAdjacents(cur string) []NodeWithCost {
	node := parseState(cur)
	next := node.nextStates()

	adj := []NodeWithCost{}
	for _, s := range next {
		adj = append(adj, NodeWithCost{
			node: s.key(),
			cost: s.cost,
		})
	}
	return adj
}

func dijkstra(start, stop string) (int, []NodeWithCost) {
	costs := make(map[string]int)
	prev := make(map[string]string)

	pq := PQueue{NodeWithCost{start, 0}}
	heap.Init(&pq)

	for len(pq) > 0 {
		cur := heap.Pop(&pq).(NodeWithCost)
		node, cost := cur.node, cur.cost

		if node == stop {
			path := []NodeWithCost{{node, cost}}
			for path[len(path)-1].node != start {
				prevNode, ok := prev[node]
				if !ok {
					panic(node)
				}
				node = prevNode
				path = append(path, NodeWithCost{node, costs[prevNode]})
			}
			for i, j := 0, len(path)-1; i < j; i, j = i+1, j-1 {
				path[i], path[j] = path[j], path[i]
			}
			return cost, path
		}

		for _, n := range getAdjacents(node) {
			next, edgeCost := n.node, n.cost
			nextCost := cost + edgeCost
			if c, ok := costs[next]; !ok || nextCost < c {
				prev[next] = node
				costs[next] = nextCost
				nextNode := NodeWithCost{next, nextCost}
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

	rooms := make([]rune, 23)
	for i := range rooms {
		rooms[i] = '.'
	}
	i := 7
	for _, col := range []int{3, 5, 7, 9} {
		rooms[i] = []rune(lines[2])[col]
		rooms[i+1] = []rune(lines[3])[col]
		i += 4
	}
	return string(rooms)
}

func changeInputForPart2(rooms string) string {
	roomsMk2 := []rune(rooms)
	addition := [4]string{"DD", "CB", "BA", "AC"}
	for i, start := range []int{7, 11, 15, 19} {
		roomsMk2[start+3] = []rune(rooms)[start+1]
		roomsMk2[start+1] = []rune(addition[i])[0]
		roomsMk2[start+2] = []rune(addition[i])[1]
	}
	return string(roomsMk2)
}

func main() {
	start1 := parseInput(bufio.NewScanner(os.Stdin))
	start2 := changeInputForPart2(start1)

	p1cost, _ := dijkstra(start1, final1)
	p2cost, _ := dijkstra(start2, final2)
	fmt.Printf("Part 1: %d\n", p1cost)
	fmt.Printf("Part 2: %d\n", p2cost)
}
