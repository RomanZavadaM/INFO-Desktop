//! Локальний міст між консоллю і відкритим вікном застосунку («спільне з'єднання»).
//!
//! Вікно (якщо користувач дозволив) відкриває вхід лише на 127.0.0.1 і записує порт та одноразовий пароль сеансу у файл сеансу.
//! Консоль читає файл і передає запити вікну, яке виконує їх своїм звичайним шляхом. Ключ пристрою між ними не передається.
//! Протокол: один рядок JSON у запиті, один рядок JSON у відповіді; обмін із сервером лишається в `transport` (через `perform`).

use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

pub const MAX_LINE: u64 = 20 * 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionInfo {
    pub port: u16,
    pub token: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BridgeRequest {
    pub token: String,
    pub method: String,
    pub path: String,
    #[serde(default)]
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BridgeResponse {
    pub status: u16,
    #[serde(default)]
    pub body: String,
    #[serde(default)]
    pub error: String,
}

pub struct Shared {
    pub token: String,
    pub enabled: AtomicBool,
}

/// Виконує запит до сервера: (метод, шлях відносно API, тіло) -> (код відповіді, текст).
pub type Perform = dyn Fn(&str, &str, &str) -> (u16, String) + Send + Sync;

pub fn session_path(dir: &Path) -> PathBuf {
    dir.join("session.json")
}

pub fn write_session(path: &Path, info: &SessionInfo) -> Result<(), String> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    let text = serde_json::to_string(info).map_err(|e| e.to_string())?;
    let tmp = path.with_extension("json.tmp");
    std::fs::write(&tmp, text).map_err(|e| e.to_string())?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(&tmp, std::fs::Permissions::from_mode(0o600));
    }
    std::fs::rename(&tmp, path).map_err(|e| e.to_string())
}

pub fn read_session(path: &Path) -> Option<SessionInfo> {
    let text = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&text).ok()
}

pub fn remove_session(path: &Path) {
    let _ = std::fs::remove_file(path);
}

pub fn new_token() -> Result<String, String> {
    let mut bytes = [0u8; 32];
    getrandom::getrandom(&mut bytes).map_err(|e| e.to_string())?;
    Ok(bytes.iter().map(|b| format!("{b:02x}")).collect())
}

pub fn tokens_equal(a: &str, b: &str) -> bool {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for i in 0..a.len() {
        diff |= a[i] ^ b[i];
    }
    diff == 0
}

/// Шлях запиту відносно API: тільки відносний, без виходу за межі, без схем і керівних символів.
pub fn valid_path(path: &str) -> bool {
    path.starts_with('/')
        && !path.starts_with("//")
        && path.len() <= 2048
        && !path.contains("..")
        && !path.contains("://")
        && path.chars().all(|c| c.is_ascii_graphic())
}

pub fn handle_request(line: &str, token: &str, enabled: bool, perform: &Perform) -> BridgeResponse {
    let fail = |status: u16, code: &str| BridgeResponse { status, body: String::new(), error: code.to_string() };
    let req: BridgeRequest = match serde_json::from_str(line) {
        Ok(r) => r,
        Err(_) => return fail(400, "bad_request"),
    };
    if !enabled {
        return fail(503, "console_disabled");
    }
    if !tokens_equal(&req.token, token) {
        return fail(401, "unauthorized");
    }
    match req.method.as_str() {
        "PING" => BridgeResponse { status: 200, body: "{\"ok\":true}".to_string(), error: String::new() },
        "GET" | "POST" => {
            if !valid_path(&req.path) {
                return fail(400, "bad_path");
            }
            let (status, body) = perform(&req.method, &req.path, &req.body);
            BridgeResponse { status, body, error: String::new() }
        }
        _ => fail(405, "bad_method"),
    }
}

fn serve_connection(stream: TcpStream, shared: &Shared, perform: &Perform) {
    let _ = stream.set_read_timeout(Some(Duration::from_secs(30)));
    let mut line = String::new();
    {
        let mut reader = BufReader::new((&stream).take(MAX_LINE));
        if reader.read_line(&mut line).is_err() {
            return;
        }
    }
    let response = handle_request(line.trim(), &shared.token, shared.enabled.load(Ordering::SeqCst), perform);
    let text = serde_json::to_string(&response).unwrap_or_else(|_| "{\"status\":500,\"body\":\"\",\"error\":\"serialize\"}".to_string());
    let mut out = &stream;
    let _ = out.write_all(text.as_bytes());
    let _ = out.write_all(b"\n");
    let _ = out.flush();
}

/// Відкриває вхід лише на 127.0.0.1 (порт обирає система) і повертає порт.
pub fn spawn_listener(shared: Arc<Shared>, perform: Arc<Perform>) -> Result<u16, String> {
    let listener = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0)).map_err(|e| e.to_string())?;
    let port = listener.local_addr().map_err(|e| e.to_string())?.port();
    std::thread::spawn(move || {
        for connection in listener.incoming() {
            if let Ok(stream) = connection {
                let shared = shared.clone();
                let perform = perform.clone();
                std::thread::spawn(move || serve_connection(stream, &shared, &*perform));
            }
        }
    });
    Ok(port)
}

