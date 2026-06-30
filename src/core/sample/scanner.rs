use std::path::{Path, PathBuf};

use super::model::{SampleClip, SampleLibrary};

#[derive(Debug)]
pub enum ScanError {
    IoError(std::io::Error),
    InvalidPath,
}

impl From<std::io::Error> for ScanError {
    fn from(e: std::io::Error) -> Self {
        ScanError::IoError(e)
    }
}

const VIDEO_EXTENSIONS: &[&str] = &[
    "mp4", "mkv", "avi", "mov", "webm", "flv", "wmv", "ts", "m4v",
];

const AUDIO_EXTENSIONS: &[&str] = &["wav", "mp3", "ogg", "flac", "aac", "m4a"];

pub fn scan_directory(dir: &Path, library: &mut SampleLibrary) -> Result<usize, ScanError> {
    let mut count = 0;

    if !dir.is_dir() {
        return Err(ScanError::InvalidPath);
    }

    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            count += scan_directory(&path, library)?;
            continue;
        }

        if let Some(clip) = process_file(&path, library.clips.len()) {
            library.add_clip(clip);
            count += 1;
        }
    }

    Ok(count)
}

fn process_file(path: &Path, next_id: usize) -> Option<SampleClip> {
    let ext = path.extension()?.to_str()?.to_lowercase();

    let is_video = VIDEO_EXTENSIONS.contains(&ext.as_str());
    let is_audio = AUDIO_EXTENSIONS.contains(&ext.as_str());

    if !is_video && !is_audio {
        return None;
    }

    let name = path
        .file_stem()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    let format = ext.to_uppercase();

    let metadata = std::fs::metadata(path).ok()?;
    let _size = metadata.len();

    Some(SampleClip {
        id: next_id,
        path: path.to_path_buf(),
        name,
        pitch: 60,
        duration: 0.0,
        sample_rate: 44100,
        format,
        tags: Vec::new(),
    })
}

pub fn find_samples_recursive(dir: &Path) -> Vec<PathBuf> {
    let mut results = Vec::new();

    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                results.extend(find_samples_recursive(&path));
            } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                let ext_lower = ext.to_lowercase();
                if VIDEO_EXTENSIONS.contains(&ext_lower.as_str())
                    || AUDIO_EXTENSIONS.contains(&ext_lower.as_str())
                {
                    results.push(path);
                }
            }
        }
    }

    results
}
