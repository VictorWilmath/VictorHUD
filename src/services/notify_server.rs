//! Accepts notifications using XSOverlay's public API, so VRCX's "XSOverlay notifications"
//! toggle (and any other app that supports XSOverlay) shows its alerts in VictorHUD.
//!
//! * Legacy UDP API: JSON datagrams on 127.0.0.1:42069 with `messageType: 1`.
//! * WebSocket API: ws://127.0.0.1:42070/?client=Name with `command: "SendNotification"`.

use std::io::ErrorKind;
use std::net::{TcpListener, TcpStream, UdpSocket};
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use base64::Engine as _;
use chrono::Local;
use crossbeam_channel::Sender;
use serde_json::Value;

use super::ServiceEvent;
use crate::state::{Artwork, Notification};
use crate::util::strip_rich_text;

const MAX_ICON_SIDE: u32 = 128;

/// Parses a legacy UDP datagram. Returns `None` for anything that isn't a notification.
pub fn parse_udp_message(bytes: &[u8], default_timeout: f32) -> Option<Notification> {
    let value: Value = serde_json::from_slice(bytes).ok()?;
    if value.get("messageType").and_then(Value::as_i64) != Some(1) {
        return None;
    }
    Some(notification_from_object(&value, default_timeout, "XSOverlay API"))
}

/// Parses a WebSocket API message (`{sender, target, command, jsonData}`).
pub fn parse_websocket_message(text: &str, client: &str, default_timeout: f32) -> Option<Notification> {
    let value: Value = serde_json::from_str(text).ok()?;
    if value.get("command").and_then(Value::as_str) != Some("SendNotification") {
        return None;
    }
    let payload = match value.get("jsonData") {
        Some(Value::String(json)) => serde_json::from_str::<Value>(json).ok()?,
        Some(object @ Value::Object(_)) => object.clone(),
        _ => return None,
    };
    let sender = value.get("sender").and_then(Value::as_str).filter(|s| !s.is_empty()).unwrap_or(client);
    Some(notification_from_object(&payload, default_timeout, sender))
}

fn notification_from_object(value: &Value, default_timeout: f32, fallback_source: &str) -> Notification {
    let text = |key: &str| value.get(key).and_then(Value::as_str).map(strip_rich_text).unwrap_or_default();
    let mut timeout = value.get("timeout").and_then(Value::as_f64).unwrap_or(default_timeout as f64) as f32;
    // Some senders pass milliseconds instead of seconds.
    if timeout > 120.0 {
        timeout /= 1000.0;
    }
    if !timeout.is_finite() || timeout <= 0.0 {
        timeout = default_timeout;
    }
    let source = Some(text("sourceApp")).filter(|s| !s.is_empty()).unwrap_or_else(|| fallback_source.to_string());
    let icon = value.get("icon").and_then(Value::as_str).and_then(|icon| {
        let use_base64 = value.get("useBase64Icon").and_then(Value::as_bool).unwrap_or(true);
        decode_icon(icon, use_base64)
    });
    Notification {
        id: 0,
        source,
        title: text("title"),
        body: text("content"),
        received: Local::now(),
        timeout: Duration::from_secs_f32(timeout.min(60.0)),
        icon,
        read: false,
    }
}

fn decode_icon(icon: &str, prefer_base64: bool) -> Option<Arc<Artwork>> {
    let icon = icon.trim();
    if icon.is_empty() || matches!(icon, "default" | "error" | "warning") {
        return None;
    }
    let from_base64 = || {
        let data = icon.split_once("base64,").map(|(_, d)| d).unwrap_or(icon);
        base64::engine::general_purpose::STANDARD
            .decode(data)
            .ok()
            .and_then(|bytes| Artwork::decode(&bytes, MAX_ICON_SIDE))
    };
    let from_file = || {
        let path = Path::new(icon);
        path.is_file()
            .then(|| std::fs::read(path).ok())
            .flatten()
            .and_then(|bytes| Artwork::decode(&bytes, MAX_ICON_SIDE))
    };
    if prefer_base64 { from_base64().or_else(from_file) } else { from_file().or_else(from_base64) }
}

