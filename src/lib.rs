#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use wasm_bindgen::prelude::*;

// 当 `console_error_panic_hook` feature 被启用时，
// 可以使用 `set_panic_hook` 函数来将 panic 信息更好地输出到 JS 控制台。
// 这在调试时非常有用。
#[cfg(feature = "console_error_panic_hook")]
#[wasm_bindgen(start)]
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

/// 将 OTF (OpenType Font) 格式的字体数据转换为 WOFF2 格式。
///
/// # 参数
/// * `otf_data` - 一个包含 OTF 字体数据的字节切片 (`&[u8]`)。
///
/// # 返回
/// * `Ok(Vec<u8>)` - 如果转换成功，返回一个包含 WOFF2 数据的 `Vec<u8>`。
/// * `Err(String)` - 如果发生任何错误（例如，无效的输入数据或转换失败），
///   返回一个描述错误的字符串。
#[wasm_bindgen]
pub fn convert_otf_to_woff2(otf_data: &[u8]) -> Result<Vec<u8>, String> {
    let woff2_buffer = woff::version2::compress(otf_data, String::from(""), 1, true)
        .expect("Failed to compress subset");

    // 3. 返回结果
    // 如果转换成功，我们从 Cursor 中取出内部的 Vec<u8> 并返回。
    Ok(woff2_buffer)
}
