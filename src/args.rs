//! 命令行参数解析模块

use anyhow::Result;
use clap::Parser;
use dotenv::dotenv;
use std::env;

/// 命令行参数结构
#[derive(Parser, Debug, Clone)]
#[clap(author, version, about = "将 PDF 或图片转换为 Markdown")]
pub struct Args {
    /// 输入文件路径（PDF 或图片）
    #[clap(short, long, required = true)]
    pub input: String,

    /// 输出目录（默认自动生成临时目录）
    #[clap(short, long)]
    pub output_dir: Option<String>,

    /// PDF 转图片的 DPI
    #[clap(short, long, default_value = "300")]
    pub dpi: u32,

    /// 线程池工作线程数
    #[clap(short, long, default_value = "5")]
    pub workers: usize,

    /// OpenAI API 密钥
    #[clap(long)]
    pub api_key: String,

    /// OpenAI 模型
    #[clap(long, default_value = "gpt-4o")]
    pub model: String,

    /// 日志级别 (INFO, DEBUG, ERROR 等)
    #[clap(short, long, default_value = "INFO")]
    pub log_level: String,
}

impl Args {
    /// 解析命令行参数
    pub fn parse_args() -> Result<Self> {
        // 加载环境变量
        dotenv().ok();

        let mut args = Args::parse();

        // 如果没有提供API密钥，尝试从环境变量获取
        if args.api_key.is_empty() {
            args.api_key = env::var("OPENAI_API_KEY").unwrap_or_else(|_| String::new());

            if args.api_key.is_empty() {
                anyhow::bail!(
                    "未提供OpenAI API密钥，请通过--api-key参数或OPENAI_API_KEY环境变量提供"
                );
            }
        }

        // 验证参数
        if args.dpi == 0 || args.workers == 0 {
            anyhow::bail!("DPI 和 workers 必须为正整数");
        }

        Ok(args)
    }
}
