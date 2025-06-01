#[cfg_attr(mobile, tauri::mobile_entry_point)]
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Emitter, Manager,
};

use tauri_plugin_deep_link::DeepLinkExt;

#[derive(Clone, serde::Serialize)]
struct Payload {
    args: Vec<String>,
    cwd: String,
}

pub fn run() {
    tauri::Builder::default()
		.plugin(tauri_plugin_single_instance::init(|app, args, cwd| {
            let _ = app
                .get_webview_window("main")
                .expect("no main window")
                .set_focus();
            println!("{}, {args:?}, {cwd}", app.package_info().name);
            app.emit("single-instance", Payload { args, cwd }).unwrap();
        }))
		.plugin(tauri_plugin_deep_link::init())
        .setup(|app| {
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit_i])?;

            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .show_menu_on_left_click(true)
                .icon(app.default_window_icon().unwrap().clone())
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        app.exit(0);
                    }
                    other => {
                        println!("menu item {} not handled", other);
                    }
                })
                .build(app)?;

			app.deep_link().on_open_url(|event| {
                println!("deep link URLs: {:?}", event.urls());
            });

            Ok(())
        })
        .plugin(tauri_plugin_single_instance::init(|app, args, cwd| {
            let _ = app
                .get_webview_window("main")
                .expect("no main window")
                .set_focus();
            println!("{}, {args:?}, {cwd}", app.package_info().name);
            app.emit("single-instance", Payload { args, cwd }).unwrap();
        }))
		.plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
