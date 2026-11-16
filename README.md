# DesktopDashboards

任意の Web アプリ、AI アーティファクト（Muse Spaces 等）、ローカル HTML ファイルを Windows のデスクトップ（最背面／壁紙レイヤー）に複数並べて常駐表示できる、Tauri v2 + WebView2 製の軽量デスクトップガジェットアプリです。

## 主な機能（Phase 1 MVP 実装済み）

- **汎用 Web デスクトップガジェット**: Web URL およびローカル HTML ファイル（`widgetfile` カスタムプロトコル経由）をデスクトップウィジェットとして表示
- **セッション維持・分離**: 専用の WebView2 `user_data_dir`（`%LOCALAPPDATA%\DesktopDashboards\webview`）を使用し、他アプリとのロック競合を防ぎつつガジェット間でログイン状態を共有
- **外部アプリ連携・多重起動防止**: `tauri-plugin-single-instance` と CLI 引数によるシームレスな呼び出し
  - `DesktopDashboards.exe`（引数なし）: 登録済みガジェットを復元（既に起動中なら管理画面を前面表示）
  - `DesktopDashboards.exe --manager`: 管理画面を前面表示
  - `DesktopDashboards.exe --add-url <URL> [--title <NAME>]`: 指定 URL を追加・有効化して表示
  - `DesktopDashboards.exe --silent`: 管理画面を開かずトレイ常駐＋有効ガジェットのみ静かに起動
- **デスクトップ固定・ナビゲーション制御**: 枠なし・タスクバー非表示・最背面固定（`HWND_BOTTOM` + `WS_EX_TOOLWINDOW` + `WM_WINDOWPOSCHANGING`）、同一オリジン内遷移許可・外部リンクの既定ブラウザ起動
- **設定の原子的永続化**: `%APPDATA%\DesktopDashboards\config.json` への原子的書き込み（`.tmp` + flush + リネーム）および 1 世代バックアップ（`.bak`）フォールバック
- **管理画面 & システムトレイ常駐**: ガジェットの追加・表示切替・位置中央リセット・再読み込み・削除、Windows 起動時の自動起動（`--silent`）トグル

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
