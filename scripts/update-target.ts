import { readFileSync, writeFileSync } from "fs"
import { constantCase, pascalCase } from "change-case"

const Derive = `#[derive(EnumIter, Debug, PartialEq, Clone, Copy)]`
const WasmDerive =
  `#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]`

function toEnum(name: string, v: string[]): string {
  const enumName = pascalCase(name)
  const codes = [
    WasmDerive,
    Derive,
    `pub enum ${enumName} {`,

    v.map((i) => pascalCase(i)).join(","),

    "}",

    // to_str
    `impl ${enumName} {
    pub fn to_str(&self)->&'static str {
    match self {`,
    v.map((i) => `${enumName}::${pascalCase(i)} => "${i}"`).join(",\n"),
    "}}}",

    // Display
    `impl std::fmt::Display for ${enumName} {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_str())
    }
}`,
    // FromStr

    `impl std::str::FromStr for ${enumName} {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {`,

    v.map((i) => `"${i}" => Ok(${enumName}::${pascalCase(i)})`).join(",\n"),

    `             ,_ => Err("Unknown ${enumName}"),
    }
    }
}`,
    // `pub const ${
    //   constantCase(enumName + "List")
    // } : [${enumName}; ${v.length}] = [`,
    // v.map((i) => `${enumName}::${pascalCase(i)}`).join(",\n"),

    // "];",
  ]

  return codes.join("\n")
}

const s = readFileSync("assets/target.txt", "utf8")

const os: string[] = []
const arch: string[] = []
const vendor: string[] = []
const abi: string[] = []
const target: string[] = []

for (const i of s.split("\n")) {
  const v = i.split("   ")[0].trim()
  target.push(v)
  const parts = v.split("-")
  arch.push(parts[0])
  vendor.push(parts[1])
  os.push(parts[2] ?? parts[1])
  abi.push(parts[3])
}

const codes: string[] = [
  `pub use strum::IntoEnumIterator;
use strum_macros::EnumIter;
`,
]
for (
  const [name, value] of Object.entries({
    os,
    arch,
    vendor,
    abi,
    target,
  })
) {
  const code = toEnum(name, [...new Set(value)].filter((i) => !!i))
  codes.push(code)
}

function implForTarget(targets: string[]): string {
  const codes = [
    `impl Target{`,

    // arch
    `pub fn arch(&self)-> Arch {
       match self {`,
    targets.map((i) =>
      `Target::${pascalCase(i)} => Arch::${pascalCase(i.split("-")[0])}`
    ).join(",\n"),
    "}}",

    // vender
    `pub fn vender(&self)-> Vendor {
      match self {`,
    targets.map((i) =>
      `Target::${pascalCase(i)} => Vendor::${pascalCase(i.split("-")[1])}`
    ).join(",\n"),
    "}}",

    // os
    `pub fn os(&self)-> Os {
      match self {`,
    targets.map((i) =>
      `Target::${pascalCase(i)} => Os::${
        pascalCase(i.split("-")[2] ?? i.split("-")[1])
      }`
    ).join(",\n"),
    "}}",
    // abi
    `pub fn abi(&self)-> Option<Abi> {
  match self {`,
    targets.map((i) =>
      `Target::${pascalCase(i)} => ${
        i.split("-")[3]
          ? "Some(Abi::" + pascalCase(i.split("-")[3]) + ")"
          : "None"
      }`
    ).join(",\n"),
    "}}",

    "}",
  ]

  return codes.join("\n")
}

function getRule(targets: string[]): string {
  const codes = [
    `pub const DEFAULT_RULE: [(Target, Arch, Vendor, Os, Option<Abi>);${targets.length}] = [`,

    targets.map((i) => {
      const [arch, vendor, os, abi] = i.split("-")
      return [
        "( Target::" + pascalCase(i),
        "Arch::" + pascalCase(arch),
        "Vendor::" + pascalCase(vendor),
        "Os::" + pascalCase(os ?? vendor),
        (abi ? "Some(Abi::" + pascalCase(abi) + ")" : "None") + ")",
      ]
    }).join(",\n"),

    "];",
  ]

  return codes.join("")
}

codes.push(implForTarget(target))
codes.push(getRule(target))
writeFileSync("src/target.rs", codes.join("\n"))
