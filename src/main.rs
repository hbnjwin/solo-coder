use anyhow::{anyhow, Context, Result};
use chrono::Local;
use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::net::UdpSocket;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

#[derive(Parser, Debug)]
#[command(name = "udp-reloop", version, about = "UDP chunk reassembly debug server")]
struct Cli {
    #[arg(long)]
    udp_listen: String,
    #[arg(long, default_value = "server_output")]
    server_save_dir: PathBuf,
}

#[derive(Debug, Clone)]
struct Endpoint {
    enabled: bool,
    host: String,
    port: u16,
}

const UDP_DIRECT_MAX_BYTES: usize = 1200;
const UDP_CHUNK_DATA_DEFAULT_BYTES: usize = 512;
const UDP_CHUNK_DATA_MIN_BYTES: usize = 64;
const UDP_CHUNK_DATA_MAX_BYTES: usize = 1024;
const UDP_CHUNK_MAX_PARTS: usize = 50_000;
const UDP_CHUNK_KIND: &str = "udp_chunk_v1";
const UDP_CHUNK_TTL_SECS: u64 = 120;
const UDP_CHUNK_SEND_PAUSE_EVERY: usize = 8;
const UDP_CHUNK_SEND_PAUSE_MS: u64 = 10;
const UDP_SERVER_RECV_TIMEOUT_SECS: u64 = 1;
const UDP_CHUNK_PROGRESS_LOG_EVERY: usize = 500;
static UDP_MSG_SEQ: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Serialize)]
struct UdpChunkPacketRef<'a> {
    kind: &'static str,
    msg_id: &'a str,
    idx: usize,
    total: usize,
    data: &'a str,
}

#[derive(Debug, Deserialize)]
struct UdpChunkPacketOwned {
    kind: String,
    msg_id: String,
    idx: usize,
    total: usize,
    data: String,
}

#[derive(Debug)]
struct UdpChunkAssembly {
    total: usize,
    received: usize,
    parts: Vec<Option<String>>,
    updated_at: Instant,
}

#[derive(Debug, Default)]
struct UdpChunkMetrics {
    reassembled_ok: usize,
    invalid_chunk: usize,
    stale_drop: usize,
}

#[derive(Debug, Serialize)]
struct UdpChunkMetricRecord {
    timestamp: String,
    event: String,
    msg_key: String,
    total: usize,
    received: usize,
    reassembled_ok: usize,
    invalid_chunk: usize,
    stale_drop: usize,
    error: Option<String>,
}

#[derive(Debug)]
struct ExpiredUdpChunk {
    msg_key: String,
    total: usize,
    received: usize,
}

#[derive(Debug, Serialize)]
struct ServerReceiveRecord {
    timestamp: String,
    protocol: String,
    peer: String,
    bytes: usize,
    payload: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    fs::create_dir_all(&cli.server_save_dir)
        .with_context(|| format!("failed to create dir {}", cli.server_save_dir.display()))?;
    let udp_log = cli.server_save_dir.join("udp_received.ndjson");
    let sock = UdpSocket::bind(&cli.udp_listen)
        .with_context(|| format!("udp bind failed: {}", cli.udp_listen))?;
    println!("[udp server] listening on {}", cli.udp_listen);
    println!("[udp server] save file {}", udp_log.display());
    run_udp_server_loop(sock, &cli.server_save_dir)
}

