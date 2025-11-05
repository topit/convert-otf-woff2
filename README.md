# convert-otf-woff2

一个使用 Rust 和 WebAssembly (WASI) 将 OTF/TTF 字体转换为 WOFF2 格式的工具。

A tool built with Rust and WebAssembly (WASI) to convert OTF/TTF fonts to WOFF2 format.

## 功能特点 Features

- ✨ 使用 Google woff2 库的完整 WOFF2 实现（通过 Rust `woff` crate）
- 🚀 WASI 技术栈，支持 C/C++ 依赖
- 🌐 通过 WASI 运行时在浏览器中运行
- 📦 真正的 WOFF2 转换，符合完整规范
- 🔒 客户端转换，保护隐私
- ⚡ 与 cn-font-split 相同的技术方案

## 技术架构 Architecture

本项目使用 **WASI (WebAssembly System Interface)** 技术，这是参考 [cn-font-split](https://github.com/KonghaYao/cn-font-split) 的实现方案。

**为什么使用 WASI?**

纯 WebAssembly (wasm32-unknown-unknown) 无法使用 C/C++ 依赖，而 WOFF2 编码需要：
- Google 的 woff2 C++ 库（业界标准）
- Brotli 压缩库（C 实现）
- 标准库和文件系统支持

WASI 解决方案：
1. 编译目标：`wasm32-wasip1` (带系统接口的 WASM)
2. 使用 `woff` crate (v0.3.4) - 封装了 Google woff2 库
3. WASI SDK 用于编译 C/C++ 代码
4. 浏览器端使用 WASI 运行时 (@tybys/wasm-util)
5. 虚拟文件系统 (memfs-browser)

**技术栈：**

- **Rust**: 核心逻辑和系统调用
- **woff crate**: Google woff2 库的 Rust 绑定
- **WASI SDK**: 编译 C/C++ 依赖
- **@tybys/wasm-util**: 浏览器端 WASI 运行时
- **memfs-browser**: 虚拟文件系统

## 快速开始 Quick Start

### 前置要求 Prerequisites

- Rust (>= 1.90.0)
- WASI SDK 24.0+
- Node.js (用于本地开发服务器)

### 构建 Build

```bash
# 克隆仓库
git clone https://github.com/topit/convert-otf-woff2.git
cd convert-otf-woff2

# 使用构建脚本（自动下载 WASI SDK）
./build-wasi.sh

# 或手动构建
rustup target add wasm32-wasip1

# 设置 WASI SDK 环境变量
export WASI_SDK_PATH=/path/to/wasi-sdk
export CC_wasm32_wasip1="${WASI_SDK_PATH}/bin/clang --sysroot=${WASI_SDK_PATH}/share/wasi-sysroot"
export CXX_wasm32_wasip1="${WASI_SDK_PATH}/bin/clang++ --sysroot=${WASI_SDK_PATH}/share/wasi-sysroot"
export AR_wasm32_wasip1="${WASI_SDK_PATH}/bin/llvm-ar"
export CARGO_TARGET_WASM32_WASIP1_RUSTFLAGS="-L ${WASI_SDK_PATH}/share/wasi-sysroot/lib/wasm32-wasip1 -l c++"

cargo build --target wasm32-wasip1 --release
```

### 运行示例 Run Example

```bash
# 启动本地服务器
python3 -m http.server 8000
# 或使用 Node.js
npx serve .

# 访问
# http://localhost:8000/index-wasi.html
```

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
        import { WASI } from 'https://cdn.jsdelivr.net/npm/@tybys/wasm-util@0.9.0/dist/+esm';
        import { Volume, createFsFromVolume } from 'https://cdn.jsdelivr.net/npm/memfs-browser@4.7.7/+esm';
        
        // Initialize virtual filesystem
        const fs = createFsFromVolume(new Volume());
        await fs.promises.mkdir('/tmp/fonts', { recursive: true });
        
        // Load WASM module
        const wasmBytes = await fetch('./target/wasm32-wasip1/release/convert_otf_woff2.wasm')
            .then(r => r.arrayBuffer());
        
        const wasi = new WASI({
            args: ['font_key'],
            env: {},
            preopens: { '/': '/' },
            fs: fs,
            print(text) { console.log(text); },
            printErr(text) { console.error(text); }
        });
        
        const { instance } = await WebAssembly.instantiate(wasmBytes, {
            wasi_snapshot_preview1: wasi.wasiImport
        });
        
        // Convert function
        document.getElementById('convert').addEventListener('click', async () => {
            const file = document.getElementById('fontFile').files[0];
            if (!file) return;
            
            const fontData = new Uint8Array(await file.arrayBuffer());
            const key = 'font_key';
            
            // Write input
            await fs.promises.writeFile(`/tmp/fonts/${key}`, fontData);
            await fs.promises.mkdir(`/tmp/${key}`, { recursive: true });
            
            // Run conversion
            await wasi.start(instance);
            
            // Read output
            const woff2Data = await fs.promises.readFile(`/tmp/${key}/font.woff2`);
            
            // Download
            const blob = new Blob([woff2Data], { type: 'font/woff2' });
            const url = URL.createObjectURL(blob);
            const a = document.createElement('a');
            a.href = url;
            a.download = file.name.replace(/\.(otf|ttf)$/i, '.woff2');
            a.click();
            URL.revokeObjectURL(url);
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

### WASI 方案说明

本项目采用 WASI (WebAssembly System Interface) 方案，这是参考 [cn-font-split](https://github.com/KonghaYao/cn-font-split) 的成熟实现。

**为什么不用纯 Rust?**

1. **WOFF2 编码库现状**：
   - Rust 生态中只有 WOFF2 **解码**库（`woff2`, `wuff` crates）
   - 没有纯 Rust 的 WOFF2 **编码**实现
   - 业界标准是 Google 的 woff2 C++ 库

2. **wasm32-unknown-unknown 的限制**：
   - 无标准库支持
   - 无法使用 C/C++ 依赖
   - 无文件系统访问

3. **WASI 的优势**：
   - 完整的标准库支持
   - 可以使用 C/C++ 依赖（通过 WASI SDK）
   - 提供文件系统等 POSIX 接口
   - 在浏览器中通过 WASI 运行时执行

### 构建产物

- **WASM 文件**: `target/wasm32-wasip1/release/convert_otf_woff2.wasm` (~871KB)
- **包含**: Google woff2 库 + Brotli 库 + Rust 代码
- **格式**: WASI 兼容的 WebAssembly

### 浏览器集成

使用两个关键库：

1. **@tybys/wasm-util**: 提供 WASI 运行时
   - 实现 WASI 系统调用
   - 管理进程和环境

2. **memfs-browser**: 提供虚拟文件系统
   - 在内存中模拟文件系统
   - 支持 WASI 的文件操作

### 与 cn-font-split 的对比

| 特性 | cn-font-split | 本项目 |
|------|--------------|--------|
| WASI 目标 | ✅ wasm32-wasip1 | ✅ wasm32-wasip1 |
| woff crate | ✅ v0.3.4 | ✅ v0.3.4 |
| WASI 运行时 | @tybys/wasm-util | @tybys/wasm-util |
| 虚拟文件系统 | memfs-browser | memfs-browser |
| 字体切割 | ✅ | ❌ (专注转换) |

### API 文档

#### WASI 模块接口

WASM 模块通过文件系统接口工作：

**输入**:
- 路径: `/tmp/fonts/{key}` - 输入字体文件
- 参数: `args[0]` - 文件标识符

**输出**:
- 路径: `/tmp/{key}/font.woff2` - 输出 WOFF2 文件

**标准错误输出**:
- 转换进度和统计信息通过 stderr 输出

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
