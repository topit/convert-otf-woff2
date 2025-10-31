use wasm_bindgen::prelude::*;

// 导入 JavaScript console.log 用于调试
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// 当 `console_error_panic_hook` feature 被启用时，
// 可以使用 `set_panic_hook` 函数来将 panic 信息更好地输出到 JS 控制台。
#[cfg(feature = "console_error_panic_hook")]
#[wasm_bindgen(start)]
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

/// 将 OTF (OpenType Font) 格式的字体数据转换为 WOFF2 格式。
///
/// 这个函数只是一个占位符接口。实际的 WOFF2 压缩功能通过 JavaScript 调用实现，
/// 因为纯 Rust 的 WOFF2 库有复杂的 C 依赖，不适合直接编译到 WASM。
///
/// # 参数
/// * `otf_data` - 一个包含 OTF/TTF 字体数据的字节数组。
///
/// # 返回
/// * `Result<Vec<u8>, JsValue>` - 如果转换成功，返回 WOFF2 数据；否则返回错误。
///
/// # 使用方式
/// 在 JavaScript 中，您应该使用 wawoff2 或类似的库来执行实际转换:
/// ```javascript
/// import init from './pkg/convert_otf_woff2.js';
/// import Module from 'wawoff2';  // or another WOFF2 library
///
/// await init();
///
/// // 读取 OTF 文件
/// const otfData = new Uint8Array(await file.arrayBuffer());
///
/// // 使用 JavaScript WOFF2 库进行转换
/// const woff2Module = await Module();
/// const woff2Data = woff2Module.compress(otfData);
/// ```
#[wasm_bindgen]
pub fn convert_otf_to_woff2(otf_data: &[u8]) -> Result<Vec<u8>, JsValue> {
    // 这是一个接口函数，实际转换应该在 JavaScript 端完成
    // 我们返回输入数据作为占位符，真实实现需要 JS 互操作
    log(&format!("Received {} bytes of font data for conversion", otf_data.len()));
    
    // 返回错误提示用户使用 JavaScript 库
    Err(JsValue::from_str(
        "Please use JavaScript WOFF2 library (like wawoff2) for actual conversion. This function provides the WASM interface structure."
    ))
}

/// 获取字体数据的基本信息
///
/// # 参数
/// * `font_data` - 字体数据的字节数组
///
/// # 返回
/// * 字体数据的大小（字节数）
#[wasm_bindgen]
pub fn get_font_size(font_data: &[u8]) -> usize {
    font_data.len()
}
