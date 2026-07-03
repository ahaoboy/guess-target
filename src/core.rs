use crate::{
    Abi,
    target::{Arch, Os, Target},
};
use is_musl::is_musl;
use once_cell::sync::Lazy;
use regex::{Regex, RegexBuilder};
use std::{borrow::Cow, collections::HashMap, process::Command, str::FromStr};
use strum::IntoEnumIterator;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub struct GuessTarget {
    #[cfg_attr(feature = "wasm", wasm_bindgen(skip))]
    pub name: String,
    pub target: Target,
    pub rank: u32,
    #[cfg_attr(feature = "wasm", wasm_bindgen(skip))]
    pub version: Option<String>,
    #[cfg_attr(feature = "wasm", wasm_bindgen(skip))]
    pub git: Option<String>,
}

#[derive(Debug, Clone)]
struct Rule {
    re: Regex,
    target: Vec<Target>,
    rank: u32,
}

const SEQ_RE: &str = r"[_ -\.]";
const NAME_RE: &str = r"(?P<name>[^/.]+)";
const EARLY_EXIT_GAP: u32 = 5;

fn build_re(s: &str) -> Regex {
    RegexBuilder::new(s).case_insensitive(true).build().unwrap()
}

// Cached regex patterns for version detection
static VERSION_REGEXES: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        // nightly-2024-01-01
        build_re(r"(?P<version>nightly-\d{4}-\d{2}-\d{2})"),
        // 1.2.3-rc1, 1.2.3-rc2
        build_re(r"(?P<version>v?\d+\.\d+\.\d+-rc\d+)"),
        // 1.2.3-alpha, 1.2.3-beta
        build_re(r"(?P<version>v?\d+\.\d+\.\d+-(alpha|beta))"),
        // 20250225
        build_re(r"(?P<version>[a-zA-Z]?\d{7,})"),
        // 1.2.3.4
        build_re(r"(?P<version>[a-zA-Z]?\d{1,4}(?:[\._-]\d{1,4}){3})"),
        // 1.2.3
        build_re(r"(?P<version>[a-zA-Z]?\d{1,4}(?:[\._-]\d{1,4}){2})"),
        // 1.2
        build_re(r"(?P<version>[a-zA-Z]?\d{1,4}[\._-]\d{1,4})"),
    ]
});

// Cached regex pattern for git hash detection
static GIT_REGEX: Lazy<Regex> = Lazy::new(|| build_re(r"(?P<git>git[-_ ][0-9a-fA-F-]{7,})\b"));

const SUFFIXES: [&str; 4] = ["latest", "alpha", "beta", "master"];

#[inline]
fn next_sep_len(s: &str) -> usize {
    match s.as_bytes().first() {
        Some(b) if matches!(*b, b'-' | b'_' | b'.' | b' ') => 1,
        _ => 0,
    }
}

/// Remove the first occurrence of `token` from `s`, along with an optional
/// trailing separator, an optional `(latest|alpha|beta|master)` suffix and
/// another separator. Replaces the previous per-call `build_re(format!(...))`
/// regex compilation — same behavior, zero regex compile cost.
fn strip_token<'a>(s: &'a str, token: &str, with_suffix: bool) -> Cow<'a, str> {
    let Some(start) = s.find(token) else {
        return Cow::Borrowed(s);
    };
    let mut end = start + token.len();
    end += next_sep_len(&s[end..]);
    if with_suffix {
        let rest = &s[end..];
        for suffix in SUFFIXES {
            if rest.len() >= suffix.len()
                && rest[..suffix.len()].eq_ignore_ascii_case(suffix)
            {
                end += suffix.len();
                end += next_sep_len(&s[end..]);
                break;
            }
        }
    }
    let mut out = String::with_capacity(s.len() - (end - start));
    out.push_str(&s[..start]);
    out.push_str(&s[end..]);
    Cow::Owned(out)
}