/// Starts the UDP and WebSocket listeners. Port conflicts are reported as warnings, not errors.
pub fn spawn(
    udp_port: u16,
    websocket_port: u16,
    default_timeout: f32,
    events: Sender<ServiceEvent>,
    shutdown: Arc<AtomicBool>,
) {
    match UdpSocket::bind(("127.0.0.1", udp_port)) {
        Ok(socket) => {
            let events = events.clone();
            let shutdown = shutdown.clone();
            std::thread::Builder::new()
                .name("notify-udp".into())
                .spawn(move || run_udp(socket, default_timeout, events, shutdown))
                .expect("spawning notify-udp thread");
        }
        Err(err) => {
            log::warn!("notification UDP port {udp_port}: {err}");
            let _ = events.send(ServiceEvent::Warning(format!(
                "Notification port {udp_port} is busy (is XSOverlay running?). VRCX alerts won't show here."
            )));
        }
    }
    match TcpListener::bind(("127.0.0.1", websocket_port)) {
        Ok(listener) => {
            std::thread::Builder::new()
                .name("notify-ws".into())
                .spawn(move || run_websocket(listener, default_timeout, events, shutdown))
                .expect("spawning notify-ws thread");
        }
        Err(err) => {
            log::warn!("notification WebSocket port {websocket_port}: {err}");
            let _ = events.send(ServiceEvent::Warning(format!(
                "Notification WebSocket port {websocket_port} is busy, so apps using XSOverlay's WebSocket API can't reach VictorHUD."
            )));
        }
    }
}

fn run_udp(socket: UdpSocket, default_timeout: f32, events: Sender<ServiceEvent>, shutdown: Arc<AtomicBool>) {
    let _ = socket.set_read_timeout(Some(Duration::from_millis(500)));
    let mut buffer = vec![0u8; 65_536];
    while !shutdown.load(Ordering::Relaxed) {
        match socket.recv_from(&mut buffer) {
            Ok((len, from)) if from.ip().is_loopback() => {
                if let Some(notification) = parse_udp_message(&buffer[..len], default_timeout) {
                    let _ = events.send(ServiceEvent::Notification(notification));
                }
            }
            Ok(_) => {}
            Err(err) if matches!(err.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) => {}
            Err(err) => {
                log::debug!("notification UDP: {err}");
                std::thread::sleep(Duration::from_millis(200));
            }
        }
    }
}

fn run_websocket(listener: TcpListener, default_timeout: f32, events: Sender<ServiceEvent>, shutdown: Arc<AtomicBool>) {
    let _ = listener.set_nonblocking(true);
    while !shutdown.load(Ordering::Relaxed) {
        match listener.accept() {
            Ok((stream, addr)) if addr.ip().is_loopback() => {
                let events = events.clone();
                let shutdown = shutdown.clone();
                let _ = std::thread::Builder::new()
                    .name("notify-ws-client".into())
                    .spawn(move || handle_websocket_client(stream, default_timeout, events, shutdown));
            }
            Ok(_) => {}
            Err(err) if err.kind() == ErrorKind::WouldBlock => std::thread::sleep(Duration::from_millis(200)),
            Err(err) => {
                log::debug!("notification WebSocket accept: {err}");
                std::thread::sleep(Duration::from_millis(500));
            }
        }
    }
}

