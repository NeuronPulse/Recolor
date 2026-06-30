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

pub struct TrackNote {
    pub pitch: String,
    pub start: f32,
    pub length: f32,
}
