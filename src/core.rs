use std::{borrow::Cow, str::FromStr};

use crate::{
    Abi,
    target::{Arch, Os, TARGET_LIST, Target},
};
use is_musl::is_musl;
use regex::{Regex, RegexBuilder};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[derive(Debug, PartialEq, Clone)]
pub struct GuessTarget {
    #[cfg_attr(feature = "wasm", wasm_bindgen(skip))]
    pub name: String,
    pub target: Target,
    #[cfg_attr(feature = "wasm", wasm_bindgen(skip))]
    pub version: Option<String>,
    #[cfg_attr(feature = "wasm", wasm_bindgen(skip))]
    pub git: Option<String>,
}

#[derive(Debug, Clone)]
struct Rule {
    re: Regex,
    target: Target,
    rank: u32,
}

const SEQ_RE: &str = r"[_ -\.]";
const NAME_RE: &str = r"(?P<name>[^/.]+)";

fn build_re(s: &str) -> Regex {
    RegexBuilder::new(s).case_insensitive(true).build().unwrap()
}

pub fn get_common_targets(target: &Target) -> Vec<(String, u32)> {
    let os = target.os();
    let arch = target.arch();
    let abi = target.abi();

    let mut os_list = match os {
        Os::Linux => vec!["linux"],
        Os::Darwin => vec!["darwin", "macos", "mac", "mac64"],
        Os::Windows => vec!["windows", "win32", "win"],
        Os::Freebsd => vec!["freebsd"],
        Os::Netbsd => vec!["netbsd"],
        _ => vec![],
    };
    let mut arch_list = match arch {
        Arch::X86_64 => vec!["x86_64", "amd64", "x64", "x86", "386", "i686", "legacy"],
        Arch::I686 => vec!["386", "i686", "x86"],
        Arch::Aarch64 => vec!["aarch64", "arm64", "armv7"],
        Arch::Arm => vec!["arm"],
        Arch::S390x => vec!["s390x"],
        Arch::Powerpc => vec!["powerpc"],
        Arch::Powerpc64 => vec!["powerpc64"],
        Arch::Riscv64gc => vec!["riscv64"],
        _ => vec![],
    };
    let mut v = vec![];

    if os == Os::Darwin && arch == Arch::Aarch64 {
        os_list.push("mac64arm");
    }
    if os == Os::Darwin {
        arch_list.push("universal");
    }
    if os == Os::Linux && arch == Arch::Aarch64 {
        os_list.push("lin64");
    }

    if os == Os::Windows && arch == Arch::X86_64 {
        os_list.push("win64");
        arch_list.push("x86-64-v3");
        v.push(("x86-64-v3".to_string(), 5));
        v.push(("portable".to_string(), 5));
    }

    if os_list.is_empty() || arch_list.is_empty() {
        return v;
    }
    let os_re = format!("({})", os_list.join("|"));
    let arch_re = format!("({})", arch_list.join("|"));

    if let Some(abi) = abi {
        v.push((format!("{}-{}-{}", os_re, arch_re, abi), 10));
        v.push((format!("{}-{}-{}", os_re, abi, arch_re,), 10));
    }

    v.push((format!("{}-{}", os_re, arch_re), 5));
    v.push((os_re, 2));
    v.push((arch_re, 1));
    v
}

fn get_rules() -> Vec<Rule> {
    let mut v = vec![];
    for target in TARGET_LIST {
        let s = target.to_str().replace("-", SEQ_RE);
        let rank = 30;
        // name-target
        let re = format!(r"{}{}{}\b", NAME_RE, SEQ_RE, s);
        v.push(Rule {
            re: build_re(&re),
            target,
            rank,
        });

        for (common_target, rank) in get_common_targets(&target) {
            let re = format!(
                r"{}{}{}\b",
                NAME_RE,
                SEQ_RE,
                common_target.replace("-", SEQ_RE)
            );
            v.push(Rule {
                re: build_re(&re),
                target,
                rank,
            });
        }
    }
    v.sort_by(|a, b| b.rank.cmp(&a.rank));
    v
}

const GIT_RE: &str = r"(?P<git>git[-_ ][0-9a-fA-F-]{7,})\b";

