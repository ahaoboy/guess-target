//! Guess a Rust target triple from facts about a foreign host.
//!
//! The other `guess_target*` entry points reason about *artifact names* or
//! the machine this crate was compiled for. This module is for the third case
//! that keeps coming up in installers (`ei`) and their UIs (`mho-tauri`):
//! a remote host reports `uname` output (and, on Linux, its libc flavour)
//! and the caller needs the target triple to download.
//!
//! Only the raw facts are supplied by the caller — the heuristics live here,
//! so every consumer agrees:
//!
//! ```
//! use guess_target::{Libc, SystemInfo};
//!
//! let info = SystemInfo::from_uname("Linux", "aarch64", None).with_libc(Libc::Gnu);
//! assert_eq!(
//!     guess_target::guess_system_target(&info).map(|t| t.to_str()),
//!     Some("aarch64-unknown-linux-musl"),
//! );
//! ```

use crate::Target;

/// C library ABI of a Linux host.
#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum Libc {
    /// glibc (the default when the host does not look like musl).
    #[default]
    Gnu,
    /// musl (Alpine, OpenWrt, static builds).
    Musl,
}

impl Libc {
    /// Lower-case name, matching `uname`-probe output (`gnu` / `musl`).
    pub fn to_str(&self) -> &'static str {
        match self {
            Libc::Gnu => "gnu",
            Libc::Musl => "musl",
        }
    }
}

impl std::fmt::Display for Libc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_str())
    }
}

impl std::str::FromStr for Libc {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq_ignore_ascii_case("gnu") {
            Ok(Libc::Gnu)
        } else if s.eq_ignore_ascii_case("musl") {
            Ok(Libc::Musl)
        } else {
            Err("Unknown Libc")
        }
    }
}

/// Facts about a host, gathered from its own tools.
///
/// `os` and `arch` are the raw `uname -s` / `uname -m` values, because every
/// Linux distro, macOS, MSYS and the BSDs spell them slightly differently;
/// normalising is part of the guess.
#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SystemInfo {
    /// `uname -s`, e.g. `Linux`, `Darwin`, `FreeBSD`, `MINGW64_NT-10.0`.
    pub os: String,
    /// `uname -m`, e.g. `x86_64`, `aarch64`, `armv7l`, `armv8l`.
    pub arch: String,
    /// Linux libc flavour; ignored on every other OS.
    pub libc: Libc,
    /// Android userspace (Linux kernel with `uname -o` = `Android`).
    pub android: bool,
}

impl SystemInfo {
    /// Start from the two values every host reports.
    pub fn new(os: impl Into<String>, arch: impl Into<String>) -> Self {
        Self {
            os: os.into(),
            arch: arch.into(),
            ..Default::default()
        }
    }

    /// Build from `uname -s`, `uname -m` and the optional `uname -o`.
    ///
    /// `uname -o` is `Android` on Android and `GNU/Linux` (or missing) on
    /// regular Linux distributions.
    pub fn from_uname(sysname: &str, machine: &str, kernel: Option<&str>) -> Self {
        Self {
            os: sysname.to_string(),
            arch: machine.to_string(),
            libc: Libc::default(),
            android: kernel.is_some_and(|kernel| kernel.eq_ignore_ascii_case("android")),
        }
    }

    /// Set the Linux libc flavour.
    pub fn with_libc(mut self, libc: Libc) -> Self {
        self.libc = libc;
        self
    }

    /// Mark the host as running an Android userspace.
    pub fn with_android(mut self, android: bool) -> Self {
        self.android = android;
        self
    }

    /// Guess the target triple for this host.
    ///
    /// Shorthand for [`guess_system_target`].
    pub fn target(&self) -> Option<Target> {
        guess_system_target(self)
    }
}

