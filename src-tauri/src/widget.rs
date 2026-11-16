use crate::config::{ConfigState, PlacementMode, SourceType, WidgetConfig};
use crate::win32;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tauri::webview::NewWindowResponse;
use tauri::{
    AppHandle, Manager, PhysicalPosition, PhysicalSize, Position, Size, WebviewUrl,
    WebviewWindowBuilder, WindowEvent,
};

pub fn widget_label(widget_id: &str) -> String {
    format!("widget-{widget_id}")
}

const WIDGET_DRAG_HANDLE_SCRIPT: &str = r#"
(() => {
  if (window.__ddDragHandleInstalled) return;
  window.__ddDragHandleInstalled = true;

  function install() {
    if (!document.body || document.getElementById('__dd_drag_host')) return;
    const host = document.createElement('div');
    host.id = '__dd_drag_host';
    host.style.cssText = 'all:initial;position:fixed;top:0;left:0;right:0;height:18px;z-index:2147483647;pointer-events:auto;';
    const shadow = host.attachShadow({ mode: 'open' });
    const bar = document.createElement('div');
    bar.textContent = '⋮⋮ ドラッグで移動';
    bar.style.cssText = [
      'width:100%',
      'height:18px',
      'display:flex',
      'align-items:center',
      'justify-content:center',
      'font-family:"Segoe UI",sans-serif',
      'font-size:11px',
      'color:rgba(255,255,255,0.9)',
      'background:rgba(15,23,42,0.75)',
      'cursor:move',
      'user-select:none',
      'opacity:0',
      'transition:opacity 0.15s ease'
    ].join(';');
    bar.addEventListener('mouseenter', () => { bar.style.opacity = '1'; });
    bar.addEventListener('mouseleave', () => { bar.style.opacity = '0'; });
    bar.addEventListener('mousedown', (e) => {
      if (e.button !== 0) return;
      e.preventDefault();
      if (window.__TAURI_INTERNALS__ && typeof window.__TAURI_INTERNALS__.invoke === 'function') {
        window.__TAURI_INTERNALS__.invoke('plugin:window|start_dragging').catch(() => {});
      }
    });
    shadow.appendChild(bar);
    document.body.appendChild(host);
  }

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', install, { once: true });
  } else {
    install();
  }
})();
"#;

/// 独立した WebView2 `user_data_dir` を解決する（他アプリとのロック競合防止）。
pub fn resolve_webview_data_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let base = app
        .path()
        .local_data_dir()
        .map_err(|e| format!("failed to resolve local_data_dir: {e}"))?;
    let dir = base.join("DesktopDashboards").join("webview");
    fs::create_dir_all(&dir)
        .map_err(|e| format!("failed to create webview data dir {}: {e}", dir.display()))?;
    Ok(dir)
}

/// ガジェット設定から WebView の初期 URL を構築する。
pub fn build_webview_url(widget: &WidgetConfig) -> Result<WebviewUrl, String> {
    match widget.source_type {
        SourceType::Url => {
            let parsed = url::Url::parse(&widget.source)
                .map_err(|e| format!("invalid URL '{}': {e}", widget.source))?;
            Ok(WebviewUrl::External(parsed))
        }
        SourceType::File => {
            let parsed = url::Url::parse(&format!("http://widgetfile.localhost/{}/", widget.id))
                .map_err(|e| format!("failed to build widgetfile URL: {e}"))?;
            Ok(WebviewUrl::External(parsed))
        }
    }
}

/// 遷移先 URL が初期 URL と同一オリジンまたは内部スキームかを判定する。
pub fn is_same_origin_or_internal(initial: &url::Url, target: &url::Url) -> bool {
    if matches!(target.scheme(), "about" | "data" | "blob") {
        return true;
    }
    initial.scheme() == target.scheme()
        && initial.host_str() == target.host_str()
        && initial.port_or_known_default() == target.port_or_known_default()
}

