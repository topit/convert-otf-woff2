# convert-otf-woff2

一个使用 Rust 和 WebAssembly 将 OTF/TTF 字体转换为 WOFF2 格式的工具。

A tool built with Rust and WebAssembly to convert OTF/TTF fonts to WOFF2 format.

## 功能特点 Features

- ✨ 基于 Rust 和 WebAssembly 构建
- 🌐 支持在浏览器中运行
- 📦 轻量级，高性能
- 🔒 客户端转换，保护隐私

## 技术架构 Architecture

本项目使用 Rust 编写核心逻辑，并通过 `wasm-bindgen` 编译为 WebAssembly，以便在浏览器中运行。

由于 WOFF2 压缩需要使用 Google 的 woff2 库（C++ 实现），目前有两种实现方案：

1. **纯 JavaScript 方案（推荐用于浏览器）**: 使用 `wawoff2` 或类似的 JavaScript WOFF2 库
2. **本地编译方案**: 使用 Rust 的 `woff` crate（需要 C 编译器支持）

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
                // 注意: 当前版本需要配合 JavaScript WOFF2 库使用
                // 例如: wawoff2
                const woff2Data = await convertWithWawoff2(fontData);
                
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
        
        // 使用 wawoff2 或其他 JavaScript WOFF2 库进行实际转换
        async function convertWithWawoff2(fontData) {
            // 这里需要集成实际的 WOFF2 压缩库
            // 示例: 使用 wawoff2
            // const Module = await import('wawoff2');
            // const woff2Module = await Module.default();
            // return woff2Module.compress(fontData);
            throw new Error('Please integrate a WOFF2 compression library');
        }
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

目前的实现策略：

1. **WASM 模块**: 提供接口和数据处理
2. **JavaScript 互操作**: 实际的 WOFF2 压缩委托给 JavaScript 库（如 `wawoff2`）

这种设计的原因：
- WOFF2 的 Rust 实现 (`woff` crate) 依赖 C/C++ 代码
- 将 C/C++ 代码编译到 WASM 需要复杂的工具链（Emscripten）
- JavaScript 生态已有成熟的 WOFF2 库可用

### 可选的集成方案

如果需要纯 Rust/WASM 实现，可以考虑：

1. 使用 Emscripten 编译 `woff` crate 的 C 依赖
2. 使用 `allsorts` 等纯 Rust 字体库（如果支持 WOFF2 压缩）
3. 自行实现 WOFF2 压缩算法（工作量大）

## 参考项目 References

- [cn-font-split](https://github.com/KonghaYao/cn-font-split) - 中文字体切割工具
- [woff](https://github.com/bodoni/woff) - Rust WOFF 库
- [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) - Rust 和 JavaScript 互操作

## 许可证 License

MIT License - 详见 [LICENSE](LICENSE) 文件

## 贡献 Contributing

欢迎提交 Issue 和 Pull Request！

## 作者 Author

topit <issac360@live.cn>

## 致谢 Acknowledgments

感谢 cn-font-split 项目提供的参考实现。
