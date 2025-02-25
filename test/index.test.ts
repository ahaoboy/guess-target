import { readFileSync } from "fs"
import { expect, test } from "vitest"
import { guessTarget, targetToString } from "../ts/index"

test("guessTarget", () => {
  const md = readFileSync("README.md", "utf8")
  const tbStart = md.indexOf("## test") + "## test".length
  const tbEnd = md.indexOf("## Platform Support")
  const lines = md.slice(tbStart, tbEnd).trim().split("\n").slice(2)
  for (const line of lines) {
    const [filename, name, targets, version] = line.slice(1, -1).split("|").map(
      (i) => i.trim(),
    )
    const ret = guessTarget(filename)
    const targetsList = targets.split(",")
    for (const target of ret) {
      expect(name).toBe(target.name)
      expect(version).toBe(target.version || "")
      expect(targetsList.includes(targetToString(target.target))).toBe(true)
    }
  }
})
