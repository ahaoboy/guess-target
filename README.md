## guess-target

```shell
cargo binstall guess-target

guess-target deno-x86_64-pc-windows-msvc
name: deno target: x86_64-pc-windows-msvc

guess-target qjs-darwin
name: qjs target: aarch64-apple-darwin
name: qjs target: x86_64-apple-darwin

guess-target ryujinx-1.2.82-macos_universal
name: ryujinx target: aarch64-apple-darwin version: 1.2.82
name: ryujinx target: x86_64-apple-darwin version: 1.2.82
```

## test

| filename                                 | name        | targets                                                                                                                                                                                                                                 | version   | git         |
| ---------------------------------------- | ----------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------- | ----------- |
| deno-x86_64-pc-windows-msvc              | deno        | x86_64-pc-windows-msvc                                                                                                                                                                                                                  |           |             |
| ryujinx-1.2.82-macos_universal           | ryujinx     | aarch64-apple-darwin,x86_64-apple-darwin                                                                                                                                                                                                | 1.2.82    |             |
| qjs-darwin                               | qjs         | aarch64-apple-darwin,x86_64-apple-darwin                                                                                                                                                                                                |           |             |
| fnm-macos                                | fnm         | aarch64-apple-darwin,x86_64-apple-darwin                                                                                                                                                                                                |           |             |
| nu-0.102.0-armv7-unknown-linux-gnueabihf | nu          | armv7-unknown-linux-gnueabihf                                                                                                                                                                                                           | 0.102.0   |             |
| 7z2409-linux-arm64                       | 7z2409      | aarch64-unknown-linux-gnu,aarch64-unknown-linux-musl,aarch64-unknown-linux-ohos                                                                                                                                                         |           |             |
| alist-windows-amd64                      | alist       | x86_64-pc-windows-gnu,x86_64-pc-windows-msvc,x86_64-pc-windows-gnullvm                                                                                                                                                                  |           |             |
| mise-v2025.2.8-macos-arm64               | mise        | aarch64-apple-darwin                                                                                                                                                                                                                    | v2025.2.8 |             |
| gh_2.67.0_linux_arm64                    | gh          | aarch64-unknown-linux-gnu,aarch64-unknown-linux-musl,aarch64-unknown-linux-ohos                                                                                                                                                         | 2.67.0    |             |
| GitHub CLI 2.67.0 linux arm64            | GitHub CLI  | aarch64-unknown-linux-gnu,aarch64-unknown-linux-musl,aarch64-unknown-linux-ohos                                                                                                                                                         | 2.67.0    |             |
| ffmpeg-n7.1-latest-win64-gpl-7.1         | ffmpeg      | x86_64-pc-windows-gnu,x86_64-pc-windows-msvc,aarch64-pc-windows-msvc,aarch64-pc-windows-gnullvm,x86_64-pc-windows-gnullvm                                                                                                               | n7.1      |             |
| graaljs-24.1.2-macos-aarch64             | graaljs     | aarch64-apple-darwin                                                                                                                                                                                                                    | 24.1.2    |             |
| hermes-cli-darwin                        | hermes-cli  | aarch64-apple-darwin,x86_64-apple-darwin                                                                                                                                                                                                |           |             |
| mpv-x86_64-v3-20250221-git-e32beaa       | mpv         | x86_64-pc-windows-gnu,x86_64-pc-windows-msvc,x86_64-pc-windows-gnullvm                                                                                                                                                                  | 20250221  | git-e32beaa |
| pnpm-win-x64                             | pnpm        | x86_64-pc-windows-gnu,x86_64-pc-windows-msvc,x86_64-pc-windows-gnullvm                                                                                                                                                                  |           |             |
| xst-lin64                                | xst         | aarch64-unknown-linux-gnu,aarch64-unknown-linux-musl,aarch64-unknown-linux-ohos                                                                                                                                                         |           |             |
| wiztree_4_23_portable                    | wiztree     | i686-pc-windows-gnu,i686-pc-windows-msvc,x86_64-pc-windows-gnu,x86_64-pc-windows-msvc,aarch64-pc-windows-msvc,aarch64-pc-windows-gnullvm,arm64ec-pc-windows-msvc,i586-pc-windows-msvc,i686-pc-windows-gnullvm,x86_64-pc-windows-gnullvm | 4_23      |             |
| ScreenToGif.2.41.1.Portable.x64          | ScreenToGif | x86_64-pc-windows-gnu,x86_64-pc-windows-msvc,x86_64-pc-windows-gnullvm                                                                                                                                                                  | 2.41.1    |             |
| ScreenToGif.2.41.1.Portable.Arm64        | ScreenToGif | aarch64-pc-windows-msvc,aarch64-pc-windows-gnullvm                                                                                                                                                                                      | 2.41.1    |             |
| alist-android-amd64                      | alist       | x86_64-linux-android                                                                                                                                                                                                                    |           |             |
| biome-win32-x64                          | biome       | x86_64-pc-windows-gnu,x86_64-pc-windows-msvc,x86_64-pc-windows-gnullvm                                                                                                                                                                  |           |             |
| node-v23.7.0-darwin-x64                  | node        | x86_64-apple-darwin                                                                                                                                                                                                                     | v23.7.0   |             |
| alist-linux-386                          | alist       | i686-unknown-linux-gnu,i686-unknown-linux-musl                                                                                                                                                                                          |           |             |
| zig-x86_64-linux-0.14.1                         | zig       | x86_64-unknown-linux-gnu,x86_64-unknown-linux-musl,x86_64-unknown-linux-gnux32,x86_64-unknown-linux-ohos                                                                                                                                                                                          |      0.14.1      |             |