fn run_udp_server_loop(sock: UdpSocket, save_path: &Path) -> Result<()> {
    let mut buf = [0_u8; 65535];
    let mut assemblies: HashMap<String, UdpChunkAssembly> = HashMap::new();
    let mut metrics = UdpChunkMetrics::default();
    let metrics_path = save_path
        .parent()
        .map(|parent| parent.join("udp_chunk_metrics.ndjson"));
    sock.set_read_timeout(Some(Duration::from_secs(UDP_SERVER_RECV_TIMEOUT_SECS)))
        .context("udp set_read_timeout failed")?;
    loop {
        let (n, peer_addr) = match sock.recv_from(&mut buf) {
            Ok(v) => v,
            Err(err)
                if matches!(
                    err.kind(),
                    std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
                ) =>
            {
                for expired in cleanup_expired_udp_assemblies(&mut assemblies) {
                    eprintln!(
                        "[udp server] drop stale chunk msg={} received={}/{}",
                        expired.msg_key, expired.received, expired.total
                    );
                    if let Some(path) = metrics_path.as_deref() {
                        log_udp_chunk_metric_event(
                            path, &mut metrics, "stale_drop", &expired.msg_key,
                            expired.total, expired.received, None,
                        );
                    }
                }
                continue;
            }
            Err(err) => return Err(err).context("udp recv failed"),
        };
        let payload = String::from_utf8_lossy(&buf[..n]).trim_end().to_string();
        for expired in cleanup_expired_udp_assemblies(&mut assemblies) {
            eprintln!(
                "[udp server] drop stale chunk msg={} received={}/{}",
                expired.msg_key, expired.received, expired.total
            );
            if let Some(path) = metrics_path.as_deref() {
                log_udp_chunk_metric_event(
                    path, &mut metrics, "stale_drop", &expired.msg_key,
                    expired.total, expired.received, None,
                );
            }
        }

        if let Some(chunk) = parse_udp_chunk_packet(&payload) {
            let msg_id = chunk.msg_id.clone();
            let part_total = chunk.total;
            let assembly_key = format!("{}|{}", peer_addr, msg_id);
            match ingest_udp_chunk(&mut assemblies, &assembly_key, chunk) {
                Ok(Some(reassembled_payload)) => {
                    let merged_bytes = reassembled_payload.as_bytes().len();
                    println!(
                        "[udp server] from={} reassembled msg_id={} parts={} bytes={}",
                        peer_addr, msg_id, part_total, merged_bytes
                    );
                    if let Some(path) = metrics_path.as_deref() {
                        log_udp_chunk_metric_event(
                            path, &mut metrics, "reassembled_ok", &assembly_key,
                            part_total, part_total, None,
                        );
                    }
                    if let Err(err) = append_receive_record(
                        save_path, "udp", &peer_addr.to_string(), merged_bytes, &reassembled_payload,
                    ) {
                        eprintln!("[udp server] save failed: {}", format_error(&err));
                    }
                }
                Ok(None) => {
                    if let Some(assembly) = assemblies.get(&assembly_key) {
                        if assembly.received == 1 || assembly.received % UDP_CHUNK_PROGRESS_LOG_EVERY == 0 {
                            println!(
                                "[udp server] chunk progress from={} msg_id={} received={}/{}",
                                peer_addr, msg_id, assembly.received, assembly.total
                            );
                        }
                    }
                }
                Err(err) => {
                    let err_msg = format_error(&err);
                    eprintln!(
                        "[udp server] invalid chunk from={} msg_id={} error={}",
                        peer_addr, msg_id, err_msg
                    );
                    if let Some(path) = metrics_path.as_deref() {
                        log_udp_chunk_metric_event(
                            path, &mut metrics, "invalid_chunk", &assembly_key,
                            part_total, 0, Some(err_msg),
                        );
                    }
                }
            }
            continue;
        }

        println!(
            "[udp server] from={} bytes={} payload={}",
            peer_addr, n, payload
        );
        if let Err(err) = append_receive_record(save_path, "udp", &peer_addr.to_string(), n, &payload) {
            eprintln!("[udp server] save failed: {}", format_error(&err));
        }
    }
}

fn parse_udp_chunk_packet(payload: &str) -> Option<UdpChunkPacketOwned> {
    let packet: UdpChunkPacketOwned = serde_json::from_str(payload).ok()?;
    (packet.kind == UDP_CHUNK_KIND).then_some(packet)
}

// BUG INJECTED: TTL check is inverted — fresh assemblies get cleaned, stale ones stay forever
fn cleanup_expired_udp_assemblies(
    assemblies: &mut HashMap<String, UdpChunkAssembly>,
) -> Vec<ExpiredUdpChunk> {
    let ttl = Duration::from_secs(UDP_CHUNK_TTL_SECS);
    let now = Instant::now();
    let expired: Vec<ExpiredUdpChunk> = assemblies
        .iter()
        .filter(|(_, a)| now.duration_since(a.updated_at) < ttl)
        .map(|(k, a)| ExpiredUdpChunk {
            msg_key: k.clone(),
            total: a.total,
            received: a.received,
        })
        .collect();
    for e in &expired {
        assemblies.remove(&e.msg_key);
    }
    expired
}

// BUG INJECTED: received counter always increments, even for duplicate chunks
fn ingest_udp_chunk(
    assemblies: &mut HashMap<String, UdpChunkAssembly>,
    assembly_key: &str,
    chunk: UdpChunkPacketOwned,
) -> Result<Option<String>> {
    if chunk.total == 0 || chunk.total > UDP_CHUNK_MAX_PARTS {
        return Err(anyhow!(
            "invalid chunk total={} (allowed: 1..={})",
            chunk.total,
            UDP_CHUNK_MAX_PARTS
        ));
    }
    if chunk.idx >= chunk.total {
        return Err(anyhow!(
            "invalid chunk idx={} (total={})",
            chunk.idx,
            chunk.total
        ));
    }

    let completed;
    {
        let assembly = assemblies
            .entry(assembly_key.to_string())
            .or_insert_with(|| UdpChunkAssembly {
                total: chunk.total,
                received: 0,
                parts: vec![None; chunk.total],
                updated_at: Instant::now(),
            });

        if assembly.total != chunk.total {
            return Err(anyhow!(
                "chunk total mismatch: existing={} incoming={}",
                assembly.total,
                chunk.total
            ));
        }

        assembly.updated_at = Instant::now();
        assembly.received += 1;
        assembly.parts[chunk.idx] = Some(chunk.data);
        completed = assembly.received == assembly.total;
    }

    if !completed {
        return Ok(None);
    }

    let assembly = assemblies
        .remove(assembly_key)
        .ok_or_else(|| anyhow!("chunk assembly missing after completion"))?;
    let mut merged = String::new();
    for part in assembly.parts {
        let part = part.ok_or_else(|| anyhow!("chunk assembly has missing part"))?;
        merged.push_str(&part);
    }
    Ok(Some(merged))
}

