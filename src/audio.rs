use std::collections::HashMap;
use std::io::BufReader;
use std::path::Path;

use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};

const SAMPLE_RATE: u32 = 44100;

pub struct AudioEngine {
    _stream: OutputStream,
    handle: OutputStreamHandle,
    master_sink: Sink,
    loaded_samples: HashMap<usize, LoadedSample>,
    rendered_buffer: Option<Vec<f32>>,
}

struct LoadedSample {
    data: Vec<f32>,
    sample_rate: u32,
}

impl AudioEngine {
    pub fn new() -> Self {
        let (stream, handle) = OutputStream::try_default().expect("无法打开音频输出设备");
        let master_sink = Sink::try_new(&handle).expect("无法创建音频输出");

        Self {
            _stream: stream,
            handle,
            master_sink,
            loaded_samples: HashMap::new(),
            rendered_buffer: None,
        }
    }

    pub fn load_sample(&mut self, id: usize, path: &Path) -> bool {
        let file = match std::fs::File::open(path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("无法打开音频文件 {}: {}", path.display(), e);
                return false;
            }
        };

        let reader = BufReader::new(file);
        let decoder = match Decoder::new(reader) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("无法解码音频文件 {}: {}", path.display(), e);
                return false;
            }
        };

        let sr = decoder.sample_rate();
        let channels = decoder.channels() as usize;
        let samples: Vec<f32> = decoder.convert_samples::<f32>().collect();

        let mono: Vec<f32> = if channels == 1 {
            samples
        } else {
            samples
                .chunks(channels)
                .map(|chunk| chunk.iter().sum::<f32>() / channels as f32)
                .collect()
        };

        self.loaded_samples.insert(
            id,
            LoadedSample {
                data: mono,
                sample_rate: sr,
            },
        );
        true
    }

    /// Pre-render all MIDI notes into a single audio buffer.
    /// This is called once when play starts, not per-frame.
    pub fn render_midi(&mut self, notes: &[crate::core::midi::MidiNote]) {
        if notes.is_empty() {
            self.rendered_buffer = None;
            return;
        }

        let total_time = notes
            .iter()
            .map(|n| n.start + n.length)
            .fold(0.0f32, f32::max);
        let total_samples = (total_time * SAMPLE_RATE as f32) as usize + SAMPLE_RATE as usize;
        let mut buffer = vec![0.0f32; total_samples];

        for note in notes {
            let freq = 440.0 * 2.0_f32.powf((note.key as f32 - 69.0) / 12.0);
            let vel = note.velocity as f32 / 127.0;
            let start_sample = (note.start * SAMPLE_RATE as f32) as usize;
            let dur_samples = (note.length * SAMPLE_RATE as f32) as usize;

            for i in 0..dur_samples {
                let pos = start_sample + i;
                if pos >= buffer.len() {
                    break;
                }
                let t = i as f32 / SAMPLE_RATE as f32;
                let env = 1.0 - (i as f32 / dur_samples as f32);
                let env = env * env;
                buffer[pos] += (2.0 * std::f32::consts::PI * freq * t).sin() * vel * env * 0.4;
            }
        }

        // clip to [-1, 1]
        for s in &mut buffer {
            *s = s.clamp(-1.0, 1.0);
        }

        self.rendered_buffer = Some(buffer);
    }

    pub fn start_playback(&mut self) {
        self.master_sink.clear();
        if let Some(ref buf) = self.rendered_buffer {
            let source = rodio::buffer::SamplesBuffer::new(1, SAMPLE_RATE, buf.clone());
            self.master_sink.append(source);
            self.master_sink.play();
        }
    }

    pub fn stop_all(&self) {
        self.master_sink.clear();
    }

    pub fn clear_rendered(&mut self) {
        self.rendered_buffer = None;
    }

    pub fn has_rendered(&self) -> bool {
        self.rendered_buffer.is_some()
    }

    pub fn is_sample_loaded(&self, id: usize) -> bool {
        self.loaded_samples.contains_key(&id)
    }

    pub fn has_samples(&self) -> bool {
        !self.loaded_samples.is_empty()
    }
}
