package main

import (
	"bytes"
	"fmt"
	"io/ioutil"
	"time"
)

// add to each row until bottom reached
// pattern 3 right 1 down
// width 31 (starting on 1)
// depth 323 (starting on 1)
// utf8 dec -> 35 = #, 46 = ., 10 = \n

type position struct {
	x int
	y int
}
type Forest struct {
	data      [][]byte
	pos       position
	width     int
	depth     int
	treeCount int
}

func (f *Forest) stepRight(steps int) {
	// get pos step right n times
	// update pos
	currentPos := f.pos.x
	if currentPos+steps > f.width {
		currentPos = currentPos + steps - f.width
	} else {
		currentPos = currentPos + steps
	}

	f.pos.x = currentPos

}

func (f *Forest) stepDown(steps int) {
	// get pos step down n times
	// update pos
	currentPos := f.pos.y
	f.pos.y = currentPos + steps

}

func (f *Forest) evaluatePos() {
	// eval byte on current pos
	x, y := f.pos.x, f.pos.y
	if f.data[y-1][x-1] == byte('#') {
		f.treeCount += 1
	}
}

func (f *Forest) StepThrough(stepWidth int, stepDepth int) int {
	defer f.reset()
	for {
		f.stepRight(stepWidth)
		f.stepDown(stepDepth)
		f.evaluatePos()
		if f.pos.y == f.depth {
			break
		}
	}
	fmt.Printf("The number of Trees are: %d \n", f.treeCount)

	return f.treeCount
}

func (f *Forest) reset() {
	f.pos.x, f.pos.y, f.treeCount = 1, 1, 0
}
func newForest(rawData []byte) *Forest {
	dataArray := bytes.Split(rawData, []byte("\n"))
	return &Forest{data: dataArray, pos: position{x: 1, y: 1}, width: len(dataArray[0]), depth: len(dataArray) - 1, treeCount: 0}
}

func main() {
	t0 := time.Now()
	data, err := ioutil.ReadFile("input.txt")
	if err != nil {
		panic(err)
	}
	fmt.Println(data[324])
	forest := newForest(data)
	slope3 := forest.StepThrough(3, 1)
	slope1 := forest.StepThrough(1, 1)
	slope5 := forest.StepThrough(5, 1)
	slope7 := forest.StepThrough(7, 1)
	slope2y := forest.StepThrough(1, 2)

	res := slope3 * slope1 * slope5 * slope7 * slope2y
	fmt.Println(res)
	fmt.Printf("\n elapsed time: %d ns", time.Since(t0))
}
