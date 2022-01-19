use futures::executor::block_on;
use std::{
    fs::File,
    io::BufReader,
    thread,
    time::Duration
};

/// This function will spin up a thread, play the audio in the background and wait till the
/// audio has completed it's playback before jumping back to the main thread
/// 
/// Note: this currently uses rodio which spawns it's own thread, so I'll need to use a sink
/// to get the actual duration for the clip
async fn play_audio() {
    println!("Playing the audio asynchronously!");

    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let file = File::open("examples_music.wav").unwrap();
    let music = stream_handle.play_once(BufReader::new(file)).unwrap();
    music.set_volume(8.0);
    thread::sleep(Duration::from_secs(10));
}

/// Allow the user to select what audio they want to play
fn select_audio() {
    println!("Please select the audio you wish to play");
    println!("There are no options available, the default tune will play");
}

fn main() {
    println!("Welcome to this simple audio player");

    let mut app_running_state = true;
    let mut input_string = String::new();

    while app_running_state {
        select_audio();
        block_on(play_audio());
        println!("Play another audio clip?");
        std::io::stdin().read_line(&mut input_string).expect("Error reading input");
        app_running_state = input_string.contains("yes");
        input_string.clear();
    }
}
