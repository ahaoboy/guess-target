/* tslint:disable */
/* eslint-disable */
export function guessTarget(s: string): GuessTarget[];
export function targetToString(target: Target): string;
export function targetGetOs(target: Target): Os;
export function targetGetArch(target: Target): Arch;
export function targetGetAbi(target: Target): Abi | undefined;
export enum Abi {
  Gnu = 0,
  Msvc = 1,
  Musl = 2,
  Gnueabi = 3,
  Gnueabihf = 4,
  Macabi = 5,
  Sim = 6,
  Gnullvm = 7,
  Ohos = 8,
  Softfloat = 9,
  Musleabi = 10,
  Musleabihf = 11,
  Elf = 12,
  Sgx = 13,
  Gnux32 = 14,
}
export enum Arch {
  Aarch64 = 0,
  I686 = 1,
  X86_64 = 2,
  Arm = 3,
  Armv7 = 4,
  Loongarch64 = 5,
  Powerpc = 6,
  Powerpc64 = 7,
  Powerpc64le = 8,
  Riscv64gc = 9,
  S390x = 10,
  Arm64ec = 11,
  Armebv7r = 12,
  Armv5te = 13,
  Armv7a = 14,
  Armv7r = 15,
  I586 = 16,
  Nvptx64 = 17,
  Riscv32i = 18,
  Riscv32im = 19,
  Riscv32imac = 20,
  Riscv32imafc = 21,
  Riscv32imc = 22,
  Riscv64imac = 23,
  Sparc64 = 24,
  Sparcv9 = 25,
  Thumbv6m = 26,
  Thumbv7em = 27,
  Thumbv7m = 28,
  Thumbv7neon = 29,
  Thumbv8mBase = 30,
  Thumbv8mMain = 31,
  Wasm32 = 32,
  Wasm32v1 = 33,
}
export enum Os {
  Darwin = 0,
  Linux = 1,
  Windows = 2,
  Freebsd = 3,
  Illumos = 4,
  Netbsd = 5,
  Ios = 6,
  Android = 7,
  Fuchsia = 8,
  None = 9,
  Uefi = 10,
  Androideabi = 11,
  Eabi = 12,
  Eabihf = 13,
  Cuda = 14,
  Solaris = 15,
  Emscripten = 16,
  Unknown = 17,
  Wasip1 = 18,
  Threads = 19,
  Wasip2 = 20,
  Redox = 21,
}
export enum Target {
  Aarch64AppleDarwin = 0,
  Aarch64UnknownLinuxGnu = 1,
  I686PcWindowsGnu = 2,
  I686PcWindowsMsvc = 3,
  I686UnknownLinuxGnu = 4,
  X86_64AppleDarwin = 5,
  X86_64PcWindowsGnu = 6,
  X86_64PcWindowsMsvc = 7,
  X86_64UnknownLinuxGnu = 8,
  Aarch64PcWindowsMsvc = 9,
  Aarch64UnknownLinuxMusl = 10,
  ArmUnknownLinuxGnueabi = 11,
  ArmUnknownLinuxGnueabihf = 12,
  Armv7UnknownLinuxGnueabihf = 13,
  Loongarch64UnknownLinuxGnu = 14,
  Loongarch64UnknownLinuxMusl = 15,
  PowerpcUnknownLinuxGnu = 16,
  Powerpc64UnknownLinuxGnu = 17,
  Powerpc64leUnknownLinuxGnu = 18,
  Powerpc64leUnknownLinuxMusl = 19,
  Riscv64gcUnknownLinuxGnu = 20,
  Riscv64gcUnknownLinuxMusl = 21,
  S390xUnknownLinuxGnu = 22,
  X86_64UnknownFreebsd = 23,
  X86_64UnknownIllumos = 24,
  X86_64UnknownLinuxMusl = 25,
  X86_64UnknownNetbsd = 26,
  Aarch64AppleIos = 27,
  Aarch64AppleIosMacabi = 28,
  Aarch64AppleIosSim = 29,
  Aarch64LinuxAndroid = 30,
  Aarch64PcWindowsGnullvm = 31,
  Aarch64UnknownFuchsia = 32,
  Aarch64UnknownLinuxOhos = 33,
  Aarch64UnknownNone = 34,
  Aarch64UnknownNoneSoftfloat = 35,
  Aarch64UnknownUefi = 36,
  ArmLinuxAndroideabi = 37,
  ArmUnknownLinuxMusleabi = 38,
  ArmUnknownLinuxMusleabihf = 39,
  Arm64ecPcWindowsMsvc = 40,
  Armebv7rNoneEabi = 41,
  Armebv7rNoneEabihf = 42,
  Armv5teUnknownLinuxGnueabi = 43,
  Armv5teUnknownLinuxMusleabi = 44,
  Armv7LinuxAndroideabi = 45,
  Armv7UnknownLinuxGnueabi = 46,
  Armv7UnknownLinuxMusleabi = 47,
  Armv7UnknownLinuxMusleabihf = 48,
  Armv7UnknownLinuxOhos = 49,
  Armv7aNoneEabi = 50,
  Armv7rNoneEabi = 51,
  Armv7rNoneEabihf = 52,
  I586PcWindowsMsvc = 53,
  I586UnknownLinuxGnu = 54,
  I586UnknownLinuxMusl = 55,
  I686LinuxAndroid = 56,
  I686PcWindowsGnullvm = 57,
  I686UnknownFreebsd = 58,
  I686UnknownLinuxMusl = 59,
  I686UnknownUefi = 60,
  Loongarch64UnknownNone = 61,
  Loongarch64UnknownNoneSoftfloat = 62,
  Nvptx64NvidiaCuda = 63,
  Riscv32iUnknownNoneElf = 64,
  Riscv32imUnknownNoneElf = 65,
  Riscv32imacUnknownNoneElf = 66,
  Riscv32imafcUnknownNoneElf = 67,
  Riscv32imcUnknownNoneElf = 68,
  Riscv64gcUnknownNoneElf = 69,
  Riscv64imacUnknownNoneElf = 70,
  Sparc64UnknownLinuxGnu = 71,
  Sparcv9SunSolaris = 72,
  Thumbv6mNoneEabi = 73,
  Thumbv7emNoneEabi = 74,
  Thumbv7emNoneEabihf = 75,
  Thumbv7mNoneEabi = 76,
  Thumbv7neonLinuxAndroideabi = 77,
  Thumbv7neonUnknownLinuxGnueabihf = 78,
  Thumbv8mBaseNoneEabi = 79,
  Thumbv8mMainNoneEabi = 80,
  Thumbv8mMainNoneEabihf = 81,
  Wasm32UnknownEmscripten = 82,
  Wasm32UnknownUnknown = 83,
  Wasm32Wasip1 = 84,
  Wasm32Wasip1Threads = 85,
  Wasm32Wasip2 = 86,
  Wasm32v1None = 87,
  X86_64AppleIos = 88,
  X86_64AppleIosMacabi = 89,
  X86_64FortanixUnknownSgx = 90,
  X86_64LinuxAndroid = 91,
  X86_64PcSolaris = 92,
  X86_64PcWindowsGnullvm = 93,
  X86_64UnknownFuchsia = 94,
  X86_64UnknownLinuxGnux32 = 95,
  X86_64UnknownLinuxOhos = 96,
  X86_64UnknownNone = 97,
  X86_64UnknownRedox = 98,
  X86_64UnknownUefi = 99,
}
export enum Vendor {
  Apple = 0,
  Unknown = 1,
  Pc = 2,
  Linux = 3,
  None = 4,
  Nvidia = 5,
  Sun = 6,
  Wasip1 = 7,
  Wasip2 = 8,
  Fortanix = 9,
}
export class GuessTarget {
  private constructor();
  free(): void;
  target: Target;
  readonly name: string;
  readonly version: string | undefined;
  readonly git: string | undefined;
}