pub fn get_common_targets(target: &Target) -> Vec<(String, u32)> {
    let os = target.os();
    let arch = target.arch();
    let abi = target.abi();

    let mut v = Vec::with_capacity(20);

    let mut os_list = match os {
        Os::Linux => vec!["linux", "linuxstatic"],
        Os::Darwin => vec!["darwin", "macos", "mac", "mac64"],
        Os::Windows => vec!["windows", "win32", "win"],
        Os::Freebsd => vec!["freebsd"],
        Os::Netbsd => vec!["netbsd"],
        Os::Android => vec!["android"],
        _ => vec![],
    };
    let mut arch_list = match arch {
        Arch::X86_64 => vec!["x86_64", "amd64", "x64", "x86", "i686", "legacy", "ia64"],
        Arch::I686 => vec!["386", "i686", "x86", "ia32", "i386"],
        Arch::Aarch64 => vec!["aarch64", "arm64", "armv8", "armv7"],
        Arch::Arm => vec!["arm", "armv6"],
        Arch::S390x => vec!["s390x"],
        Arch::Powerpc => vec!["powerpc", "ppc"],
        Arch::Powerpc64 => vec!["powerpc64", "ppc64"],
        Arch::Powerpc64le => vec!["ppc64le", "powerpc64le"],
        Arch::Riscv64gc => vec!["riscv64", "riscv"],
        Arch::Armv7 => vec!["armv7", "armv7l"],
        _ => vec![],
    };

    if os == Os::Darwin && arch == Arch::X86_64 {
        os_list.push("osx-13");
    }
    if os == Os::Darwin && arch == Arch::Aarch64 {
        os_list.push("mac64arm");
        os_list.push("osx-14");
    }
    if os == Os::Darwin {
        arch_list.push("universal");
    }
    if os == Os::Linux && arch == Arch::Aarch64 {
        os_list.push("lin64");
        os_list.push("arm64v8");
    }

    if os == Os::Windows {
        if arch == Arch::X86_64 || arch == Arch::Aarch64 {
            os_list.push("win64");
        }
        v.push(("portable".to_string(), 1));
        if arch == Arch::X86_64 {
            arch_list.push("x86-64-v3");
            v.push(("x86-64-v3".to_string(), 5));
            v.push((
                format!(
                    "(light)?{}?portable{}({})",
                    SEQ_RE,
                    SEQ_RE,
                    arch_list.join("|")
                ),
                5,
            ));
        }

        if arch == Arch::Aarch64 {
            v.push((
                format!(
                    "(light)?{}?portable{}({})",
                    SEQ_RE,
                    SEQ_RE,
                    arch_list.join("|")
                ),
                5,
            ));
        }
    }

    if os_list.is_empty() || arch_list.is_empty() {
        return v;
    }
    let os_re = format!("({})", os_list.join("|"));
    let arch_re = format!("({})", arch_list.join("|"));

    if let Some(abi) = abi {
        v.push((format!("{}-{}-{}$", os_re, arch_re, abi), 15));
        v.push((format!("{}-{}-{}$", arch_re, os_re, abi), 15));
        v.push((format!("{}-{}-{}$", os_re, abi, arch_re,), 15));

        v.push((format!("{}-{}-{}", os_re, arch_re, abi), 10));
        v.push((format!("{}-{}-{}", arch_re, os_re, abi), 10));
        v.push((format!("{}-{}-{}", os_re, abi, arch_re,), 10));
    }

    v.push((format!("{}-{}$", os_re, arch_re), 7));
    v.push((format!("{}-{}$", arch_re, os_re), 7));

    v.push((format!("{}-{}", os_re, arch_re), 5));
    v.push((format!("{}-{}", arch_re, os_re), 5));
    v.push((os_re, 2));
    v.push((arch_re, 1));
    v
}

// Cached rules - built once and reused
static RULES: Lazy<Vec<Rule>> = Lazy::new(build_rules);

