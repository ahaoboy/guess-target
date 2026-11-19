/// Forward Cargo's compile-time `TARGET` to the crate.
///
/// `TARGET` is only set for build scripts, so the library re-exports it as
/// [`guess_target::BUILD_TARGET`] for callers that need the exact target
/// triple this binary was compiled for (e.g. self-upgraders).
fn main() {
    let target = std::env::var("TARGET").expect("TARGET env var not set by Cargo");
    println!("cargo:rustc-env=BUILD_TARGET={target}");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=TARGET");
}
