#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] 

// use bridge::bridge_init;
use i_slint_backend_winit::WinitWindowAccessor;
use slint::{SharedString, Color};
use windows_sys::Win32::UI::WindowsAndMessaging::GetSystemMetrics;
use winit::dpi::LogicalPosition;

mod app;
mod tools;
// mod bridge;

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
   
    main.global::<bridge>().on_string_to_color(|mut hex:SharedString| {

        let default_color = Color::default();
        
        if  !hex.starts_with("#")  || !(hex.len()==4 || hex.len()==7 || hex.len()==9) {
            return default_color;
        }


        if hex.len() == 4 {
            hex = SharedString::from(format!("#{}{}{}{}{}{}", &hex[1..2], &hex[1..2], &hex[2..3], &hex[2..3], &hex[3..4], &hex[3..4]).to_owned())
        };


        let r = match u8::from_str_radix(&hex[1..3], 16) {
            Ok(val) => val,
            Err(_) => return default_color,
        };
        let g = match u8::from_str_radix(&hex[3..5], 16) {
            Ok(val) => val,
            Err(_) => return default_color,
        };
        let b = match u8::from_str_radix(&hex[5..7], 16) {
            Ok(val) => val,
            Err(_) => return default_color,
        };
 
        if hex.len() == 9 {
            let a = match u8::from_str_radix(&hex[7..9], 16) {
                Ok(val) => val,
                Err(_) => return default_color,
            };
            return Color::from_argb_u8(a, r, g, b);

        } else {
            return Color::from_rgb_u8(r, g, b);
        }

    });

    main.global::<bridge>().on_color_to_string(|value:Color| {
        let hex = format!("#{:02x}{:02x}{:02x}{:02x}", value.red(), value.green(), value.blue(), value.alpha());
        let mut ss = SharedString::default();
        ss.push_str(hex.as_str());
        return ss;
    });

    main.run().unwrap();
}



#[cfg(test)]
mod tests{
    #[test]
    fn it_hex(){
        let hex = "!333";
        

        println!("no #:   {}", !hex.starts_with("#") );
        println!("no hax: {}", !(hex.len()==4 || hex.len()==7 || hex.len()==9) );
        println!("no-hex: {}", !hex.starts_with("#")  || !(hex.len()==4 || hex.len()==7 || hex.len()==9) );
    }
}