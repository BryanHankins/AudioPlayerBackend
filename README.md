Welcome to the backend of the Rust-based Audio Player project!
This repository provides the core logic and audio processing for a custom web-based audio player.
https://github.com/BryanHankins/AudioPlayerFrontend/
This backend is written in Rust and compiled to WebAssembly (WASM) to enable high-performance audio playback directly in the browser.
It handles decoding and playback of audio files, interfacing seamlessly with a JavaScript frontend.

Currently Supported Audio Formats: .wav, .ogg, .flac

Limitations: .mp3 files are not supported due to WebAssembly limitations with the Rodio library.
🛠️ Technologies Used

Technology	Purpose
Rust	Backend development and audio logic
WebAssembly	Run Rust in the browser for better performance
Rodio	Rust audio playback library
wasm-bindgen	Create JS bindings for WASM modules
JavaScript/TypeScript	Connect frontend and backend

AudioPlayerBackend/
├── src/               # Rust source files
├── target/            # Build outputs
├── Audio_Player.js    # JavaScript bindings for WASM
├── Audio_Player.d.ts  # TypeScript typings
├── Audio_Player_bg.wasm # Compiled WebAssembly binary
├── Cargo.toml         # Rust project configuration
├── package.json       # NPM project configuration
└── README.md          # Project documentation

2. Build Instructions
bash
Copy
Edit
# Clone the repository
git clone https://github.com/BryanHankins/AudioPlayerBackend.git
cd AudioPlayerBackend

# Build the Rust project to WebAssembly
wasm-pack build --target web

# Install any necessary frontend dependencies
npm install

⚠️ Known Issues
MP3 Playback: Due to codec restrictions in rodio with WebAssembly, MP3 decoding is not supported.
(WAV, OGG, and FLAC formats work properly.)

Future Improvements:

Add MP3 support via a lightweight decoder (e.g., minimp3 crate).

Add frontend fallback when unsupported files are uploaded.