/// Клієнтська сторона (консоль): надіслати запит відкритому вікну.
pub fn call(info: &SessionInfo, request: &BridgeRequest) -> Result<BridgeResponse, String> {
    let address = SocketAddr::from((Ipv4Addr::LOCALHOST, info.port));
    let mut stream = TcpStream::connect_timeout(&address, Duration::from_secs(2))
        .map_err(|e| format!("Вікно застосунку не відповідає: {e}"))?;
    let _ = stream.set_read_timeout(Some(Duration::from_secs(180)));
    let _ = stream.set_write_timeout(Some(Duration::from_secs(30)));
    let mut text = serde_json::to_string(request).map_err(|e| e.to_string())?;
    text.push('\n');
    stream.write_all(text.as_bytes()).map_err(|e| e.to_string())?;
    stream.flush().map_err(|e| e.to_string())?;
    let mut line = String::new();
    BufReader::new((&stream).take(MAX_LINE)).read_line(&mut line).map_err(|e| e.to_string())?;
    serde_json::from_str(line.trim()).map_err(|_| "Некоректна відповідь вікна застосунку".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn echo(method: &str, path: &str, body: &str) -> (u16, String) {
        (200, format!("{method} {path} {body}"))
    }

    fn request(token: &str, method: &str, path: &str, body: &str) -> String {
        serde_json::to_string(&BridgeRequest { token: token.into(), method: method.into(), path: path.into(), body: body.into() }).unwrap()
    }

    #[test]
    fn path_rules() {
        for ok in ["/status", "/queue?limit=5&cursor=0&bucket=all", "/queue/7/why?channel=a&scope=route"] {
            assert!(valid_path(ok), "{ok}");
        }
        for bad in ["status", "//host/x", "/a/../b", "/a b", "http://x/", "/x\n", "/текст", ""] {
            assert!(!valid_path(bad), "{bad:?}");
        }
    }

    #[test]
    fn token_comparison() {
        assert!(tokens_equal("abc", "abc"));
        assert!(!tokens_equal("abc", "abd"));
        assert!(!tokens_equal("abc", "abcd"));
        let (a, b) = (new_token().unwrap(), new_token().unwrap());
        assert_eq!(a.len(), 64);
        assert_ne!(a, b);
        assert!(a.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn request_handling_rules() {
        let ok = handle_request(&request("t", "GET", "/status", ""), "t", true, &echo);
        assert_eq!((ok.status, ok.body.as_str(), ok.error.as_str()), (200, "GET /status ", ""));
        let post = handle_request(&request("t", "POST", "/queue", "{\"a\":1}"), "t", true, &echo);
        assert_eq!(post.body, "POST /queue {\"a\":1}");
        assert_eq!(handle_request(&request("t", "PING", "", ""), "t", true, &echo).status, 200);
        assert_eq!(handle_request(&request("x", "GET", "/status", ""), "t", true, &echo).status, 401);
        assert_eq!(handle_request(&request("t", "GET", "/status", ""), "t", false, &echo).status, 503);
        assert_eq!(handle_request(&request("t", "DELETE", "/status", ""), "t", true, &echo).status, 405);
        assert_eq!(handle_request(&request("t", "GET", "/a/../b", ""), "t", true, &echo).status, 400);
        assert_eq!(handle_request("не json", "t", true, &echo).status, 400);
    }

    #[test]
    fn session_file_round_trip() {
        let dir = std::env::temp_dir().join(format!("info-desktop-core-session-{}", std::process::id()));
        let file = session_path(&dir);
        let info = SessionInfo { port: 4242, token: "tok".into() };
        write_session(&file, &info).unwrap();
        assert_eq!(read_session(&file), Some(info));
        remove_session(&file);
        assert_eq!(read_session(&file), None);
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn works_over_a_real_local_socket() {
        let shared = Arc::new(Shared { token: "secret-token".into(), enabled: AtomicBool::new(true) });
        let perform: Arc<Perform> = Arc::new(echo);
        let port = spawn_listener(shared.clone(), perform).unwrap();
        let info = SessionInfo { port, token: "secret-token".into() };
        let good = BridgeRequest { token: "secret-token".into(), method: "GET".into(), path: "/channels".into(), body: String::new() };
        let resp = call(&info, &good).unwrap();
        assert_eq!((resp.status, resp.body.as_str()), (200, "GET /channels "));
        let bad = BridgeRequest { token: "wrong".into(), ..good.clone() };
        assert_eq!(call(&info, &bad).unwrap().status, 401);
        shared.enabled.store(false, Ordering::SeqCst);
        assert_eq!(call(&info, &good).unwrap().status, 503);
        shared.enabled.store(true, Ordering::SeqCst);
        assert_eq!(call(&info, &good).unwrap().status, 200);
    }
}
