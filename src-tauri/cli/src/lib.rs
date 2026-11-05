//! Консольний режим INFO Desktop: розбір команд, профіль з'єднання, коди завершення.
//! Обмін із сервером іде лише через спільний `transport` (ті самі правила, що й у вікні застосунку).

use info_desktop_core::transport;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub const API: &str = "/api/info-desktop/v1";

pub const EXIT_OK: i32 = 0;
pub const EXIT_FAILED: i32 = 1;
pub const EXIT_USAGE: i32 = 2;
pub const EXIT_NO_CONNECTION: i32 = 3;
pub const EXIT_REFUSED: i32 = 4;

pub const USAGE: &str = "INFO Desktop, консольний режим

Використання: info-desktop-cli [--profile ФАЙЛ] КОМАНДА

Команди:
  profile show                       показати збережений профіль з'єднання (без ключів)
  profile set --server АДРЕСА --port ПОРТ --device-id НОМЕР --ca-file ФАЙЛ
                                     зберегти профіль; ФАЙЛ містить значення server_ca_der з реєстраційного файлу
  status                             перевірка зв'язку із сервером
  channels                           список каналів
  queue list [--limit N] [--cursor N] [--channel КОД] [--bucket ГРУПА]
                                     сторінка редакційної черги

Ключ пристрою береться зі сховища ОС (той самий, що й у вікна застосунку) і ніколи не виводиться.
Коди завершення: 0 успіх, 1 помилка, 2 неправильні аргументи, 3 немає зв'язку або ключа, 4 сервер відмовив.";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Profile {
    pub server: String,
    pub port: u16,
    pub device_id: String,
    pub server_ca_der: String,
}