## Platform Support

https://doc.rust-lang.org/nightly/rustc/platform-support.html#platform-support

### Tier 1

```
aarch64-apple-darwin
aarch64-unknown-linux-gnu
i686-pc-windows-gnu
i686-pc-windows-msvc
i686-unknown-linux-gnu
x86_64-apple-darwin
x86_64-pc-windows-gnu
x86_64-pc-windows-msvc
x86_64-unknown-linux-gnu
```

### Tier 2 with Host Tools

```
aarch64-pc-windows-msvc
aarch64-unknown-linux-musl
arm-unknown-linux-gnueabi
arm-unknown-linux-gnueabihf
armv7-unknown-linux-gnueabihf
loongarch64-unknown-linux-gnu
loongarch64-unknown-linux-musl
powerpc-unknown-linux-gnu
powerpc64-unknown-linux-gnu
powerpc64le-unknown-linux-gnu
powerpc64le-unknown-linux-musl
riscv64gc-unknown-linux-gnu
riscv64gc-unknown-linux-musl
s390x-unknown-linux-gnu
x86_64-unknown-freebsd
x86_64-unknown-illumos
x86_64-unknown-linux-musl
x86_64-unknown-netbsd
```

### Tier 2 without Host Tools

```
aarch64-apple-ios
aarch64-apple-ios-macabi
aarch64-apple-ios-sim
aarch64-linux-android
aarch64-pc-windows-gnullvm
aarch64-unknown-fuchsia
aarch64-unknown-linux-ohos
aarch64-unknown-none
aarch64-unknown-none-softfloat
aarch64-unknown-uefi
arm-linux-androideabi
arm-unknown-linux-musleabi
arm-unknown-linux-musleabihf
arm64ec-pc-windows-msvc
armebv7r-none-eabi
armebv7r-none-eabihf
armv5te-unknown-linux-gnueabi
armv5te-unknown-linux-musleabi
armv7-linux-androideabi
armv7-unknown-linux-gnueabi
armv7-unknown-linux-musleabi
armv7-unknown-linux-musleabihf
armv7-unknown-linux-ohos
armv7a-none-eabi
armv7r-none-eabi
armv7r-none-eabihf
i586-pc-windows-msvc
i586-unknown-linux-gnu
i586-unknown-linux-musl
i686-linux-android
i686-pc-windows-gnullvm
i686-unknown-freebsd
i686-unknown-linux-musl
i686-unknown-uefi
loongarch64-unknown-none
loongarch64-unknown-none-softfloat
nvptx64-nvidia-cuda
riscv32i-unknown-none-elf
riscv32im-unknown-none-elf
riscv32imac-unknown-none-elf
riscv32imafc-unknown-none-elf
riscv32imc-unknown-none-elf
riscv64gc-unknown-none-elf
riscv64imac-unknown-none-elf
sparc64-unknown-linux-gnu
sparcv9-sun-solaris
thumbv6m-none-eabi
thumbv7em-none-eabi
thumbv7em-none-eabihf
thumbv7m-none-eabi
thumbv7neon-linux-androideabi
thumbv7neon-unknown-linux-gnueabihf
thumbv8m.base-none-eabi
thumbv8m.main-none-eabi
thumbv8m.main-none-eabihf
wasm32-unknown-emscripten
wasm32-unknown-unknown
wasm32-wasip1
wasm32-wasip1-threads
wasm32-wasip2
wasm32v1-none
x86_64-apple-ios
x86_64-apple-ios-macabi
x86_64-fortanix-unknown-sgx
x86_64-linux-android
x86_64-pc-solaris
x86_64-pc-windows-gnullvm
x86_64-unknown-fuchsia
x86_64-unknown-linux-gnux32
x86_64-unknown-linux-ohos
x86_64-unknown-none
x86_64-unknown-redox
x86_64-unknown-uefi
```

