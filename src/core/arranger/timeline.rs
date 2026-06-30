use crate::core::midi::MidiNote;
use crate::core::sample::SampleClip;

#[derive(Clone, Debug)]
pub struct PlacedClip {
    pub sample_id: usize,
    pub note_index: usize,
    pub start_time: f32,
    pub pitch_shift: i8,
}

#[derive(Clone, Debug)]
pub struct ArrangementTrack {
    pub midi_track_index: usize,
    pub clips: Vec<PlacedClip>,
}

#[derive(Clone, Debug)]
pub struct Arrangement {
    pub tracks: Vec<ArrangementTrack>,
}

impl Arrangement {
    pub fn new() -> Self {
        Self { tracks: Vec::new() }
    }

    pub fn clear(&mut self) {
        self.tracks.clear();
    }

    pub fn get_clip_for_note(&self, track_index: usize, note_index: usize) -> Option<&PlacedClip> {
        self.tracks
            .iter()
            .find(|t| t.midi_track_index == track_index)
            .and_then(|t| t.clips.iter().find(|c| c.note_index == note_index))
    }

    pub fn place_clip(&mut self, track_index: usize, clip: PlacedClip) {
        let track = self
            .tracks
            .iter_mut()
            .find(|t| t.midi_track_index == track_index);

        if let Some(track) = track {
            track.clips.retain(|c| c.note_index != clip.note_index);
            track.clips.push(clip);
        } else {
            self.tracks.push(ArrangementTrack {
                midi_track_index: track_index,
                clips: vec![clip],
            });
        }
    }

    pub fn remove_clip(&mut self, track_index: usize, note_index: usize) {
        if let Some(track) = self
            .tracks
            .iter_mut()
            .find(|t| t.midi_track_index == track_index)
        {
            track.clips.retain(|c| c.note_index != note_index);
        }
    }
}

pub fn auto_match(
    notes: &[MidiNote],
    samples: &[SampleClip],
    max_semitones: u8,
) -> Vec<PlacedClip> {
    let mut placements = Vec::new();

    for (note_idx, note) in notes.iter().enumerate() {
        let best = samples
            .iter()
            .filter(|s| s.pitch.abs_diff(note.key) <= max_semitones)
            .min_by_key(|s| s.pitch.abs_diff(note.key) as u32);

        if let Some(sample) = best {
            let pitch_shift = note.key as i8 - sample.pitch as i8;
            placements.push(PlacedClip {
                sample_id: sample.id,
                note_index: note_idx,
                start_time: note.start,
                pitch_shift,
            });
        }
    }

    placements
}
