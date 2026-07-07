use hbb_common::regex::Regex;
use std::ops::Deref;

mod en;

pub const LANGS: &[(&str, &str)] = &[
    ("en", "English"),
];

pub(crate) fn cjk_ui_unavailable() -> bool {
    cfg!(all(
        target_os = "linux",
        target_arch = "aarch64",
        feature = "flutter"
    ))
}

pub(crate) fn is_cjk_lang(lang_or_locale: &str) -> bool {
    let lang = lang_or_locale
        .split(|c| c == '-' || c == '_')
        .next()
        .unwrap_or_default()
        .to_lowercase();
    matches!(lang.as_str(), "zh" | "ja" | "ko")
}

fn resolve_lang(saved_lang: &str, locale: &str, cjk_fallback: bool) -> String {
    let locale = locale.to_lowercase();
    let mut lang = saved_lang.to_lowercase();
    if cjk_fallback && is_cjk_lang(&lang) {
        return "en".to_owned();
    }
    if lang.is_empty() {
        // zh_CN on Linux, zh-Hans-CN on mac, zh_CN_#Hans on Android
        if locale.starts_with("zh") {
            lang = (if locale.contains("tw") {
                "zh-tw"
            } else {
                "zh-cn"
            })
            .to_owned();
        }
    }
    if lang.is_empty() {
        lang = locale
            .split("-")
            .next()
            .map(|x| x.split("_").next().unwrap_or_default())
            .unwrap_or_default()
            .to_owned();
    }
    if cjk_fallback && is_cjk_lang(&lang) {
        "en".to_owned()
    } else {
        lang
    }
}

#[cfg(not(any(target_os = "android", target_os = "ios")))]
pub fn translate(name: String) -> String {
    let locale = sys_locale::get_locale().unwrap_or_default();
    translate_locale(name, &locale)
}

pub fn translate_locale(name: String, locale: &str) -> String {
    let lang = resolve_lang(
        &hbb_common::config::LocalConfig::get_option("lang"),
        locale,
        cjk_ui_unavailable(),
    );
    let m = match lang.as_str() {
        _ => en::T.deref(),
    };
    let (name, placeholder_value) = extract_placeholder(&name);
    let replace = |s: &&str| {
        let mut s = s.to_string();
        if let Some(value) = placeholder_value.as_ref() {
            s = s.replace("{}", &value);
        }
        if !crate::is_rustdesk() {
            if s.contains("RustDesk")
                && !name.starts_with("upgrade_rustdesk_server_pro")
                && name != "powered_by_me"
            {
                let app_name = crate::get_app_name();
                if !app_name.contains("RustDesk") {
                    s = s.replace("RustDesk", &app_name);
                } else {
                    // https://github.com/rustdesk/rustdesk-server-pro/issues/845
                    // If app_name contains "RustDesk" (e.g., "RustDesk-Admin"), we need to avoid
                    // replacing "RustDesk" within the already-substituted app_name, which would
                    // cause duplication like "RustDesk-Admin" -> "RustDesk-Admin-Admin".
                    //
                    // app_name only contains alphanumeric and hyphen.
                    const PLACEHOLDER: &str = "#A-P-P-N-A-M-E#";
                    if !s.contains(PLACEHOLDER) {
                        s = s.replace(&app_name, PLACEHOLDER);
                        s = s.replace("RustDesk", &app_name);
                        s = s.replace(PLACEHOLDER, &app_name);
                    } else {
                        // It's very unlikely to reach here.
                        // Skip replacement to avoid incorrect result.
                    }
                }
            }
        }
        s
    };
    if let Some(v) = m.get(&name as &str) {
        if !v.is_empty() {
            return replace(v);
        }
    }
    if lang != "en" {
        if let Some(v) = en::T.get(&name as &str) {
            if !v.is_empty() {
                return replace(v);
            }
        }
    }
    replace(&name.as_str())
}

// Matching pattern is {}
// Write {value} in the UI and {} in the translation file
//
// Example:
// Write in the UI: translate("There are {24} hours in a day")
// Write in the translation file: ("There are {} hours in a day", "{} hours make up a day")
fn extract_placeholder(input: &str) -> (String, Option<String>) {
    if let Ok(re) = Regex::new(r#"\{(.*?)\}"#) {
        if let Some(captures) = re.captures(input) {
            if let Some(inner_match) = captures.get(1) {
                let name = re.replace(input, "{}").to_string();
                let value = inner_match.as_str().to_string();
                return (name, Some(value));
            }
        }
    }
    (input.to_string(), None)
}

mod test {
    #[test]
    fn test_extract_placeholders() {
        use super::extract_placeholder as f;

        assert_eq!(f(""), ("".to_string(), None));
        assert_eq!(
            f("{3} sessions"),
            ("{} sessions".to_string(), Some("3".to_string()))
        );
        assert_eq!(f(" } { "), (" } { ".to_string(), None));
        // Allow empty value
        assert_eq!(
            f("{} sessions"),
            ("{} sessions".to_string(), Some("".to_string()))
        );
        // Match only the first one
        assert_eq!(
            f("{2} times {4} makes {8}"),
            ("{} times {4} makes {8}".to_string(), Some("2".to_string()))
        );
    }

    #[test]
    fn test_resolve_lang_forces_english_for_saved_cjk_when_target_disables_cjk() {
        use super::resolve_lang as f;

        assert_eq!(f("zh-cn", "en-US", true), "en");
        assert_eq!(f("zh-tw", "en-US", true), "en");
        assert_eq!(f("ja", "en-US", true), "en");
        assert_eq!(f("ko", "en-US", true), "en");
    }

    #[test]
    fn test_resolve_lang_forces_english_for_cjk_locale_when_target_disables_cjk() {
        use super::resolve_lang as f;

        assert_eq!(f("", "zh_CN", true), "en");
        assert_eq!(f("", "ja-JP", true), "en");
        assert_eq!(f("", "ko_KR", true), "en");
    }

    #[test]
    fn test_resolve_lang_preserves_cjk_when_target_allows_cjk() {
        use super::resolve_lang as f;

        assert_eq!(f("zh-cn", "en-US", false), "zh-cn");
        assert_eq!(f("", "zh_TW", false), "zh-tw");
        assert_eq!(f("", "ja-JP", false), "ja");
    }
}
