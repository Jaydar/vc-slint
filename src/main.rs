use i_slint_backend_winit::WinitWindowAccessor;
use windows_sys::Win32::UI::WindowsAndMessaging::GetSystemMetrics;
slint::include_modules!();

fn main() {
    let screen_width = unsafe { GetSystemMetrics(0) };
    let screen_height = unsafe { GetSystemMetrics(1) };

    // 计算窗口位置
    let window_width = 320.0;
    let window_height = 568.0;
    let win_pos_x = (screen_width / 2 - (window_width as i32) / 2) as f32;
    let win_pos_y = (screen_height / 2 - (window_height as i32) / 2) as f32;

    let main = MainWindow::new().unwrap();
    // main.window().set_size(slint::LogicalSize::new(window_width,window_height));
    main.window()
        .set_position(slint::LogicalPosition::new(win_pos_x, win_pos_y));
 
    println!("{}", main.window().has_winit_window());




    // let ui_handle = main.as_weak();

    // main.on_request_increase_value(move || {
    //     let ui = ui_handle.unwrap();
    //     // ui.set_counter(ui.get_counter() + 1);
    // });


    main.window()
    .with_winit_window(|ww: &winit::window::Window| {
        let buttons = ww.enabled_buttons();
        ww.set_enabled_buttons(buttons ^ winit::window::WindowButtons::MAXIMIZE);
    });

    main.run().unwrap();

}
