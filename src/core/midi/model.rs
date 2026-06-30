#[derive(Clone, Debug)]
pub struct MidiNote {
    pub key: u8,
    pub pitch: String,
    pub start: f32,
    pub length: f32,
    pub velocity: u8,
    pub channel: u8,
    pub track_index: usize,
}

#[derive(Clone, Debug)]
pub struct MidiTrack {
    pub name: String,
    pub note_count: usize,
    pub channel: u8,
}

#[derive(Clone, Debug)]
pub struct MidiData {
    pub file_name: String,
    pub tracks: Vec<MidiTrack>,
    pub notes: Vec<MidiNote>,
    pub tempo: f32,
}

impl MidiData {
    pub fn total_duration(&self) -> f32 {
        self.notes
            .iter()
            .map(|n| n.start + n.length)
            .fold(0.0f32, f32::max)
    }

    pub fn notes_for_track(&self, track_index: usize) -> Vec<&MidiNote> {
        self.notes
            .iter()
            .filter(|n| n.track_index == track_index)
            .collect()
    }

    pub fn pitch_range(&self) -> (u8, u8) {
        let min = self.notes.iter().map(|n| n.key).min().unwrap_or(0);
        let max = self.notes.iter().map(|n| n.key).max().unwrap_or(127);
        (min, max)
    }
}
