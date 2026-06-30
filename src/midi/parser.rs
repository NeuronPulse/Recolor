use std::path::Path;

use midly::{MetaMessage, MidiMessage, Smf, TrackEventKind};

use crate::models::{MidiData, MidiTrack, TrackNote};
use crate::midi::pitch::key_to_name;

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

    let mut all_notes: Vec<TrackNote> = Vec::new();
    let mut tracks: Vec<MidiTrack> = Vec::new();
    let mut tempo: f32 = 120.0;

    for (track_idx, track) in smf.tracks.iter().enumerate() {
        let mut track_name = format!("Track {}", track_idx + 1);
        let mut notes_on: std::collections::HashMap<(u8, u8), (u64, u8)> =
            std::collections::HashMap::new();
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
                TrackEventKind::Meta(MetaMessage::Tempo(t)) => {
                    tempo = 60_000_000.0 / t.as_int() as f32;
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
                            } else {
                                if let Some((start_tick, velocity)) =
                                    notes_on.remove(&(key.as_int(), channel.as_int()))
                                {
                                    let start_time =
                                        tick_to_seconds(start_tick, tempo, ticks_per_beat);
                                    let end_time =
                                        tick_to_seconds(current_tick, tempo, ticks_per_beat);
                                    let length = end_time - start_time;

                                    all_notes.push(TrackNote {
                                        key: key.as_int(),
                                        pitch: key_to_name(key.as_int()),
                                        start: start_time as f32,
                                        length: length as f32,
                                        velocity,
                                        channel: channel.as_int(),
                                        track_index: track_idx,
                                    });
                                    note_count += 1;
                                }
                            }
                        }
                        MidiMessage::NoteOff { key, .. } => {
                            if let Some((start_tick, velocity)) =
                                notes_on.remove(&(key.as_int(), channel.as_int()))
                            {
                                let start_time =
                                    tick_to_seconds(start_tick, tempo, ticks_per_beat);
                                let end_time =
                                    tick_to_seconds(current_tick, tempo, ticks_per_beat);
                                let length = end_time - start_time;

                                all_notes.push(TrackNote {
                                    key: key.as_int(),
                                    pitch: key_to_name(key.as_int()),
                                    start: start_time as f32,
                                    length: length as f32,
                                    velocity,
                                    channel: track_channel,
                                    track_index: track_idx,
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

    if all_notes.is_empty() {
        return Err(MidiParseError::NoNotes);
    }

    all_notes.sort_by(|a, b| a.start.partial_cmp(&b.start).unwrap());

    Ok(MidiData {
        file_name,
        tracks,
        notes: all_notes,
        tempo,
    })
}

fn tick_to_seconds(tick: u64, tempo: f32, ticks_per_beat: u16) -> f64 {
    let seconds_per_beat = 60.0 / tempo;
    (tick as f64) * (seconds_per_beat as f64) / (ticks_per_beat as f64)
}
