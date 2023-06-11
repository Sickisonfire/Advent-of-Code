import * as fs from "node:fs"
import * as path from "node:path"


export default () => {
  const inputPath = path.join(__dirname, "input.txt")
  let rawString = fs.readFileSync(inputPath).toString().slice(0, -1)
  let lines = rawString.split("\n").map(l => l.split(""))

  partOne(lines)
  partTwo(lines)

}

const getCharPrio = (char: string) => {

  let encoder = new TextEncoder()
  let encodedValueOffset = [96, 38]
  let charPrio = 0

  let isUpperCase = (s: string) => /^[A-Z]$/.test(s)

  if (isUpperCase(char)) {
    charPrio = encoder.encode(char)[0] - encodedValueOffset[1]
  } else {
    charPrio = encoder.encode(char)[0] - encodedValueOffset[0]
  }
  return charPrio
}

const partOne = (lines: string[][]) => {

  let total = 0

  for (const line of lines) {
    const s = new Set()
    let char = ""
    const len = line.length

    const l = line.slice(0, len / 2)
    const r = line.slice(len / 2)

    l.forEach(el => s.add(el))
    r.forEach(el => { if (s.has(el)) char = el })

    let charPrio = getCharPrio(char)
    total += charPrio
  }
  console.log("Part 1:", total)
}

const partTwo = (lines: string[][]) => {
  let lineCounter = 1

  let total = 0

  let s = new Map()
  for (const line of lines) {
    switch (lineCounter) {
      case 1:
        s.clear()

        line.forEach(el => s.set(el, false))
        lineCounter += 1
        break

      case 2:
        line.forEach(el => { if (s.has(el)) s.set(el, true) })
        lineCounter += 1
        break
      case 3:
        let char = ""
        line.forEach(el => { if (s.get(el) === true) char = el })
        total += getCharPrio(char)
        lineCounter = 1
        break
      default:
        break
    }
  }
  console.log("Part 2:", total)
}
