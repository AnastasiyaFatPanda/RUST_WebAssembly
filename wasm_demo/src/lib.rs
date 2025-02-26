use wasm_bindgen::prelude::*;

// Экспортируем функцию, которая будет доступна в JavaScript
#[wasm_bindgen]
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

#[wasm_bindgen]
pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}