fn build_rules() -> Vec<Rule> {
    let mut v = Vec::with_capacity(400);

    // Optimize: single iteration to categorize targets by dash count
    let (target3, target2, target1): (Vec<_>, Vec<_>, Vec<_>) = Target::iter().fold(
        (vec![], vec![], vec![]),
        |(mut t3, mut t2, mut t1), target| {
            match target.to_str().matches('-').count() {
                3 => t3.push(target),
                2 => t2.push(target),
                1 => t1.push(target),
                _ => {}
            }
            (t3, t2, t1)
        },
    );

    for (t, rank) in [(target3, 30), (target2, 25), (target1, 20)] {
        let s = t
            .iter()
            .map(|i| i.to_str().replace("-", SEQ_RE))
            .collect::<Vec<_>>()
            .join("|");
        let re = format!(r"^{}{}(?<target>{})\b", NAME_RE, SEQ_RE, s);
        v.push(Rule {
            re: build_re(&re),
            target: vec![],
            rank,
        });
    }

    let mut re_map = HashMap::with_capacity(400);

    for target in Target::iter() {
        for (common_target, rank) in get_common_targets(&target) {
            let re = format!(
                r"^{}{}{}\b",
                NAME_RE,
                SEQ_RE,
                common_target.replace("-", SEQ_RE)
            );
            re_map.entry((re, rank)).or_insert(vec![]).push(target);
        }
    }

    for ((re, rank), target) in re_map {
        v.push(Rule {
            re: build_re(&re),
            target,
            rank,
        });
    }

    v.sort_by(|a, b| match b.rank.cmp(&a.rank) {
        std::cmp::Ordering::Equal => b.re.as_str().len().cmp(&a.re.as_str().len()),
        cmp => cmp,
    });
    v
}

fn guess_git(s: &str) -> (Option<Cow<'_, str>>, Cow<'_, str>) {
    if let Some(caps) = GIT_REGEX.captures(s)
        && let Some(git) = caps.name("git").map(|i| i.as_str())
    {
        let cleaned = strip_token(s, git, false);
        return (Some(std::borrow::Cow::Borrowed(git)), cleaned);
    }
    (None, std::borrow::Cow::Borrowed(s))
}

fn guess_version(s: &str) -> (Option<Cow<'_, str>>, Cow<'_, str>) {
    for re in VERSION_REGEXES.iter() {
        if let Some(caps) = re.captures(s)
            && let Some(version) = caps.name("version").map(|i| i.as_str())
        {
            // skip arch
            if version == "x86_64" {
                continue;
            }
            let cleaned = strip_token(s, version, true);
            return (Some(std::borrow::Cow::Borrowed(version)), cleaned);
        }
    }
    (None, std::borrow::Cow::Borrowed(s))
}

#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen(js_name = guessTarget))]
pub fn guess_target(s: &str) -> Vec<GuessTarget> {
    let rules = &*RULES; // Use cached rules
    let mut v = Vec::with_capacity(10);
    let mut last_rank: u32 = 0;

    let (version, cleaned) = guess_version(s);
    let (git, cleaned) = guess_git(&cleaned);

    // Pre-compute once instead of converting Cow -> String per result.
    let version_str: Option<String> = version.as_ref().map(|i| i.to_string());
    let git_str: Option<String> = git.as_ref().map(|i| i.to_string());

    for rule in rules {
        // Early exit: if we have results and current rule priority is significantly lower
        if !v.is_empty() && last_rank > 0 && rule.rank + EARLY_EXIT_GAP < last_rank {
            break;
        }

        if last_rank > rule.rank {
            return v;
        }

        if let Some(cap) = rule.re.captures(&cleaned) {
            // Safe capture group access with default value
            let name = cap.name("name").map(|m| m.as_str()).unwrap_or("");

            let mut targets = rule.target.clone();
            if let Some(t) = cap
                .name("target")
                .and_then(|i| Target::from_str(i.as_str()).ok())
            {
                targets.push(t);
            }
            for target in targets {
                v.push(GuessTarget {
                    name: name.to_string(),
                    target,
                    version: version_str.clone(),
                    git: git_str.clone(),
                    rank: rule.rank,
                });
            }
            last_rank = rule.rank;
        }
    }
    v
}