impl Profile {
    pub fn url(&self, path: &str) -> String {
        format!("https://{}:{}{}{}", self.server.trim(), self.port, API, path)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    ProfileShow,
    ProfileSet { server: String, port: u16, device_id: String, ca_file: String },
    Status,
    Channels,
    QueueList { limit: u32, cursor: u32, channel: String, bucket: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cli {
    pub profile: Option<String>,
    pub command: Command,
}

fn take_opt(rest: &mut Vec<String>, name: &str) -> Result<Option<String>, String> {
    if let Some(i) = rest.iter().position(|a| a == name) {
        if i + 1 >= rest.len() {
            return Err(format!("Після {name} потрібне значення"));
        }
        let value = rest.remove(i + 1);
        rest.remove(i);
        return Ok(Some(value));
    }
    Ok(None)
}

fn need(value: Option<String>, name: &str) -> Result<String, String> {
    value.ok_or_else(|| format!("Потрібен параметр {name}"))
}

fn code_value(value: &str, name: &str) -> Result<String, String> {
    let ok = !value.is_empty()
        && value.len() <= 64
        && value.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == '.');
    if ok {
        Ok(value.to_string())
    } else {
        Err(format!("Некоректне значення {name}: дозволені літери, цифри, _ - ."))
    }
}

fn number(value: Option<String>, name: &str, default: u32) -> Result<u32, String> {
    match value {
        None => Ok(default),
        Some(v) => v.parse::<u32>().map_err(|_| format!("{name} має бути невід'ємним числом")),
    }
}

pub fn parse_args(args: &[String]) -> Result<Cli, String> {
    let mut rest: Vec<String> = args.to_vec();
    let profile = take_opt(&mut rest, "--profile")?;
    let first = rest.first().map(String::as_str).unwrap_or("");
    let command = match first {
        "profile" => match rest.get(1).map(String::as_str) {
            Some("show") => {
                rest.drain(..2);
                Command::ProfileShow
            }
            Some("set") => {
                rest.drain(..2);
                let server = need(take_opt(&mut rest, "--server")?, "--server")?;
                let port = need(take_opt(&mut rest, "--port")?, "--port")?
                    .parse::<u16>()
                    .map_err(|_| "--port має бути числом 1..65535".to_string())?;
                if port == 0 {
                    return Err("--port має бути числом 1..65535".to_string());
                }
                let device_id = need(take_opt(&mut rest, "--device-id")?, "--device-id")?;
                let ca_file = need(take_opt(&mut rest, "--ca-file")?, "--ca-file")?;
                if server.contains('/') || server.contains(' ') {
                    return Err("--server має бути іменем або IP, не адресою".to_string());
                }
                Command::ProfileSet { server, port, device_id, ca_file }
            }
            _ => return Err("Невідома підкоманда profile (show або set)".to_string()),
        },
        "status" => {
            rest.drain(..1);
            Command::Status
        }
        "channels" => {
            rest.drain(..1);
            Command::Channels
        }
        "queue" => match rest.get(1).map(String::as_str) {
            Some("list") => {
                rest.drain(..2);
                let limit = number(take_opt(&mut rest, "--limit")?, "--limit", 100)?;
                let cursor = number(take_opt(&mut rest, "--cursor")?, "--cursor", 0)?;
                let channel = match take_opt(&mut rest, "--channel")? {
                    Some(v) => code_value(&v, "--channel")?,
                    None => String::new(),
                };
                let bucket = match take_opt(&mut rest, "--bucket")? {
                    Some(v) => code_value(&v, "--bucket")?,
                    None => "all".to_string(),
                };
                if limit == 0 || limit > 500 {
                    return Err("--limit має бути від 1 до 500".to_string());
                }
                Command::QueueList { limit, cursor, channel, bucket }
            }
            _ => return Err("Невідома підкоманда queue (list)".to_string()),
        },
        "" => return Err("Не вказано команду".to_string()),
        other => return Err(format!("Невідома команда: {other}")),
    };
    if let Some(extra) = rest.first() {
        return Err(format!("Зайвий аргумент: {extra}"));
    }
    Ok(Cli { profile, command })
}

/// Шлях запиту до сервера для команд, що читають (без початку адреси).
pub fn request_path(command: &Command) -> Option<String> {
    match command {
        Command::Status => Some("/status".to_string()),
        Command::Channels => Some("/channels".to_string()),
        Command::QueueList { limit, cursor, channel, bucket } => {
            let mut path = format!("/queue?limit={limit}&cursor={cursor}&bucket={bucket}");
            if !channel.is_empty() {
                path.push_str(&format!("&channel={channel}"));
            }
            Some(path)
        }
        _ => None,
    }
}

pub fn exit_for_status(status: u16) -> i32 {
    match status {
        200..=299 => EXIT_OK,
        400..=499 => EXIT_REFUSED,
        _ => EXIT_FAILED,
    }
}

/// Де лежить файл профілю: явний шлях, змінна INFO_DESKTOP_PROFILE або стандартна тека конфігурації ОС.
pub fn profile_path_from(
    explicit: Option<&str>,
    env: &dyn Fn(&str) -> Option<String>,
    os: &str,
) -> Result<PathBuf, String> {
    if let Some(p) = explicit {
        return Ok(PathBuf::from(p));
    }
    if let Some(p) = env("INFO_DESKTOP_PROFILE").filter(|v| !v.trim().is_empty()) {
        return Ok(PathBuf::from(p));
    }
    let base = match os {
        "windows" => env("APPDATA").map(PathBuf::from),
        "macos" => env("HOME").map(|h| PathBuf::from(h).join("Library").join("Application Support")),
        _ => env("XDG_CONFIG_HOME")
            .filter(|v| !v.trim().is_empty())
            .map(PathBuf::from)
            .or_else(|| env("HOME").map(|h| PathBuf::from(h).join(".config"))),
    };
    base.map(|b| b.join("info-desktop").join("connection.json"))
        .ok_or_else(|| "Не вдалося визначити теку конфігурації; вкажіть --profile ФАЙЛ".to_string())
}

pub fn profile_path(explicit: Option<&str>) -> Result<PathBuf, String> {
    let os = if cfg!(windows) {
        "windows"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else {
        "linux"
    };
    profile_path_from(explicit, &|k| std::env::var(k).ok(), os)
}

pub fn load_profile(path: &std::path::Path) -> Result<Profile, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|_| format!("Немає профілю з'єднання: {} (команда profile set)", path.display()))?;
    serde_json::from_str(&text).map_err(|_| "Профіль з'єднання пошкоджений".to_string())
}

pub fn save_profile(path: &std::path::Path, profile: &Profile) -> Result<(), String> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    let text = serde_json::to_string_pretty(profile).map_err(|e| e.to_string())?;
    let tmp = path.with_extension("json.tmp");
    std::fs::write(&tmp, text).map_err(|e| e.to_string())?;
    std::fs::rename(&tmp, path).map_err(|e| e.to_string())
}

