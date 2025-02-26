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