/// * `"linux"`
/// * `"windows"`
/// * `"macos"`
/// * `"android"`
/// * `"ios"`
/// * `"openbsd"`
/// * `"freebsd"`
/// * `"netbsd"`
/// * `"wasi"`
/// * `"hermit"`
/// * `"aix"`
/// * `"apple"`
/// * `"dragonfly"`
/// * `"emscripten"`
/// * `"espidf"`
/// * `"fortanix"`
/// * `"uefi"`
/// * `"fuchsia"`
/// * `"haiku"`
/// * `"hermit"`
/// * `"watchos"`
/// * `"visionos"`
/// * `"tvos"`
/// * `"horizon"`
/// * `"hurd"`
/// * `"illumos"`
/// * `"l4re"`
/// * `"nto"`
/// * `"redox"`
/// * `"solaris"`
/// * `"solid_asp3`
/// * `"vita"`
/// * `"vxworks"`
/// * `"xous"`
pub const fn get_local_os() -> Os {
    if cfg!(target_os = "macos") {
        Os::Darwin
    } else if cfg!(target_os = "linux") {
        Os::Linux
    } else if cfg!(target_os = "windows") {
        Os::Windows
    } else if cfg!(target_os = "freebsd") {
        Os::Freebsd
    } else if cfg!(target_os = "netbsd") {
        Os::Netbsd
    } else if cfg!(target_os = "illumos") {
        Os::Illumos
    } else if cfg!(target_os = "ios") {
        Os::Ios
    } else if cfg!(target_os = "android") {
        Os::Android
    } else if cfg!(target_os = "fuchsia") {
        Os::Fuchsia
    } else if cfg!(target_os = "redox") {
        Os::Redox
    } else if cfg!(target_os = "solaris") {
        Os::Solaris
    } else if cfg!(target_os = "emscripten") {
        Os::Emscripten
    } else if cfg!(target_os = "wasi") {
        Os::Wasip1
    } else if cfg!(target_os = "none") {
        Os::None
    } else if cfg!(target_os = "uefi") {
        Os::Uefi
    } else {
        Os::Unknown
    }
}

/// * `"x86"`
/// * `"x86_64"`
/// * `"arm"`
/// * `"aarch64"`
/// * `"m68k"`
/// * `"mips"`
/// * `"mips32r6"`
/// * `"mips64"`
/// * `"mips64r6"`
/// * `"csky"`
/// * `"powerpc"`
/// * `"powerpc64"`
/// * `"riscv32"`
/// * `"riscv64"`
/// * `"s390x"`
/// * `"sparc"`
/// * `"sparc64"`
/// * `"hexagon"`
/// * `"loongarch64"`
pub const fn get_local_arch() -> Arch {
    if cfg!(target_arch = "x86") {
        Arch::I686
    } else if cfg!(target_arch = "riscv64") {
        Arch::Riscv64gc
    } else if cfg!(target_arch = "aarch64") {
        Arch::Aarch64
    } else if cfg!(target_arch = "x86_64") {
        Arch::X86_64
    } else if cfg!(target_arch = "arm") {
        Arch::Arm
    } else if cfg!(target_arch = "loongarch64") {
        Arch::Loongarch64
    } else if cfg!(target_arch = "powerpc") {
        Arch::Powerpc
    } else if cfg!(target_arch = "powerpc64") {
        if cfg!(target_endian = "little") {
            Arch::Powerpc64le
        } else {
            Arch::Powerpc64
        }
    } else if cfg!(target_arch = "s390x") {
        Arch::S390x
    } else if cfg!(target_arch = "wasm32") {
        Arch::Wasm32
    } else {
        Arch::X86_64
    }
}

/// Deprecated alias for [`get_local_arch`] (typo fix, kept for compatibility).
#[deprecated(note = "renamed to `get_local_arch`")]
pub fn get_loacal_arch() -> Arch {
    get_local_arch()
}

static IS_MSYS: Lazy<bool> = Lazy::new(detect_msys);

