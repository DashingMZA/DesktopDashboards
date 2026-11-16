/// Result of parsing CLI arguments.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CliOptions {
    /// Whether to open the manager window and bring it to the front (`--manager`).
    pub open_manager: bool,
    /// Whether to start quietly without opening the manager window (`--silent`).
    pub silent: bool,
    /// Target URL to add and display (`--add-url <URL>`).
    pub add_url: Option<String>,
    /// Widget title used with `--add-url` (`--title <NAME>`).
    pub title: Option<String>,
}

/// Parses CLI options from an argument slice that excludes the executable name.
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
            "Commuter Train Timetable",
        ]));
        assert_eq!(
            opts.add_url.as_deref(),
            Some("https://muse.ai/library/artifacts/sample")
        );
        assert_eq!(opts.title.as_deref(), Some("Commuter Train Timetable"));
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
