use std::fs;
use std::net::{IpAddr, SocketAddr, UdpSocket};
use std::path::Path;
use std::thread;
use std::time::Duration;

use anyhow::{ensure, Result};
use clap::ValueEnum;
use midly::{MetaMessage, MidiMessage, Smf, Timing, TrackEventKind};
use rosc::{encoder, OscMessage, OscPacket, OscType};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{
    current_unix_seconds, inspect_deck_transport, inspect_session, new_runtime_id,
    read_json_or_default, write_pretty_json,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, ValueEnum, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RealtimeAdapterProtocol {
    OscUdp,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, ValueEnum, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RealtimeDispatchMode {
    Timed,
    Immediate,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RealtimeDispatchSource {
    SessionPreview,
    DeckTransport,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct RealtimeAdapterRecord {
    pub adapter_id: String,
    pub display_name: String,
    pub protocol: RealtimeAdapterProtocol,
    pub host: IpAddr,
    pub port: u16,
    pub base_path: String,
    pub created_at_unix_seconds: u64,
    pub updated_at_unix_seconds: u64,
    pub dispatches: Vec<RealtimeDispatchRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct RealtimeDispatchRecord {
    pub dispatch_id: String,
    pub created_at_unix_seconds: u64,
    pub actor_id: String,
    pub source: RealtimeDispatchSource,
    pub session_id: Option<String>,
    pub deck_id: Option<String>,
    pub preview_id: Option<String>,
    pub clip_id: Option<String>,
    pub dispatch_mode: RealtimeDispatchMode,
    pub time_scale: f64,
    pub message_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct NewRealtimeAdapterRequest {
    pub display_name: String,
    pub protocol: RealtimeAdapterProtocol,
    pub host: IpAddr,
    pub port: u16,
    pub base_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct SendRealtimePreviewRequest {
    pub actor_id: String,
    pub session_id: String,
    pub preview_id: String,
    pub dispatch_mode: RealtimeDispatchMode,
    pub time_scale: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct SendRealtimeTransportRequest {
    pub actor_id: String,
    pub deck_id: String,
    pub dispatch_mode: RealtimeDispatchMode,
    pub time_scale: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct RealtimeDispatchSummary {
    pub adapter: RealtimeAdapterRecord,
    pub dispatch: RealtimeDispatchRecord,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct RealtimeStoreFile {
    version: u32,
    adapters: Vec<RealtimeAdapterRecord>,
}

pub fn create_realtime_adapter(
    store_path: &Path,
    request: NewRealtimeAdapterRequest,
) -> Result<RealtimeAdapterRecord> {
    validate_new_adapter_request(&request)?;
    let now = current_unix_seconds();
    let adapter = RealtimeAdapterRecord {
        adapter_id: new_runtime_id("realtime-adapter"),
        display_name: request.display_name,
        protocol: request.protocol,
        host: request.host,
        port: request.port,
        base_path: normalize_base_path(&request.base_path),
        created_at_unix_seconds: now,
        updated_at_unix_seconds: now,
        dispatches: Vec::new(),
    };

    let mut store = load_store(store_path)?;
    store.adapters.push(adapter.clone());
    save_store(store_path, &store)?;
    Ok(adapter)
}

pub fn inspect_realtime_adapter(
    store_path: &Path,
    adapter_id: &str,
) -> Result<RealtimeAdapterRecord> {
    ensure!(!adapter_id.trim().is_empty(), "adapter id cannot be empty");
    let store = load_store(store_path)?;
    store
        .adapters
        .into_iter()
        .find(|adapter| adapter.adapter_id == adapter_id)
        .ok_or_else(|| anyhow::anyhow!("realtime adapter '{}' was not found", adapter_id))
}

pub fn list_realtime_adapters(store_path: &Path) -> Result<Vec<RealtimeAdapterRecord>> {
    let mut store = load_store(store_path)?;
    store.adapters.sort_by(|left, right| {
        left.created_at_unix_seconds
            .cmp(&right.created_at_unix_seconds)
            .then(left.adapter_id.cmp(&right.adapter_id))
    });
    Ok(store.adapters)
}

pub fn send_preview_to_realtime_adapter(
    store_path: &Path,
    session_store_path: &Path,
    adapter_id: &str,
    request: SendRealtimePreviewRequest,
) -> Result<RealtimeDispatchSummary> {
    validate_preview_request(adapter_id, &request)?;
    let session = inspect_session(session_store_path, &request.session_id)?;
    let preview = session
        .previews
        .iter()
        .find(|preview| preview.preview_id == request.preview_id)
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("preview '{}' was not found", request.preview_id))?;

    let bytes = fs::read(&preview.midi.path)?;
    let smf = Smf::parse(&bytes)?;
    let socket_addr;
    let base_path;
    {
        let adapter = inspect_realtime_adapter(store_path, adapter_id)?;
        socket_addr = SocketAddr::new(adapter.host, adapter.port);
        base_path = adapter.base_path;
    }

    let message_count = send_preview_messages(
        socket_addr,
        &base_path,
        &request.session_id,
        &request.preview_id,
        &smf,
        request.dispatch_mode,
        request.time_scale,
    )?;

    record_dispatch(
        store_path,
        adapter_id,
        RealtimeDispatchRecord {
            dispatch_id: new_runtime_id("realtime-dispatch"),
            created_at_unix_seconds: current_unix_seconds(),
            actor_id: request.actor_id,
            source: RealtimeDispatchSource::SessionPreview,
            session_id: Some(request.session_id),
            deck_id: None,
            preview_id: Some(request.preview_id),
            clip_id: None,
            dispatch_mode: request.dispatch_mode,
            time_scale: request.time_scale,
            message_count,
        },
    )
}

pub fn send_transport_to_realtime_adapter(
    store_path: &Path,
    daw_store_path: &Path,
    adapter_id: &str,
    request: SendRealtimeTransportRequest,
) -> Result<RealtimeDispatchSummary> {
    validate_transport_request(adapter_id, &request)?;
    let snapshot = inspect_deck_transport(daw_store_path, &request.deck_id)?;
    let socket_addr;
    let base_path;
    {
        let adapter = inspect_realtime_adapter(store_path, adapter_id)?;
        socket_addr = SocketAddr::new(adapter.host, adapter.port);
        base_path = adapter.base_path;
    }

    let mut messages = Vec::new();
    match &snapshot.active_clip {
        Some(clip) => {
            messages.push(TimedPacket {
                delay_micros: 0,
                packet: transport_message(
                    &base_path,
                    "transport/play",
                    vec![
                        OscType::String(snapshot.deck.deck_id.clone()),
                        OscType::String(snapshot.deck.session_id.clone()),
                        OscType::String(clip.clip_id.clone()),
                        OscType::String(clip.label.clone()),
                    ],
                ),
            });
            messages.push(TimedPacket {
                delay_micros: 0,
                packet: transport_message(
                    &base_path,
                    "transport/clip",
                    vec![
                        OscType::String(clip.clip_id.clone()),
                        OscType::String(clip.preview_id.clone()),
                        OscType::String(clip.label.clone()),
                    ],
                ),
            });
        }
        None => {
            messages.push(TimedPacket {
                delay_micros: 0,
                packet: transport_message(
                    &base_path,
                    "transport/stop",
                    vec![
                        OscType::String(snapshot.deck.deck_id.clone()),
                        OscType::String(snapshot.deck.session_id.clone()),
                    ],
                ),
            });
        }
    }
    let message_count = send_packets(
        socket_addr,
        messages,
        request.dispatch_mode,
        request.time_scale,
    )?;

    record_dispatch(
        store_path,
        adapter_id,
        RealtimeDispatchRecord {
            dispatch_id: new_runtime_id("realtime-dispatch"),
            created_at_unix_seconds: current_unix_seconds(),
            actor_id: request.actor_id,
            source: RealtimeDispatchSource::DeckTransport,
            session_id: Some(snapshot.deck.session_id.clone()),
            deck_id: Some(snapshot.deck.deck_id.clone()),
            preview_id: snapshot
                .active_clip
                .as_ref()
                .map(|clip| clip.preview_id.clone()),
            clip_id: snapshot
                .active_clip
                .as_ref()
                .map(|clip| clip.clip_id.clone()),
            dispatch_mode: request.dispatch_mode,
            time_scale: request.time_scale,
            message_count,
        },
    )
}

fn record_dispatch(
    store_path: &Path,
    adapter_id: &str,
    dispatch: RealtimeDispatchRecord,
) -> Result<RealtimeDispatchSummary> {
    let mut store = load_store(store_path)?;
    let adapter = store
        .adapters
        .iter_mut()
        .find(|adapter| adapter.adapter_id == adapter_id)
        .ok_or_else(|| anyhow::anyhow!("realtime adapter '{}' was not found", adapter_id))?;
    adapter.dispatches.push(dispatch.clone());
    adapter.updated_at_unix_seconds = current_unix_seconds();
    let summary = RealtimeDispatchSummary {
        adapter: adapter.clone(),
        dispatch,
    };
    save_store(store_path, &store)?;
    Ok(summary)
}

fn send_preview_messages(
    target: SocketAddr,
    base_path: &str,
    session_id: &str,
    preview_id: &str,
    smf: &Smf<'_>,
    dispatch_mode: RealtimeDispatchMode,
    time_scale: f64,
) -> Result<usize> {
    let timing = timing_ticks_per_beat(smf.header.timing)?;
    let mut packets =
        collect_note_packets(base_path, session_id, preview_id, smf, timing, 500_000)?;
    let last_delay = packets
        .last()
        .map(|packet| packet.delay_micros)
        .unwrap_or(0);
    packets.push(TimedPacket {
        delay_micros: last_delay,
        packet: transport_message(
            base_path,
            "preview/complete",
            vec![
                OscType::String(session_id.to_string()),
                OscType::String(preview_id.to_string()),
            ],
        ),
    });

    send_packets(target, packets, dispatch_mode, time_scale)
}

fn collect_note_packets(
    base_path: &str,
    session_id: &str,
    preview_id: &str,
    smf: &Smf<'_>,
    timing: u16,
    mut tempo_micros_per_beat: u32,
) -> Result<Vec<TimedPacket>> {
    let mut packets = vec![TimedPacket {
        delay_micros: 0,
        packet: transport_message(
            base_path,
            "preview/start",
            vec![
                OscType::String(session_id.to_string()),
                OscType::String(preview_id.to_string()),
            ],
        ),
    }];

    for track in &smf.tracks {
        let mut absolute_ticks = 0u64;
        for event in track {
            absolute_ticks += u64::from(event.delta.as_int());
            match event.kind {
                TrackEventKind::Meta(MetaMessage::Tempo(tempo)) => {
                    tempo_micros_per_beat = tempo.as_int();
                }
                TrackEventKind::Midi { channel, message } => match message {
                    MidiMessage::NoteOn { key, vel } if vel.as_int() > 0 => {
                        packets.push(TimedPacket {
                            delay_micros: ticks_to_micros(
                                absolute_ticks,
                                timing,
                                tempo_micros_per_beat,
                            ),
                            packet: transport_message(
                                base_path,
                                "note_on",
                                vec![
                                    OscType::String(session_id.to_string()),
                                    OscType::String(preview_id.to_string()),
                                    OscType::Int(i32::from(channel.as_int())),
                                    OscType::Int(i32::from(key.as_int())),
                                    OscType::Int(i32::from(vel.as_int())),
                                ],
                            ),
                        })
                    }
                    MidiMessage::NoteOn { key, vel: _ } | MidiMessage::NoteOff { key, vel: _ } => {
                        packets.push(TimedPacket {
                            delay_micros: ticks_to_micros(
                                absolute_ticks,
                                timing,
                                tempo_micros_per_beat,
                            ),
                            packet: transport_message(
                                base_path,
                                "note_off",
                                vec![
                                    OscType::String(session_id.to_string()),
                                    OscType::String(preview_id.to_string()),
                                    OscType::Int(i32::from(channel.as_int())),
                                    OscType::Int(i32::from(key.as_int())),
                                ],
                            ),
                        })
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    Ok(packets)
}

#[derive(Debug)]
struct TimedPacket {
    delay_micros: u64,
    packet: OscPacket,
}

fn send_packets(
    target: SocketAddr,
    packets: Vec<TimedPacket>,
    dispatch_mode: RealtimeDispatchMode,
    time_scale: f64,
) -> Result<usize> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.connect(target)?;

    let mut sent = 0usize;
    let mut last_delay = 0u64;
    for timed in packets {
        if matches!(dispatch_mode, RealtimeDispatchMode::Timed) && time_scale > 0.0 {
            let delta = timed.delay_micros.saturating_sub(last_delay);
            if delta > 0 {
                let sleep = ((delta as f64) * time_scale).round() as u64;
                if sleep > 0 {
                    thread::sleep(Duration::from_micros(sleep));
                }
            }
            last_delay = timed.delay_micros;
        }
        let bytes = encoder::encode(&timed.packet)?;
        socket.send(&bytes)?;
        sent += 1;
    }
    Ok(sent)
}

fn transport_message(base_path: &str, suffix: &str, args: Vec<OscType>) -> OscPacket {
    OscPacket::Message(OscMessage {
        addr: format!("{}/{}", normalize_base_path(base_path), suffix),
        args,
    })
}

fn timing_ticks_per_beat(timing: Timing) -> Result<u16> {
    match timing {
        Timing::Metrical(ticks) => Ok(ticks.as_int()),
        Timing::Timecode(_, _) => Err(anyhow::anyhow!(
            "timecode MIDI timing is not supported by the realtime adapter"
        )),
    }
}

fn ticks_to_micros(ticks: u64, ticks_per_beat: u16, tempo_micros_per_beat: u32) -> u64 {
    ticks
        .saturating_mul(u64::from(tempo_micros_per_beat))
        .saturating_div(u64::from(ticks_per_beat))
}

fn normalize_base_path(input: &str) -> String {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        "/state_space_music_box".to_string()
    } else if trimmed.starts_with('/') {
        trimmed.trim_end_matches('/').to_string()
    } else {
        format!("/{}", trimmed.trim_end_matches('/'))
    }
}

fn validate_new_adapter_request(request: &NewRealtimeAdapterRequest) -> Result<()> {
    ensure!(
        !request.display_name.trim().is_empty(),
        "display name cannot be empty"
    );
    ensure!(request.port > 0, "port must be greater than zero");
    ensure!(
        matches!(request.protocol, RealtimeAdapterProtocol::OscUdp),
        "unsupported realtime adapter protocol"
    );
    ensure!(
        !normalize_base_path(&request.base_path).trim().is_empty(),
        "base path cannot be empty"
    );
    Ok(())
}

fn validate_preview_request(adapter_id: &str, request: &SendRealtimePreviewRequest) -> Result<()> {
    ensure!(!adapter_id.trim().is_empty(), "adapter id cannot be empty");
    ensure!(
        !request.actor_id.trim().is_empty(),
        "actor id cannot be empty"
    );
    ensure!(
        !request.session_id.trim().is_empty(),
        "session id cannot be empty"
    );
    ensure!(
        !request.preview_id.trim().is_empty(),
        "preview id cannot be empty"
    );
    ensure!(request.time_scale >= 0.0, "time_scale must be non-negative");
    Ok(())
}

fn validate_transport_request(
    adapter_id: &str,
    request: &SendRealtimeTransportRequest,
) -> Result<()> {
    ensure!(!adapter_id.trim().is_empty(), "adapter id cannot be empty");
    ensure!(
        !request.actor_id.trim().is_empty(),
        "actor id cannot be empty"
    );
    ensure!(
        !request.deck_id.trim().is_empty(),
        "deck id cannot be empty"
    );
    ensure!(request.time_scale >= 0.0, "time_scale must be non-negative");
    Ok(())
}

fn load_store(store_path: &Path) -> Result<RealtimeStoreFile> {
    let mut store: RealtimeStoreFile = read_json_or_default(store_path)?;
    if store.version == 0 {
        store.version = 1;
    }
    Ok(store)
}

fn save_store(store_path: &Path, store: &RealtimeStoreFile) -> Result<()> {
    write_pretty_json(store_path, store)
}

#[cfg(test)]
mod tests {
    use std::net::UdpSocket;

    use tempfile::tempdir;

    use super::*;
    use crate::generation::{demo_preset, save_preset};
    use crate::governance::{
        add_preview_clip_to_deck, create_deck, create_session, default_daw_store_path,
        default_session_store_path, render_session_preview, AddDeckPreviewRequest, NewDeckRequest,
        NewSessionRequest,
    };

    #[test]
    fn test_send_preview_and_transport_to_osc_adapter() {
        let dir = tempdir().unwrap();
        let preset_dir = dir.path().join("presets");
        let runtime_dir = dir.path().join("runtime");
        let session_store = default_session_store_path(&runtime_dir);
        let daw_store = default_daw_store_path(&runtime_dir);
        let realtime_store = runtime_dir.join("realtime-adapters.json");

        let mut preset = demo_preset();
        preset.name = "live-demo".to_string();
        save_preset(&preset, &preset_dir).unwrap();

        let listener = UdpSocket::bind("127.0.0.1:0").unwrap();
        listener
            .set_read_timeout(Some(Duration::from_millis(250)))
            .unwrap();
        let port = listener.local_addr().unwrap().port();

        let adapter = create_realtime_adapter(
            &realtime_store,
            NewRealtimeAdapterRequest {
                display_name: "Loopback".to_string(),
                protocol: RealtimeAdapterProtocol::OscUdp,
                host: "127.0.0.1".parse().unwrap(),
                port,
                base_path: "/agentic_dj".to_string(),
            },
        )
        .unwrap();

        let session = create_session(
            &session_store,
            &preset_dir,
            NewSessionRequest {
                display_name: "Live Session".to_string(),
                preset_name: "live-demo".to_string(),
                seed: 3,
                actor_id: "tester".to_string(),
            },
        )
        .unwrap();
        let preview = render_session_preview(
            &session_store,
            &preset_dir,
            &runtime_dir,
            &session.session_id,
            "tester",
        )
        .unwrap();

        let preview_summary = send_preview_to_realtime_adapter(
            &realtime_store,
            &session_store,
            &adapter.adapter_id,
            SendRealtimePreviewRequest {
                actor_id: "tester".to_string(),
                session_id: session.session_id.clone(),
                preview_id: preview.preview.preview_id.clone(),
                dispatch_mode: RealtimeDispatchMode::Immediate,
                time_scale: 0.0,
            },
        )
        .unwrap();
        assert!(preview_summary.dispatch.message_count >= 3);

        let deck = create_deck(
            &daw_store,
            &session_store,
            NewDeckRequest {
                display_name: "Deck A".to_string(),
                session_id: session.session_id.clone(),
                actor_id: "tester".to_string(),
            },
        )
        .unwrap();
        let deck = add_preview_clip_to_deck(
            &daw_store,
            &session_store,
            &deck.deck_id,
            AddDeckPreviewRequest {
                actor_id: "tester".to_string(),
                label: "Clip".to_string(),
                session_id: session.session_id.clone(),
                preview_id: preview.preview.preview_id.clone(),
            },
        )
        .unwrap();
        let launched = crate::governance::launch_deck_clip(
            &daw_store,
            &deck.deck_id,
            crate::governance::LaunchDeckClipRequest {
                actor_id: "tester".to_string(),
                clip_id: deck.clips[0].clip_id.clone(),
            },
        )
        .unwrap();
        assert!(launched.active_clip.is_some());

        let transport_summary = send_transport_to_realtime_adapter(
            &realtime_store,
            &daw_store,
            &adapter.adapter_id,
            SendRealtimeTransportRequest {
                actor_id: "tester".to_string(),
                deck_id: deck.deck_id.clone(),
                dispatch_mode: RealtimeDispatchMode::Immediate,
                time_scale: 0.0,
            },
        )
        .unwrap();
        assert!(transport_summary.dispatch.message_count >= 1);

        let mut buf = [0u8; 2048];
        let (size, _) = listener.recv_from(&mut buf).unwrap();
        let packet = rosc::decoder::decode_udp(&buf[..size]).unwrap().1;
        match packet {
            OscPacket::Message(message) => {
                assert!(message.addr.starts_with("/agentic_dj/"));
            }
            other => panic!("unexpected OSC packet: {other:?}"),
        }
    }
}
