package main

import (
	"bytes"
	"fmt"
	"io/ioutil"
)

//read data
//parse data
//get map length of passport
//if len 8 --> valid
//if len 7 --> loop and check if cid missing
//..
//if len < 7 --> not valid
type passport struct {
	fields map[string]string
}
type Passports struct {
	passports  []passport
	validCount uint
}

func ParsePassport(data []byte) passport {
	//create new pp object
	newPassport := passport{make(map[string]string)}
	//replace all \n with " "
	data = bytes.ReplaceAll(data, []byte("\n"), []byte(" "))
	//trim last space
	data = bytes.TrimSpace(data)
	//split data into fields at " "
	fieldsSplitted := bytes.Split(data, []byte(" "))
	//loop over fields

	for _, s := range fieldsSplitted {
		// split the field in key and value at ":"
		field := bytes.Split(s, []byte(":"))
		//add key and value to map of pp object
		newPassport.fields[string(field[0])] = string(field[1])
	}

	return newPassport
}

func SeperateInputData(data []byte) [][]byte {
	return bytes.Split(data, []byte("\n\n"))
}

func validatePassport(pp passport) bool {
	switch len(pp.fields) {
	case 8:
		return true
	case 7:
		if _, ok := pp.fields["cid"]; ok {
			return false
		} else {
			return true
		}
	default:
		return false
	}
}
func validateBYR(pp passport {}
func main() {
	passportList := Passports{}
	count := 0
	data, err := ioutil.ReadFile("input.txt")
	if err != nil {
		fmt.Errorf("Error: %s", err)
		return
	}
	seperatedData := SeperateInputData(data)
	for _, pp := range seperatedData {
		passportObject := ParsePassport(pp)
		passportList.passports = append(passportList.passports, passportObject)
	}

	for _, pp := range passportList.passports {
		if validatePassport(pp) {
			count++
		}
	}
	fmt.Println(count)
}
