use futures::executor::block_on;

/// Audio clips contain a path, the name of the clip, and the duration
struct AudioClip {
    name: String,
    duration: u32,
    bytes: [u8],
}

/// This function will spin up a thread, play the audio in the background and wait till the
/// audio has completed it's playback before jumping back to the main thread
async fn play_audio() {
    println!("I'm running in async!");

    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let file = std::fs::File::open("examples_music.wav").unwrap();
    let music = stream_handle.play_once(std::io::BufReader::new(file)).unwrap();
    music.set_volume(8.0);
    std::thread::sleep(std::time::Duration::from_secs(10));
}

/// Allow the user to select what audio they want to play
fn select_audio() {
    println!("Please select the audio you wish to play");
}

fn main() {
    println!("Welcome to the audio project!");

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
