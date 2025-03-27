//! PDF和图片转Markdown工具库

use anyhow::Result;

pub mod ai_converter;
pub mod args;
pub mod file_processor;
pub mod logger;

/// 处理文件并返回Markdown
pub async fn process_file(args: args::Args) -> Result<String> {
    // 设置日志
    logger::setup_logging(&args.log_level)?;

    // 处理文件
    let processor = file_processor::FileProcessor::new(&args.input, &args.output_dir, args.dpi);
    let images = processor.process()?;
    if images.is_empty() {
        log::error!("无有效图片生成");
        return Ok(String::new());
    }

    // AI 转换
    let converter = ai_converter::AIConverter::new(&args.api_key, &args.model);
    let markdown_parts = converter.convert_images(images, args.workers).await?;

    // 合并结果
    Ok(markdown_parts.join("\n\n"))
}
