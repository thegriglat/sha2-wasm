mod utils;

use wasm_bindgen::prelude::wasm_bindgen;

use sha2::{Digest, Sha256, Sha512};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn sha256(s: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(s);
    format!("{:x}",  hasher.finalize())
}

#[wasm_bindgen]
pub fn sha512(s: String) -> String {
    let mut hasher = Sha512::new();
    hasher.update(s);
    format!("{:x}", hasher.finalize())
}
