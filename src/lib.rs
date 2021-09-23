use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn fibonacci(num: i32) -> i32 {
    if num <= 1 {
        num
    } else {
        fibonacci(num - 1) + fibonacci(num - 2)
    }
}
