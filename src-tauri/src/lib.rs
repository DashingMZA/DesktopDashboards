use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

static WIDGET_SEQ: AtomicUsize = AtomicUsize::new(1);

fn resolve_webview_data_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let base = app
        .path()
        .local_data_dir()
        .map_err(|e| format!("failed to resolve local_data_dir: {e}"))?;
    let dir = base.join("DesktopDashboards").join("webview");
    std::fs::create_dir_all(&dir)
        .map_err(|e| format!("failed to create webview data dir {}: {e}", dir.display()))?;
    Ok(dir)
}

#[tauri::command]
fn get_webview_data_dir(app: AppHandle) -> Result<String, String> {
    let dir = resolve_webview_data_dir(&app)?;
    Ok(dir.to_string_lossy().into_owned())
}

#[tauri::command]
async fn open_spike_widget(
    app: AppHandle,
    url: String,
    always_on_bottom: bool,
    decorations: bool,
) -> Result<String, String> {
    let parsed = url::Url::parse(&url).map_err(|e| format!("invalid URL '{url}': {e}"))?;
    let data_dir = resolve_webview_data_dir(&app)?;
    let seq = WIDGET_SEQ.fetch_add(1, Ordering::Relaxed);
    let label = format!("widget-{seq}");

    let mut builder = WebviewWindowBuilder::new(&app, &label, WebviewUrl::External(parsed))
        .title(format!("DesktopDashboards Widget #{seq}"))
        .data_directory(data_dir)
        .inner_size(480.0, 760.0)
        .decorations(decorations)
        .skip_taskbar(always_on_bottom)
        .always_on_bottom(always_on_bottom);

    if !decorations {
        builder = builder.shadow(false);
    }

    builder
        .build()
        .map_err(|e| format!("failed to create widget window: {e}"))?;

    Ok(label)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_webview_data_dir,
            open_spike_widget
        ])
        .run(tauri::generate_context!())
        .expect("error while running DesktopDashboards");
}
