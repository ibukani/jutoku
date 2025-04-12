use tauri::App;
use tauri_plugin_autostart::ManagerExt;

pub fn init_autostart(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    // オートスタートの初期化
    let _ = app.handle().plugin(tauri_plugin_autostart::init(
        tauri_plugin_autostart::MacosLauncher::LaunchAgent,
        Some(vec!["--flag1", "--flag2"]),
    ));

    // Get the autostart manager
    let autostart_manager = app.autolaunch();
    // Enable autostart
    let _ = autostart_manager.enable();

    Ok(())
}
