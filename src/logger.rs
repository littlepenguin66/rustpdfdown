//! 日志配置模块

use anyhow::Result;
use env_logger::Builder;
use log::LevelFilter;
use std::str::FromStr;

/// 配置日志
pub fn setup_logging(log_level: &str) -> Result<()> {
    let level = LevelFilter::from_str(log_level)
        .map_err(|_| anyhow::anyhow!("无效的日志级别: {}", log_level))?;

    Builder::new()
        .filter(None, level)
        .format_timestamp(Some(env_logger::TimestampPrecision::Seconds))
        .format_module_path(false)
        .target(env_logger::Target::Stderr)
        .init();

    Ok(())
}
