use std::collections::HashSet;

#[derive(Clone, PartialEq)]
pub enum Panel {
    Library,
    Tracks,
    Generate,
}

pub struct ViewState {
    pub active_panel: Panel,
    pub selected_track: Option<usize>,
    pub error_message: Option<String>,
    pub is_playing: bool,
    pub current_time: f32,
    pub piano_roll: super::piano_roll::PianoRollState,
    pub playing_notes: HashSet<usize>,
}

impl ViewState {
    pub fn new() -> Self {
        Self {
            active_panel: Panel::Library,
            selected_track: None,
            error_message: None,
            is_playing: false,
            current_time: 0.0,
            piano_roll: super::piano_roll::PianoRollState::default(),
            playing_notes: HashSet::new(),
        }
    }
}