### Tier 3 (TODO)

Supporting tier3 will cause regular expression matching to be severely slowed
down

```
aarch64-apple-tvos
aarch64-apple-tvos-sim
aarch64-apple-visionos
aarch64-apple-visionos-sim
aarch64-apple-watchos
aarch64-apple-watchos-sim
aarch64-kmc-solid_asp3
aarch64-nintendo-switch-freestanding
aarch64-unknown-freebsd
aarch64-unknown-hermit
aarch64-unknown-illumos
aarch64-unknown-linux-gnu_ilp32
aarch64-unknown-netbsd
aarch64-unknown-nto-qnx700
aarch64-unknown-nto-qnx710
aarch64-unknown-nto-qnx710_iosock
aarch64-unknown-nto-qnx800
aarch64-unknown-nuttx
aarch64-unknown-openbsd
aarch64-unknown-redox
aarch64-unknown-teeos
aarch64-unknown-trusty
aarch64-uwp-windows-msvc
aarch64-wrs-vxworks
aarch64_be-unknown-linux-gnu
aarch64_be-unknown-linux-gnu_ilp32
aarch64_be-unknown-netbsd
amdgcn-amd-amdhsa
arm64_32-apple-watchos
arm64e-apple-darwin
arm64e-apple-ios
arm64e-apple-tvos
armeb-unknown-linux-gnueabi
armv4t-none-eabi
armv4t-unknown-linux-gnueabi
armv5te-none-eabi
armv5te-unknown-linux-uclibceabi
armv6-unknown-freebsd
armv6-unknown-netbsd-eabihf
armv6k-nintendo-3ds
armv7-rtems-eabihf
armv7-sony-vita-newlibeabihf
armv7-unknown-freebsd
armv7-unknown-linux-uclibceabi
armv7-unknown-linux-uclibceabihf
armv7-unknown-netbsd-eabihf
armv7-unknown-trusty
armv7-wrs-vxworks-eabihf
armv7a-kmc-solid_asp3-eabi
armv7a-kmc-solid_asp3-eabihf
armv7a-none-eabihf
armv7k-apple-watchos
armv7s-apple-ios
armv8r-none-eabihf
armv7a-nuttx-eabi
armv7a-nuttx-eabihf
avr-none
bpfeb-unknown-none
bpfel-unknown-none
csky-unknown-linux-gnuabiv2
csky-unknown-linux-gnuabiv2hf
hexagon-unknown-linux-musl
hexagon-unknown-none-elf
i386-apple-ios
i586-unknown-netbsd
i586-unknown-redox
i686-apple-darwin
i686-pc-nto-qnx700
i686-unknown-haiku
i686-unknown-hurd-gnu
i686-unknown-netbsd
i686-unknown-openbsd
i686-uwp-windows-gnu
i686-uwp-windows-msvc
i686-win7-windows-gnu
i686-win7-windows-msvc
i686-wrs-vxworks
loongarch64-unknown-linux-ohos
m68k-unknown-linux-gnu
m68k-unknown-none-elf
mips-unknown-linux-gnu
mips-unknown-linux-musl
mips-unknown-linux-uclibc
mips64-openwrt-linux-musl
mips64-unknown-linux-gnuabi64
mips64-unknown-linux-muslabi64
mips64el-unknown-linux-gnuabi64
mips64el-unknown-linux-muslabi64
mipsel-sony-psp
mipsel-sony-psx
mipsel-unknown-linux-gnu
mipsel-unknown-linux-musl
mipsel-unknown-linux-uclibc
mipsel-unknown-netbsd
mipsel-unknown-none
mips-mti-none-elf
mipsel-mti-none-elf
mipsisa32r6-unknown-linux-gnu
mipsisa32r6el-unknown-linux-gnu
mipsisa64r6-unknown-linux-gnuabi64
mipsisa64r6el-unknown-linux-gnuabi64
msp430-none-elf
powerpc-unknown-freebsd
powerpc-unknown-linux-gnuspe
powerpc-unknown-linux-musl
powerpc-unknown-linux-muslspe
powerpc-unknown-netbsd
powerpc-unknown-openbsd
powerpc-wrs-vxworks
powerpc-wrs-vxworks-spe
powerpc64-ibm-aix
powerpc64-unknown-freebsd
powerpc64-unknown-linux-musl
powerpc64-unknown-openbsd
powerpc64-wrs-vxworks
powerpc64le-unknown-freebsd
riscv32-wrs-vxworks
riscv32e-unknown-none-elf
riscv32em-unknown-none-elf
riscv32emc-unknown-none-elf
riscv32gc-unknown-linux-gnu
riscv32gc-unknown-linux-musl
riscv32im-risc0-zkvm-elf
riscv32ima-unknown-none-elf
riscv32imac-esp-espidf
riscv32imac-unknown-nuttx-elf
riscv32imac-unknown-xous-elf
riscv32imafc-esp-espidf
riscv32imafc-unknown-nuttx-elf
riscv32imc-esp-espidf
riscv32imc-unknown-nuttx-elf
riscv64-linux-android
riscv64-wrs-vxworks
riscv64gc-unknown-freebsd
riscv64gc-unknown-fuchsia
riscv64gc-unknown-hermit
riscv64gc-unknown-netbsd
riscv64gc-unknown-nuttx-elf
riscv64gc-unknown-openbsd
riscv64imac-unknown-nuttx-elf
s390x-unknown-linux-musl
sparc-unknown-linux-gnu
sparc-unknown-none-elf
sparc64-unknown-netbsd
sparc64-unknown-openbsd
thumbv4t-none-eabi
thumbv5te-none-eabi
thumbv6m-nuttx-eabi
thumbv7a-pc-windows-msvc
thumbv7a-uwp-windows-msvc
thumbv7a-nuttx-eabi
thumbv7a-nuttx-eabihf
thumbv7em-nuttx-eabi
thumbv7em-nuttx-eabihf
thumbv7m-nuttx-eabi
thumbv7neon-unknown-linux-musleabihf
thumbv8m.base-nuttx-eabi
thumbv8m.main-nuttx-eabi
thumbv8m.main-nuttx-eabihf
wasm64-unknown-unknown
x86_64-apple-tvos
x86_64-apple-watchos-sim
x86_64-pc-cygwin
x86_64-pc-nto-qnx710
x86_64-pc-nto-qnx710_iosock
x86_64-pc-nto-qnx800
x86_64-unikraft-linux-musl
x86_64-unknown-dragonfly
x86_64-unknown-haiku
x86_64-unknown-hermit
x86_64-unknown-hurd-gnu
x86_64-unknown-l4re-uclibc
x86_64-unknown-linux-none
x86_64-unknown-openbsd
x86_64-unknown-trusty
x86_64-uwp-windows-gnu
x86_64-uwp-windows-msvc
x86_64-win7-windows-gnu
x86_64-win7-windows-msvc
x86_64-wrs-vxworks
x86_64h-apple-darwin
xtensa-esp32-espidf
xtensa-esp32-none-elf
xtensa-esp32s2-espidf
xtensa-esp32s2-none-elf
xtensa-esp32s3-espidf
xtensa-esp32s3-none-elf
```