/// ガジェットウィンドウを生成する（コマンド内からの呼び出し時のデッドロック防止のため async fn）。
pub async fn create_widget_window(app: AppHandle, widget: WidgetConfig) -> Result<(), String> {
    let label = widget_label(&widget.id);
    if app.get_webview_window(&label).is_some() {
        return Ok(());
    }

    let webview_url = build_webview_url(&widget)?;
    let initial_url = match &webview_url {
        WebviewUrl::External(u) => Some(u.clone()),
        _ => None,
    };
    let data_dir = resolve_webview_data_dir(&app)?;

    let scale = app
        .primary_monitor()
        .ok()
        .flatten()
        .map(|m| m.scale_factor())
        .unwrap_or(1.0)
        .max(1.0);
    let target_w = widget.width.max(120);
    let target_h = widget.height.max(80);

    let mut builder = WebviewWindowBuilder::new(&app, &label, webview_url)
        .title(&widget.title)
        .data_directory(data_dir)
        .position(widget.x as f64 / scale, widget.y as f64 / scale)
        .inner_size(target_w as f64 / scale, target_h as f64 / scale)
        .decorations(false)
        .shadow(false)
        .skip_taskbar(true)
        .always_on_bottom(true)
        .initialization_script(WIDGET_DRAG_HANDLE_SCRIPT)
        .on_new_window(|url, _features| {
            if matches!(url.scheme(), "http" | "https") {
                let _ = tauri_plugin_opener::open_url(url.as_str(), None::<&str>);
            }
            NewWindowResponse::Deny
        });

    if let Some(base_url) = initial_url {
        builder = builder.on_navigation(move |target_url| {
            if is_same_origin_or_internal(&base_url, target_url) {
                true
            } else {
                if matches!(target_url.scheme(), "http" | "https") {
                    let _ = tauri_plugin_opener::open_url(target_url.as_str(), None::<&str>);
                }
                false
            }
        });
    }

    let window = builder
        .build()
        .map_err(|e| format!("failed to create widget window {label}: {e}"))?;

    // 物理ピクセル座標で厳密に配置し、Moved イベントによる座標ズレを防ぐ
    let _ = window.set_position(Position::Physical(PhysicalPosition::new(
        widget.x, widget.y,
    )));
    let _ = window.set_size(Size::Physical(PhysicalSize::new(target_w, target_h)));

    // 最背面固定（WS_EX_TOOLWINDOW + HWND_BOTTOM + WM_WINDOWPOSCHANGING サブクラス）
    let _ = PlacementMode::Bottom;
    if let Err(e) = win32::apply_bottom_mode(&window) {
        eprintln!("[widget] failed to apply bottom mode to {label}: {e}");
    }

    // 位置・サイズ変更時のデバウンス保存
    let app_handle = app.clone();
    let widget_id = widget.id.clone();
    window.on_window_event(move |event| match event {
        WindowEvent::Moved(pos) => {
            // Windows の最小化時（-32000 等）を誤保存しないための簡易ガード
            if pos.x <= -16000 || pos.y <= -16000 {
                return;
            }
            if let Some(state) = app_handle.try_state::<Arc<ConfigState>>() {
                if let Ok(mut cfg) = state.config.lock() {
                    if let Some(w) = cfg.widgets.iter_mut().find(|w| w.id == widget_id) {
                        w.x = pos.x;
                        w.y = pos.y;
                    }
                }
                ConfigState::schedule_debounced_save(state.inner());
            }
        }
        WindowEvent::Resized(size) => {
            if size.width == 0 || size.height == 0 {
                return;
            }
            if let Some(state) = app_handle.try_state::<Arc<ConfigState>>() {
                if let Ok(mut cfg) = state.config.lock() {
                    if let Some(w) = cfg.widgets.iter_mut().find(|w| w.id == widget_id) {
                        w.width = size.width;
                        w.height = size.height;
                    }
                }
                ConfigState::schedule_debounced_save(state.inner());
            }
        }
        _ => {}
    });

    Ok(())
}

/// ガジェットウィンドウを破棄（close）してメモリを解放する。
pub fn close_widget_window(app: &AppHandle, widget_id: &str) {
    let label = widget_label(widget_id);
    if let Some(win) = app.get_webview_window(&label) {
        let _ = win.close();
    }
}

/// 指定ガジェットウィンドウを再読み込みする。
pub fn reload_widget_window(app: &AppHandle, widget_id: &str) {
    let label = widget_label(widget_id);
    if let Some(win) = app.get_webview_window(&label) {
        let _ = win.reload();
    }
}

/// 表示中の全ガジェットウィンドウを再読み込みする。
pub fn reload_all_widget_windows(app: &AppHandle) {
    for (label, win) in app.webview_windows() {
        if label.starts_with("widget-") {
            let _ = win.reload();
        }
    }
}

