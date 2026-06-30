use std::path::{Path, PathBuf};

use crate::core::arranger::Arrangement;
use crate::core::midi::{MidiData, MidiNote};
use crate::core::sample::{SampleClip, SampleLibrary};

pub struct Project {
    pub midi: Option<MidiData>,
    pub samples: SampleLibrary,
    pub arrangement: Arrangement,
    pub file_path: Option<PathBuf>,
    pub name: String,
    pub bpm: f32,
}

impl Project {
    pub fn new() -> Self {
        Self {
            midi: None,
            samples: SampleLibrary::new(),
            arrangement: Arrangement::new(),
            file_path: None,
            name: "未命名项目".into(),
            bpm: 120.0,
        }
    }

    pub fn load_midi(&mut self, path: &Path) -> Result<(), String> {
        let midi = crate::core::midi::parse_midi_file(path)
            .map_err(|e| format!("MIDI 解析失败: {}", e))?;
        self.bpm = midi.tempo;
        self.midi = Some(midi);
        self.arrangement.clear();
        Ok(())
    }

    pub fn import_samples_from_dir(&mut self, dir: &Path) -> Result<usize, String> {
        crate::core::sample::scan_directory(dir, &mut self.samples)
            .map_err(|e| format!("扫描失败: {:?}", e))
    }

    pub fn add_sample(&mut self, clip: SampleClip) -> usize {
        self.samples.add_clip(clip)
    }

    pub fn notes(&self) -> &[MidiNote] {
        self.midi
            .as_ref()
            .map(|m| m.notes.as_slice())
            .unwrap_or(&[])
    }

    pub fn samples(&self) -> &[SampleClip] {
        &self.samples.clips
    }

    pub fn total_duration(&self) -> f32 {
        self.midi
            .as_ref()
            .map(|m| m.total_duration())
            .unwrap_or(0.0)
    }

    pub fn note_count(&self) -> usize {
        self.midi.as_ref().map(|m| m.notes.len()).unwrap_or(0)
    }

    pub fn track_count(&self) -> usize {
        self.midi.as_ref().map(|m| m.tracks.len()).unwrap_or(0)
    }

    pub fn is_loaded(&self) -> bool {
        self.midi.is_some()
    }

    pub fn save(&self, path: &Path) -> Result<(), String> {
        let json = self.to_json();
        std::fs::write(path, json).map_err(|e| format!("保存失败: {}", e))
    }

    pub fn load(&mut self, path: &Path) -> Result<(), String> {
        let json = std::fs::read_to_string(path).map_err(|e| format!("读取失败: {}", e))?;
        self.load_json(&json)
    }

    fn to_json(&self) -> String {
        let mut json = String::from("{\n");
        json.push_str(&format!("  \"name\": {:?},\n", self.name));
        json.push_str(&format!("  \"bpm\": {},\n", self.bpm));

        json.push_str("  \"samples\": [\n");
        for (i, clip) in self.samples.clips.iter().enumerate() {
            json.push_str(&format!(
                "    {{\"id\": {}, \"name\": {:?}, \"path\": {:?}, \"pitch\": {}}}",
                clip.id, clip.name, clip.path, clip.pitch
            ));
            if i < self.samples.clips.len() - 1 {
                json.push(',');
            }
            json.push('\n');
        }
        json.push_str("  ],\n");

        json.push_str(&format!(
            "  \"arrangement\": {{\"track_count\": {}}}\n",
            self.arrangement.tracks.len()
        ));

        json.push('}');
        json
    }

    fn load_json(&mut self, _json: &str) -> Result<(), String> {
        Err("载入功能尚未实现".into())
    }
}
