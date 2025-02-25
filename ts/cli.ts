import { getLocalTarget, guessTarget, targetToString } from "./index"

console.log(
  "local target:",
  getLocalTarget().map((i) => targetToString(i)).join(","),
)

const filename = process.argv[2]
if (!filename) {
  console.log(`guess-target <name>`)
  process.exit()
}

for (const i of guessTarget(filename)) {
  const name = `name: ${i.name}`
  const target = `target: ${targetToString(i.target)}`
  const version = i.version ? `version: ${i.version}` : ""
  console.log([
    name,
    target,
    version,
  ].join(" "))
}
