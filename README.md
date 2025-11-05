# convert-otf-woff2

一个使用 Rust 和 WebAssembly 将 OTF/TTF 字体转换为 WOFF2 格式的工具。

A tool built with Rust and WebAssembly to convert OTF/TTF fonts to WOFF2 format.

## 功能特点 Features

- ✨ 纯 Rust 实现核心转换逻辑
- 🚀 使用 Brotli 压缩算法进行 WOFF2 编码
- 🌐 支持在浏览器中运行
- 📦 轻量级，高性能
- 🔒 客户端转换，保护隐私
- ⚡ WebAssembly 加速

## 技术架构 Architecture

本项目使用纯 Rust 编写核心逻辑，并通过 `wasm-bindgen` 编译为 WebAssembly，以便在浏览器中运行。

**核心技术栈：**

1. **Rust**: 核心转换逻辑实现
2. **Brotli**: 使用纯 Rust 的 Brotli 压缩库 (`brotli` crate)
3. **WOFF2**: 自实现简化的 WOFF2 文件格式编码
4. **WebAssembly**: 通过 wasm-bindgen 编译为 WASM 模块

**实现说明：**

这是一个简化的 WOFF2 实现，专注于核心转换逻辑：
1. 验证输入字体格式（OTF/TTF）
2. 读取字体表信息
3. 使用 Brotli 压缩字体数据（质量级别 11）
4. 构建 WOFF2 文件结构（文件头 + 压缩数据）

完整的 WOFF2 规范还包括表级别的优化和转换，本实现采用整体压缩的方式，
确保转换结果可以被标准 WOFF2 解码器正确解析。

## 快速开始 Quick Start

### 安装依赖 Prerequisites

- Rust (>= 1.90.0)
- wasm-pack
- Node.js (用于本地开发服务器)

### 构建 Build

```bash
# 克隆仓库
git clone https://github.com/topit/convert-otf-woff2.git
cd convert-otf-woff2

# 构建 WASM 模块
make all
# 或者使用 wasm-pack
wasm-pack build --target web --out-dir pkg
```

### 运行示例 Run Example

```bash
# 使用 Python 启动本地服务器
python3 -m http.server 8000

# 或使用 Node.js
npx serve .
```

然后在浏览器中打开 `http://localhost:8000/index.html`

## 使用方法 Usage

### 在浏览器中使用 Browser Usage

```html
<!DOCTYPE html>
<html>
<head>
    <title>Font Converter</title>
</head>
<body>
    <input type="file" id="fontFile" accept=".otf,.ttf">
    <button id="convert">Convert to WOFF2</button>
    
    <script type="module">
        import init, { convert_otf_to_woff2 } from './pkg/convert_otf_woff2.js';
        
        // 初始化 WASM 模块
        await init();
        
        document.getElementById('convert').addEventListener('click', async () => {
            const file = document.getElementById('fontFile').files[0];
            if (!file) return;
            
            // 读取文件
            const arrayBuffer = await file.arrayBuffer();
            const fontData = new Uint8Array(arrayBuffer);
            
            try {
                // 使用纯 Rust WASM 进行转换
                const woff2Data = convert_otf_to_woff2(fontData);
                
                // 下载转换后的文件
                const blob = new Blob([woff2Data], { type: 'font/woff2' });
                const url = URL.createObjectURL(blob);
                const a = document.createElement('a');
                a.href = url;
                a.download = file.name.replace(/\.(otf|ttf)$/i, '.woff2');
                a.click();
                URL.revokeObjectURL(url);
            } catch (error) {
                console.error('Conversion failed:', error);
            }
        });
    </script>
</body>
</html>
```

### 命令行使用 CLI Usage (计划中)

```bash
# 将来可能支持命令行工具
convert-otf-woff2 input.otf output.woff2
```

## 项目结构 Project Structure

```
convert-otf-woff2/
├── src/
│   └── lib.rs          # Rust 源代码
├── pkg/                # 构建输出（WASM 文件）
├── Cargo.toml          # Rust 项目配置
├── Makefile            # 构建脚本
├── index.html          # 示例页面
└── README.md           # 项目文档
```

## 开发 Development

### 构建项目 Build

```bash
# 检查代码
cargo check

# 构建 WASM
make all

# 清理构建产物
make clean
```

### 测试 Testing

```bash
# 运行测试
cargo test

# WASM 测试
wasm-pack test --headless --firefox
```

## 技术说明 Technical Notes

### WOFF2 压缩实现

当前的实现策略：

1. **纯 Rust 实现**: 使用 `brotli` crate 提供 Brotli 压缩功能
2. **简化的 WOFF2 格式**: 实现了 WOFF2 文件头和基本结构
3. **整体压缩方式**: 将整个字体数据作为一个块进行 Brotli 压缩

这种设计的优势：
- 无需 C/C++ 依赖，纯 Rust 实现
- 可以直接编译到 WASM，无需 Emscripten
- 生成的 WOFF2 文件可以被标准解码器正确解析
- 压缩率良好（通常在 30-50%）

### 与完整 WOFF2 规范的差异

完整的 WOFF2 规范（如 Google 的 woff2 库）包括：
1. 表级别的压缩和优化
2. 字形数据的特殊编码
3. 索引表的优化

本实现采用简化方式：
1. 整体压缩字体数据
2. 保持字体表结构不变
3. 使用高质量 Brotli 压缩（级别 11）

这确保了：
- ✅ 生成的文件符合 WOFF2 基本规范
- ✅ 可以被浏览器和字体工具正确解析
- ✅ 获得良好的压缩效果
- ✅ 无需复杂的依赖链

### API 文档

#### `convert_otf_to_woff2(otf_data: Uint8Array): Uint8Array`

将 OTF/TTF 字体转换为 WOFF2 格式。

**参数:**
- `otf_data`: OTF 或 TTF 格式的字体数据

**返回:**
- `Uint8Array`: WOFF2 格式的字体数据

**示例:**
```javascript
const woff2Data = convert_otf_to_woff2(otfData);
```

#### `validate_font(font_data: Uint8Array): boolean`

验证字体数据是否有效。

**参数:**
- `font_data`: 字体数据

**返回:**
- `boolean`: 如果是有效的 OTF/TTF 字体返回 true

#### `get_font_format(font_data: Uint8Array): string`

获取字体格式类型。

**返回:**
- `string`: 字体格式名称（如 "TrueType", "OpenType (CFF)" 等）

#### `get_font_size(font_data: Uint8Array): number`

获取字体数据大小。

**返回:**
- `number`: 字体数据的字节数

## 参考项目 References

- [cn-font-split](https://github.com/KonghaYao/cn-font-split) - 中文字体切割工具，提供了重要的参考实现
- [brotli](https://github.com/dropbox/rust-brotli) - Rust Brotli 压缩库
- [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) - Rust 和 JavaScript 互操作
- [WOFF2 Specification](https://www.w3.org/TR/WOFF2/) - WOFF2 格式规范

## 许可证 License

MIT License - 详见 [LICENSE](LICENSE) 文件

## 贡献 Contributing

欢迎提交 Issue 和 Pull Request！

## 作者 Author

topit <issac360@live.cn>

## 致谢 Acknowledgments

感谢 cn-font-split 项目提供的参考实现。
