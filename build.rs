
fn main() {
    // slint_build::compile("ui/appwindow.slint").unwrap();

    let conf = slint_build::CompilerConfiguration::default()
        .with_style("fluent".to_owned());
    
    slint_build::compile_with_config("ui/main.slint",conf).unwrap();
    slint_build::print_rustc_flags().unwrap();
}
