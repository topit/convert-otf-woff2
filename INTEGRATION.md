# 集成 WOFF2 压缩库指南

本项目提供了 WASM 接口，但实际的 WOFF2 压缩需要集成额外的库。以下是几种集成方案：

## 方案 1: 使用 wawoff2 (推荐)

wawoff2 是 Google woff2 的 WebAssembly 编译版本。

### 安装

```bash
npm install wawoff2
```

### 使用

```javascript
import init, { validate_font, get_font_format } from './pkg/convert_otf_woff2.js';
import { compress, decompress } from 'wawoff2';

// 初始化我们的 WASM 模块
await init();

// 读取字体文件
const fontFile = document.querySelector('input[type="file"]').files[0];
const arrayBuffer = await fontFile.arrayBuffer();
const otfData = new Uint8Array(arrayBuffer);

// 验证字体
const isValid = validate_font(otfData);
if (!isValid) {
    console.error('Invalid font file');
    return;
}

const format = get_font_format(otfData);
console.log(`Font format: ${format}`);

// 转换为 WOFF2
const woff2Data = await compress(otfData);

// 下载文件
const blob = new Blob([woff2Data], { type: 'font/woff2' });
const url = URL.createObjectURL(blob);
const a = document.createElement('a');
a.href = url;
a.download = fontFile.name.replace(/\.(otf|ttf)$/i, '.woff2');
a.click();
URL.revokeObjectURL(url);
```

## 方案 2: 使用 CDN

如果不想安装 npm 包，可以使用 CDN:

```html
<!DOCTYPE html>
<html>
<head>
    <title>Font Converter</title>
</head>
<body>
    <input type="file" id="fontFile" accept=".otf,.ttf">
    <button id="convert">Convert</button>

    <script type="module">
        import init, { validate_font, get_font_format } from './pkg/convert_otf_woff2.js';
        
        // 使用 CDN 加载 wawoff2
        const { compress } = await import('https://cdn.skypack.dev/wawoff2');
        
        await init();
        
        document.getElementById('convert').addEventListener('click', async () => {
            const file = document.getElementById('fontFile').files[0];
            if (!file) return;
            
            const arrayBuffer = await file.arrayBuffer();
            const fontData = new Uint8Array(arrayBuffer);
            
            if (validate_font(fontData)) {
                const format = get_font_format(fontData);
                console.log(`Converting ${format} font...`);
                
                const woff2Data = await compress(fontData);
                
                // 下载
                const blob = new Blob([woff2Data], { type: 'font/woff2' });
                const url = URL.createObjectURL(blob);
                const a = document.createElement('a');
                a.href = url;
                a.download = file.name.replace(/\.(otf|ttf)$/i, '.woff2');
                a.click();
                URL.revokeObjectURL(url);
                
                console.log('Conversion complete!');
            } else {
                alert('Invalid font file!');
            }
        });
    </script>
</body>
</html>
```

## 方案 3: 使用 fonttools Python (Node.js 环境)

如果在 Node.js 环境中，可以使用 Python 的 fonttools:

```javascript
const { execSync } = require('child_process');
const fs = require('fs');

function convertToWoff2(inputPath, outputPath) {
    // 确保安装了 fonttools: pip install fonttools brotli
    execSync(`fonttools ttLib.woff2 compress ${inputPath} -o ${outputPath}`);
}

// 使用
convertToWoff2('input.otf', 'output.woff2');
```

## 完整示例项目结构

```
convert-otf-woff2/
├── pkg/                    # WASM 编译输出
│   ├── convert_otf_woff2.js
│   ├── convert_otf_woff2_bg.wasm
│   └── ...
├── index.html              # 基础示例
├── example-wawoff2.html    # 完整集成示例
├── package.json
└── README.md
```

### package.json 示例

```json
{
  "name": "convert-otf-woff2-demo",
  "version": "1.0.0",
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "vite build"
  },
  "dependencies": {
    "wawoff2": "^2.0.1"
  },
  "devDependencies": {
    "vite": "^5.0.0"
  }
}
```

## 性能对比

| 方法 | 速度 | 体积 | 浏览器兼容性 |
|------|------|------|-------------|
| wawoff2 | 快 | ~50KB | 所有现代浏览器 |
| fonttools (Python) | 中等 | N/A | 仅 Node.js |
| 原生 C++ (通过 FFI) | 最快 | 最小 | 需要编译 |

## 注意事项

1. **CORS**: 如果从 CDN 加载字体，确保服务器配置了正确的 CORS 头
2. **文件大小**: WOFF2 通常比 OTF/TTF 小 30-50%
3. **浏览器兼容性**: WOFF2 在所有现代浏览器中都支持
4. **许可证**: 确保字体文件的许可证允许转换和分发

## 故障排除

### WASM 模块加载失败

确保:
- 已运行 `wasm-pack build --target web`
- pkg 目录存在且包含所有必需文件
- 使用 HTTP 服务器运行（不是 file:// 协议）

### wawoff2 导入失败

```bash
# 确保安装了 wawoff2
npm install wawoff2

# 或使用 CDN
# import { compress } from 'https://cdn.skypack.dev/wawoff2'
```

### 转换后的字体无法使用

- 检查原始字体文件是否损坏
- 使用 `validate_font()` 验证字体
- 确保 Content-Type 正确设置为 `font/woff2`

## 更多资源

- [wawoff2 GitHub](https://github.com/fontello/wawoff2)
- [Google woff2](https://github.com/google/woff2)
- [MDN: WOFF2](https://developer.mozilla.org/en-US/docs/Web/CSS/@font-face)
- [Font Tools](https://github.com/fonttools/fonttools)