fn handle_websocket_client(
    stream: TcpStream,
    default_timeout: f32,
    events: Sender<ServiceEvent>,
    shutdown: Arc<AtomicBool>,
) {
    let _ = stream.set_nonblocking(false);
    let mut client = String::from("App");
    // The error type is fixed by tungstenite's handshake callback signature.
    #[allow(clippy::result_large_err)]
    let callback = |request: &tungstenite::handshake::server::Request, response| {
        if let Some(query) = request.uri().query() {
            for pair in query.split('&') {
                if let Some(name) = pair.strip_prefix("client=") {
                    client = name.replace("%20", " ").replace('+', " ");
                }
            }
        }
        Ok(response)
    };
    let mut socket = match tungstenite::accept_hdr(stream, callback) {
        Ok(socket) => socket,
        Err(err) => {
            log::debug!("notification WebSocket handshake: {err}");
            return;
        }
    };
    let _ = socket.get_ref().set_read_timeout(Some(Duration::from_millis(500)));
    while !shutdown.load(Ordering::Relaxed) {
        match socket.read() {
            Ok(tungstenite::Message::Text(text)) => {
                if let Some(notification) = parse_websocket_message(text.as_str(), &client, default_timeout) {
                    let _ = events.send(ServiceEvent::Notification(notification));
                }
            }
            Ok(tungstenite::Message::Close(_)) => break,
            Ok(_) => {}
            Err(tungstenite::Error::Io(err)) if matches!(err.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) => {}
            Err(_) => break,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ONE_PIXEL_PNG: &str =
        "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8BQDwAEhQGAhKmMIQAAAABJRU5ErkJggg==";

    #[test]
    fn parses_vrcx_style_udp_notifications() {
        let message = format!(
            r#"{{"messageType":1,"index":0,"timeout":5.0,"height":110,"opacity":1.0,"volume":0.7,"audioPath":"","title":"<color=#00b8ff>Kazuha</color> has joined","content":"The Black Cat","useBase64Icon":true,"icon":"{ONE_PIXEL_PNG}","sourceApp":"VRCX"}}"#
        );
        let n = parse_udp_message(message.as_bytes(), 4.0).unwrap();
        assert_eq!(n.title, "Kazuha has joined");
        assert_eq!(n.body, "The Black Cat");
        assert_eq!(n.source, "VRCX");
        assert_eq!(n.timeout, Duration::from_secs(5));
        assert_eq!(n.icon.unwrap().size, [1, 1]);
    }

    #[test]
    fn ignores_other_udp_message_types_and_garbage() {
        assert!(parse_udp_message(br#"{"messageType":2,"title":"x"}"#, 4.0).is_none());
        assert!(parse_udp_message(b"not json", 4.0).is_none());
    }

    #[test]
    fn millisecond_timeouts_and_missing_fields() {
        let n = parse_udp_message(br#"{"messageType":1,"title":"Hi","timeout":3000}"#, 4.0).unwrap();
        assert_eq!(n.timeout, Duration::from_secs(3));
        assert_eq!(n.source, "XSOverlay API");
        assert!(n.icon.is_none());
    }

    #[test]
    fn parses_websocket_notifications() {
        let text = r#"{"sender":"VRCX","target":"xsoverlay","command":"SendNotification","jsonData":"{\"type\":1,\"title\":\"Invite\",\"content\":\"Moonbeam invited you\",\"timeout\":4}","rawData":null}"#;
        let n = parse_websocket_message(text, "fallback", 5.0).unwrap();
        assert_eq!(n.title, "Invite");
        assert_eq!(n.body, "Moonbeam invited you");
        assert_eq!(n.source, "VRCX");
        assert!(parse_websocket_message(r#"{"command":"UpdateSettings","jsonData":"{}"}"#, "x", 5.0).is_none());
    }

    #[test]
    fn end_to_end_over_loopback() {
        let (tx, rx) = crossbeam_channel::unbounded();
        let shutdown = Arc::new(AtomicBool::new(false));
        let udp_port = UdpSocket::bind("127.0.0.1:0").unwrap().local_addr().unwrap().port();
        let ws_port = TcpListener::bind("127.0.0.1:0").unwrap().local_addr().unwrap().port();
        spawn(udp_port, ws_port, 4.0, tx, shutdown.clone());

        let sender = UdpSocket::bind("127.0.0.1:0").unwrap();
        sender.send_to(br#"{"messageType":1,"title":"Over UDP"}"#, ("127.0.0.1", udp_port)).unwrap();
        match rx.recv_timeout(Duration::from_secs(3)).unwrap() {
            ServiceEvent::Notification(n) => assert_eq!(n.title, "Over UDP"),
            other => panic!("unexpected event {other:?}"),
        }

        let (mut ws, _) = tungstenite::connect(format!("ws://127.0.0.1:{ws_port}/?client=TestApp")).unwrap();
        ws.send(tungstenite::Message::text(
            r#"{"sender":"","target":"xsoverlay","command":"SendNotification","jsonData":"{\"title\":\"Over WS\"}"}"#,
        ))
        .unwrap();
        match rx.recv_timeout(Duration::from_secs(3)).unwrap() {
            ServiceEvent::Notification(n) => {
                assert_eq!(n.title, "Over WS");
                assert_eq!(n.source, "TestApp");
            }
            other => panic!("unexpected event {other:?}"),
        }
        shutdown.store(true, Ordering::Relaxed);
    }
}