/// 設定されている全ガジェットの有効状態に合わせてウィンドウを同期する。
pub async fn sync_all_widgets(app: AppHandle) {
    let widgets = {
        let Some(state) = app.try_state::<Arc<ConfigState>>() else {
            return;
        };
        let Ok(cfg) = state.config.lock() else {
            return;
        };
        cfg.widgets.clone()
    };

    for widget in widgets {
        if widget.enabled {
            if let Err(e) = create_widget_window(app.clone(), widget).await {
                eprintln!("[widget] failed to restore widget: {e}");
            }
        } else {
            close_widget_window(&app, &widget.id);
        }
    }
}

/// ローカル HTML (`source_type: "file"`) 用カスタムプロトコル `widgetfile` のハンドラ。
pub fn handle_widgetfile_protocol(
    app: &AppHandle,
    request: tauri::http::Request<Vec<u8>>,
) -> tauri::http::Response<Vec<u8>> {
    let path = request.uri().path();
    let trimmed = path.trim_start_matches('/');
    let mut parts = trimmed.splitn(2, '/');
    let widget_id = parts.next().unwrap_or("");
    let rel_path = parts.next().unwrap_or("");

    if widget_id.is_empty() {
        return text_response(400, "missing widget id");
    }

    let source_path = {
        let Some(state) = app.try_state::<Arc<ConfigState>>() else {
            return text_response(500, "config state unavailable");
        };
        let Ok(cfg) = state.config.lock() else {
            return text_response(500, "config lock poisoned");
        };
        cfg.widgets
            .iter()
            .find(|w| w.id == widget_id && w.source_type == SourceType::File)
            .map(|w| PathBuf::from(&w.source))
    };

    let Some(entry_file) = source_path else {
        return text_response(404, "widget not found");
    };

    let target_file = if rel_path.is_empty() {
        entry_file.clone()
    } else {
        let Some(base_dir) = entry_file.parent() else {
            return text_response(404, "invalid base directory");
        };
        let candidate = base_dir.join(rel_path);
        match (fs::canonicalize(base_dir), fs::canonicalize(&candidate)) {
            (Ok(canon_base), Ok(canon_target)) if canon_target.starts_with(&canon_base) => {
                canon_target
            }
            _ => return text_response(403, "forbidden path"),
        }
    };

    match fs::read(&target_file) {
        Ok(bytes) => {
            let mime = guess_mime_type(&target_file);
            tauri::http::Response::builder()
                .status(200)
                .header("Content-Type", mime)
                .body(bytes)
                .unwrap_or_else(|_| text_response(500, "response build error"))
        }
        Err(_) => text_response(404, "file not found"),
    }
}

fn text_response(status: u16, msg: &str) -> tauri::http::Response<Vec<u8>> {
    tauri::http::Response::builder()
        .status(status)
        .header("Content-Type", "text/plain; charset=utf-8")
        .body(msg.as_bytes().to_vec())
        .unwrap()
}

fn guess_mime_type(path: &Path) -> &'static str {
    match path
        .extension()
        .and_then(|e| e.to_str())
        .map(|s| s.to_ascii_lowercase())
        .as_deref()
    {
        Some("html") | Some("htm") => "text/html; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("js") | Some("mjs") => "application/javascript; charset=utf-8",
        Some("json") => "application/json; charset=utf-8",
        Some("svg") => "image/svg+xml",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("webp") => "image/webp",
        Some("woff2") => "font/woff2",
        _ => "application/octet-stream",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn navigation_same_origin_and_external() {
        let base = url::Url::parse("https://muse.ai/library/artifacts/abc").unwrap();
        let same = url::Url::parse("https://muse.ai/spaces/xyz").unwrap();
        let diff_host = url::Url::parse("https://example.com/page").unwrap();
        let diff_scheme = url::Url::parse("http://muse.ai/page").unwrap();
        let about = url::Url::parse("about:blank").unwrap();

        assert!(is_same_origin_or_internal(&base, &same));
        assert!(!is_same_origin_or_internal(&base, &diff_host));
        assert!(!is_same_origin_or_internal(&base, &diff_scheme));
        assert!(is_same_origin_or_internal(&base, &about));
    }
}
