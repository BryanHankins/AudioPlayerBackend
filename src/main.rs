mod shared;

use rodio::{Decoder, OutputStream, Sink, Source};
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
use std::collections::VecDeque;


fn main() {
    // Use shared functionality
    let processed_data = shared::process_data_with_vecdeque("Example input for shared functionality.");
    for data in processed_data {
        println!("Processed from shared module: {}", data);

    }

    // Set up the output stream to the default sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    // Create a sink attached to the stream handle for complex audio control
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Load and play an audio file through the sink
    let file_path = "examples/music.ogg"; // Ensure this path is correct for your project setup
    let file = BufReader::new(File::open(file_path).expect("Failed to open audio file"));
    let source = Decoder::new(file).expect("Failed to decode audio file");
    sink.append(source);

    // Also, create a sine wave tone and play it through the same sink
    let tone = rodio::source::SineWave::new(440).take_duration(Duration::from_secs_f32(0.25)).amplify(0.20);
    sink.append(tone);

    // This blocks the current thread until the sounds finish playing
    sink.sleep_until_end();
}


// So what I want to do  The main file is the part of the program where the audio is being proccess , Sink is the single music song that plays , Web assembly is the place where I comebine my front end and backend
// Sink in my own words , like in a video game , Background music , sound effects and dialogue are differnt sinks , But for explain a single song is just one sink .