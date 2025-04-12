use tauri::{AppHandle, Manager, WebviewWindow};

// ウィンドウの位置を画面の右上に移動する関数
fn move_window_top_right(window: &WebviewWindow) {
    let screen = window.primary_monitor().unwrap();
    let screen_width = screen.as_ref().map_or(0, |s| s.size().width as i32);
    let window_size = window.outer_size().unwrap();
    let window_width = window_size.width as i32;
    window
        .set_position(tauri::Position::Physical(tauri::PhysicalPosition {
            x: screen_width - window_width,
            y: 0,
        }))
        .unwrap();
}

pub struct ClockWindow;

impl ClockWindow {
    pub fn run(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
        let window = app
            .get_webview_window("clock")
            .expect("Failed to get clock window");

        if window.is_visible().unwrap() {
            window.hide().unwrap();
        } else {
            move_window_top_right(&window);
            window.show().unwrap();
        }

        Ok(())
    }

    pub fn test() {
        println!("ClockWindow test function called.");
    }
}
