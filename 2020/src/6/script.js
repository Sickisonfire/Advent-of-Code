const fs = require('fs')

class Person {
	constructor() {
		this.answers = [] 
		
	}
	
	getAnswers(inputString)	{
		this.answers = inputString.split('')
	}
	printAnswers(){
		console.log(this.answers)
	}
}

class Group {
	constructor(members) {
		this.members = []
		this.answers = new Set()
	}
	
	addMember(person) {
		this.members.push(person)
	}
	
	printMembers() {
		console.log(this.members)
	}
	calcAnswersPartOne(){
		this.members.forEach(member => member.answers.forEach(entry => this.answers.add(entry)))
	}	
	getAnswerCount() {
		return this.answers.size
	}
	calcAnswersPartTwo(){
	// TODO
	}
}

function parseData(data) {
	let parsedData = []
	const groups = data.split("\n\n")
		groups.forEach(group => {
			const g = new Group()
			const splitted = group.split("\n")	
			splitted.forEach(personEntry => {
				const p = new Person()
				p.getAnswers(personEntry)
				g.addMember(p)
			})
			g.calcAnswersPartOne()
			parsedData.push(g)

		})
		return parsedData
}

function calculateTotalAnswerCount(data){
	let count = 0
		data.forEach(group => {
			count = count + group.getAnswerCount()
		})
	return count
}

function solvePartOne(data) {
	const parsedData = parseData(data)

	const totalAnswerCount = calculateTotalAnswerCount(parsedData)
	console.log(`Solution for Part 1: ${totalAnswerCount}`)
}

function solvePartTwo(data) {
	console.log(`Solution for Part 2: No Solution`)
}
const data = fs.readFileSync('data.txt', 'utf8')
solvePartOne(data)

solvePartTwo(data)
