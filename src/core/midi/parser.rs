use std::collections::HashMap;
use std::path::Path;

use midly::{MetaMessage, MidiMessage, Smf, TrackEventKind};

use super::model::{MidiData, MidiNote, MidiTrack};
use super::pitch::key_to_name;

#[derive(Debug)]
pub enum MidiParseError {
    IoError(std::io::Error),
    ParseError(midly::Error),
    NoTracks,
    NoNotes,
}

impl From<std::io::Error> for MidiParseError {
    fn from(e: std::io::Error) -> Self {
        MidiParseError::IoError(e)
    }
}

impl From<midly::Error> for MidiParseError {
    fn from(e: midly::Error) -> Self {
        MidiParseError::ParseError(e)
    }
}

impl std::fmt::Display for MidiParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MidiParseError::IoError(e) => write!(f, "IO error: {}", e),
            MidiParseError::ParseError(e) => write!(f, "Parse error: {}", e),
            MidiParseError::NoTracks => write!(f, "No tracks found"),
            MidiParseError::NoNotes => write!(f, "No notes found"),
        }
    }
}

pub fn parse_midi_file(path: &Path) -> Result<MidiData, MidiParseError> {
    let data = std::fs::read(path)?;
    let smf = Smf::parse(&data)?;

    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    let ticks_per_beat = match smf.header.timing {
        midly::Timing::Metrical(tpb) => tpb.as_int(),
        midly::Timing::Timecode(..) => 480,
    };

    // ── Pass 1: collect ALL tempo events from ALL tracks ──
    let mut tempo_events: Vec<(u64, u32)> = Vec::new();
    for track in &smf.tracks {
        let mut tick: u64 = 0;
        for event in track {
            tick += event.delta.as_int() as u64;
            if let TrackEventKind::Meta(MetaMessage::Tempo(t)) = event.kind {
                tempo_events.push((tick, t.as_int()));
            }
        }
    }
    tempo_events.sort_by_key(|e| e.0);

    // ── Pass 2: collect notes as ticks, collect track metadata ──
    struct RawNote {
        key: u8,
        velocity: u8,
        channel: u8,
        track_index: usize,
        start_tick: u64,
        end_tick: u64,
    }

    let mut raw_notes: Vec<RawNote> = Vec::new();
    let mut tracks: Vec<MidiTrack> = Vec::new();

    for (track_idx, track) in smf.tracks.iter().enumerate() {
        let mut track_name = format!("Track {}", track_idx + 1);
        let mut notes_on: HashMap<(u8, u8), (u64, u8)> = HashMap::new();
        let mut current_tick: u64 = 0;
        let mut track_channel: u8 = 0;
        let mut note_count: usize = 0;

        for event in track {
            current_tick += event.delta.as_int() as u64;

            match event.kind {
                TrackEventKind::Meta(MetaMessage::TrackName(name)) => {
                    if let Ok(name_str) = std::str::from_utf8(name) {
                        track_name = name_str.to_string();
                    }
                }
                TrackEventKind::Midi { channel, message } => {
                    track_channel = channel.as_int();
                    match message {
                        MidiMessage::NoteOn { key, vel } => {
                            if vel.as_int() > 0 {
                                notes_on.insert(
                                    (key.as_int(), channel.as_int()),
                                    (current_tick, vel.as_int()),
                                );
                            } else if let Some((start_tick, velocity)) =
                                notes_on.remove(&(key.as_int(), channel.as_int()))
                            {
                                raw_notes.push(RawNote {
                                    key: key.as_int(),
                                    velocity,
                                    channel: channel.as_int(),
                                    track_index: track_idx,
                                    start_tick,
                                    end_tick: current_tick,
                                });
                                note_count += 1;
                            }
                        }
                        MidiMessage::NoteOff { key, .. } => {
                            if let Some((start_tick, velocity)) =
                                notes_on.remove(&(key.as_int(), channel.as_int()))
                            {
                                raw_notes.push(RawNote {
                                    key: key.as_int(),
                                    velocity,
                                    channel: track_channel,
                                    track_index: track_idx,
                                    start_tick,
                                    end_tick: current_tick,
                                });
                                note_count += 1;
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        if note_count > 0 {
            tracks.push(MidiTrack {
                name: track_name,
                note_count,
                channel: track_channel,
            });
        }
    }

    if tracks.is_empty() {
        return Err(MidiParseError::NoTracks);
    }

    if raw_notes.is_empty() {
        return Err(MidiParseError::NoNotes);
    }

    // ── Pass 3: convert ticks to seconds using complete tempo map ──
    let all_notes: Vec<MidiNote> = raw_notes
        .into_iter()
        .map(|rn| {
            let start = tick_to_seconds(rn.start_tick, &tempo_events, ticks_per_beat);
            let end = tick_to_seconds(rn.end_tick, &tempo_events, ticks_per_beat);
            let length = (end - start).max(0.01);
            MidiNote {
                key: rn.key,
                pitch: key_to_name(rn.key),
                start: start as f32,
                length: length as f32,
                velocity: rn.velocity,
                channel: rn.channel,
                track_index: rn.track_index,
            }
        })
        .collect();

    let mut sorted_notes = all_notes;
    sorted_notes.sort_by(|a, b| a.start.partial_cmp(&b.start).unwrap());

    let tempo = tempo_events
        .first()
        .map(|(_, uspb)| 60_000_000.0 / *uspb as f32)
        .unwrap_or(120.0);

    Ok(MidiData {
        file_name,
        tracks,
        notes: sorted_notes,
        tempo,
    })
}

fn tick_to_seconds(tick: u64, tempo_events: &[(u64, u32)], ticks_per_beat: u16) -> f64 {
    // find the active tempo for this tick
    let uspb = tempo_events
        .iter()
        .rev()
        .find(|(t, _)| *t <= tick)
        .map(|(_, u)| *u)
        .unwrap_or(500_000); // default 120 BPM = 500,000 us/beat

    let seconds_per_beat = uspb as f64 / 1_000_000.0;
    (tick as f64 / ticks_per_beat as f64) * seconds_per_beat
}
