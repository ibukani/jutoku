use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    App, Manager,
};

use crate::feature::ClockWindow;

pub fn init_systemtray(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    // システムトレイのアイコンを作成
    let separator = PredefinedMenuItem::separator(app)?;
    let quit_icon = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let clock_icon = MenuItem::with_id(app, "clock", "Clock", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&clock_icon, &separator, &quit_icon])?;

    // Trayのビルド
    let _tray = tauri::tray::TrayIconBuilder::new()
        .menu(&menu)
        .icon(app.default_window_icon().unwrap().clone())
        .on_menu_event(|app, event| match event.id().as_ref() {
            "quit" => {
                app.exit(0);
            }
            "clock" => {
                ClockWindow::run(app);
            }
            _ => {}
        })
        .build(app)?;
    Ok(())
}
