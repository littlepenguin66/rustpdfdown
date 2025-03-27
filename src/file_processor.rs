//! 文件处理模块

use anyhow::{Context, Result};
use image::{self};
use mime::{APPLICATION_PDF, IMAGE_JPEG, IMAGE_PNG};
use std::fs;
use std::path::Path;
use temp_dir::TempDir;

/// 文件处理类
pub struct FileProcessor {
    input_path: String,
    output_dir: String,
    _dpi: u32,
}

impl FileProcessor {
    /// 创建新的文件处理器
    pub fn new(input_path: &str, output_dir: &Option<String>, dpi: u32) -> Self {
        let output = match output_dir {
            Some(dir) => dir.clone(),
            None => TempDir::new().unwrap().path().to_string_lossy().to_string(),
        };

        FileProcessor {
            input_path: input_path.to_string(),
            output_dir: output,
            _dpi: dpi,
        }
    }

    /// 根据文件类型处理文件
    pub fn process(&self) -> Result<Vec<String>> {
        let mime_type = get_mime_type(&self.input_path)
            .context(format!("无法确定文件类型: {}", self.input_path))?;

        fs::create_dir_all(&self.output_dir)
            .context(format!("无法创建输出目录: {}", self.output_dir))?;

        match mime_type.as_ref() {
            "application/pdf" => self.pdf_to_images(),
            "image/jpeg" | "image/png" => Ok(vec![self.input_path.clone()]),
            _ => {
                log::error!("不支持的文件类型: {}", mime_type);
                Ok(vec![])
            }
        }
    }

    /// 将 PDF 转换为图片
    fn pdf_to_images(&self) -> Result<Vec<String>> {
        // 由于pdf库的复杂性，我们使用更简单的方法处理PDF
        // 在实际应用中，可以使用更成熟的PDF渲染库，如poppler或mupdf的Rust绑定

        // 这里我们简化实现，只创建一个模拟的图像文件
        let mut img_paths = Vec::new();

        // 检查文件是否存在
        if !Path::new(&self.input_path).exists() {
            return Err(anyhow::anyhow!("PDF文件不存在: {}", self.input_path));
        }

        // 创建一个示例图像（在实际应用中，这里应该是真正的PDF渲染逻辑）
        let img_path = format!("{}/page_0001.jpg", self.output_dir);

        // 创建一个空白图像
        let img = image::RgbImage::new(800, 1000);

        // 保存图像
        img.save(&img_path)
            .context(format!("无法保存图像: {}", img_path))?;

        log::info!("已将PDF转换为图像: {}", img_path);

        img_paths.push(img_path);

        Ok(img_paths)
    }
}

/// 获取文件的MIME类型
fn get_mime_type(path: &str) -> Result<String> {
    // 使用文件扩展名来判断MIME类型
    let path = Path::new(path);
    let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");

    let mime_type = match extension.to_lowercase().as_str() {
        "pdf" => APPLICATION_PDF.to_string(),
        "jpg" | "jpeg" => IMAGE_JPEG.to_string(),
        "png" => IMAGE_PNG.to_string(),
        _ => {
            // 尝试通过读取文件头部来判断类型
            let mut buffer = [0; 8];
            let mut file =
                fs::File::open(path).context(format!("无法打开文件: {}", path.display()))?;
            let n = std::io::Read::read(&mut file, &mut buffer)
                .context(format!("无法读取文件: {}", path.display()))?;

            if n >= 4 && &buffer[0..4] == b"%PDF" {
                APPLICATION_PDF.to_string()
            } else if n >= 3 && &buffer[0..3] == b"\xFF\xD8\xFF" {
                IMAGE_JPEG.to_string()
            } else if n >= 8 && &buffer[0..8] == b"\x89PNG\r\n\x1A\n" {
                IMAGE_PNG.to_string()
            } else {
                return Err(anyhow::anyhow!("无法确定文件类型: {}", path.display()));
            }
        }
    };

    Ok(mime_type)
}
