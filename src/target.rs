#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Os {
    Darwin,
    Linux,
    Windows,
    Freebsd,
    Illumos,
    Netbsd,
}
impl Os {
    pub fn to_str(&self) -> &'static str {
        match self {
            Os::Darwin => "darwin",
            Os::Linux => "linux",
            Os::Windows => "windows",
            Os::Freebsd => "freebsd",
            Os::Illumos => "illumos",
            Os::Netbsd => "netbsd",
        }
    }
}
impl std::fmt::Display for Os {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_str())
    }
}
impl std::str::FromStr for Os {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "darwin" => Ok(Os::Darwin),
            "linux" => Ok(Os::Linux),
            "windows" => Ok(Os::Windows),
            "freebsd" => Ok(Os::Freebsd),
            "illumos" => Ok(Os::Illumos),
            "netbsd" => Ok(Os::Netbsd),
            _ => Err("Unknown Os"),
        }
    }
}
pub const OS_LIST: [Os; 6] = [
    Os::Darwin,
    Os::Linux,
    Os::Windows,
    Os::Freebsd,
    Os::Illumos,
    Os::Netbsd,
];
#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Arch {
    Aarch64,
    I686,
    X86_64,
    Arm,
    Armv7,
    Loongarch64,
    Powerpc,
    Powerpc64,
    Powerpc64le,
    Riscv64gc,
    S390x,
}
impl Arch {
    pub fn to_str(&self) -> &'static str {
        match self {
            Arch::Aarch64 => "aarch64",
            Arch::I686 => "i686",
            Arch::X86_64 => "x86_64",
            Arch::Arm => "arm",
            Arch::Armv7 => "armv7",
            Arch::Loongarch64 => "loongarch64",
            Arch::Powerpc => "powerpc",
            Arch::Powerpc64 => "powerpc64",
            Arch::Powerpc64le => "powerpc64le",
            Arch::Riscv64gc => "riscv64gc",
            Arch::S390x => "s390x",
        }
    }
}
impl std::fmt::Display for Arch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_str())
    }
}
impl std::str::FromStr for Arch {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "aarch64" => Ok(Arch::Aarch64),
            "i686" => Ok(Arch::I686),
            "x86_64" => Ok(Arch::X86_64),
            "arm" => Ok(Arch::Arm),
            "armv7" => Ok(Arch::Armv7),
            "loongarch64" => Ok(Arch::Loongarch64),
            "powerpc" => Ok(Arch::Powerpc),
            "powerpc64" => Ok(Arch::Powerpc64),
            "powerpc64le" => Ok(Arch::Powerpc64le),
            "riscv64gc" => Ok(Arch::Riscv64gc),
            "s390x" => Ok(Arch::S390x),
            _ => Err("Unknown Arch"),
        }
    }
}
pub const ARCH_LIST: [Arch; 11] = [
    Arch::Aarch64,
    Arch::I686,
    Arch::X86_64,
    Arch::Arm,
    Arch::Armv7,
    Arch::Loongarch64,
    Arch::Powerpc,
    Arch::Powerpc64,
    Arch::Powerpc64le,
    Arch::Riscv64gc,
    Arch::S390x,
];
#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Vendor {
    Apple,
    Unknown,
    Pc,
}
impl Vendor {
    pub fn to_str(&self) -> &'static str {
        match self {
            Vendor::Apple => "apple",
            Vendor::Unknown => "unknown",
            Vendor::Pc => "pc",
        }
    }
}
impl std::fmt::Display for Vendor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_str())
    }
}
impl std::str::FromStr for Vendor {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "apple" => Ok(Vendor::Apple),
            "unknown" => Ok(Vendor::Unknown),
            "pc" => Ok(Vendor::Pc),
            _ => Err("Unknown Vendor"),
        }
    }
}
pub const VENDOR_LIST: [Vendor; 3] = [Vendor::Apple, Vendor::Unknown, Vendor::Pc];
#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Abi {
    Gnu,
    Msvc,
    Musl,
    Gnueabi,
    Gnueabihf,
}
impl Abi {
    pub fn to_str(&self) -> &'static str {
        match self {
            Abi::Gnu => "gnu",
            Abi::Msvc => "msvc",
            Abi::Musl => "musl",
            Abi::Gnueabi => "gnueabi",
            Abi::Gnueabihf => "gnueabihf",
        }
    }
}
impl std::fmt::Display for Abi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_str())
    }
}
impl std::str::FromStr for Abi {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "gnu" => Ok(Abi::Gnu),
            "msvc" => Ok(Abi::Msvc),
            "musl" => Ok(Abi::Musl),
            "gnueabi" => Ok(Abi::Gnueabi),
            "gnueabihf" => Ok(Abi::Gnueabihf),
            _ => Err("Unknown Abi"),
        }
    }
}
pub const ABI_LIST: [Abi; 5] = [Abi::Gnu, Abi::Msvc, Abi::Musl, Abi::Gnueabi, Abi::Gnueabihf];
#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Target {
    Aarch64AppleDarwin,
    Aarch64UnknownLinuxGnu,
    I686PcWindowsGnu,
    I686PcWindowsMsvc,
    I686UnknownLinuxGnu,
    X86_64AppleDarwin,
    X86_64PcWindowsGnu,
    X86_64PcWindowsMsvc,
    X86_64UnknownLinuxGnu,
    Aarch64PcWindowsMsvc,
    Aarch64UnknownLinuxMusl,
    ArmUnknownLinuxGnueabi,
    ArmUnknownLinuxGnueabihf,
    Armv7UnknownLinuxGnueabihf,
    Loongarch64UnknownLinuxGnu,
    Loongarch64UnknownLinuxMusl,
    PowerpcUnknownLinuxGnu,
    Powerpc64UnknownLinuxGnu,
    Powerpc64leUnknownLinuxGnu,
    Powerpc64leUnknownLinuxMusl,
    Riscv64gcUnknownLinuxGnu,
    Riscv64gcUnknownLinuxMusl,
    S390xUnknownLinuxGnu,
    X86_64UnknownFreebsd,
    X86_64UnknownIllumos,
    X86_64UnknownLinuxMusl,
    X86_64UnknownNetbsd,
}
impl Target {
    pub fn to_str(&self) -> &'static str {
        match self {
            Target::Aarch64AppleDarwin => "aarch64-apple-darwin",
            Target::Aarch64UnknownLinuxGnu => "aarch64-unknown-linux-gnu",
            Target::I686PcWindowsGnu => "i686-pc-windows-gnu",
            Target::I686PcWindowsMsvc => "i686-pc-windows-msvc",
            Target::I686UnknownLinuxGnu => "i686-unknown-linux-gnu",
            Target::X86_64AppleDarwin => "x86_64-apple-darwin",
            Target::X86_64PcWindowsGnu => "x86_64-pc-windows-gnu",
            Target::X86_64PcWindowsMsvc => "x86_64-pc-windows-msvc",
            Target::X86_64UnknownLinuxGnu => "x86_64-unknown-linux-gnu",
            Target::Aarch64PcWindowsMsvc => "aarch64-pc-windows-msvc",
            Target::Aarch64UnknownLinuxMusl => "aarch64-unknown-linux-musl",
            Target::ArmUnknownLinuxGnueabi => "arm-unknown-linux-gnueabi",
            Target::ArmUnknownLinuxGnueabihf => "arm-unknown-linux-gnueabihf",
            Target::Armv7UnknownLinuxGnueabihf => "armv7-unknown-linux-gnueabihf",
            Target::Loongarch64UnknownLinuxGnu => "loongarch64-unknown-linux-gnu",
            Target::Loongarch64UnknownLinuxMusl => "loongarch64-unknown-linux-musl",
            Target::PowerpcUnknownLinuxGnu => "powerpc-unknown-linux-gnu",
            Target::Powerpc64UnknownLinuxGnu => "powerpc64-unknown-linux-gnu",
            Target::Powerpc64leUnknownLinuxGnu => "powerpc64le-unknown-linux-gnu",
            Target::Powerpc64leUnknownLinuxMusl => "powerpc64le-unknown-linux-musl",
            Target::Riscv64gcUnknownLinuxGnu => "riscv64gc-unknown-linux-gnu",
            Target::Riscv64gcUnknownLinuxMusl => "riscv64gc-unknown-linux-musl",
            Target::S390xUnknownLinuxGnu => "s390x-unknown-linux-gnu",
            Target::X86_64UnknownFreebsd => "x86_64-unknown-freebsd",
            Target::X86_64UnknownIllumos => "x86_64-unknown-illumos",
            Target::X86_64UnknownLinuxMusl => "x86_64-unknown-linux-musl",
            Target::X86_64UnknownNetbsd => "x86_64-unknown-netbsd",
        }
    }
}
impl std::fmt::Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_str())
    }
}
impl std::str::FromStr for Target {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "aarch64-apple-darwin" => Ok(Target::Aarch64AppleDarwin),
            "aarch64-unknown-linux-gnu" => Ok(Target::Aarch64UnknownLinuxGnu),
            "i686-pc-windows-gnu" => Ok(Target::I686PcWindowsGnu),
            "i686-pc-windows-msvc" => Ok(Target::I686PcWindowsMsvc),
            "i686-unknown-linux-gnu" => Ok(Target::I686UnknownLinuxGnu),
            "x86_64-apple-darwin" => Ok(Target::X86_64AppleDarwin),
            "x86_64-pc-windows-gnu" => Ok(Target::X86_64PcWindowsGnu),
            "x86_64-pc-windows-msvc" => Ok(Target::X86_64PcWindowsMsvc),
            "x86_64-unknown-linux-gnu" => Ok(Target::X86_64UnknownLinuxGnu),
            "aarch64-pc-windows-msvc" => Ok(Target::Aarch64PcWindowsMsvc),
            "aarch64-unknown-linux-musl" => Ok(Target::Aarch64UnknownLinuxMusl),
            "arm-unknown-linux-gnueabi" => Ok(Target::ArmUnknownLinuxGnueabi),
            "arm-unknown-linux-gnueabihf" => Ok(Target::ArmUnknownLinuxGnueabihf),
            "armv7-unknown-linux-gnueabihf" => Ok(Target::Armv7UnknownLinuxGnueabihf),
            "loongarch64-unknown-linux-gnu" => Ok(Target::Loongarch64UnknownLinuxGnu),
            "loongarch64-unknown-linux-musl" => Ok(Target::Loongarch64UnknownLinuxMusl),
            "powerpc-unknown-linux-gnu" => Ok(Target::PowerpcUnknownLinuxGnu),
            "powerpc64-unknown-linux-gnu" => Ok(Target::Powerpc64UnknownLinuxGnu),
            "powerpc64le-unknown-linux-gnu" => Ok(Target::Powerpc64leUnknownLinuxGnu),
            "powerpc64le-unknown-linux-musl" => Ok(Target::Powerpc64leUnknownLinuxMusl),
            "riscv64gc-unknown-linux-gnu" => Ok(Target::Riscv64gcUnknownLinuxGnu),
            "riscv64gc-unknown-linux-musl" => Ok(Target::Riscv64gcUnknownLinuxMusl),
            "s390x-unknown-linux-gnu" => Ok(Target::S390xUnknownLinuxGnu),
            "x86_64-unknown-freebsd" => Ok(Target::X86_64UnknownFreebsd),
            "x86_64-unknown-illumos" => Ok(Target::X86_64UnknownIllumos),
            "x86_64-unknown-linux-musl" => Ok(Target::X86_64UnknownLinuxMusl),
            "x86_64-unknown-netbsd" => Ok(Target::X86_64UnknownNetbsd),
            _ => Err("Unknown Target"),
        }
    }
}
pub const TARGET_LIST: [Target; 27] = [
    Target::Aarch64AppleDarwin,
    Target::Aarch64UnknownLinuxGnu,
    Target::I686PcWindowsGnu,
    Target::I686PcWindowsMsvc,
    Target::I686UnknownLinuxGnu,
    Target::X86_64AppleDarwin,
    Target::X86_64PcWindowsGnu,
    Target::X86_64PcWindowsMsvc,
    Target::X86_64UnknownLinuxGnu,
    Target::Aarch64PcWindowsMsvc,
    Target::Aarch64UnknownLinuxMusl,
    Target::ArmUnknownLinuxGnueabi,
    Target::ArmUnknownLinuxGnueabihf,
    Target::Armv7UnknownLinuxGnueabihf,
    Target::Loongarch64UnknownLinuxGnu,
    Target::Loongarch64UnknownLinuxMusl,
    Target::PowerpcUnknownLinuxGnu,
    Target::Powerpc64UnknownLinuxGnu,
    Target::Powerpc64leUnknownLinuxGnu,
    Target::Powerpc64leUnknownLinuxMusl,
    Target::Riscv64gcUnknownLinuxGnu,
    Target::Riscv64gcUnknownLinuxMusl,
    Target::S390xUnknownLinuxGnu,
    Target::X86_64UnknownFreebsd,
    Target::X86_64UnknownIllumos,
    Target::X86_64UnknownLinuxMusl,
    Target::X86_64UnknownNetbsd,
];
impl Target {
    pub fn arch(&self) -> Arch {
        match self {
            Target::Aarch64AppleDarwin => Arch::Aarch64,
            Target::Aarch64UnknownLinuxGnu => Arch::Aarch64,
            Target::I686PcWindowsGnu => Arch::I686,
            Target::I686PcWindowsMsvc => Arch::I686,
            Target::I686UnknownLinuxGnu => Arch::I686,
            Target::X86_64AppleDarwin => Arch::X86_64,
            Target::X86_64PcWindowsGnu => Arch::X86_64,
            Target::X86_64PcWindowsMsvc => Arch::X86_64,
            Target::X86_64UnknownLinuxGnu => Arch::X86_64,
            Target::Aarch64PcWindowsMsvc => Arch::Aarch64,
            Target::Aarch64UnknownLinuxMusl => Arch::Aarch64,
            Target::ArmUnknownLinuxGnueabi => Arch::Arm,
            Target::ArmUnknownLinuxGnueabihf => Arch::Arm,
            Target::Armv7UnknownLinuxGnueabihf => Arch::Armv7,
            Target::Loongarch64UnknownLinuxGnu => Arch::Loongarch64,
            Target::Loongarch64UnknownLinuxMusl => Arch::Loongarch64,
            Target::PowerpcUnknownLinuxGnu => Arch::Powerpc,
            Target::Powerpc64UnknownLinuxGnu => Arch::Powerpc64,
            Target::Powerpc64leUnknownLinuxGnu => Arch::Powerpc64le,
            Target::Powerpc64leUnknownLinuxMusl => Arch::Powerpc64le,
            Target::Riscv64gcUnknownLinuxGnu => Arch::Riscv64gc,
            Target::Riscv64gcUnknownLinuxMusl => Arch::Riscv64gc,
            Target::S390xUnknownLinuxGnu => Arch::S390x,
            Target::X86_64UnknownFreebsd => Arch::X86_64,
            Target::X86_64UnknownIllumos => Arch::X86_64,
            Target::X86_64UnknownLinuxMusl => Arch::X86_64,
            Target::X86_64UnknownNetbsd => Arch::X86_64,
        }
    }
    pub fn vender(&self) -> Vendor {
        match self {
            Target::Aarch64AppleDarwin => Vendor::Apple,
            Target::Aarch64UnknownLinuxGnu => Vendor::Unknown,
            Target::I686PcWindowsGnu => Vendor::Pc,
            Target::I686PcWindowsMsvc => Vendor::Pc,
            Target::I686UnknownLinuxGnu => Vendor::Unknown,
            Target::X86_64AppleDarwin => Vendor::Apple,
            Target::X86_64PcWindowsGnu => Vendor::Pc,
            Target::X86_64PcWindowsMsvc => Vendor::Pc,
            Target::X86_64UnknownLinuxGnu => Vendor::Unknown,
            Target::Aarch64PcWindowsMsvc => Vendor::Pc,
            Target::Aarch64UnknownLinuxMusl => Vendor::Unknown,
            Target::ArmUnknownLinuxGnueabi => Vendor::Unknown,
            Target::ArmUnknownLinuxGnueabihf => Vendor::Unknown,
            Target::Armv7UnknownLinuxGnueabihf => Vendor::Unknown,
            Target::Loongarch64UnknownLinuxGnu => Vendor::Unknown,
            Target::Loongarch64UnknownLinuxMusl => Vendor::Unknown,
            Target::PowerpcUnknownLinuxGnu => Vendor::Unknown,
            Target::Powerpc64UnknownLinuxGnu => Vendor::Unknown,
            Target::Powerpc64leUnknownLinuxGnu => Vendor::Unknown,
            Target::Powerpc64leUnknownLinuxMusl => Vendor::Unknown,
            Target::Riscv64gcUnknownLinuxGnu => Vendor::Unknown,
            Target::Riscv64gcUnknownLinuxMusl => Vendor::Unknown,
            Target::S390xUnknownLinuxGnu => Vendor::Unknown,
            Target::X86_64UnknownFreebsd => Vendor::Unknown,
            Target::X86_64UnknownIllumos => Vendor::Unknown,
            Target::X86_64UnknownLinuxMusl => Vendor::Unknown,
            Target::X86_64UnknownNetbsd => Vendor::Unknown,
        }
    }
    pub fn os(&self) -> Os {
        match self {
            Target::Aarch64AppleDarwin => Os::Darwin,
            Target::Aarch64UnknownLinuxGnu => Os::Linux,
            Target::I686PcWindowsGnu => Os::Windows,
            Target::I686PcWindowsMsvc => Os::Windows,
            Target::I686UnknownLinuxGnu => Os::Linux,
            Target::X86_64AppleDarwin => Os::Darwin,
            Target::X86_64PcWindowsGnu => Os::Windows,
            Target::X86_64PcWindowsMsvc => Os::Windows,
            Target::X86_64UnknownLinuxGnu => Os::Linux,
            Target::Aarch64PcWindowsMsvc => Os::Windows,
            Target::Aarch64UnknownLinuxMusl => Os::Linux,
            Target::ArmUnknownLinuxGnueabi => Os::Linux,
            Target::ArmUnknownLinuxGnueabihf => Os::Linux,
            Target::Armv7UnknownLinuxGnueabihf => Os::Linux,
            Target::Loongarch64UnknownLinuxGnu => Os::Linux,
            Target::Loongarch64UnknownLinuxMusl => Os::Linux,
            Target::PowerpcUnknownLinuxGnu => Os::Linux,
            Target::Powerpc64UnknownLinuxGnu => Os::Linux,
            Target::Powerpc64leUnknownLinuxGnu => Os::Linux,
            Target::Powerpc64leUnknownLinuxMusl => Os::Linux,
            Target::Riscv64gcUnknownLinuxGnu => Os::Linux,
            Target::Riscv64gcUnknownLinuxMusl => Os::Linux,
            Target::S390xUnknownLinuxGnu => Os::Linux,
            Target::X86_64UnknownFreebsd => Os::Freebsd,
            Target::X86_64UnknownIllumos => Os::Illumos,
            Target::X86_64UnknownLinuxMusl => Os::Linux,
            Target::X86_64UnknownNetbsd => Os::Netbsd,
        }
    }
    pub fn abi(&self) -> Option<Abi> {
        match self {
            Target::Aarch64AppleDarwin => None,
            Target::Aarch64UnknownLinuxGnu => Some(Abi::Gnu),
            Target::I686PcWindowsGnu => Some(Abi::Gnu),
            Target::I686PcWindowsMsvc => Some(Abi::Msvc),
            Target::I686UnknownLinuxGnu => Some(Abi::Gnu),
            Target::X86_64AppleDarwin => None,
            Target::X86_64PcWindowsGnu => Some(Abi::Gnu),
            Target::X86_64PcWindowsMsvc => Some(Abi::Msvc),
            Target::X86_64UnknownLinuxGnu => Some(Abi::Gnu),
            Target::Aarch64PcWindowsMsvc => Some(Abi::Msvc),
            Target::Aarch64UnknownLinuxMusl => Some(Abi::Musl),
            Target::ArmUnknownLinuxGnueabi => Some(Abi::Gnueabi),
            Target::ArmUnknownLinuxGnueabihf => Some(Abi::Gnueabihf),
            Target::Armv7UnknownLinuxGnueabihf => Some(Abi::Gnueabihf),
            Target::Loongarch64UnknownLinuxGnu => Some(Abi::Gnu),
            Target::Loongarch64UnknownLinuxMusl => Some(Abi::Musl),
            Target::PowerpcUnknownLinuxGnu => Some(Abi::Gnu),
            Target::Powerpc64UnknownLinuxGnu => Some(Abi::Gnu),
            Target::Powerpc64leUnknownLinuxGnu => Some(Abi::Gnu),
            Target::Powerpc64leUnknownLinuxMusl => Some(Abi::Musl),
            Target::Riscv64gcUnknownLinuxGnu => Some(Abi::Gnu),
            Target::Riscv64gcUnknownLinuxMusl => Some(Abi::Musl),
            Target::S390xUnknownLinuxGnu => Some(Abi::Gnu),
            Target::X86_64UnknownFreebsd => None,
            Target::X86_64UnknownIllumos => None,
            Target::X86_64UnknownLinuxMusl => Some(Abi::Musl),
            Target::X86_64UnknownNetbsd => None,
        }
    }
}
pub const DEFAULT_RULE: [(Target, Arch, Vendor, Os, Option<Abi>); 27] = [
    (
        Target::Aarch64AppleDarwin,
        Arch::Aarch64,
        Vendor::Apple,
        Os::Darwin,
        None,
    ),
    (
        Target::Aarch64UnknownLinuxGnu,
        Arch::Aarch64,
        Vendor::Unknown,
        Os::Linux,
        Some(Abi::Gnu),
    ),
    (
        Target::I686PcWindowsGnu,
        Arch::I686,
        Vendor::Pc,
        Os::Windows,
        Some(Abi::Gnu),
    ),
    (
        Target::I686PcWindowsMsvc,
        Arch::I686,
        Vendor::Pc,
        Os::Windows,
        Some(Abi::Msvc),
    ),
    (
        Target::I686UnknownLinuxGnu,
        Arch::I686,
        Vendor::Unknown,
        Os::Linux,
        Some(Abi::Gnu),
    ),
    (
        Target::X86_64AppleDarwin,
        Arch::X86_64,
        Vendor::Apple,
        Os::Darwin,
        None,
    ),
    (
        Target::X86_64PcWindowsGnu,
        Arch::X86_64,
        Vendor::Pc,
        Os::Windows,
        Some(Abi::Gnu),
    ),
    (
        Target::X86_64PcWindowsMsvc,
        Arch::X86_64,
        Vendor::Pc,
        Os::Windows,
        Some(Abi::Msvc),
    ),
    (
        Target::X86_64UnknownLinuxGnu,
        Arch::X86_64,
        Vendor::Unknown,
        Os::Linux,
        Some(Abi::Gnu),
    ),
    (
        Target::Aarch64PcWindowsMsvc,
        Arch::Aarch64,
        Vendor::Pc,
        Os::Windows,
        Some(Abi::Msvc),
    ),
    (
        Target::Aarch64UnknownLinuxMusl,
        Arch::Aarch64,
        Vendor::Unknown,
        Os::Linux,
        Some(Abi::Musl),
    ),
    (
        Target::ArmUnknownLinuxGnueabi,
        Arch::Arm,
        Vendor::Unknown,
        Os::Linux,
        Some(Abi::Gnueabi),
    ),
    (
        Target::ArmUnknownLinuxGnueabihf,
        Arch::Arm,
        Vendor::Unknown,
        Os::Linux,
        Some(Abi::Gnueabihf),
    ),
    (
        Target::Armv7UnknownLinuxGnueabihf,
        Arch::Armv7,
        Vendor::Unknown,
        Os::Linux,
        Some(Abi::Gnueabihf),
    ),
    (
        Target::Loongarch64UnknownLinuxGnu,
        Arch::Loongarch64,
        Vendor::Unknown,
        Os::Linux,
        Some(Abi::Gnu),
    ),
    (
        Target::Loongarch64UnknownLinuxMusl,
        Arch::Loongarch64,
        Vendor::Unknown,
        Os::Linux,
        Some(Abi::Musl),
    ),
    (
        Target::PowerpcUnknownLinuxGnu,
        Arch::Powerpc,
        Vendor::Unknown,
        Os::Linux,
        Some(Abi::Gnu),
    ),
    (
        Target::Powerpc64UnknownLinuxGnu,
        Arch::Powerpc64,
        Vendor::Unknown,
        Os::Linux,
        Some(Abi::Gnu),
    ),
    (
        Target::Powerpc64leUnknownLinuxGnu,
        Arch::Powerpc64le,
        Vendor::Unknown,
        Os::Linux,
        Some(Abi::Gnu),
    ),
    (
        Target::Powerpc64leUnknownLinuxMusl,
        Arch::Powerpc64le,
        Vendor::Unknown,
        Os::Linux,
        Some(Abi::Musl),
    ),
    (
        Target::Riscv64gcUnknownLinuxGnu,
        Arch::Riscv64gc,
        Vendor::Unknown,
        Os::Linux,
        Some(Abi::Gnu),
    ),
    (
        Target::Riscv64gcUnknownLinuxMusl,
        Arch::Riscv64gc,
        Vendor::Unknown,
        Os::Linux,
        Some(Abi::Musl),
    ),
    (
        Target::S390xUnknownLinuxGnu,
        Arch::S390x,
        Vendor::Unknown,
        Os::Linux,
        Some(Abi::Gnu),
    ),
    (
        Target::X86_64UnknownFreebsd,
        Arch::X86_64,
        Vendor::Unknown,
        Os::Freebsd,
        None,
    ),
    (
        Target::X86_64UnknownIllumos,
        Arch::X86_64,
        Vendor::Unknown,
        Os::Illumos,
        None,
    ),
    (
        Target::X86_64UnknownLinuxMusl,
        Arch::X86_64,
        Vendor::Unknown,
        Os::Linux,
        Some(Abi::Musl),
    ),
    (
        Target::X86_64UnknownNetbsd,
        Arch::X86_64,
        Vendor::Unknown,
        Os::Netbsd,
        None,
    ),
];
