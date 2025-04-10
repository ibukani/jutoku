use tauri::{
    menu::{Menu, MenuItem},
    Manager, WebviewWindow,
};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

pub fn move_window_top_right(window: &WebviewWindow) {
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // システムトレイのアイコンを作成
            let quit_icon = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let show_icon = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit_icon, &show_icon])?;
            let _tray = tauri::tray::TrayIconBuilder::new()
                .menu(&menu)
                .icon(app.default_window_icon().unwrap().clone())
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "quit" => {
                        app.exit(0);
                    }
                    "show" => {
                        // ウィンドウの作成
                        let window = app.get_webview_window("clock").unwrap();

                        if window.is_visible().unwrap() {
                            window.hide().unwrap();
                        } else {
                            move_window_top_right(&window);
                            window.show().unwrap();
                        }
                    }
                    _ => {}
                })
                .build(app)?;
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
