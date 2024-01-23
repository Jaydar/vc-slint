use std::fs;
use std::path::Path;

fn main() {
    let conf = slint_build::CompilerConfiguration::default().with_style("fluent".to_owned());
    slint_build::compile_with_config("ui/main.slint",conf).unwrap();
    slint_build::print_rustc_flags().unwrap();







    // icon
    #[cfg(target_os = "windows")]
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        winresource::WindowsResource::new()
            .set_icon("./ui/assets/slint.ico")
            .compile()
            .unwrap();
    }


     // 调用 generate_bindings 函数处理目录
     generate_bindings(
        Path::new("./vendor/ffmpeg-windows/include"),
        Path::new("./src/ffi/ffmpeg")
    );

}


fn generate_bindings(root_path: &Path, out_dir: &Path) {
    // 递归遍历目录
    for entry in fs::read_dir(root_path).expect("Failed to read directory") {
        if let Ok(entry) = entry {
            let file_path = entry.path();

            // 确保是目录
            if file_path.is_dir() {
                // 递归处理子目录，传递正确的输出目录
                let subdir = out_dir.join(file_path.file_name().unwrap());
                generate_bindings(&file_path, &subdir);
            } else if file_path.is_file() && file_path.extension().map_or(false, |ext| ext == "h") {
                // 是文件且文件扩展名是 .h
                let file_name = file_path.file_name().unwrap().to_str().unwrap();
                println!("Generating bindings for: {:?}", file_path);
                generate_single_bindings(&file_path, &out_dir, file_name);
            }
        }
    }
}

fn generate_single_bindings(header_path: &Path, out_dir: &Path, header_file: &str) {
    println!("cargo:rerun-if-changed={}", header_path.display());

    // 使用 bindgen 生成 Rust 绑定
    let bindings = bindgen::Builder::default()
        .header(header_path.to_str().unwrap())
        .clang_arg("--no-clang") // 使用 --no-clang 选项
        .generate()
        .expect("Unable to generate bindings");

    // 获取输出路径，并确保目录存在
    let out_path = out_dir.join(format!("{}_bindings.rs", header_file));
    fs::create_dir_all(out_path.parent().unwrap()).expect("Failed to create output directory");

    // 将生成的 Rust 绑定写入文件
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}