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
}
export enum Os {
  Darwin = 0,
  Linux = 1,
  Windows = 2,
  Freebsd = 3,
  Illumos = 4,
  Netbsd = 5,
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
}
export enum Vendor {
  Apple = 0,
  Unknown = 1,
  Pc = 2,
}
export class GuessTarget {
  private constructor();
  free(): void;
  target: Target;
  readonly name: string;
  readonly version: string | undefined;
  readonly git: string | undefined;
}

