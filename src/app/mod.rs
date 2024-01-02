use std::{fs, io::Write};

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "./vendor/"]
struct Vendor;

pub async fn init() {
    // 获取系统临时目录
    let temp_dir_path = std::env::temp_dir();
    // 合并临时目录和文件路径，并检查文件是否存在，如果不存在则提取
    let ffmpeg_path = temp_dir_path.join("temp_video_cover").join("ffmpeg.exe");

    // 判断文件是否存在
    if !ffmpeg_path.exists() {
        fs::create_dir_all(ffmpeg_path.parent().unwrap()).unwrap();
        // 获取嵌入的 ffmpeg 文件内容
        let ffmpeg_binary = Vendor::get("ffmpeg.exe").unwrap();
        let mut file = fs::File::create(&ffmpeg_path).unwrap();
        file.write_all(ffmpeg_binary.data.as_ref()).unwrap();
    }

}
