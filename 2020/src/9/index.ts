import * as fs from "node:fs"
import * as path from "node:path"


export default () => {
  const inputPath = path.join(__dirname, "input.txt")
  let rawString = fs.readFileSync(inputPath).toString().slice(0, -1)
  let lines = rawString.split("\n").map(l => l.split(""))

  let total = 0
  let encoder = new TextEncoder()
  let encodedValueOffset = [96, 38]

  let isUpperCase = (s: string) => /^[A-Z]$/.test(s)

  for (const line of lines) {
    const s = new Set()
    let char = ""
    const len = line.length

    const l = line.slice(0, len / 2)
    const r = line.slice(len / 2)

    l.forEach(el => s.add(el))
    r.forEach(el => { if (s.has(el)) char = el })

    let charPrio = 0
    if (isUpperCase(char)) {
      charPrio = encoder.encode(char)[0] - encodedValueOffset[1]
    } else {
      charPrio = encoder.encode(char)[0] - encodedValueOffset[0]
    }
    total += charPrio
    console.log(charPrio)
  }
  console.log(total)

}
