# CapRawr — Architecture & Reverse Engineering Analysis

## Target: CapCut v9.7.1.4203 (ByteDance)

### Format & Stack Analysis

| Component | Technology | Size | Purpose |
|---|---|---|---|
| `CapCut.exe` | C++ PE64 launcher | 90KB | Tiny bootstrap → loads `videoeditor.dll` |
| `videoeditor.dll` | C++ native | 69MB | **Core video editing engine** (NLE core) |
| `lens.dll` | C++ native | 84MB | **AI/ML vision pipeline** (effects, filters, face detection) |
| `lynx.dll` | C++ native | 23MB | ByteDance Lynx cross-platform UI framework |
| `FusionUI.dll` | C++ native | 2.3MB | Custom UI component system |
| `ffmpeg.dll` | C native | 0.5MB | FFmpeg wrapper (thin — real codecs elsewhere) |
| Qt6 (19 DLLs) | C++ | ~30MB total | UI framework: Qt6 Quick/QML |
| `EffectPlatform.dll` | C++ | 1.5MB | Effect download/management |
| `jazz.dll` | C++ | 1.4MB | Unknown (audio processing?) |
| `nuro.dll` | C++ | 0.8MB | Neural rendering |
| `parfait.dll` | C++ | 1.3MB | Crash reporting (ByteDance parfait) |
| `DeepAgentsService.dll` | C++ | - | AI agent orchestration |
| `SvtAv1Enc.dll` | C | - | AV1 encoder (SVT-AV1) |
| `ByteVC1_dec.dll` | C++ | - | ByteDance proprietary codec |
| `dav1d.dll` | C | - | AV1 decoder |
| OpenVINO (`openvino*.dll`) | C++ | - | Intel AI inference |
| `speechsdk.dll` | C++ | - | Speech recognition/TTS |

### Architecture Pattern
```
CapCut.exe (launcher, 90KB)
  └─ loads videoeditor.dll (NLE engine)
       ├─ Qt6 QML UI (QtQuick)
       ├─ Lynx (ByteDance cross-platform rendering)
       ├─ FFmpeg pipeline (decode/encode)
       ├─ lens.dll (AI effects, filters, face/body)
       ├─ Audio pipeline (voice change, denoise, TTS)
       ├─ Timeline model (tracks, segments, keyframes)
       └─ Export/Render engine
```

### 195 Identified Actions (Feature Map)
Categories found in `videoeditor.dll`:
- **Timeline**: cut, split, move, trim, zoom, FPS, track management
- **Audio**: fade in/out, voice change, loudness, denoise, enhance, TTS
- **Video**: effects, transitions, speed change, smart crop, keyframes
- **Text**: text-to-video, translations, text animation, sync
- **AI**: smart crop, quality enhance, beauty face, object lock, auto-remove
- **Composition**: group, break apart, merge, sticker
- **Export**: range, HD replace, render settings
- **Templates**: apply template, replace materials

### External Dependencies (DLLs)
- FFmpeg (decode/encode pipeline)
- SVT-AV1 (AV1 encoding)
- dav1d (AV1 decoding)
- OpenVINO (Intel AI inference)
- libmp3lame (MP3 encoding)
- libraw (RAW image decode)
- JPEG XL (jxl.dll)
- lcms2 (color management)
- ICU (internationalization)
- Qt6 Quick/QML (UI)
- MMKV (key-value storage)
- OpenGL (GPU rendering)

---

## CapRawr Rebuild Architecture (Rust)

### Equivalent Rust Stack

| CapCut Component | CapRawr Rust Equivalent |
|---|---|
| Qt6 QML UI | **egui** + **eframe** (immediate mode GUI) |
| FFmpeg pipeline | **ffmpeg-next** (Rust FFmpeg bindings) |
| Timeline NLE | Custom Rust NLE engine |
| Audio pipeline | **symphonia** (decode) + **cpal** (playback) |
| GPU rendering | **wgpu** (WebGPU/Vulkan/DX12) |
| AI/ML effects | **ort** (ONNX Runtime) or **candle** |
| Video codecs | FFmpeg via ffmpeg-next |
| File I/O | **tokio** (async) + std::fs |
| Project format | **serde** + JSON |
| Image processing | **image** crate |

### Module Structure
```
caprawr/
├── src/
│   ├── main.rs              # App entry, window setup
│   ├── app.rs               # Main application state
│   ├── ui/
│   │   ├── mod.rs
│   │   ├── timeline.rs      # Timeline panel
│   │   ├── preview.rs       # Video preview
│   │   ├── properties.rs    # Properties panel
│   │   ├── media_browser.rs # Media import
│   │   └── toolbar.rs       # Top toolbar
│   ├── engine/
│   │   ├── mod.rs
│   │   ├── project.rs       # Project file model
│   │   ├── timeline.rs      # Timeline data model
│   │   ├── track.rs         # Track (video/audio/text)
│   │   ├── clip.rs          # Clip/Segment
│   │   ├── keyframe.rs      # Keyframe animation
│   │   └── effect.rs        # Effect definitions
│   ├── media/
│   │   ├── mod.rs
│   │   ├── decoder.rs       # FFmpeg decode
│   │   ├── encoder.rs       # FFmpeg encode
│   │   ├── audio.rs         # Audio playback
│   │   └── thumbnail.rs     # Thumbnail generation
│   ├── render/
│   │   ├── mod.rs
│   │   ├── pipeline.rs      # GPU render pipeline
│   │   ├── compositor.rs    # Layer compositing
│   │   └── export.rs        # Export/render to file
│   └── util/
│       ├── mod.rs
│       └── time.rs          # Timecode utilities
├── Cargo.toml
└── ARCHITECTURE.md
```
