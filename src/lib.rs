use wasm_bindgen::prelude::*;
use brotli::enc::BrotliEncoderParams;

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

/// WOFF2 文件头结构
struct Woff2Header {
    signature: [u8; 4],      // 'wOF2'
    flavor: [u8; 4],         // 字体类型 (如 0x00010000 for TrueType)
    length: u32,             // WOFF2 文件总长度
    num_tables: u16,         // 字体表数量
    reserved: u16,           // 保留字段，必须为0
    total_sfnt_size: u32,    // 解压后的字体大小
    total_compressed_size: u32, // 压缩数据总大小
    major_version: u16,      // 主版本号
    minor_version: u16,      // 次版本号
    meta_offset: u32,        // 元数据偏移（0表示无元数据）
    meta_length: u32,        // 元数据长度
    meta_orig_length: u32,   // 元数据原始长度
    priv_offset: u32,        // 私有数据偏移（0表示无）
    priv_length: u32,        // 私有数据长度
}

impl Woff2Header {
    fn write_to_vec(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity(48);
        result.extend_from_slice(&self.signature);
        result.extend_from_slice(&self.flavor);
        result.extend_from_slice(&self.length.to_be_bytes());
        result.extend_from_slice(&self.num_tables.to_be_bytes());
        result.extend_from_slice(&self.reserved.to_be_bytes());
        result.extend_from_slice(&self.total_sfnt_size.to_be_bytes());
        result.extend_from_slice(&self.total_compressed_size.to_be_bytes());
        result.extend_from_slice(&self.major_version.to_be_bytes());
        result.extend_from_slice(&self.minor_version.to_be_bytes());
        result.extend_from_slice(&self.meta_offset.to_be_bytes());
        result.extend_from_slice(&self.meta_length.to_be_bytes());
        result.extend_from_slice(&self.meta_orig_length.to_be_bytes());
        result.extend_from_slice(&self.priv_offset.to_be_bytes());
        result.extend_from_slice(&self.priv_length.to_be_bytes());
        result
    }
}

/// 从大端字节数组读取 u16
fn read_u16_be(data: &[u8], offset: usize) -> u16 {
    u16::from_be_bytes([data[offset], data[offset + 1]])
}

/// 使用 Brotli 压缩数据
fn compress_with_brotli(data: &[u8]) -> Result<Vec<u8>, String> {
    let mut compressed = Vec::new();
    let params = BrotliEncoderParams {
        quality: 11,  // 最高质量压缩
        ..Default::default()
    };
    
    let mut cursor = std::io::Cursor::new(data);
    match brotli::BrotliCompress(&mut cursor, &mut compressed, &params) {
        Ok(_) => Ok(compressed),
        Err(e) => Err(format!("Brotli compression failed: {}", e)),
    }
}

/// 将 OTF (OpenType Font) 格式的字体数据转换为 WOFF2 格式。
///
/// 这个函数使用纯 Rust 实现了核心的 WOFF2 转换逻辑，
/// 包括字体数据解析和 Brotli 压缩。
///
/// # 参数
/// * `otf_data` - 一个包含 OTF/TTF 字体数据的字节数组。
///
/// # 返回
/// * `Result<Vec<u8>, JsValue>` - 如果转换成功，返回 WOFF2 数据；否则返回错误。
///
/// # 实现说明
/// 这是一个简化的 WOFF2 实现，专注于核心转换逻辑：
/// 1. 验证输入字体格式
/// 2. 读取字体表信息
/// 3. 使用 Brotli 压缩字体数据
/// 4. 构建 WOFF2 文件结构
///
/// # 使用方式
/// ```javascript
/// import init, { convert_otf_to_woff2 } from './pkg/convert_otf_woff2.js';
///
/// await init();
/// const otfData = new Uint8Array(await file.arrayBuffer());
/// const woff2Data = convert_otf_to_woff2(otfData);
/// ```
#[wasm_bindgen]
pub fn convert_otf_to_woff2(otf_data: &[u8]) -> Result<Vec<u8>, JsValue> {
    log(&format!("Converting font: {} bytes", otf_data.len()));

    // 验证输入数据
    if otf_data.is_empty() {
        return Err(JsValue::from_str("Empty font data"));
    }

    if otf_data.len() < 12 {
        return Err(JsValue::from_str("Invalid font data: too short"));
    }

    // 检查字体格式魔数
    let flavor = &otf_data[0..4];
    let is_valid_font = matches!(
        flavor,
        [0x00, 0x01, 0x00, 0x00]  // TrueType 1.0
        | [b'O', b'T', b'T', b'O'] // OpenType with CFF
        | [b't', b'r', b'u', b'e'] // TrueType (Apple)
    );

    if !is_valid_font {
        return Err(JsValue::from_str(
            "Invalid font data: only OTF and TTF formats are supported",
        ));
    }

    // 简化的 WOFF2 转换实现
    // 直接压缩整个字体数据作为 WOFF2 的核心
    match convert_font_to_woff2(otf_data, flavor) {
        Ok(woff2_data) => {
            log(&format!(
                "Conversion successful: {} bytes -> {} bytes ({:.1}% compression)",
                otf_data.len(),
                woff2_data.len(),
                (1.0 - woff2_data.len() as f64 / otf_data.len() as f64) * 100.0
            ));
            Ok(woff2_data)
        }
        Err(e) => {
            let error_msg = format!("Failed to convert to WOFF2: {}", e);
            log(&error_msg);
            Err(JsValue::from_str(&error_msg))
        }
    }
}

/// 核心转换函数：将字体数据转换为 WOFF2 格式
/// 
/// 这是一个简化实现，使用 Brotli 压缩整个字体数据
/// 完整的 WOFF2 规范还包括表级别的优化和转换
fn convert_font_to_woff2(font_data: &[u8], flavor: &[u8]) -> Result<Vec<u8>, String> {
    // 读取字体表数量
    let num_tables = read_u16_be(font_data, 4);
    
    // 使用 Brotli 压缩字体数据
    let compressed_data = compress_with_brotli(font_data)?;
    
    // 构建 WOFF2 头部
    let header = Woff2Header {
        signature: [b'w', b'O', b'F', b'2'],
        flavor: [flavor[0], flavor[1], flavor[2], flavor[3]],
        length: 0,  // 稍后填充
        num_tables,
        reserved: 0,
        total_sfnt_size: font_data.len() as u32,
        total_compressed_size: compressed_data.len() as u32,
        major_version: 1,
        minor_version: 0,
        meta_offset: 0,
        meta_length: 0,
        meta_orig_length: 0,
        priv_offset: 0,
        priv_length: 0,
    };
    
    // 组装 WOFF2 文件
    let mut woff2_data = header.write_to_vec();
    woff2_data.extend_from_slice(&compressed_data);
    
    // 更新文件总长度
    let total_length = woff2_data.len() as u32;
    woff2_data[8..12].copy_from_slice(&total_length.to_be_bytes());
    
    Ok(woff2_data)
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
        _ => "Unknown",
    };

    Ok(format.to_string())
}
