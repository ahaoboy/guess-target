import {
  Arch,
  targetGetAbi,
  targetGetArch,
  targetGetOs,
  targetToString,
} from "./wasm"
import { Target } from "./wasm"
import { Abi } from "./wasm"
import { Os } from "./wasm"
import { isMusl } from "is-musl"

export * from "./wasm"

const OsMap: Record<string, string> = {}
const ArchMap: Record<string, string> = {}
const AbiMap: Record<string, string> = {}

// * * `'aix'`
// * * `'darwin'`
// * * `'freebsd'`
// * * `'linux'`
// * * `'openbsd'`
// * * `'sunos'`
// * * `'win32'`
export function getLocalOs(): Os {
  switch (process.platform) {
    case "darwin":
      return Os.Darwin
    case "win32":
      return Os.Windows
    case "freebsd":
      return Os.Freebsd
    case "linux":
      return Os.Linux
    case "netbsd":
      return Os.Netbsd
    case "openbsd":
    case "aix":
    case "android":
    case "haiku":
    case "sunos":
    case "cygwin":
  }
  throw new Error("failed to getLocalOs")
}

export function getLocalArch(): Arch {
  switch (process.arch) {
    case "arm":
      return Arch.Arm
    case "arm64":
      return Arch.Aarch64
    case "ia32":
      return Arch.I686
    case "loong64":
      return Arch.Loongarch64
    case "x64":
      return Arch.X86_64
    case "ppc":
      return Arch.Powerpc
    case "ppc64":
      return Arch.Powerpc64
    case "riscv64":
      return Arch.Riscv64gc
    case "s390":
      return Arch.S390x
    case "s390x":
      return Arch.S390x
    case "mips":
    case "mipsel":
  }
  throw new Error("failed to getLocalArch")
}

export function getLocalAbi(): Abi[] {
  if (isMusl()) {
    return [Abi.Musl]
  }
  if (process.platform === "win32") {
    return [Abi.Msvc, Abi.Gnu]
  }
  return []
}

export function getLocalTarget(): Target[] {
  const os = getLocalOs()
  const arch = getLocalArch()
  const abi = getLocalAbi()
  const v: Target[] = []
  for (const target of Object.values(Target) as Target[]) {
    const targetOs = targetGetOs(target)
    const targetArch = targetGetArch(target)
    const targetAbi = targetGetAbi(target)
    const fitAbi = targetAbi ? abi.includes(targetAbi) : true
    if (os === targetOs && arch === targetArch && fitAbi) {
      v.push(target)
    }
  }
  return v
}
