use anyhow::{Context, Result};
use chrono::Local;
use serde::Serialize;
use serde_json::Value as JsonValue;
use std::io::Write;
use std::net::{TcpStream, ToSocketAddrs, UdpSocket};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use crate::config::{Endpoint, RuntimeConfig, format_error};

const UDP_DIRECT_MAX_BYTES: usize = 1200;
const UDP_CHUNK_SEND_RETRY_TIMES: usize = 3;
const UDP_CHUNK_SEND_RETRY_MS: u64 = 2;
static UDP_MSG_SEQ: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Serialize)]
pub struct QueryReport {
    pub timestamp: String,
    pub source: String,
    pub db_type: String,
    pub query_key: String,
    pub row_count: usize,
    pub csv_file: Option<String>,
    pub json_file: Option<String>,
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Debug, Serialize)]
struct QueryDataPacket {
    kind: String,
    timestamp: String,
    source: String,
    db_type: String,
    query_key: String,
    row_count: usize,
    data: JsonValue,
}

pub fn send_report(cfg: &RuntimeConfig, report: &QueryReport) {
    let payload = match serde_json::to_string(report) {
        Ok(s) => s,
        Err(err) => { eprintln!("failed to serialize report: {err}"); return; }
    };
    if let Some(tcp) = &cfg.tcp {
        if tcp.enabled {
            match send_tcp(tcp, &payload) {
                Ok(_) => println!("  report sent via tcp to {}", tcp.addr()),
                Err(err) => eprintln!("  report tcp send failed: {}", format_error(&err)),
            }
        }
    }
    if let Some(udp) = &cfg.udp {
        if udp.enabled {
            match send_udp(udp, &payload, cfg.udp_chunk_size) {
                Ok(_) => println!("  report sent via udp to {}", udp.addr()),
                Err(err) => eprintln!("  report udp send failed: {}", format_error(&err)),
            }
        }
    }
}

pub fn send_tcp(endpoint: &Endpoint, payload: &str) -> Result<()> {
    let addr = endpoint.addr();
    let sock_addr = addr.to_socket_addrs()
        .with_context(|| format!("resolve tcp address failed `{}`", addr))?
        .next()
        .ok_or_else(|| anyhow::anyhow!("resolve tcp address failed `{}`", addr))?;
    let mut stream = TcpStream::connect_timeout(&sock_addr, Duration::from_secs(5))
        .with_context(|| format!("tcp connect failed: {}", addr))?;
    stream.set_write_timeout(Some(Duration::from_secs(10)))
        .context("tcp set_write_timeout failed")?;
    let mut line = payload.to_string();
    line.push('\n');
    stream.write_all(line.as_bytes())
        .with_context(|| format!("tcp write failed: {}", addr))?;
    stream.flush().context("tcp flush failed")?;
    Ok(())
}

pub fn send_udp(endpoint: &Endpoint, payload: &str, chunk_size: usize) -> Result<()> {
    let addr = endpoint.addr();
    let sock = UdpSocket::bind("0.0.0.0:0").context("udp bind failed")?;
    sock.connect(addr.as_str())
        .with_context(|| format!("udp connect failed: {}", addr))?;

    if payload.len() <= UDP_DIRECT_MAX_BYTES {
        let mut tried = 0;
        loop {
            tried += 1;
            match sock.send(payload.as_bytes()) {
                Ok(_) => return Ok(()),
                Err(e) if tried < UDP_CHUNK_SEND_RETRY_TIMES => {
                    eprintln!("[udp] send retry {}/{} error={}", tried, UDP_CHUNK_SEND_RETRY_TIMES, e);
                    std::thread::sleep(Duration::from_millis(UDP_CHUNK_SEND_RETRY_MS));
                }
                Err(e) => return Err(e).context("udp send failed after retries"),
            }
        }
    }

    // chunked send
    let msg_id = format!("{}_{}", Local::now().format("%Y%m%d%H%M%S"), UDP_MSG_SEQ.fetch_add(1, Ordering::Relaxed));
    let total = (payload.len() + chunk_size - 1) / chunk_size;
    for (idx, chunk) in payload.as_bytes().chunks(chunk_size).enumerate() {
        let chunk_data = String::from_utf8_lossy(chunk).to_string();
        let packet = serde_json::json!({
            "kind": "udp_chunk_v1",
            "msg_id": msg_id,
            "idx": idx,
            "total": total,
            "data": chunk_data,
        });
        let line = serde_json::to_string(&packet).context("failed to serialize udp chunk")?;
        let mut tried = 0;
        loop {
            tried += 1;
            match sock.send(line.as_bytes()) {
                Ok(_) => break,
                Err(e) if tried < UDP_CHUNK_SEND_RETRY_TIMES => {
                    eprintln!("[udp chunk] send retry {}/{} idx={}/{} error={}", tried, UDP_CHUNK_SEND_RETRY_TIMES, idx+1, total, e);
                    std::thread::sleep(Duration::from_millis(UDP_CHUNK_SEND_RETRY_MS));
                }
                Err(e) => return Err(e).context("udp chunk send failed after retries"),
            }
        }
        if (idx + 1) % 8 == 0 {
            std::thread::sleep(Duration::from_millis(10));
        }
    }
    Ok(())
}

pub fn send_transport_test_packet(cfg: &RuntimeConfig) -> Result<()> {
    let payload = format!(
        "{{\"kind\":\"transport_test\",\"timestamp\":\"{}\",\"message\":\"hello from tcp-reconnect\"}}",
        Local::now().format("%Y-%m-%d %H:%M:%S")
    );
    let mut tried = 0usize;
    let mut ok = 0usize;
    let mut errors = Vec::new();
    if let Some(tcp) = &cfg.tcp {
        if tcp.enabled {
            tried += 1;
            match send_tcp(tcp, &payload) {
                Ok(_) => { ok += 1; println!("[tcpserver] enable=1 target={} send_test=OK", tcp.addr()); }
                Err(err) => { let msg = format!("[tcpserver] enable=1 target={} send_test=FAILED error={}", tcp.addr(), format_error(&err)); println!("{msg}"); errors.push(msg); }
            }
        }
    }
    if let Some(udp) = &cfg.udp {
        if udp.enabled {
            tried += 1;
            match send_udp(udp, &payload, cfg.udp_chunk_size) {
                Ok(_) => { ok += 1; println!("[udpserver] enable=1 target={} send_test=OK", udp.addr()); }
                Err(err) => { let msg = format!("[udpserver] enable=1 target={} send_test=FAILED error={}", udp.addr(), format_error(&err)); println!("{msg}"); errors.push(msg); }
            }
        }
    }
    if tried == 0 { return Err(anyhow::anyhow!("no enabled tcp/udp endpoint found")); }
    if ok == tried { println!("Transport test passed: {ok}/{tried}"); return Ok(()); }
    Err(anyhow::anyhow!("transport test failed: {ok}/{tried} passed\n{}", errors.join("\n")))
}
