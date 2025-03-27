use anyhow::Result;
use rustpdfdown::args::Args;

#[tokio::main]
async fn main() -> Result<()> {
    // 解析命令行参数
    let args = Args::parse_args()?;

    // 处理文件
    let markdown = rustpdfdown::process_file(args.clone()).await?;

    // 输出结果
    println!("{}", markdown);

    // 如果没有指定输出目录，则清理临时目录
    if args.output_dir.is_none() {
        // 在Rust中，temp-dir库会在TempDir实例被丢弃时自动清理临时目录
        // 所以我们不需要显式清理，这与Python版本的功能相同
        log::debug!("临时目录将在程序结束时自动清理");
    }

    Ok(())
}
