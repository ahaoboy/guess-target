use std::str::FromStr;

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
}

#[derive(Debug, Clone)]
struct Rule {
    re: Regex,
    target: Target,
    rank: u32,
}

const SEQ_RE: &str = "[_ -]";
const VERSION_RE: &str = "v?(?<version>\\d+\\.\\d+\\.\\d+|latest|beta|alpha)";
const BIN_RE: &str = "(?<name>[^/]+)";

fn build_re(s: &str) -> Regex {
    RegexBuilder::new(s)
        .case_insensitive(false)
        .build()
        .unwrap()
}

fn target_to_rules(target: &str, rank: u32) -> Vec<(String, u32)> {
    let target_str = target.replace("_", "-").replace("-", SEQ_RE);
    vec![
        //  name-version-target
        (
            format!(
                "^{}{}{}{}{}$",
                BIN_RE, SEQ_RE, VERSION_RE, SEQ_RE, target_str
            ),
            20 + rank,
        ),
        // name-target-version
        (
            format!(
                "^{}{}{}{}{}$",
                BIN_RE, target_str, VERSION_RE, SEQ_RE, SEQ_RE
            ),
            20 + rank,
        ),
        // name-target
        (format!("^{}{}{}$", BIN_RE, SEQ_RE, target_str), 10 + rank),
    ]
}

pub fn get_common_targets(target: &Target) -> Vec<(String, u32)> {
    let os = target.os();
    let arch = target.arch();
    let abi = target.abi();

    let mut os_list = match os {
        Os::Linux => vec!["linux", "lin64"],
        Os::Darwin => vec!["darwin", "macos", "mac", "mac64"],
        Os::Windows => vec!["windows", "win32", "win", "win64"],
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

    v.push((os_re, 1));
    v
}

fn get_rules() -> Vec<Rule> {
    let mut v = vec![];
    for target in TARGET_LIST {
        for (rule, rank) in target_to_rules(target.to_str(), 10).into_iter() {
            v.push(Rule {
                re: build_re(&rule),
                target,
                rank,
            });
        }

        for (common_target, rank) in get_common_targets(&target) {
            for (re, rank) in target_to_rules(&common_target, rank) {
                v.push(Rule {
                    re: build_re(&re),
                    target,
                    rank,
                });
            }
        }
    }
    v.sort_by(|a, b| b.rank.cmp(&a.rank));
    v
}

#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen(js_name = guessTarget))]
pub fn guess_target(s: &str) -> Vec<GuessTarget> {
    let rules = get_rules();
    let mut v = vec![];
    let mut last_rank = 0;

    for rule in &rules {
        if last_rank > rule.rank {
            return v;
        }
        if let Some(cap) = rule.re.captures(s) {
            let name = &cap["name"];
            let version = cap.name("version").map(|i| i.as_str().to_string());
            v.push(GuessTarget {
                name: name.to_string(),
                target: rule.target,
                version,
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
    use crate::guess_target;

    use super::get_rules;
    #[test]
    fn test_get_rules() {
        let rules = get_rules();
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
            let [filename, name, targets, version] = line[1..line.len() - 1]
                .split("|")
                .map(str::trim)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap_or(["", "", "", ""]);

            let ret = guess_target(filename);

            for i in &ret {
                println!("{} {:?}", filename, i);
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
}
