#[derive(Clone, PartialEq)]
pub enum Panel {
    Library,
    Tracks,
    Generate,
}

pub struct SampleItem {
    pub name: String,
    pub duration: String,
    pub sample_rate: String,
    pub pitch: String,
    pub format: String,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct TrackNote {
    pub key: u8,
    pub pitch: String,
    pub start: f32,
    pub length: f32,
    pub velocity: u8,
    pub channel: u8,
    pub track_index: usize,
}

pub struct MidiTrack {
    pub name: String,
    pub note_count: usize,
    #[allow(dead_code)]
    pub channel: u8,
}

pub struct MidiData {
    pub file_name: String,
    pub tracks: Vec<MidiTrack>,
    pub notes: Vec<TrackNote>,
    pub tempo: f32,
}
