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
/// 这个函数作为 WASM 接口，实际的 WOFF2 压缩通过 JavaScript 库实现。
/// 对于更好的浏览器兼容性，推荐使用 Google 的 woff2 JavaScript 库。
///
/// # 参数
/// * `otf_data` - 一个包含 OTF/TTF 字体数据的字节数组。
///
/// # 返回
/// * `Result<Vec<u8>, JsValue>` - 如果转换成功，返回 WOFF2 数据；否则返回错误。
///
/// # 使用方式
/// 在 JavaScript 中，推荐使用以下方式实现完整转换:
///
/// ```javascript
/// // 方案1: 使用 fonttools wasm (推荐)
/// import init from './pkg/convert_otf_woff2.js';
/// import { compress } from 'wawoff2';  // npm install wawoff2
///
/// await init();
/// const otfData = new Uint8Array(await file.arrayBuffer());
/// const woff2Data = await compress(otfData);
///
/// // 方案2: 使用纯 JS 实现
/// // 参考 https://github.com/google/woff2
/// ```
#[wasm_bindgen]
pub fn convert_otf_to_woff2(otf_data: &[u8]) -> Result<Vec<u8>, JsValue> {
    log(&format!("Converting font: {} bytes", otf_data.len()));

    // 验证输入数据
    if otf_data.is_empty() {
        return Err(JsValue::from_str("Empty font data"));
    }

    // 简单验证是否为字体文件（检查魔数）
    if otf_data.len() < 4 {
        return Err(JsValue::from_str("Invalid font data: too short"));
    }

    // 检查常见字体格式的魔数
    let magic = &otf_data[0..4];
    let is_valid_font = match magic {
        [0x00, 0x01, 0x00, 0x00] => true, // TrueType 1.0
        [b'O', b'T', b'T', b'O'] => true, // OpenType with CFF
        [b't', b'r', b'u', b'e'] => true, // TrueType (Apple)
        [b't', b'y', b'p', b'1'] => true, // PostScript Type 1
        [b'w', b'O', b'F', b'F'] => true, // WOFF 1.0
        [b'w', b'O', b'F', b'2'] => true, // WOFF 2.0
        _ => false,
    };

    if !is_valid_font {
        return Err(JsValue::from_str("Invalid font data: unknown format"));
    }

    // 返回提示信息
    // 由于 WOFF2 压缩需要复杂的 C 库，在 WASM 环境中建议使用 JavaScript 库
    Err(JsValue::from_str(
        "WASM interface ready. For actual WOFF2 conversion, please use JavaScript libraries like 'wawoff2' or Google's woff2.js. Example: import { compress } from 'wawoff2'; const woff2Data = await compress(otfData);"
    ))
}

/// 验证字体数据是否有效
///
/// # 参数
/// * `font_data` - 字体数据的字节数组
///
/// # 返回
/// * `Result<bool, JsValue>` - 如果是有效字体返回 true
#[wasm_bindgen]
pub fn validate_font(font_data: &[u8]) -> Result<bool, JsValue> {
    if font_data.len() < 4 {
        return Ok(false);
    }

    let magic = &font_data[0..4];
    let is_valid = matches!(
        magic,
        [0x00, 0x01, 0x00, 0x00]
            | [b'O', b'T', b'T', b'O']
            | [b't', b'r', b'u', b'e']
            | [b't', b'y', b'p', b'1']
            | [b'w', b'O', b'F', b'F']
            | [b'w', b'O', b'F', b'2']
    );

    Ok(is_valid)
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

/// 获取字体格式类型
///
/// # 参数
/// * `font_data` - 字体数据的字节数组
///
/// # 返回
/// * `Result<String, JsValue>` - 字体格式名称
#[wasm_bindgen]
pub fn get_font_format(font_data: &[u8]) -> Result<String, JsValue> {
    if font_data.len() < 4 {
        return Err(JsValue::from_str("Font data too short"));
    }

    let magic = &font_data[0..4];
    let format = match magic {
        [0x00, 0x01, 0x00, 0x00] => "TrueType",
        [b'O', b'T', b'T', b'O'] => "OpenType (CFF)",
        [b't', b'r', b'u', b'e'] => "TrueType (Apple)",
        [b't', b'y', b'p', b'1'] => "PostScript Type 1",
        [b'w', b'O', b'F', b'F'] => "WOFF 1.0",
        [b'w', b'O', b'F', b'2'] => "WOFF 2.0",
        _ => "Unknown",
    };

    Ok(format.to_string())
}