fn guess_git(s: &str) -> (Option<Cow<'_, str>>, Cow<'_, str>) {
    {
        let re = GIT_RE;
        let re = build_re(re);
        if let Some(caps) = re.captures(s) {
            if let Some(version) = caps.name("git").map(|i| i.as_str()) {
                let clean_re = build_re(&format!("{}[-_\\. ]?", version));
                let cleaned = clean_re.replace(s, "");
                println!("cleand {} {}", version, cleaned);
                return (Some(std::borrow::Cow::Borrowed(version)), cleaned);
            }
        }
    }
    (None, std::borrow::Cow::Borrowed(s))
}

fn guess_version(s: &str) -> (Option<Cow<'_, str>>, Cow<'_, str>) {
    for re in [
        // 20250225
        r#"(?P<version>[a-zA-Z]?\d{7,})"#,
        // 1.2.3.4
        r#"(?P<version>[a-zA-Z]?\d{1,4}(?:[\._-]\d{1,4}){3})"#,
        // 1.2.3
        r#"(?P<version>[a-zA-Z]?\d{1,4}(?:[\._-]\d{1,4}){2})"#,
        // 1.2
        r#"(?P<version>[a-zA-Z]?\d{1,4}[\._-]\d{1,4})"#,
    ] {
        let re = build_re(re);
        if let Some(caps) = re.captures(s) {
            if let Some(version) = caps.name("version").map(|i| i.as_str()) {
                // skip arch
                if ["x86_64"].contains(&version) {
                    continue;
                }
                let clean_re = build_re(&format!(
                    "{}[-_\\. ]?(latest|alpha|beta|master)?[-_\\. ]?",
                    version
                ));
                let cleaned = clean_re.replace(s, "");
                return (Some(std::borrow::Cow::Borrowed(version)), cleaned);
            }
        }
    }
    (None, std::borrow::Cow::Borrowed(s))
}

#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen(js_name = guessTarget))]
pub fn guess_target(s: &str) -> Vec<GuessTarget> {
    let rules = get_rules();
    let mut v = vec![];
    let mut last_rank = 0;

    let (version, cleaned) = guess_version(s);
    let (git, cleaned) = guess_git(&cleaned);

    for rule in &rules {
        if last_rank > rule.rank {
            return v;
        }
        if let Some(cap) = rule.re.captures(&cleaned) {
            let name = &cap["name"];
            v.push(GuessTarget {
                name: name.to_string(),
                target: rule.target,
                version: version.clone().map(|i| i.to_string()),
                git: git.clone().map(|i| i.to_string()),
            });
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
pub fn get_local_os() -> Os {
    match std::env::consts::OS {
        "macos" => Os::Darwin,
        s => Os::from_str(s).expect("failed to get_local_os"),
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
pub fn get_loacal_arch() -> Arch {
    match std::env::consts::ARCH {
        "x86" => Arch::I686,
        "riscv64" => Arch::Riscv64gc,
        s => Arch::from_str(s).expect("failed to get_loacal_arch"),
    }
}

pub fn get_local_abi() -> Vec<Abi> {
    if is_musl() {
        return vec![Abi::Musl];
    };

    if cfg!(windows) {
        return vec![Abi::Msvc, Abi::Gnu];
    }

    if cfg!(target_os = "linux") {
        return vec![Abi::Gnu];
    }
    vec![]
}

pub fn get_local_target() -> Vec<Target> {
    let mut v = vec![];
    let os = get_local_os();
    let arch = get_loacal_arch();
    let abi = get_local_abi();
    for i in TARGET_LIST {
        let target_abi = i.abi();
        let fit_abi = match target_abi {
            Some(a) => abi.contains(&a),
            None => true,
        };

        if i.os() == os && i.arch() == arch && fit_abi {
            v.push(i);
        }
    }

    v
}

#[cfg(test)]
mod test {
    use crate::{core::guess_git, guess_target};
    use super::{get_rules, guess_version};

    #[test]
    fn test_get_rules() {
        let rules = get_rules();
        println!("{}", rules.len());
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
            let [filename, name, targets, version] = line[1..line.len() - 1]
                .split("|")
                .map(str::trim)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap_or(["", "", "", ""]);

            let ret = guess_target(filename);

            for i in &ret {
                assert_eq!(i.name, name);
                assert_eq!(i.version.clone().unwrap_or("".to_string()), version);
            }

            let s = ret
                .iter()
                .map(|i| i.target.to_str())
                .collect::<Vec<_>>()
                .join(",");

            assert_eq!(s, targets);
        }
    }

    #[test]
    fn test_guess_version() {
        for (a, b, c) in [
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
}
