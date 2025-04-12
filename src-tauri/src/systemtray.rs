use tauri::{
    menu::{Menu, MenuItem},
    App, Manager,
};

pub fn init_systemtray(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    // システムトレイのアイコンを作成
    let quit_icon = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let clock_icon = MenuItem::with_id(app, "clock", "clock", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&quit_icon, &clock_icon])?;
    let _tray = tauri::tray::TrayIconBuilder::new()
        .menu(&menu)
        .icon(app.default_window_icon().unwrap().clone())
        .on_menu_event(|app, event| match event.id().as_ref() {
            "quit" => {
                app.exit(0);
            }
            "clock" => {
                // ウィンドウの作成
                let window = app.get_webview_window("clock").unwrap();

                if window.is_visible().unwrap() {
                    window.hide().unwrap();
                } else {
                    crate::move_window_top_right(&window);
                    window.show().unwrap();
                }
            }
            _ => {}
        })
        .build(app)?;
    Ok(())
}
