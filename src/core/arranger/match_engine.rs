use crate::core::midi::MidiNote;
use crate::core::sample::SampleClip;

use super::timeline::{auto_match, PlacedClip};

pub fn suggest_matches(notes: &[MidiNote], samples: &[SampleClip]) -> Vec<(usize, usize, f32)> {
    let mut suggestions = Vec::new();

    for (note_idx, note) in notes.iter().enumerate() {
        let mut best_score = f32::MIN;
        let mut best_sample_idx = 0;

        for (sample_idx, sample) in samples.iter().enumerate() {
            let pitch_diff = (note.key as i32 - sample.pitch as i32).unsigned_abs() as f32;
            let pitch_score = 1.0 - (pitch_diff / 12.0).min(1.0);

            let duration_ratio = if sample.duration > 0.0 {
                let ratio = note.length / sample.duration;
                if ratio > 1.0 {
                    1.0 / ratio
                } else {
                    ratio
                }
            } else {
                0.5
            };

            let score = pitch_score * 0.7 + duration_ratio * 0.3;

            if score > best_score {
                best_score = score;
                best_sample_idx = sample_idx;
            }
        }

        if best_score > 0.3 {
            suggestions.push((note_idx, best_sample_idx, best_score));
        }
    }

    suggestions
}

pub fn match_with_preference(
    midi: &crate::core::midi::MidiData,
    library: &crate::core::sample::SampleLibrary,
    prefer_exact: bool,
) -> Vec<PlacedClip> {
    let max_semitones = if prefer_exact { 0 } else { 2 };
    auto_match(&midi.notes, &library.clips, max_semitones)
}