fn pretty(body: &str) -> String {
    match serde_json::from_str::<serde_json::Value>(body) {
        Ok(v) => serde_json::to_string_pretty(&v).unwrap_or_else(|_| body.to_string()),
        Err(_) => body.to_string(),
    }
}

/// Виконує команду. Повертає код завершення й текст (для успіху його друкують у stdout, для помилки в stderr).
pub async fn execute(cli: &Cli) -> (i32, String) {
    let path = match profile_path(cli.profile.as_deref()) {
        Ok(p) => p,
        Err(e) => return (EXIT_NO_CONNECTION, e),
    };
    match &cli.command {
        Command::ProfileShow => match load_profile(&path) {
            Ok(p) => (
                EXIT_OK,
                serde_json::json!({"file": path.display().to_string(), "server": p.server, "port": p.port, "device_id": p.device_id}).to_string(),
            ),
            Err(e) => (EXIT_NO_CONNECTION, e),
        },
        Command::ProfileSet { server, port, device_id, ca_file } => {
            let ca = match std::fs::read_to_string(ca_file) {
                Ok(t) => t.trim().to_string(),
                Err(_) => return (EXIT_FAILED, format!("Не вдалося прочитати файл сертифіката: {ca_file}")),
            };
            if let Err(e) = transport::client_with_cert(&ca) {
                return (EXIT_FAILED, e);
            }
            let profile = Profile { server: server.clone(), port: *port, device_id: device_id.clone(), server_ca_der: ca };
            match save_profile(&path, &profile) {
                Ok(()) => (EXIT_OK, format!("Профіль збережено: {}", path.display())),
                Err(e) => (EXIT_FAILED, e),
            }
        }
        read => {
            let profile = match load_profile(&path) {
                Ok(p) => p,
                Err(e) => return (EXIT_NO_CONNECTION, e),
            };
            let request = match request_path(read) {
                Some(r) => r,
                None => return (EXIT_USAGE, "Команду не підтримано".to_string()),
            };
            match transport::device_get(&profile.url(&request), &profile.device_id, &profile.server_ca_der).await {
                Ok(resp) => (exit_for_status(resp.status), pretty(&resp.body)),
                Err(e) => (EXIT_NO_CONNECTION, e),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn parses_simple_commands_and_global_profile_flag() {
        let cli = parse_args(&args(&["--profile", "p.json", "status"])).unwrap();
        assert_eq!(cli, Cli { profile: Some("p.json".into()), command: Command::Status });
        assert_eq!(parse_args(&args(&["channels"])).unwrap().command, Command::Channels);
        assert_eq!(parse_args(&args(&["profile", "show"])).unwrap().command, Command::ProfileShow);
    }

    #[test]
    fn parses_queue_list_with_defaults_and_options() {
        assert_eq!(
            parse_args(&args(&["queue", "list"])).unwrap().command,
            Command::QueueList { limit: 100, cursor: 0, channel: String::new(), bucket: "all".into() }
        );
        let cmd = parse_args(&args(&["queue", "list", "--limit", "20", "--channel", "politics", "--bucket", "review", "--cursor", "40"])).unwrap().command;
        assert_eq!(cmd, Command::QueueList { limit: 20, cursor: 40, channel: "politics".into(), bucket: "review".into() });
        assert_eq!(request_path(&cmd).unwrap(), "/queue?limit=20&cursor=40&bucket=review&channel=politics");
    }

    #[test]
    fn rejects_bad_arguments() {
        for bad in [
            vec![],
            vec!["nope"],
            vec!["status", "extra"],
            vec!["queue", "list", "--limit", "0"],
            vec!["queue", "list", "--limit", "abc"],
            vec!["queue", "list", "--channel", "a/b"],
            vec!["queue", "list", "--channel"],
            vec!["profile", "set", "--server", "https://x/", "--port", "1", "--device-id", "d", "--ca-file", "f"],
            vec!["profile", "set", "--server", "x", "--port", "0", "--device-id", "d", "--ca-file", "f"],
            vec!["profile", "set", "--server", "x", "--port", "1"],
        ] {
            assert!(parse_args(&args(&bad)).is_err(), "має бути помилка: {bad:?}");
        }
    }

    #[test]
    fn profile_set_is_parsed() {
        let cmd = parse_args(&args(&["profile", "set", "--server", "10.8.0.1", "--port", "47443", "--device-id", "dev-1", "--ca-file", "ca.txt"])).unwrap().command;
        assert_eq!(cmd, Command::ProfileSet { server: "10.8.0.1".into(), port: 47443, device_id: "dev-1".into(), ca_file: "ca.txt".into() });
    }

    #[test]
    fn request_paths_for_read_commands_only() {
        assert_eq!(request_path(&Command::Status).unwrap(), "/status");
        assert_eq!(request_path(&Command::Channels).unwrap(), "/channels");
        assert!(request_path(&Command::ProfileShow).is_none());
    }

    #[test]
    fn exit_codes_follow_documented_meaning() {
        assert_eq!(exit_for_status(200), EXIT_OK);
        assert_eq!(exit_for_status(204), EXIT_OK);
        assert_eq!(exit_for_status(403), EXIT_REFUSED);
        assert_eq!(exit_for_status(409), EXIT_REFUSED);
        assert_eq!(exit_for_status(503), EXIT_FAILED);
    }

    #[test]
    fn profile_url_uses_https_api_base() {
        let p = Profile { server: " 10.8.0.1 ".into(), port: 47443, device_id: "d".into(), server_ca_der: "x".into() };
        assert_eq!(p.url("/status"), "https://10.8.0.1:47443/api/info-desktop/v1/status");
    }

    #[test]
    fn profile_path_rules_per_os() {
        let env = |k: &str| match k {
            "APPDATA" => Some("C:\\Users\\u\\AppData\\Roaming".to_string()),
            "HOME" => Some("/home/u".to_string()),
            _ => None,
        };
        assert_eq!(profile_path_from(Some("x.json"), &env, "linux").unwrap(), PathBuf::from("x.json"));
        assert_eq!(
            profile_path_from(None, &env, "windows").unwrap(),
            PathBuf::from("C:\\Users\\u\\AppData\\Roaming").join("info-desktop").join("connection.json")
        );
        assert_eq!(profile_path_from(None, &env, "linux").unwrap(), PathBuf::from("/home/u/.config").join("info-desktop").join("connection.json"));
        assert!(profile_path_from(None, &|_| None, "linux").is_err());
        let with_override = |k: &str| if k == "INFO_DESKTOP_PROFILE" { Some("/tmp/p.json".to_string()) } else { None };
        assert_eq!(profile_path_from(None, &with_override, "linux").unwrap(), PathBuf::from("/tmp/p.json"));
    }

    #[test]
    fn profile_round_trips_through_file_without_secrets() {
        let dir = std::env::temp_dir().join(format!("info-desktop-cli-test-{}", std::process::id()));
        let file = dir.join("connection.json");
        let p = Profile { server: "10.8.0.1".into(), port: 47443, device_id: "dev-1".into(), server_ca_der: "QUFB".into() };
        save_profile(&file, &p).unwrap();
        assert_eq!(load_profile(&file).unwrap(), p);
        let text = std::fs::read_to_string(&file).unwrap();
        assert!(!text.contains("secret"));
        std::fs::remove_dir_all(&dir).ok();
        assert!(load_profile(&file).is_err());
    }
}
