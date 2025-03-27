# RustPDFDown

一个将 PDF 或图片转换为 Markdown 的 Rust 工具。

## 功能特点

- 支持 PDF 文件转换为 Markdown
- 支持 JPG、PNG 等图片格式转换为 Markdown
- 使用 OpenAI API 进行图像内容识别和转换
- 多线程并行处理，提高转换效率
- 自动创建临时目录或指定输出目录
- 可配置的日志级别

## 安装

### 前提条件

- Rust 环境 (推荐使用 [rustup](https://rustup.rs/) 安装)
- OpenAI API 密钥

### 从源码安装

```bash
# 克隆仓库
git clone https://github.com/yourusername/rustpdfdown.git
cd rustpdfdown

# 编译项目
cargo build --release

# 安装到系统路径（可选）
cargo install --path .
```

## 使用方法

### 基本用法

```bash
rustpdfdown --input 文档.pdf --api-key your_openai_api_key
```

### 命令行参数

| 参数               | 描述                       | 默认值           |
| ------------------ | -------------------------- | ---------------- |
| `-i, --input`      | 输入文件路径（PDF 或图片） | 必填             |
| `-o, --output-dir` | 输出目录                   | 自动生成临时目录 |
| `-d, --dpi`        | PDF 转图片的 DPI           | 300              |
| `-w, --workers`    | 线程池工作线程数           | 5                |
| `--api-key`        | OpenAI API 密钥            | 从环境变量获取   |
| `--model`          | OpenAI 模型                | gpt-4o           |
| `-l, --log-level`  | 日志级别                   | INFO             |

### 环境变量配置

你可以在项目根目录创建一个`.env`文件，设置以下环境变量：

```
OPENAI_API_KEY=your_openai_api_key
```

## 示例

### 转换 PDF 文件

```bash
rustpdfdown --input document.pdf --output-dir ./output
```

### 转换图片文件

```bash
rustpdfdown --input image.jpg --api-key your_openai_api_key
```

### 使用不同的模型

```bash
rustpdfdown --input document.pdf --model gpt-4-turbo
```

### 调整 DPI 和工作线程数

```bash
rustpdfdown --input document.pdf --dpi 600 --workers 10
```

## 项目结构

- `src/main.rs`: 程序入口点
- `src/args.rs`: 命令行参数解析
- `src/file_processor.rs`: 文件处理模块
- `src/ai_converter.rs`: AI 转换模块
- `src/logger.rs`: 日志配置模块
- `src/lib.rs`: 库入口点

## 注意事项

- 当前版本的 PDF 转换功能是简化实现，实际应用中可以使用更成熟的 PDF 渲染库
- 转换大型文件可能需要更多的 API 调用，请注意 OpenAI API 的使用限制和费用
- 如果未指定输出目录，程序会自动创建临时目录并在程序结束时清理

## 许可证

[MIT](LICENSE)
