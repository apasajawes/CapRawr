/// Format seconds into standard timecode: HH:MM:SS:FF
pub fn format_timecode(seconds: f64, fps: f64) -> String {
    let total_secs = seconds.max(0.0);
    let hours = (total_secs / 3600.0) as u32;
    let mins = ((total_secs % 3600.0) / 60.0) as u32;
    let secs = (total_secs % 60.0) as u32;
    let frames = ((total_secs % 1.0) * fps).round() as u32;
    format!("{:02}:{:02}:{:02}:{:02}", hours, mins, secs, frames)
}

/// Parse timecode string "HH:MM:SS:FF" back to seconds
pub fn parse_timecode(tc: &str, fps: f64) -> Option<f64> {
    let parts: Vec<&str> = tc.split(':').collect();
    if parts.len() != 4 {
        return None;
    }
    let h: f64 = parts[0].parse().ok()?;
    let m: f64 = parts[1].parse().ok()?;
    let s: f64 = parts[2].parse().ok()?;
    let f: f64 = parts[3].parse().ok()?;
    Some(h * 3600.0 + m * 60.0 + s + f / fps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timecode_roundtrip() {
        let tc = format_timecode(73.5, 30.0);
        assert_eq!(tc, "00:01:13:15");
        let parsed = parse_timecode(&tc, 30.0).unwrap();
        assert!((parsed - 73.5).abs() < 1e-4);
    }
}
