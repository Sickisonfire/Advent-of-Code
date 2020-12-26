package main

import (
	"bytes"
	"fmt"
	"io/ioutil"
	"sort"
)

//FBBBFBBLRR
//rows: 0 - 127
//cols: 0 - 7
//F = front
//B = back
//L = left
//R = right
type PlaneSeat struct {
	seatLocationString string
	seatID             uint
}
type PlaneSeatList struct {
	seats         []*PlaneSeat
	numberOfSeats uint
	highestID     uint
	lowestID      uint
}

func readDataAndSplit(path string) [][]byte {
	data, err := ioutil.ReadFile(path)
	if err != nil {
		fmt.Println(err)
	}
	data = bytes.TrimSpace(data)
	return bytes.Split(data, []byte("\n"))
}
func newPlaneSeat(seatLocationString string) *PlaneSeat {
	ps := PlaneSeat{seatLocationString, 0}
	return &ps
}
func (ps *PlaneSeat) findRow() uint {
	rowString := ps.seatLocationString[:7]
	lower, upper, mid := uint(0), uint(127), uint(0)
	for _, r := range rowString {
		mid = lower + (upper-lower)/2
		if r == 'F' {
			// update upper
			upper = mid
		} else if r == 'B' {
			// update lower
			lower = mid + 1
		}
	}
	return lower
}

func (ps *PlaneSeat) findCol() uint {
	colString := ps.seatLocationString[7:]
	left, right, mid := uint(0), uint(7), uint(0)
	for _, r := range colString {
		mid = left + (right-left)/2
		if r == 'L' {
			// update left
			right = mid
		} else if r == 'R' {
			// update right
			left = mid + 1
		}
	}
	return left
}
func (ps *PlaneSeat) calcSeatID() uint {
	row, col := ps.findRow(), ps.findCol()
	ps.seatID = row*8 + col
	return ps.seatID

}

func (psl *PlaneSeatList) addSeat(ps *PlaneSeat) {
	psl.seats = append(psl.seats, ps)
}
func (psl *PlaneSeatList) getHighestSeatID() uint {
	for _, seat := range psl.seats {
		if seat.seatID > psl.highestID {
			psl.highestID = seat.seatID
		}
	}
	return psl.highestID
}

func (psl *PlaneSeatList) getLowestSeatID() uint {
	for _, seat := range psl.seats {
		if seat.seatID < psl.lowestID {
			psl.lowestID = seat.seatID
		}
	}
	return psl.lowestID
}
func main() {
	seatList := PlaneSeatList{lowestID: uint(1000)}

	seats := readDataAndSplit("input.txt")
	for _, seat := range seats {
		newSeat := newPlaneSeat(string(seat))
		newSeat.calcSeatID()
		seatList.addSeat(newSeat)
	}
	highestID := seatList.getHighestSeatID()
	lowestID := seatList.getLowestSeatID()
	fmt.Println(highestID, lowestID)

	//part 2
	seatIDArray := []int{}
	for _, s := range seatList.seats {
		seatIDArray = append(seatIDArray, int(s.seatID))
	}
	sort.Ints(seatIDArray)
	count := seatIDArray[0]
	for _, v := range seatIDArray {

		if v != count {
			fmt.Printf("the seat number is: %v", v-1)
			break

		}
		count++
	}
}
