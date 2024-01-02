#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] 

use i_slint_backend_winit::WinitWindowAccessor;
use windows_sys::Win32::UI::WindowsAndMessaging::GetSystemMetrics;
use winit::dpi::LogicalPosition;

mod app;

slint::include_modules!();

#[tokio::main]
async fn main() {

    let _ = tokio::spawn(app::init());

    let screen_width = unsafe { GetSystemMetrics(0) };
    let screen_height = unsafe { GetSystemMetrics(1) };

    // 计算窗口位置
    let window_width = 320.0;
    let window_height = 568.0;
    let win_pos_x = (screen_width / 2 - (window_width as i32) / 2) as f32;
    let win_pos_y = (screen_height / 2 - (window_height as i32) / 2) as f32;

    let main = MainWindow::new().unwrap();
    main.window()
        .with_winit_window(|ww: &winit::window::Window| {
            ww.set_outer_position(LogicalPosition::new(win_pos_x, win_pos_y));
            let buttons = ww.enabled_buttons();
            ww.set_enabled_buttons(buttons ^ winit::window::WindowButtons::MAXIMIZE);
        });
   
    main.run().unwrap();
}