fn detect_msys() -> bool {
    if std::env::var("MSYSTEM").is_ok() {
        return true;
    }
    // `uname` only exists in MSYS/Cygwin/Unix; short-circuit elsewhere.
    if !cfg!(target_os = "windows") {
        return false;
    }
    Command::new("uname")
        .arg("-o")
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|s| {
            let s = s.to_lowercase();
            s.contains("msys") || s.contains("mingw")
        })
        .unwrap_or(false)
}

fn is_msys() -> bool {
    *IS_MSYS
}

pub fn get_local_abi() -> Vec<Abi> {
    if is_musl() {
        return vec![Abi::Musl];
    };

    if cfg!(windows) {
        if is_msys() {
            return vec![Abi::Msvc, Abi::Gnu];
        }
        return vec![Abi::Msvc];
    }

    if cfg!(target_os = "linux") {
        return vec![Abi::Gnu];
    }
    vec![]
}

/// The exact target triple the current binary was compiled for, resolved at
/// compile time from `cfg!(target_arch/target_os/target_env)` and cached in a
/// `Lazy` slice — zero allocation after the first call.
///
/// This is *not* a guess: it reflects the toolchain that produced this binary.
/// For runtime probing of the host (e.g. detecting musl/msys from a
/// cross-compiled binary), use [`guess_local_target`].
pub fn get_local_target() -> &'static [Target] {
    &LOCAL_TARGET
}

static LOCAL_TARGET: Lazy<Vec<Target>> = Lazy::new(|| {
    let arch = get_local_arch();
    let os = get_local_os();
    // The `target_env` values below cover both common (gnu/musl/msvc/...) and
    // rare cross-compile targets (gnueabi/gnullvm/softfloat/...). The latter
    // are only recognized when actually compiling for those targets, so we
    // silence the `unexpected_cfgs` lint that fires on an unrelated host.
    #[allow(unexpected_cfgs)]
    let abi = if cfg!(target_env = "musl") {
        Some(Abi::Musl)
    } else if cfg!(target_env = "gnu") {
        Some(Abi::Gnu)
    } else if cfg!(target_env = "msvc") {
        Some(Abi::Msvc)
    } else if cfg!(target_env = "gnueabi") {
        Some(Abi::Gnueabi)
    } else if cfg!(target_env = "gnueabihf") {
        Some(Abi::Gnueabihf)
    } else if cfg!(target_env = "musleabi") {
        Some(Abi::Musleabi)
    } else if cfg!(target_env = "musleabihf") {
        Some(Abi::Musleabihf)
    } else if cfg!(target_env = "gnullvm") {
        Some(Abi::Gnullvm)
    } else if cfg!(target_env = "ohos") {
        Some(Abi::Ohos)
    } else if cfg!(target_env = "sgx") {
        Some(Abi::Sgx)
    } else if cfg!(target_env = "gnux32") {
        Some(Abi::Gnux32)
    } else if cfg!(target_env = "softfloat") {
        Some(Abi::Softfloat)
    } else if cfg!(target_env = "elf") {
        Some(Abi::Elf)
    } else if cfg!(target_env = "macabi") {
        Some(Abi::Macabi)
    } else if cfg!(target_env = "sim") {
        Some(Abi::Sim)
    } else {
        None
    };

    Target::iter()
        .filter(|t| t.arch() == arch && t.os() == os && t.abi() == abi)
        .collect()
});

