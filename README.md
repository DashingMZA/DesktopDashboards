# DesktopDashboards

任意の Web アプリ、AI アーティファクト（Muse Spaces 等）、ローカル HTML ファイルを Windows のデスクトップ（最背面／壁紙レイヤー）に複数並べて常駐表示できる、Tauri v2 + WebView2 製の軽量デスクトップガジェットアプリです。

## 特徴（計画中を含む）

- **汎用 Web デスクトップガジェット**: URL やローカル HTML を登録するだけでデスクトップウィジェット化
- **セッション維持・分離**: 専用の WebView2 `user_data_dir`（`%LOCALAPPDATA%\DesktopDashboards\webview`）を使用し、他アプリとのロック競合を防ぎつつガジェット間でログイン状態を共有
- **外部アプリ連携**: `--add-url` / `--manager` / `--silent` などの CLI 引数と多重起動防止（`single-instance`）によるシームレスな呼び出し
- **デスクトップ固定**: 枠なし・タスクバー非表示・最背面固定（`HWND_BOTTOM` + `WS_EX_TOOLWINDOW`）

## 技術スタック

- **対象 OS**: Windows 10 / 11 (x64)
- **フレームワーク**: Tauri v2 (Rust) + WebView2 + HTML/CSS/JS

## 開発・ビルド方法

```bash
# 依存パッケージのインストール
npm install

# 開発モードで起動
npm run dev

# リリースビルド
npm run build
```

## ドキュメント

- [仕様書 (docs/SPEC.md)](docs/SPEC.md)