/// Guess the Rust target triple matching the host described by `info`.
///
/// Returns `None` when the OS or CPU is not covered.
///
/// Two deliberate preferences, inherited from the installers this replaces:
///
/// * **32-bit ARM** is reported by the kernel of 64-bit Raspberry Pis
///   (`armv8l`) and routers (`armv7l`) alike; the 32-bit `armv7`/`arm`
///   triples are chosen, never `aarch64`.
/// * **ARM on Linux prefers musl**: glibc builds frequently fail on those
///   systems, so `aarch64`/`armv7`/`arm` select the musl triple even when the
///   host reports glibc. `x86_64` and `i686` keep the detected libc.
pub fn guess_system_target(info: &SystemInfo) -> Option<Target> {
    let arch = normalize_arch(&info.arch);
    let os = info.os.to_ascii_lowercase();

    if os.starts_with("linux") {
        return guess_linux(&arch, info.libc, info.android);
    }
    if os.starts_with("darwin") {
        return Some(match arch.as_str() {
            "aarch64" => Target::Aarch64AppleDarwin,
            _ => Target::X86_64AppleDarwin,
        });
    }
    if os.starts_with("mingw")
        || os.starts_with("msys")
        || os.starts_with("cygwin")
        || os.starts_with("windows")
    {
        return Some(match arch.as_str() {
            "aarch64" => Target::Aarch64PcWindowsMsvc,
            "i686" => Target::I686PcWindowsMsvc,
            _ => Target::X86_64PcWindowsMsvc,
        });
    }
    if os.starts_with("freebsd") {
        return match arch.as_str() {
            "x86_64" => Some(Target::X86_64UnknownFreebsd),
            "i686" => Some(Target::I686UnknownFreebsd),
            _ => None,
        };
    }
    if os.starts_with("netbsd") {
        return matches!(arch.as_str(), "x86_64").then_some(Target::X86_64UnknownNetbsd);
    }
    if os.starts_with("illumos") {
        return matches!(arch.as_str(), "x86_64").then_some(Target::X86_64UnknownIllumos);
    }

    None
}

/// Linux rules, including the Android userland.
fn guess_linux(arch: &str, libc: Libc, android: bool) -> Option<Target> {
    if android {
        return match arch {
            "x86_64" => Some(Target::X86_64LinuxAndroid),
            "aarch64" => Some(Target::Aarch64LinuxAndroid),
            "i686" => Some(Target::I686LinuxAndroid),
            "armv7" => Some(Target::Armv7LinuxAndroideabi),
            _ => None,
        };
    }

    let musl = libc == Libc::Musl;
    Some(match arch {
        "x86_64" if musl => Target::X86_64UnknownLinuxMusl,
        "x86_64" => Target::X86_64UnknownLinuxGnu,
        // ARM prefers musl even on glibc hosts (see the doc comment above).
        "aarch64" => Target::Aarch64UnknownLinuxMusl,
        "i686" if musl => Target::I686UnknownLinuxMusl,
        "i686" => Target::I686UnknownLinuxGnu,
        "riscv64" if musl => Target::Riscv64gcUnknownLinuxMusl,
        "riscv64" => Target::Riscv64gcUnknownLinuxGnu,
        "loongarch64" if musl => Target::Loongarch64UnknownLinuxMusl,
        "loongarch64" => Target::Loongarch64UnknownLinuxGnu,
        "s390x" => Target::S390xUnknownLinuxGnu,
        "powerpc64le" if musl => Target::Powerpc64leUnknownLinuxMusl,
        "powerpc64le" => Target::Powerpc64leUnknownLinuxGnu,
        "armv7" => Target::Armv7UnknownLinuxMusleabihf,
        "arm" => Target::ArmUnknownLinuxMusleabihf,
        _ => return None,
    })
}

/// Fold the `uname -m` spellings into the names used by Rust targets.
fn normalize_arch(arch: &str) -> String {
    let arch = arch.trim().to_ascii_lowercase();
    if arch == "amd64" || arch == "x64" {
        return "x86_64".to_string();
    }
    if arch == "arm64" || arch == "arm64e" {
        return "aarch64".to_string();
    }
    // `armv8l`: a 32-bit userland on a 64-bit ARM kernel.
    if arch.starts_with("armv7") || arch == "armv8l" {
        return "armv7".to_string();
    }
    if arch.starts_with("armv6") || arch == "arm" || arch.starts_with("armv5") {
        return "arm".to_string();
    }
    if arch == "i386" || arch == "i486" || arch == "i586" || arch == "i686" || arch == "x86" {
        return "i686".to_string();
    }
    if arch == "powerpc64le" || arch == "ppc64le" {
        return "powerpc64le".to_string();
    }
    arch
}

#[cfg(test)]
mod tests {
    use super::*;

