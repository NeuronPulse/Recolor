use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct SampleClip {
    pub id: usize,
    pub path: PathBuf,
    pub name: String,
    pub pitch: u8,
    pub duration: f32,
    pub sample_rate: u32,
    pub format: String,
    pub tags: Vec<String>,
}

impl SampleClip {
    pub fn pitch_name(&self) -> String {
        super::super::midi::pitch::key_to_name(self.pitch)
    }
}

pub struct SampleLibrary {
    pub clips: Vec<SampleClip>,
    next_id: usize,
}

impl SampleLibrary {
    pub fn new() -> Self {
        Self {
            clips: Vec::new(),
            next_id: 0,
        }
    }

    pub fn add_clip(&mut self, clip: SampleClip) -> usize {
        let id = self.next_id;
        let mut clip = clip;
        clip.id = id;
        self.clips.push(clip);
        self.next_id += 1;
        id
    }

    pub fn remove_clip(&mut self, id: usize) -> bool {
        let before = self.clips.len();
        self.clips.retain(|c| c.id != id);
        self.clips.len() < before
    }

    pub fn get_clip(&self, id: usize) -> Option<&SampleClip> {
        self.clips.iter().find(|c| c.id == id)
    }

    pub fn find_by_pitch(&self, pitch: u8) -> Vec<&SampleClip> {
        self.clips.iter().filter(|c| c.pitch == pitch).collect()
    }

    pub fn find_similar(&self, pitch: u8, max_semitones: u8) -> Vec<&SampleClip> {
        self.clips
            .iter()
            .filter(|c| c.pitch.abs_diff(pitch) <= max_semitones)
            .collect()
    }

    pub fn len(&self) -> usize {
        self.clips.len()
    }

    pub fn is_empty(&self) -> bool {
        self.clips.is_empty()
    }
}
