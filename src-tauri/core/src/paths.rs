//! Де застосунок і консоль тримають свої невеликі несекретні файли (профіль з'єднання, файл сеансу).

use std::path::PathBuf;

/// Тека `info-desktop` у стандартній теці конфігурації ОС (середовище передається, щоб правило було перевірюване тестами).
pub fn config_dir_from(env: &dyn Fn(&str) -> Option<String>, os: &str) -> Result<PathBuf, String> {
    let base = match os {
        "windows" => env("APPDATA").map(PathBuf::from),
        "macos" => env("HOME").map(|h| PathBuf::from(h).join("Library").join("Application Support")),
        _ => env("XDG_CONFIG_HOME")
            .filter(|v| !v.trim().is_empty())
            .map(PathBuf::from)
            .or_else(|| env("HOME").map(|h| PathBuf::from(h).join(".config"))),
    };
    base.map(|b| b.join("info-desktop"))
        .ok_or_else(|| "Не вдалося визначити теку конфігурації".to_string())
}

pub fn current_os() -> &'static str {
    if cfg!(windows) {
        "windows"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else {
        "linux"
    }
}

pub fn config_dir() -> Result<PathBuf, String> {
    config_dir_from(&|k| std::env::var(k).ok(), current_os())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_dir_per_os() {
        let env = |k: &str| match k {
            "APPDATA" => Some("C:\\Users\\u\\AppData\\Roaming".to_string()),
            "HOME" => Some("/home/u".to_string()),
            _ => None,
        };
        assert_eq!(config_dir_from(&env, "windows").unwrap(), PathBuf::from("C:\\Users\\u\\AppData\\Roaming").join("info-desktop"));
        assert_eq!(config_dir_from(&env, "linux").unwrap(), PathBuf::from("/home/u/.config").join("info-desktop"));
        assert_eq!(
            config_dir_from(&env, "macos").unwrap(),
            PathBuf::from("/home/u").join("Library").join("Application Support").join("info-desktop")
        );
        let xdg = |k: &str| if k == "XDG_CONFIG_HOME" { Some("/x".to_string()) } else { None };
        assert_eq!(config_dir_from(&xdg, "linux").unwrap(), PathBuf::from("/x").join("info-desktop"));
        assert!(config_dir_from(&|_| None, "linux").is_err());
    }
}
