use wasm_bindgen::prelude::*;

// 当 `console_error_panic_hook` feature 被启用时，
// 可以使用 `set_panic_hook` 函数来将 panic 信息更好地输出到 JS 控制台。
// 这在调试时非常有用。
#[cfg(feature = "console_error_panic_hook")]
#[wasm_bindgen(start)]
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

fn read_binary_file(file_path: &str) -> std::io::Result<Vec<u8>> {
    use std::io::Read;
    let mut file = std::fs::File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

#[wasm_bindgen]
pub fn convert_otf_to_woff2() {
    let args: Vec<String> = std::env::args().collect();
    let target = "/tmp/fonts/".to_owned() + &args[0];
    let input = read_binary_file(&target).expect("Failed to read input file");

    let woff2_buffer = woff::version2::compress(&input, String::from(""), 1, true)
        .expect("Failed to compress subset");

    let path = "/tmp/".to_owned() + &args[0];
    let _ = std::fs::write(path, woff2_buffer).unwrap();
}

#[no_mangle]
pub fn _start() {
    convert_otf_to_woff2()
}