    fn target(os: &str, arch: &str, libc: Libc, android: bool) -> Option<&'static str> {
        let info = SystemInfo::new(os, arch)
            .with_libc(libc)
            .with_android(android);
        guess_system_target(&info).map(|t| t.to_str())
    }

    #[test]
    fn guesses_linux_desktops() {
        assert_eq!(
            target("Linux", "x86_64", Libc::Gnu, false),
            Some("x86_64-unknown-linux-gnu")
        );
        assert_eq!(
            target("Linux", "x86_64", Libc::Musl, false),
            Some("x86_64-unknown-linux-musl")
        );
        assert_eq!(
            target("Linux", "amd64", Libc::Gnu, false),
            Some("x86_64-unknown-linux-gnu")
        );
        assert_eq!(
            target("Linux", "i686", Libc::Musl, false),
            Some("i686-unknown-linux-musl")
        );
        assert_eq!(
            target("Linux", "riscv64", Libc::Gnu, false),
            Some("riscv64gc-unknown-linux-gnu")
        );
        assert_eq!(
            target("Linux", "loongarch64", Libc::Musl, false),
            Some("loongarch64-unknown-linux-musl")
        );
        assert_eq!(
            target("Linux", "s390x", Libc::Musl, false),
            Some("s390x-unknown-linux-gnu")
        );
        assert_eq!(
            target("Linux", "ppc64le", Libc::Gnu, false),
            Some("powerpc64le-unknown-linux-gnu")
        );
    }

    #[test]
    fn arm_prefers_musl() {
        // `aarch64` and `armv7` select musl even on glibc hosts: glibc
        // artifacts are the ones that break on ARM.
        assert_eq!(
            target("Linux", "aarch64", Libc::Gnu, false),
            Some("aarch64-unknown-linux-musl")
        );
        assert_eq!(
            target("Linux", "aarch64", Libc::Musl, false),
            Some("aarch64-unknown-linux-musl")
        );
        assert_eq!(
            target("Linux", "armv7l", Libc::Gnu, false),
            Some("armv7-unknown-linux-musleabihf")
        );
        assert_eq!(
            target("Linux", "armv8l", Libc::Musl, false),
            Some("armv7-unknown-linux-musleabihf")
        );
        assert_eq!(
            target("Linux", "armv6l", Libc::Gnu, false),
            Some("arm-unknown-linux-musleabihf")
        );
    }

    #[test]
    fn guesses_android() {
        assert_eq!(
            target("Linux", "aarch64", Libc::Gnu, true),
            Some("aarch64-linux-android")
        );
        assert_eq!(
            target("Linux", "armv7l", Libc::Gnu, true),
            Some("armv7-linux-androideabi")
        );
        assert_eq!(
            target("Linux", "x86_64", Libc::Gnu, true),
            Some("x86_64-linux-android")
        );
        assert_eq!(
            target("Linux", "x86", Libc::Gnu, true),
            Some("i686-linux-android")
        );
    }

    #[test]
    fn guesses_darwin_and_windows() {
        assert_eq!(
            target("Darwin", "arm64", Libc::Gnu, false),
            Some("aarch64-apple-darwin")
        );
        assert_eq!(
            target("Darwin", "x86_64", Libc::Gnu, false),
            Some("x86_64-apple-darwin")
        );
        assert_eq!(
            target("MINGW64_NT-10.0", "x86_64", Libc::Gnu, false),
            Some("x86_64-pc-windows-msvc")
        );
        assert_eq!(
            target("MSYS_NT-10.0", "aarch64", Libc::Gnu, false),
            Some("aarch64-pc-windows-msvc")
        );
        assert_eq!(
            target("CYGWIN_NT-10.0", "i386", Libc::Gnu, false),
            Some("i686-pc-windows-msvc")
        );
    }

    #[test]
    fn guesses_bsds() {
        assert_eq!(
            target("FreeBSD", "x86_64", Libc::Gnu, false),
            Some("x86_64-unknown-freebsd")
        );
        assert_eq!(
            target("NetBSD", "x86_64", Libc::Gnu, false),
            Some("x86_64-unknown-netbsd")
        );
        assert_eq!(
            target("SunOS", "x86_64", Libc::Gnu, false),
            None,
            "Solaris is not covered"
        );
    }

    #[test]
    fn rejects_unknown_hosts() {
        assert_eq!(target("Plan9", "mips", Libc::Gnu, false), None);
        assert_eq!(target("Linux", "mips", Libc::Gnu, false), None);
    }

    #[test]
    fn builds_from_uname() {
        let info = SystemInfo::from_uname("Linux", "aarch64", Some("Android"));
        assert!(info.android);
        assert_eq!(info.libc, Libc::Gnu);
        assert_eq!(
            info.target().map(|t| t.to_str()),
            Some("aarch64-linux-android")
        );

        let info = SystemInfo::from_uname("Linux", "x86_64", Some("GNU/Linux"));
        assert!(!info.android);
        assert_eq!(
            info.clone()
                .with_libc(Libc::Musl)
                .target()
                .map(|t| t.to_str()),
            Some("x86_64-unknown-linux-musl")
        );
        assert_eq!(
            SystemInfo::from_uname("Linux", "x86_64", None)
                .target()
                .map(|t| t.to_str()),
            Some("x86_64-unknown-linux-gnu")
        );
    }

    #[test]
    fn parses_libc() {
        assert_eq!("musl".parse::<Libc>(), Ok(Libc::Musl));
        assert_eq!("GNU".parse::<Libc>(), Ok(Libc::Gnu));
        assert!("uclibc".parse::<Libc>().is_err());
        assert_eq!(Libc::default().to_string(), "gnu");
        assert_eq!(Libc::Musl.to_string(), "musl");
    }
}