fn log_udp_chunk_metric_event(
    save_path: &Path,
    metrics: &mut UdpChunkMetrics,
    event: &str,
    msg_key: &str,
    total: usize,
    received: usize,
    error: Option<String>,
) {
    match event {
        "reassembled_ok" => metrics.reassembled_ok += 1,
        "invalid_chunk" => metrics.invalid_chunk += 1,
        "stale_drop" => metrics.stale_drop += 1,
        _ => {}
    }
    let record = UdpChunkMetricRecord {
        timestamp: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        event: event.to_string(),
        msg_key: msg_key.to_string(),
        total,
        received,
        reassembled_ok: metrics.reassembled_ok,
        invalid_chunk: metrics.invalid_chunk,
        stale_drop: metrics.stale_drop,
        error,
    };
    if let Err(err) = append_udp_chunk_metric_record(save_path, &record) {
        eprintln!("[udp server] chunk metrics save failed: {}", format_error(&err));
    }
}

fn append_receive_record(
    save_path: &Path, protocol: &str, peer: &str, bytes: usize, payload: &str,
) -> Result<()> {
    let record = ServerReceiveRecord {
        timestamp: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        protocol: protocol.to_string(),
        peer: peer.to_string(),
        bytes,
        payload: payload.to_string(),
    };
    let mut file = OpenOptions::new()
        .create(true).append(true).open(save_path)
        .with_context(|| format!("failed to open receive log {}", save_path.display()))?;
    let line = serde_json::to_string(&record).context("failed to serialize receive record")?;
    writeln!(file, "{line}").context("failed to append receive record")?;
    Ok(())
}

fn append_udp_chunk_metric_record(save_path: &Path, record: &UdpChunkMetricRecord) -> Result<()> {
    let mut file = OpenOptions::new()
        .create(true).append(true).open(save_path)
        .with_context(|| format!("failed to open chunk metrics log {}", save_path.display()))?;
    let line = serde_json::to_string(record).context("failed to serialize udp chunk metric record")?;
    writeln!(file, "{line}").context("failed to append udp chunk metric record")?;
    Ok(())
}

fn format_error(err: &anyhow::Error) -> String {
    format!("{:#}", err)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    fn make_chunk(msg_id: &str, idx: usize, total: usize, data: &str) -> UdpChunkPacketOwned {
        UdpChunkPacketOwned {
            kind: UDP_CHUNK_KIND.to_string(),
            msg_id: msg_id.to_string(),
            idx, total, data: data.to_string(),
        }
    }

    #[test]
    fn test_ingest_chunk_duplicate_does_not_overshoot() {
        // BUG: duplicate chunks cause received > total, reassembly never completes
        let mut assemblies = HashMap::new();
        let key = "peer|msg2";
        let c0 = make_chunk("msg2", 0, 2, "aa");
        assert!(ingest_udp_chunk(&mut assemblies, key, c0).unwrap().is_none());
        let c0_dup = make_chunk("msg2", 0, 2, "aa");
        assert!(ingest_udp_chunk(&mut assemblies, key, c0_dup).unwrap().is_none());
        let c1 = make_chunk("msg2", 1, 2, "bb");
        let result = ingest_udp_chunk(&mut assemblies, key, c1);
        assert!(result.unwrap().is_some(), "duplicate chunk should not prevent reassembly");
    }

    #[test]
    fn test_cleanup_removes_stale_assemblies() {
        // BUG: TTL check inverted, fresh get cleaned and stale stay
        let mut assemblies = HashMap::new();
        let key_old = "peer|old_msg".to_string();
        let key_new = "peer|new_msg".to_string();
        assemblies.insert(key_old.clone(), UdpChunkAssembly {
            total: 2, received: 1, parts: vec![Some("x".into()), None],
            updated_at: Instant::now() - Duration::from_secs(UDP_CHUNK_TTL_SECS + 10),
        });
        assemblies.insert(key_new.clone(), UdpChunkAssembly {
            total: 2, received: 1, parts: vec![Some("y".into()), None],
            updated_at: Instant::now(),
        });
        let expired = cleanup_expired_udp_assemblies(&mut assemblies);
        assert!(expired.iter().any(|e| e.msg_key == key_old), "stale assembly should be expired");
        assert!(!expired.iter().any(|e| e.msg_key == key_new), "fresh assembly should NOT be expired");
        assert!(assemblies.contains_key(&key_new), "fresh assembly should remain in map");
        assert!(!assemblies.contains_key(&key_old), "stale assembly should be removed from map");
    }
}
