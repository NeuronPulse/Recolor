const NOTE_NAMES: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

pub fn key_to_name(key: u8) -> String {
    let octave = (key / 12) as i8 - 1;
    let note_index = (key % 12) as usize;
    format!("{}{}", NOTE_NAMES[note_index], octave)
}

pub fn name_to_key(name: &str) -> Option<u8> {
    let name = name.trim();
    if name.len() < 2 {
        return None;
    }

    let (note_part, octave_part) = if name.starts_with('#') || name.starts_with('b') {
        return None;
    } else {
        let sharp = name.ends_with('#');
        let flat = name.ends_with('b');

        if sharp || flat {
            (&name[..name.len() - 1], &name[name.len() - 1..])
        } else {
            (&name[..1], &name[1..])
        }
    };

    let note_index = match note_part {
        "C" => 0,
        "C#" | "Db" => 1,
        "D" => 2,
        "D#" | "Eb" => 3,
        "E" => 4,
        "F" => 5,
        "F#" | "Gb" => 6,
        "G" => 7,
        "G#" | "Ab" => 8,
        "A" => 9,
        "A#" | "Bb" => 10,
        "B" => 11,
        _ => return None,
    };

    let octave: i8 = octave_part.parse().ok()?;
    if !(-1..=9).contains(&octave) {
        return None;
    }

    Some(((octave + 1) as u8 * 12 + note_index as u8) & 0x7F)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_to_name() {
        assert_eq!(key_to_name(60), "C4");
        assert_eq!(key_to_name(69), "A4");
        assert_eq!(key_to_name(0), "C-1");
        assert_eq!(key_to_name(127), "G9");
    }

    #[test]
    fn test_name_to_key() {
        assert_eq!(name_to_key("C4"), Some(60));
        assert_eq!(name_to_key("A4"), Some(69));
        assert_eq!(name_to_key("C-1"), Some(0));
        assert_eq!(name_to_key("G9"), Some(127));
        assert_eq!(name_to_key("C#4"), Some(61));
        assert_eq!(name_to_key("Db4"), Some(61));
    }
}
