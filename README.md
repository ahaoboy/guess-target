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

| filename                                 | name        | targets                                              | version   | git         |
| ---------------------------------------- | ----------- | ---------------------------------------------------- | --------- | ----------- |
| deno-x86_64-pc-windows-msvc              | deno        | x86_64-pc-windows-msvc                               |           |             |
| ryujinx-1.2.82-macos_universal           | ryujinx     | aarch64-apple-darwin,x86_64-apple-darwin             | 1.2.82    |             |
| qjs-darwin                               | qjs         | aarch64-apple-darwin,x86_64-apple-darwin             |           |             |
| fnm-macos                                | fnm         | aarch64-apple-darwin,x86_64-apple-darwin             |           |             |
| nu-0.102.0-armv7-unknown-linux-gnueabihf | nu          | armv7-unknown-linux-gnueabihf                        | 0.102.0   |             |
| 7z2409-linux-arm64                       | 7z2409      | aarch64-unknown-linux-gnu,aarch64-unknown-linux-musl |           |             |
| alist-windows-amd64                      | alist       | x86_64-pc-windows-gnu,x86_64-pc-windows-msvc         |           |             |
| mise-v2025.2.8-macos-arm64               | mise        | aarch64-apple-darwin                                 | v2025.2.8 |             |
| gh_2.67.0_linux_arm64                    | gh          | aarch64-unknown-linux-gnu,aarch64-unknown-linux-musl | 2.67.0    |             |
| GitHub CLI 2.67.0 linux arm64            | GitHub CLI  | aarch64-unknown-linux-gnu,aarch64-unknown-linux-musl | 2.67.0    |             |
| ffmpeg-n7.1-latest-win64-gpl-7.1         | ffmpeg      | x86_64-pc-windows-gnu,x86_64-pc-windows-msvc         | n7.1      |             |
| graaljs-24.1.2-macos-aarch64             | graaljs     | aarch64-apple-darwin                                 | 24.1.2    |             |
| hermes-cli-darwin                        | hermes-cli  | aarch64-apple-darwin,x86_64-apple-darwin             |           |             |
| mpv-x86_64-v3-20250221-git-e32beaa       | mpv         | x86_64-pc-windows-gnu,x86_64-pc-windows-msvc         | 20250221  | git-e32beaa |
| pnpm-win-x64                             | pnpm        | x86_64-pc-windows-gnu,x86_64-pc-windows-msvc         |           |             |
| xst-lin64                                | xst         | aarch64-unknown-linux-gnu,aarch64-unknown-linux-musl |           |             |
| wiztree_4_23_portable                    | wiztree     | x86_64-pc-windows-gnu,x86_64-pc-windows-msvc         | 4_23      |             |
| ScreenToGif.2.41.1.Portable.x64          | ScreenToGif | x86_64-pc-windows-gnu,x86_64-pc-windows-msvc         | 2.41.1    |             |

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

### Tier 2

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
