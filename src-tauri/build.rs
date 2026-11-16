fn main() {
    let attrs = tauri_build::Attributes::new().app_manifest(
        tauri_build::AppManifest::new().commands(&[
            "get_webview_data_dir",
            "list_widgets",
            "add_widget",
            "toggle_widget",
            "update_widget_bounds",
            "update_widget_settings",
            "widget_bar_action",
            "reset_widget_position",
            "delete_widget",
            "reload_widget",
            "reload_all_widgets",
            "get_autostart",
            "set_autostart",
        ]),
    );
    tauri_build::try_build(attrs).expect("failed to run tauri-build");
}
