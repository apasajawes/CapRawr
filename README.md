# CapRawr 🎬

> A video editor built from scratch in Rust, inspired by CapCut reverse engineering analysis.

## Overview

CapRawr is a clean-room reimplementation of core non-linear video editing (NLE) capabilities, designed with modern Rust:

- **GUI**: egui + eframe (immediate mode, lightweight, 60fps)
- **Data Model**: Multi-track timeline, non-destructive editing, keyframe animation
- **File Format**: JSON-based project files (`.caprawr`)
- **Architecture**: Modular engine / UI / media separation

## Reverse Engineering Insights from CapCut

Analysis of CapCut v9.7.1 (ByteDance) revealed:
- **`CapCut.exe`** is only a 90KB bootstrap launcher
- Core logic resides in **`videoeditor.dll`** (69MB native C++)
- 195 distinct editing actions identified (Timeline, Audio, Video, AI, Text, Transitions)
- UI uses **Qt6 QML** + ByteDance's custom **Lynx** framework
- Media pipeline wraps **FFmpeg**, **SVT-AV1**, **dav1d**, **OpenVINO**
- AI pipeline handled by **`lens.dll`** (84MB)

CapRawr takes the proven NLE architecture patterns from this analysis and implements them cleanly in safe Rust.

## Current Features

- [x] Multi-track timeline (Video, Audio, Text tracks)
- [x] Non-destructive clip trimming & splitting
- [x] Timecode display & navigation (HH:MM:SS:FF)
- [x] Interactive playhead & scrubbing
- [x] Clip selection & property editing (position, duration, speed, volume, opacity)
- [x] Timeline zoom & scrolling
- [x] Project save/load (`.caprawr` JSON format)
- [x] Keyframe animation engine with easing (Linear, EaseIn, EaseOut, EaseInOut)
- [x] Media file import dialog
- [ ] FFmpeg decode/encode integration (next phase)
- [ ] Real-time video preview playback (next phase)
- [ ] GPU effect pipeline via wgpu (next phase)

## Building

```bash
cargo build --release
cargo run
```

## Running Tests

```bash
cargo test --bin caprawr
```

## License

MIT
