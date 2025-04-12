use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, RwLock},
};

use tauri::App;
use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut, ShortcutState};

use crate::feature::ClockWindow;

pub fn init_shortcuts<'a>(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let app_handle = app.handle().clone();

    // ショートカットの初期化
    let shortcut_clock = JutokuShortcut::new(
        vec![Shortcut::new(Some(Modifiers::ALT), Code::ArrowLeft)],
        move || {
            ClockWindow::run(&app_handle).unwrap();
        },
    );

    register_jutoku_shortcuts(app, vec![shortcut_clock]);

    Ok(())
}

fn register_jutoku_shortcuts(app: &App, jutoku_shortcuts: Vec<JutokuShortcut>) {
    // 使用するショートカットをすべてまとめる (登録用に必要)
    let shortcut_key_set: HashSet<Shortcut> = jutoku_shortcuts
        .iter()
        .flat_map(|shortcut| shortcut.key.clone())
        .collect();
    // ショートカットキーの押されるすべてのパターンと
    // 最後のキーが含まれている場合だけは処理を保存する
    let mut shortcut_id_all_pattern_map: HashMap<
        Vec<u32>,
        Option<Box<dyn Fn() -> () + Send + Sync>>,
    > = HashMap::new();

    for jutoku_shortcut in jutoku_shortcuts.iter() {
        let mut shortcut_ids = vec![];
        // 一番最後のキー以外を残して、全てのキーのIDを取得
        for n in 0..(jutoku_shortcut.key.len() - 1) {
            shortcut_ids.push(jutoku_shortcut.key[n].id());
            shortcut_id_all_pattern_map.insert(shortcut_ids.clone(), None);
        }

        // 最後のキーのIDを取得し、関数とともに登録
        shortcut_ids.push(jutoku_shortcut.key.last().unwrap().id());

        let f_clone = jutoku_shortcut.f.clone();
        shortcut_id_all_pattern_map.insert(
            shortcut_ids.clone(),
            Some(Box::new(move || {
                f_clone();
            })),
        );
    }

    // ショートカットキーの押下状態を保持するための変数
    let current_pressed_key_ids = Arc::new(RwLock::new(Vec::new()));
    let last_press_time = Arc::new(RwLock::new(std::time::Instant::now()));

    let _ = app.handle().plugin(
        tauri_plugin_global_shortcut::Builder::new()
            .with_shortcuts(shortcut_key_set)
            .expect("Failed to create shortcut")
            .with_handler(move |_app, shortcut, event| {
                if event.state() != ShortcutState::Released {
                    return;
                }

                let mut current_pressed_key_ids_shared = current_pressed_key_ids.write().unwrap();
                let mut last_press_time_shared = last_press_time.write().unwrap();

                // 最後に押された時間が1秒以上経過している場合、押されてきたキーをリセット
                if last_press_time_shared.elapsed().as_secs() > 1 {
                    current_pressed_key_ids_shared.clear();
                }

                current_pressed_key_ids_shared.push(shortcut.id());

                // キーのパターンが一致するかを確認
                if !shortcut_id_all_pattern_map.contains_key(&*current_pressed_key_ids_shared) {
                    // 一致しない場合、カウントをリセット
                    current_pressed_key_ids_shared.clear();
                    return;
                }

                // すべてのキーが押された場合、処理を実行
                if let Some(last_key_and_count) = shortcut_id_all_pattern_map
                    .get(&*current_pressed_key_ids_shared)
                    .unwrap()
                {
                    // ここに処理を記述
                    last_key_and_count();
                    current_pressed_key_ids_shared.clear();
                    return;
                }

                // 一致する場合、最終押下時間を更新
                *last_press_time_shared = std::time::Instant::now();

                // 任意の回数目のショートカットが押されたかを確認
            })
            .build(),
    );
}

struct JutokuShortcut {
    key: Vec<Shortcut>,
    f: Arc<dyn Fn() + Send + Sync + 'static>,
}

impl JutokuShortcut {
    fn new<F: Fn() + Send + Sync + 'static>(shortcuts: Vec<Shortcut>, f: F) -> Self {
        JutokuShortcut {
            key: shortcuts,
            f: Arc::new(f),
        }
    }
}
