//! Консольний режим INFO Desktop: розбір команд, профіль з'єднання, коди завершення.
//! Обмін із сервером іде лише через спільний `transport` (ті самі правила, що й у вікні застосунку).

use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
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

З'єднання:
  profile show                       показати профіль з'єднання (без ключів)
  profile set --server АДРЕСА --port ПОРТ --device-id НОМЕР --ca-file ФАЙЛ
                                     зберегти профіль; ФАЙЛ містить значення server_ca_der з реєстраційного файлу
  status | channels                  перевірка зв'язку | список каналів

Черга:
  queue list [--limit N] [--cursor N] [--channel КОД] [--bucket ГРУПА]
  queue show НОМЕР                   матеріал із джерелами
  why НОМЕР --channel КОД [--scope route|auto|digest|instant|relevance]
                                     чому матеріал потрапив (не потрапив) у канал
  create --channel КОД --title ТЕКСТ (--text ТЕКСТ | --text-file ФАЙЛ) [--url АДРЕСА]
  save НОМЕР --title ТЕКСТ (--text ТЕКСТ | --text-file ФАЙЛ) [--channel КОД]
  skip НОМЕР | defer НОМЕР [--minutes N] | publish НОМЕР

Налаштування (XLSX):
  settings status                    чи є перевірений файл, відбиток, чи була проба
  settings download --out ФАЙЛ       завантажити поточний XLSX (існуючий файл не перезаписується)
  settings check ФАЙЛ                перевірити файл без застосування
  settings probe [--hours N]         запустити пробу «що зміниться»; settings probe-status показує результат
  settings apply [--without-probe] [--yes]
                                     застосувати перевірений файл; без --yes лише показати, що буде застосовано
  settings rollback [--yes]          відкотити останнє застосування; без --yes лише повідомити

Теми:
  themes list
  themes test (--words СЛОВА | --words-file ФАЙЛ) (--text ТЕКСТ | --text-file ФАЙЛ)
  themes save --topic КОД (--words СЛОВА | --words-file ФАЙЛ)
                                     СЛОВА: через кому; у файлі по одному в рядку

