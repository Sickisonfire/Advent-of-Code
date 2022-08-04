package main

import "testing"

func TestFindSeatRow(t *testing.T) {
	seat1 := newPlaneSeat("FBFBBFFRLR")
	row := seat1.findRow()

	if row != uint(44) {
		t.Errorf("Wrong row, row should be 44 but got %v", row)
	}
}

func TestFindSeatCol(t *testing.T) {
	seat1 := newPlaneSeat("FBFBBFFRLR")
	col := seat1.findCol()

	if col != uint(5) {
		t.Errorf("Wrong col, col should be 5 but got %v", col)
	}
}

func TestCalcSeatID(t *testing.T) {
	seat1 := newPlaneSeat("FBFBBFFRLR")
	seatID := seat1.calcSeatID()

	if seatID != uint(357) {
		t.Errorf("SeatID not correct, should be 357, got %v", seatID)
	}
}

func TestReadAndSplitData(t *testing.T) {
	data := readDataAndSplit("input_test.txt")

	if len(data) != 9 {
		t.Errorf("Amount of seats is not correct, should be 9, got %v", len(data))
	}
}
