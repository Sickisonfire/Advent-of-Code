package main

import (
	_ "fmt"
	"io/ioutil"
	"testing"
)

var data []byte

func init() {

	data, _ = ioutil.ReadFile("input_test.txt")
}

func TestSeperateInputData(t *testing.T) {
	arr := SeperateInputData(data)
	if len(arr) != 5 {
		t.Errorf("Amount of pps is not right, got: %d", len(arr))
	}
}
func TestParsePassport(t *testing.T) {
	arr := SeperateInputData(data)
	for _, passport := range arr {
		passportObject := ParsePassport(passport)
		if len(passportObject.fields) != 6 {
			t.Errorf("PassportObject should contain 6 fields, got %d", len(passportObject.fields))
		}
		if _, ok := passportObject.fields["iyr"]; !ok {
			t.Errorf("PassportObject should have key iyr in map but is missing")
		}
		if passportObject.fields["iyr"] != "2013" {
			t.Errorf("Fieldvalue of passportObject is incorrect expected 2013, got %s", passportObject.fields["iyr"])
		}
		break
	}
}

func TestValidateASinglePassport(t *testing.T) {
	arr := SeperateInputData(data)
	passports := []passport{}
	for _, passport := range arr {
		passportObject := ParsePassport(passport)

		passports = append(passports, passportObject)
	}

	if !validatePassport(passports[4]) {
		t.Errorf("passport contains all fields but validatepassport returned not valid")
	}
	if validatePassport(passports[0]) {
		t.Errorf("passport is missing pid but validatepassport returned valid")
	}
	if !validatePassport(passports[2]) {
		t.Errorf("passport is only missing cid but validatepassport returned not valid")
	}
}
