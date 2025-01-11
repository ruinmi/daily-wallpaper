use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{App, Manager};
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};
use tauri_plugin_sql::{Migration, MigrationKind};

mod commands;
mod wallpaper;

pub fn run() {
    let migrations = vec![Migration {
        version: 1,
        description: "create_initial_tables",
        sql: include_str!("../sql/init/migrations.sql"),
        kind: MigrationKind::Up,
    }];

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = app.get_webview_window("main")
                .expect("no main window")
                .set_focus();
        }))
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:data.db", migrations)
                .build(),
        )
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            autostart(app);
            tray_setup(app)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::open_folder,
            commands::open_file_in_folder,
            commands::set_wallpaper_command,
            commands::fetch_wallpaper
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn tray_setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&quit])?;

    TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => {
                app.exit(0);
            }
            _ => (),
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                let _ = show_window(app, "main");
            }
        })
        .build(app)?;
    Ok(())
}

fn show_window(app: &tauri::AppHandle, label: &str) -> tauri::Result<()> {
    let window = app.get_webview_window(label).unwrap();
    let is_visible = window.is_visible()?;
    if is_visible {
        window.unminimize()?;
    } else {
        window.show()?;
    }
    window.set_focus()?;
    Ok(())
}

#[cfg(desktop)]
fn autostart(app: &mut App) {
    let autostart_manager = app.autolaunch();
    let _ = autostart_manager.enable();
}