Ключ пристрою береться зі сховища ОС (той самий, що й у вікна застосунку) і ніколи не виводиться.
Підтвердження --yes потрібне лише для застосування й відкату налаштувань.
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
pub enum Text {
    Inline(String),
    File(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    ProfileShow,
    ProfileSet { server: String, port: u16, device_id: String, ca_file: String },
    Status,
    Channels,
    QueueList { limit: u32, cursor: u32, channel: String, bucket: String },
    QueueShow { id: u64 },
    Why { id: u64, channel: String, scope: String },
    Create { channel: String, title: String, text: Text, url: String },
    Save { id: u64, title: String, text: Text, channel: Option<String> },
    Skip { id: u64 },
    Defer { id: u64, minutes: u32 },
    Publish { id: u64 },
    SettingsStatus,
    SettingsDownload { out: String },
    SettingsCheck { file: String },
    SettingsProbe { hours: u32 },
    SettingsProbeStatus,
    SettingsApply { without_probe: bool, yes: bool },
    SettingsRollback { yes: bool },
    ThemesList,
    ThemesTest { words: Text, text: Text },
    ThemesSave { topic: String, words: Text },
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

fn take_flag(rest: &mut Vec<String>, name: &str) -> bool {
    if let Some(i) = rest.iter().position(|a| a == name) {
        rest.remove(i);
        return true;
    }
    false
}

fn pop(rest: &mut Vec<String>) -> Option<String> {
    if rest.is_empty() {
        None
    } else {
        Some(rest.remove(0))
    }
}

fn take_id(rest: &mut Vec<String>) -> Result<u64, String> {
    let raw = pop(rest).ok_or_else(|| "Потрібен номер матеріалу".to_string())?;
    raw.parse::<u64>().map_err(|_| format!("Некоректний номер матеріалу: {raw}"))
}

fn need(value: Option<String>, name: &str) -> Result<String, String> {
    value.ok_or_else(|| format!("Потрібен параметр {name}"))
}

fn take_either(rest: &mut Vec<String>, inline: &str, file: &str) -> Result<Text, String> {
    let a = take_opt(rest, inline)?;
    let b = take_opt(rest, file)?;
    match (a, b) {
        (Some(v), None) => Ok(Text::Inline(v)),
        (None, Some(f)) => Ok(Text::File(f)),
        (None, None) => Err(format!("Потрібен {inline} або {file}")),
        _ => Err(format!("Вкажіть лише одне: {inline} або {file}")),
    }
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

fn number(value: Option<String>, name: &str, default: u32, min: u32, max: u32) -> Result<u32, String> {
    let n = match value {
        None => default,
        Some(v) => v.parse::<u32>().map_err(|_| format!("{name} має бути числом"))?,
    };
    if n < min || n > max {
        return Err(format!("{name} має бути від {min} до {max}"));
    }
    Ok(n)
}

pub fn parse_args(args: &[String]) -> Result<Cli, String> {
    let mut rest: Vec<String> = args.to_vec();
    let profile = take_opt(&mut rest, "--profile")?;
    let first = pop(&mut rest).unwrap_or_default();
    let command = match first.as_str() {
        "profile" => match pop(&mut rest).as_deref() {
            Some("show") => Command::ProfileShow,
            Some("set") => {
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
        "status" => Command::Status,
        "channels" => Command::Channels,
        "queue" => match pop(&mut rest).as_deref() {
            Some("list") => {
                let limit = number(take_opt(&mut rest, "--limit")?, "--limit", 100, 1, 500)?;
                let cursor = number(take_opt(&mut rest, "--cursor")?, "--cursor", 0, 0, u32::MAX)?;
                let channel = match take_opt(&mut rest, "--channel")? {
                    Some(v) => code_value(&v, "--channel")?,
                    None => String::new(),
                };
                let bucket = match take_opt(&mut rest, "--bucket")? {
                    Some(v) => code_value(&v, "--bucket")?,
                    None => "all".to_string(),
                };
                Command::QueueList { limit, cursor, channel, bucket }
            }
            Some("show") => Command::QueueShow { id: take_id(&mut rest)? },
            _ => return Err("Невідома підкоманда queue (list або show)".to_string()),
        },
        "why" => {
            let id = take_id(&mut rest)?;
            let channel = code_value(&need(take_opt(&mut rest, "--channel")?, "--channel")?, "--channel")?;
            let scope = take_opt(&mut rest, "--scope")?.unwrap_or_else(|| "route".to_string());
            if !["route", "auto", "digest", "instant", "relevance"].contains(&scope.as_str()) {
                return Err("--scope: route, auto, digest, instant або relevance".to_string());
            }
            Command::Why { id, channel, scope }
        }
        "create" => {
            let channel = code_value(&need(take_opt(&mut rest, "--channel")?, "--channel")?, "--channel")?;
            let title = need(take_opt(&mut rest, "--title")?, "--title")?;
            let text = take_either(&mut rest, "--text", "--text-file")?;
            let url = take_opt(&mut rest, "--url")?.unwrap_or_default();
            Command::Create { channel, title, text, url }
        }
        "save" => {
            let id = take_id(&mut rest)?;
            let title = need(take_opt(&mut rest, "--title")?, "--title")?;
            let text = take_either(&mut rest, "--text", "--text-file")?;
            let channel = match take_opt(&mut rest, "--channel")? {
                Some(v) => Some(code_value(&v, "--channel")?),
                None => None,
            };
            Command::Save { id, title, text, channel }
        }
        "skip" => Command::Skip { id: take_id(&mut rest)? },
        "publish" => Command::Publish { id: take_id(&mut rest)? },
        "defer" => {
            let id = take_id(&mut rest)?;
            let minutes = number(take_opt(&mut rest, "--minutes")?, "--minutes", 60, 1, 1440)?;
            Command::Defer { id, minutes }
        }
        "settings" => match pop(&mut rest).as_deref() {
            Some("status") => Command::SettingsStatus,
            Some("download") => Command::SettingsDownload { out: need(take_opt(&mut rest, "--out")?, "--out")? },
            Some("check") => Command::SettingsCheck { file: pop(&mut rest).ok_or_else(|| "Потрібен файл XLSX".to_string())? },
            Some("probe") => Command::SettingsProbe { hours: number(take_opt(&mut rest, "--hours")?, "--hours", 48, 1, 720)? },
            Some("probe-status") => Command::SettingsProbeStatus,
            Some("apply") => {
                let yes = take_flag(&mut rest, "--yes");
                let without_probe = take_flag(&mut rest, "--without-probe");
                Command::SettingsApply { without_probe, yes }
            }
            Some("rollback") => Command::SettingsRollback { yes: take_flag(&mut rest, "--yes") },
            _ => return Err("Невідома підкоманда settings".to_string()),
        },
        "themes" => match pop(&mut rest).as_deref() {
            Some("list") => Command::ThemesList,
            Some("test") => {
                let words = take_either(&mut rest, "--words", "--words-file")?;
                let text = take_either(&mut rest, "--text", "--text-file")?;
                Command::ThemesTest { words, text }
            }
            Some("save") => {
                let topic = code_value(&need(take_opt(&mut rest, "--topic")?, "--topic")?, "--topic")?;
                let words = take_either(&mut rest, "--words", "--words-file")?;
                Command::ThemesSave { topic, words }
            }
            _ => return Err("Невідома підкоманда themes (list, test, save)".to_string()),
        },
        "" => return Err("Не вказано команду".to_string()),
        other => return Err(format!("Невідома команда: {other}")),
    };
    if let Some(extra) = rest.first() {
        return Err(format!("Зайвий аргумент: {extra}"));
    }
    Ok(Cli { profile, command })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Req {
    Get(String),
    Post(String, String),
}

fn resolve(text: &Text, read: &dyn Fn(&str) -> Result<String, String>) -> Result<String, String> {
    match text {
        Text::Inline(v) => Ok(v.clone()),
        Text::File(f) => read(f).map(|t| t.trim().to_string()),
    }
}

/// Слова з рядка через кому або з файлу по одному в рядку.
pub fn split_words(raw: &str) -> Vec<String> {
    raw.split(|c| c == ',' || c == '\n')
        .map(|w| w.trim().to_string())
        .filter(|w| !w.is_empty())
        .collect()
}

/// Запит для команд, які зводяться до одного звернення до сервера; `None` для тих, що потребують кількох кроків або працюють локально.
pub fn build_request(command: &Command, read: &dyn Fn(&str) -> Result<String, String>) -> Result<Option<Req>, String> {
    let post = |path: String, body: serde_json::Value| Ok(Some(Req::Post(path, body.to_string())));
    match command {
        Command::Status => Ok(Some(Req::Get("/status".to_string()))),
        Command::Channels => Ok(Some(Req::Get("/channels".to_string()))),
        Command::QueueList { limit, cursor, channel, bucket } => {
            let mut path = format!("/queue?limit={limit}&cursor={cursor}&bucket={bucket}");
            if !channel.is_empty() {
                path.push_str(&format!("&channel={channel}"));
            }
            Ok(Some(Req::Get(path)))
        }
        Command::QueueShow { id } => Ok(Some(Req::Get(format!("/queue/{id}")))),
        Command::Why { id, channel, scope } => Ok(Some(Req::Get(format!("/queue/{id}/why?channel={channel}&scope={scope}")))),
        Command::Create { channel, title, text, url } => {
            let draft = resolve(text, read)?;
            post("/queue".to_string(), serde_json::json!({"channel": channel, "title": title, "draft_text": draft, "url": url}))
        }
        Command::Save { id, title, text, channel } => {
            let draft = resolve(text, read)?;
            let mut body = serde_json::json!({"title": title, "draft_text": draft});
            if let Some(c) = channel {
                body["channel"] = serde_json::Value::String(c.clone());
            }
            post(format!("/queue/{id}/save"), body)
        }
        Command::Skip { id } => post(format!("/queue/{id}/skip"), serde_json::json!({})),
        Command::Defer { id, minutes } => post(format!("/queue/{id}/defer"), serde_json::json!({"minutes": minutes})),
        Command::Publish { id } => post(format!("/queue/{id}/publish"), serde_json::json!({})),
        Command::SettingsStatus => Ok(Some(Req::Get("/settings/status".to_string()))),
        Command::SettingsProbeStatus => Ok(Some(Req::Get("/settings/probe".to_string()))),
        Command::SettingsProbe { hours } => post("/settings/probe".to_string(), serde_json::json!({"hours": hours})),
        Command::ThemesList => Ok(Some(Req::Get("/themes".to_string()))),
        Command::ThemesTest { words, text } => {
            let words = split_words(&resolve(words, read)?);
            let text = resolve(text, read)?;
            post("/themes/test".to_string(), serde_json::json!({"words": words, "text": text}))
        }
        Command::ThemesSave { topic, words } => {
            let words = split_words(&resolve(words, read)?);
            post("/themes/save".to_string(), serde_json::json!({"topic": topic, "words": words}))
        }
        _ => Ok(None),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApplyPlan {
    NoPending,
    NeedProbe,
    Preview(String),
    Go(String),
}

/// Рішення про застосування XLSX за відповіддю `settings status`: застосовуємо рівно той файл (за відбитком), що перевірений, і лише після проби.
pub fn apply_decision(status_body: &str, without_probe: bool, yes: bool) -> Result<ApplyPlan, String> {
    let v: serde_json::Value = serde_json::from_str(status_body).map_err(|_| "Некоректна відповідь сервера".to_string())?;
    let digest = v.get("pending_digest").and_then(|x| x.as_str()).unwrap_or("").to_string();
    if digest.is_empty() {
        return Ok(ApplyPlan::NoPending);
    }
    let probe_ok = v.get("probe_matches_pending").and_then(|x| x.as_bool()).unwrap_or(false);
    if !probe_ok && !without_probe {
        return Ok(ApplyPlan::NeedProbe);
    }
    Ok(if yes { ApplyPlan::Go(digest) } else { ApplyPlan::Preview(digest) })
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

fn finish(result: Result<transport::ReadonlyResponse, String>) -> (i32, String) {
    match result {
        Ok(resp) => (exit_for_status(resp.status), pretty(&resp.body)),
        Err(e) => (EXIT_NO_CONNECTION, e),
    }
}

async fn call(profile: &Profile, req: &Req) -> (i32, String) {
    match req {
        Req::Get(path) => finish(transport::device_get(&profile.url(path), &profile.device_id, &profile.server_ca_der).await),
        Req::Post(path, body) => {
            finish(transport::device_post(&profile.url(path), &profile.device_id, &profile.server_ca_der, body.clone()).await)
        }
    }
}

async fn run_remote(command: &Command, profile: &Profile) -> (i32, String) {
    let read = |p: &str| std::fs::read_to_string(p).map_err(|_| format!("Не вдалося прочитати файл: {p}"));
    match build_request(command, &read) {
        Err(e) => return (EXIT_FAILED, e),
        Ok(Some(req)) => return call(profile, &req).await,
        Ok(None) => {}
    }
    match command {
        Command::SettingsDownload { out } => {
            if std::path::Path::new(out).exists() {
                return (EXIT_FAILED, format!("Файл уже існує, його не перезаписано: {out}"));
            }
            let (code, body) = call(profile, &Req::Get("/settings/xlsx".to_string())).await;
            if code != EXIT_OK {
                return (code, body);
            }
            let value: serde_json::Value = match serde_json::from_str(&body) {
                Ok(v) => v,
                Err(_) => return (EXIT_FAILED, "Некоректна відповідь сервера".to_string()),
            };
            let encoded = value.get("base64").and_then(|x| x.as_str()).unwrap_or("");
            let bytes = match BASE64.decode(encoded) {
                Ok(b) => b,
                Err(_) => return (EXIT_FAILED, "Сервер повернув некоректний файл".to_string()),
            };
            match std::fs::write(out, &bytes) {
                Ok(()) => (EXIT_OK, serde_json::json!({"file": out, "size": bytes.len()}).to_string()),
                Err(e) => (EXIT_FAILED, e.to_string()),
            }
        }
        Command::SettingsCheck { file } => {
            let bytes = match std::fs::read(file) {
                Ok(b) => b,
                Err(_) => return (EXIT_FAILED, format!("Не вдалося прочитати файл: {file}")),
            };
            let body = serde_json::json!({"base64": BASE64.encode(bytes)}).to_string();
            call(profile, &Req::Post("/settings/xlsx/check".to_string(), body)).await
        }
        Command::SettingsApply { without_probe, yes } => {
            let (code, status_body) = call(profile, &Req::Get("/settings/status".to_string())).await;
            if code != EXIT_OK {
                return (code, status_body);
            }
            match apply_decision(&status_body, *without_probe, *yes) {
                Err(e) => (EXIT_FAILED, e),
                Ok(ApplyPlan::NoPending) => (EXIT_FAILED, "Немає перевіреного файлу: спершу settings check ФАЙЛ".to_string()),
                Ok(ApplyPlan::NeedProbe) => (
                    EXIT_FAILED,
                    "Проба «що зміниться» для цього файлу не виконана: settings probe, потім settings probe-status; або додайте --without-probe".to_string(),
                ),
                Ok(ApplyPlan::Preview(digest)) => (
                    EXIT_OK,
                    format!("Було б застосовано перевірений файл із відбитком {digest}. Додайте --yes, щоб застосувати."),
                ),
                Ok(ApplyPlan::Go(digest)) => {
                    let body = serde_json::json!({"expected_digest": digest}).to_string();
                    call(profile, &Req::Post("/settings/xlsx/apply".to_string(), body)).await
                }
            }
        }
        Command::SettingsRollback { yes } => {
            if !*yes {
                return (EXIT_OK, "Було б повернуто стан перед останнім застосуванням. Додайте --yes, щоб відкотити.".to_string());
            }
            call(profile, &Req::Post("/settings/xlsx/rollback".to_string(), "{}".to_string())).await
        }
        _ => (EXIT_USAGE, "Команду не підтримано".to_string()),
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
        other => match load_profile(&path) {
            Ok(profile) => run_remote(other, &profile).await,
            Err(e) => (EXIT_NO_CONNECTION, e),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| s.to_string()).collect()
    }

    fn none(_: &str) -> Result<String, String> {
        Err("немає файлів".to_string())
    }

    fn fake_files(path: &str) -> Result<String, String> {
        match path {
            "text.txt" => Ok("  Текст допису  \n".to_string()),
            "words.txt" => Ok("слово\nдруге, третє\n".to_string()),
            _ => Err("немає файлу".to_string()),
        }
    }

    fn req(a: &[&str]) -> Req {
        build_request(&parse_args(&args(a)).unwrap().command, &none).unwrap().unwrap()
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
        let a = ["queue", "list", "--limit", "20", "--channel", "politics", "--bucket", "review", "--cursor", "40"];
        assert_eq!(
            parse_args(&args(&a)).unwrap().command,
            Command::QueueList { limit: 20, cursor: 40, channel: "politics".into(), bucket: "review".into() }
        );
        assert_eq!(req(&a), Req::Get("/queue?limit=20&cursor=40&bucket=review&channel=politics".to_string()));
    }

    #[test]
    fn rejects_bad_arguments() {
        let bad_list: Vec<Vec<&str>> = vec![
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
        ];
        for bad in bad_list {
            assert!(parse_args(&args(&bad)).is_err(), "має бути помилка: {bad:?}");
        }
    }

    #[test]
    fn profile_set_is_parsed() {
        let a = ["profile", "set", "--server", "10.8.0.1", "--port", "47443", "--device-id", "dev-1", "--ca-file", "ca.txt"];
        assert_eq!(
            parse_args(&args(&a)).unwrap().command,
            Command::ProfileSet { server: "10.8.0.1".into(), port: 47443, device_id: "dev-1".into(), ca_file: "ca.txt".into() }
        );
    }

    #[test]
    fn parses_item_commands() {
        assert_eq!(parse_args(&args(&["queue", "show", "7"])).unwrap().command, Command::QueueShow { id: 7 });
        assert_eq!(
            parse_args(&args(&["why", "7", "--channel", "politics"])).unwrap().command,
            Command::Why { id: 7, channel: "politics".into(), scope: "route".into() }
        );
        assert!(parse_args(&args(&["why", "7", "--channel", "politics", "--scope", "bad"])).is_err());
        assert!(parse_args(&args(&["why", "x", "--channel", "politics"])).is_err());
        assert_eq!(parse_args(&args(&["defer", "5"])).unwrap().command, Command::Defer { id: 5, minutes: 60 });
        assert_eq!(parse_args(&args(&["defer", "5", "--minutes", "30"])).unwrap().command, Command::Defer { id: 5, minutes: 30 });
        assert!(parse_args(&args(&["defer", "5", "--minutes", "5000"])).is_err());
        assert_eq!(parse_args(&args(&["publish", "9"])).unwrap().command, Command::Publish { id: 9 });
        assert_eq!(parse_args(&args(&["skip", "9"])).unwrap().command, Command::Skip { id: 9 });
        assert!(parse_args(&args(&["skip"])).is_err());
    }

    #[test]
    fn create_and_save_build_expected_requests() {
        let a = ["create", "--channel", "politics", "--title", "Заголовок", "--text-file", "text.txt", "--url", "https://x.test/a"];
        let cmd = parse_args(&args(&a)).unwrap().command;
        match build_request(&cmd, &fake_files).unwrap().unwrap() {
            Req::Post(path, body) => {
                assert_eq!(path, "/queue");
                let v: serde_json::Value = serde_json::from_str(&body).unwrap();
                assert_eq!(v["channel"], "politics");
                assert_eq!(v["title"], "Заголовок");
                assert_eq!(v["draft_text"], "Текст допису");
                assert_eq!(v["url"], "https://x.test/a");
            }
            other => panic!("очікувався POST: {other:?}"),
        }
        let save = parse_args(&args(&["save", "12", "--title", "Т", "--text", "Т2", "--channel", "world"])).unwrap().command;
        match build_request(&save, &fake_files).unwrap().unwrap() {
            Req::Post(path, body) => {
                assert_eq!(path, "/queue/12/save");
                let v: serde_json::Value = serde_json::from_str(&body).unwrap();
                assert_eq!(v["title"], "Т");
                assert_eq!(v["draft_text"], "Т2");
                assert_eq!(v["channel"], "world");
            }
            other => panic!("очікувався POST: {other:?}"),
        }
        assert!(parse_args(&args(&["create", "--channel", "a", "--title", "t", "--text", "x", "--text-file", "f"])).is_err());
        let missing = parse_args(&args(&["create", "--channel", "a", "--title", "t", "--text-file", "nope"])).unwrap().command;
        assert!(build_request(&missing, &fake_files).is_err());
    }

    #[test]
    fn action_requests_use_documented_paths() {
        assert_eq!(req(&["skip", "3"]), Req::Post("/queue/3/skip".into(), "{}".into()));
        assert_eq!(req(&["publish", "3"]), Req::Post("/queue/3/publish".into(), "{}".into()));
        assert_eq!(req(&["defer", "3", "--minutes", "15"]), Req::Post("/queue/3/defer".into(), "{\"minutes\":15}".into()));
        assert_eq!(req(&["why", "3", "--channel", "usa", "--scope", "auto"]), Req::Get("/queue/3/why?channel=usa&scope=auto".into()));
        assert_eq!(req(&["settings", "status"]), Req::Get("/settings/status".into()));
        assert_eq!(req(&["settings", "probe", "--hours", "24"]), Req::Post("/settings/probe".into(), "{\"hours\":24}".into()));
        assert_eq!(req(&["themes", "list"]), Req::Get("/themes".into()));
        assert_eq!(req(&["status"]), Req::Get("/status".into()));
        assert_eq!(req(&["channels"]), Req::Get("/channels".into()));
    }

    #[test]
    fn multi_step_and_local_commands_have_no_single_request() {
        let list: Vec<Vec<&str>> = vec![
            vec!["settings", "apply"],
            vec!["settings", "rollback"],
            vec!["settings", "check", "f.xlsx"],
            vec!["settings", "download", "--out", "o.xlsx"],
            vec!["profile", "show"],
        ];
        for a in list {
            let command = parse_args(&args(&a)).unwrap().command;
            assert_eq!(build_request(&command, &none).unwrap(), None, "{a:?}");
        }
    }

    #[test]
    fn themes_words_come_from_text_or_file() {
        assert_eq!(split_words("а, б,,в\nг"), vec!["а", "б", "в", "г"]);
        let test = parse_args(&args(&["themes", "test", "--words-file", "words.txt", "--text", "Це текст"])).unwrap().command;
        match build_request(&test, &fake_files).unwrap().unwrap() {
            Req::Post(path, body) => {
                assert_eq!(path, "/themes/test");
                let v: serde_json::Value = serde_json::from_str(&body).unwrap();
                assert_eq!(v["words"], serde_json::json!(["слово", "друге", "третє"]));
                assert_eq!(v["text"], "Це текст");
            }
            other => panic!("очікувався POST: {other:?}"),
        }
        let save = parse_args(&args(&["themes", "save", "--topic", "politics", "--words", "а,б"])).unwrap().command;
        match build_request(&save, &fake_files).unwrap().unwrap() {
            Req::Post(path, body) => {
                assert_eq!(path, "/themes/save");
                let v: serde_json::Value = serde_json::from_str(&body).unwrap();
                assert_eq!(v["words"], serde_json::json!(["а", "б"]));
                assert_eq!(v["topic"], "politics");
            }
            other => panic!("очікувався POST: {other:?}"),
        }
    }

    #[test]
    fn apply_needs_matching_probe_and_explicit_yes() {
        let ready = r#"{"pending_digest":"abc","probe_matches_pending":true}"#;
        let no_probe = r#"{"pending_digest":"abc","probe_matches_pending":false}"#;
        let empty = r#"{"pending_digest":"","probe_matches_pending":false}"#;
        assert_eq!(apply_decision(ready, false, false).unwrap(), ApplyPlan::Preview("abc".into()));
        assert_eq!(apply_decision(ready, false, true).unwrap(), ApplyPlan::Go("abc".into()));
        assert_eq!(apply_decision(no_probe, false, true).unwrap(), ApplyPlan::NeedProbe);
        assert_eq!(apply_decision(no_probe, true, true).unwrap(), ApplyPlan::Go("abc".into()));
        assert_eq!(apply_decision(empty, true, true).unwrap(), ApplyPlan::NoPending);
        assert!(apply_decision("не json", false, false).is_err());
    }

    #[test]
    fn only_settings_changes_have_yes_flag() {
        assert_eq!(
            parse_args(&args(&["settings", "apply"])).unwrap().command,
            Command::SettingsApply { without_probe: false, yes: false }
        );
        assert_eq!(
            parse_args(&args(&["settings", "apply", "--yes", "--without-probe"])).unwrap().command,
            Command::SettingsApply { without_probe: true, yes: true }
        );
        assert_eq!(parse_args(&args(&["settings", "rollback", "--yes"])).unwrap().command, Command::SettingsRollback { yes: true });
        assert!(parse_args(&args(&["publish", "1", "--yes"])).is_err());
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
        assert_eq!(
            profile_path_from(None, &env, "linux").unwrap(),
            PathBuf::from("/home/u/.config").join("info-desktop").join("connection.json")
        );
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
