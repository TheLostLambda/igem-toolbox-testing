mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn count_bases(dna: String) -> String {
    let (mut a, mut c, mut g, mut t) = (0, 0, 0, 0);
    for base in dna.chars() {
        if base == 'A' {
            a += 1;
        }
        if base == 'C' {
            c += 1;
        }
        if base == 'G' {
            g += 1;
        }
        if base == 'T' {
            t += 1;
        }
    }
    format!("A: {a} C: {c} G: {g} T: {t}")
}
