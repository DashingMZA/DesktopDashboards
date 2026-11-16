/// CLI 引数の解析結果。
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CliOptions {
    /// 管理画面ウィンドウを開いて前面表示するか (`--manager`)。
    pub open_manager: bool,
    /// 管理画面を開かず静かに起動するか (`--silent`)。
    pub silent: bool,
    /// 追加・表示する対象 URL (`--add-url <URL>`)。
    pub add_url: Option<String>,
    /// `--add-url` 時のガジェットタイトル (`--title <NAME>`)。
    pub title: Option<String>,
}

/// 実行ファイル名を除いた引数スライスから CLI オプションを解析する。
pub fn parse_args(args: &[String]) -> CliOptions {
    let mut opts = CliOptions::default();
    let mut i = 0;

    while i < args.len() {
        let arg = &args[i];
        if arg == "--manager" {
            opts.open_manager = true;
        } else if arg == "--silent" {
            opts.silent = true;
        } else if let Some(val) = arg.strip_prefix("--add-url=") {
            if !val.is_empty() {
                opts.add_url = Some(val.to_string());
            }
        } else if arg == "--add-url" {
            if let Some(next) = args.get(i + 1) {
                if !next.starts_with("--") {
                    opts.add_url = Some(next.clone());
                    i += 1;
                }
            }
        } else if let Some(val) = arg.strip_prefix("--title=") {
            if !val.is_empty() {
                opts.title = Some(val.to_string());
            }
        } else if arg == "--title" {
            if let Some(next) = args.get(i + 1) {
                if !next.starts_with("--") {
                    opts.title = Some(next.clone());
                    i += 1;
                }
            }
        }
        i += 1;
    }

    opts
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(items: &[&str]) -> Vec<String> {
        items.iter().map(|v| (*v).to_string()).collect()
    }

    #[test]
    fn parse_empty_args() {
        let opts = parse_args(&[]);
        assert_eq!(opts, CliOptions::default());
    }

    #[test]
    fn parse_manager_and_silent() {
        let opts = parse_args(&s(&["--manager", "--silent"]));
        assert!(opts.open_manager);
        assert!(opts.silent);
    }

    #[test]
    fn parse_add_url_with_title() {
        let opts = parse_args(&s(&[
            "--add-url",
            "https://muse.ai/library/artifacts/sample",
            "--title",
            "通勤電車 時刻表",
        ]));
        assert_eq!(
            opts.add_url.as_deref(),
            Some("https://muse.ai/library/artifacts/sample")
        );
        assert_eq!(opts.title.as_deref(), Some("通勤電車 時刻表"));
    }

    #[test]
    fn parse_equals_syntax() {
        let opts = parse_args(&s(&[
            "--add-url=https://example.com",
            "--title=Example",
        ]));
        assert_eq!(opts.add_url.as_deref(), Some("https://example.com"));
        assert_eq!(opts.title.as_deref(), Some("Example"));
    }
}
