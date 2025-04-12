use tauri::{Manager, WebviewWindow};

pub mod autostart;
pub mod feature;
pub mod shortcut;
pub mod systemtray;

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
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = app
                .get_webview_window("main")
                .expect("no main window")
                .set_focus();
            let _ = app.get_webview_window("clock").expect("no clock window");
        }))
        .setup(|app| {
            // オートスタート
            autostart::init_autostart(app)?;

            // ショートカットの初期化
            shortcut::init_shortcuts(app)?;

            // システムトレイのアイコンの初期化
            systemtray::init_systemtray(app)?;

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                // ウィンドウを閉じるのではなく、非表示にする
                api.prevent_close();
                window.hide().unwrap();
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
