use dirs;
use rust_embed::RustEmbed;
use std::{fs, io::Write};

#[derive(RustEmbed)]
#[folder = "./vendor/"]
struct Vendor;

use crate::tools;

pub async fn init() {
    let data = tools::parse_cargo_toml().unwrap();
    let app_name = data.get("name").and_then(|value| value.as_str()).unwrap();
    let ffmpeg_path = dirs::home_dir().unwrap().join(app_name).join("ffmpeg.exe");

    // 判断文件是否存在
    if !ffmpeg_path.exists() {
        fs::create_dir_all(ffmpeg_path.parent().unwrap()).unwrap();
        // 获取嵌入的 ffmpeg 文件内容
        let ffmpeg_binary = Vendor::get("ffmpeg.exe").unwrap();
        let mut file = fs::File::create(&ffmpeg_path).unwrap();
        file.write_all(ffmpeg_binary.data.as_ref()).unwrap();
    }
}
