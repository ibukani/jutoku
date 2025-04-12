use tauri::{
    menu::{Menu, MenuItem},
    Manager, WebviewWindow,
};
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};
pub mod feature;
pub mod shortcut;

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
            let _ = app.handle().plugin(tauri_plugin_autostart::init(
                MacosLauncher::LaunchAgent,
                Some(vec!["--flag1", "--flag2"]),
            ));

            // Get the autostart manager
            let autostart_manager = app.autolaunch();
            // Enable autostart
            let _ = autostart_manager.enable();

            // ショートカットの初期化
            shortcut::init_shortcuts(app);

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