/// Runtime guess of the host target by probing the environment
/// (`is_musl`, `MSYSTEM`/`uname`). Prefer [`get_local_target`] when you want
/// the compile-time target triple of the running binary.
pub fn guess_local_target() -> Vec<Target> {
    let os = get_local_os();
    let arch = get_local_arch();
    let abi = get_local_abi();
    Target::iter()
        .filter(|i| {
            let fit_abi = match i.abi() {
                Some(a) => abi.contains(&a),
                None => true,
            };
            i.os() == os && i.arch() == arch && fit_abi
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::{build_rules, guess_version};
    use crate::{Target, core::guess_git, guess_target};
    use strum::IntoEnumIterator;

    #[test]
    fn test_get_rules() {
        let rules = build_rules();
        assert!(!rules.is_empty());
    }
    #[test]
    fn test_guess_target() {
        let md = include_str!("../README.md");
        let table_start = md.find("## test").unwrap() + "## test".len();
        let table_end = md.find("## Platform Support").unwrap();
        let table = md[table_start..table_end].trim();
        let lines = table.lines().skip(2);

        for line in lines {
            println!("line {}", line);
            let [filename, name, targets, version, git] = line[1..line.len() - 1]
                .split("|")
                .map(str::trim)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap_or(["", "", "", "", ""]);

            let ret = guess_target(filename);

            for i in &ret {
                assert_eq!(i.name, name);
                assert_eq!(i.version.clone().unwrap_or("".to_string()), version);
                assert_eq!(i.git.clone().unwrap_or("".to_string()), git);
            }
            let s = ret.iter().map(|i| i.target.to_str()).collect::<Vec<_>>();
            let targets: Vec<_> = targets.split(",").collect();
            for i in s {
                assert!(targets.contains(&i));
            }
        }
    }

    #[test]
    fn test_guess_version() {
        for (a, b, c) in [
            (
                "ScreenToGif.2.41.1.Light.Portable.x64",
                Some("2.41.1"),
                "ScreenToGif.Light.Portable.x64",
            ),
            (
                "ryujinx-1.2.82-macos_universal",
                Some("1.2.82"),
                "ryujinx-macos_universal",
            ),
            (
                "deno-x86_64-pc-windows-msvc",
                None,
                "deno-x86_64-pc-windows-msvc",
            ),
            (
                "ffmpeg-n7.1-latest-win64-gpl-7.1",
                Some("n7.1"),
                "ffmpeg-win64-gpl-7.1",
            ),
            (
                "ScreenToGif.2.41.1.Portable.x64",
                Some("2.41.1"),
                "ScreenToGif.Portable.x64",
            ),
            (
                "GitHub CLI 2.67.0 linux arm64",
                Some("2.67.0"),
                "GitHub CLI linux arm64",
            ),
            (
                "mise-v2025.2.8-macos-arm64",
                Some("v2025.2.8"),
                "mise-macos-arm64",
            ),
            (
                "nu-0.102.0-armv7-unknown-linux-gnueabihf",
                Some("0.102.0"),
                "nu-armv7-unknown-linux-gnueabihf",
            ),
            (
                "mpv-x86_64-v3-20250221-git-e32beaa",
                Some("20250221"),
                "mpv-x86_64-v3-git-e32beaa",
            ),
            ("gh_2.67.0_linux_arm64", Some("2.67.0"), "gh_linux_arm64"),
        ] {
            let (version, cleaned) = guess_version(a);
            println!("{:?}", version);
            assert_eq!(c, cleaned);
            assert_eq!(
                b.unwrap_or_default().to_string(),
                version.unwrap_or_default().to_string(),
            );
        }
    }

    #[test]
    fn test_guess_git() {
        for (a, b, c) in [
            (
                "ScreenToGif.2.41.1.Light.Portable.x64",
                None,
                "ScreenToGif.2.41.1.Light.Portable.x64",
            ),
            (
                "ryujinx-1.2.82-macos_universal",
                None,
                "ryujinx-1.2.82-macos_universal",
            ),
            (
                "deno-x86_64-pc-windows-msvc",
                None,
                "deno-x86_64-pc-windows-msvc",
            ),
            (
                "mpv-x86_64-v3-20250221-git-e32beaa",
                Some("git-e32beaa"),
                "mpv-x86_64-v3-20250221-",
            ),
        ] {
            let (git, cleaned) = guess_git(a);
            assert_eq!(c, cleaned);
            assert_eq!(
                b.unwrap_or_default().to_string(),
                git.unwrap_or_default().to_string()
            );
        }
    }

    #[test]
    fn test_default() {
        let name = "guess-target";
        for (s, t) in Target::iter().map(|t| (format!("{name}-{t}",), t)) {
            let guess = guess_target(&s);
            for k in guess {
                assert_eq!(k.name, name);
                assert_eq!(k.target, t);
            }
        }
    }
}
