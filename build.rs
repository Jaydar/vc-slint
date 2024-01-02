
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
}
