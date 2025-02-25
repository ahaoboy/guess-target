use crate::{Abi, Arch, GuessTarget, Os, Target};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
impl GuessTarget {
    #[wasm_bindgen(getter = name)]
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(getter = version)]
    pub fn get_version(&self) -> Option<String> {
        self.version.clone()
    }
    #[wasm_bindgen(getter = git)]
    pub fn get_git(&self) -> Option<String> {
        self.git.clone()
    }
}

#[wasm_bindgen(js_name = targetToString)]
pub fn target_to_string(target: Target) -> String {
    target.to_str().to_string()
}
#[wasm_bindgen(js_name = targetGetOs)]
pub fn target_get_os(target: Target) -> Os {
    target.os()
}
#[wasm_bindgen(js_name = targetGetArch)]
pub fn target_get_arch(target: Target) -> Arch {
    target.arch()
}
#[wasm_bindgen(js_name = targetGetAbi)]
pub fn target_get_abi(target: Target) -> Option<Abi> {
    target.abi()
}
