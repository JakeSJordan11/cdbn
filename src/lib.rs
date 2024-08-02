use std::ffi::OsStr;
use std::io::Write;
use std::path::Path;
use std::thread;
use std::time::Duration;
use std::{env, fs};

use rand::Rng;
use walkdir::WalkDir;

pub fn startup_animation() {
    // define the frames for the spinning CD-ROM animation
    let frames = [
        "\x1B[31m\u{25CF}\x1B[0m", // Red filled circle character
        "\x1B[32m\u{25D0}\x1B[0m", // Green circle with left half black
        "\x1B[33m\u{25D1}\x1B[0m", // Yellow circle with right half black
        "\x1B[34m\u{25D2}\x1B[0m", // Blue circle with lower half black
        "\x1B[35m\u{25D3}\x1B[0m", // Magenta circle with upper half black
        "\x1B[36m\u{25D4}\x1B[0m", // Cyan circle with upper half black
        "\x1B[37m\u{25D5}\x1B[0m", // White circle with upper half black
    ];

    // loop through the frames and print each frame with a random delay
    let duration = Duration::from_secs_f64(rand::thread_rng().gen_range(0.5..1.0));

    for frame in frames
        .iter()
        .cycle()
        .take(duration.as_millis() as usize / 100)
    {
        print!("\r\x1B[33m{}\x1B[0m", frame);
        std::io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(100));
    }
}

pub fn run() {
    startup_animation();

    //print you looking to burn a cd kid?
    println!("Ya looking to burn a CD kid?");

    let mut burn_cd = String::new();
    std::io::stdin().read_line(&mut burn_cd).unwrap();
    let burn_cd = burn_cd.trim();

    if burn_cd == "n" || burn_cd == "no" || burn_cd == "nah" {
        println!("den get outta here!");
        return;
    }

    if burn_cd == "y"
        || burn_cd == "yes"
        || burn_cd == "yea"
        || burn_cd == "sure"
        || burn_cd.is_empty()
    {
        // make a default audio_library directory at ~/Music/Abelton/Project Library
        let default_audio_library = format!(
            "{}/Music/Ableton/Project Library",
            env::var("HOME").unwrap()
        );

        // ask the user to select a user library location
        println!("where's ya stash? (default: {})", default_audio_library);

        let mut audio_library = String::new();
        std::io::stdin().read_line(&mut audio_library).unwrap();
        let mut audio_library = audio_library.trim();
        // if the user does not provide an audio_library directory, use the default
        if audio_library.is_empty() {
            audio_library = &default_audio_library;
        }

        let default_audio_collection = format!(
            "{}/Music/Music Library/Jake Jordan",
            env::var("HOME").unwrap()
        );

        // ask the user to select an audio_collection location
        println!(
            "where'dya want your collection? (default: {})",
            default_audio_collection
        );

        let mut audio_collection = String::new();
        std::io::stdin().read_line(&mut audio_collection).unwrap();
        let mut audio_collection = audio_collection.trim();
        // if the user does not provide an audio_collection directory, use the default
        if audio_collection.is_empty() {
            audio_collection = &default_audio_collection;
        }

        // Get the list of directories named "Renders" in the audio_library directory recursively
        let renders_dirs: Vec<_> = WalkDir::new(audio_library)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_dir() && e.file_name() == "Renders")
            .map(|e| e.path().to_owned())
            .collect();

        // Print the paths of the directories named "Renders"
        for dir in renders_dirs {
            // Get the list of files in the audio_library directory
            // Copy each file from audio_library to audio_collection directory
            (match fs::read_dir(dir) {
                Ok(files) => files,
                Err(err) => {
                    println!("Failed to read audio_library directory: {}", err);
                    return;
                }
            })
            .for_each(|file| {
                if let Ok(file) = file {
                    let audio_library_path = file.path();
                    let file_name = audio_library_path.file_name().unwrap();
                    let audio_collection_path = Path::new(audio_collection).join(file_name);
                    if audio_library_path.extension().and_then(OsStr::to_str) == Some("mp3") {
                        fs::copy(&audio_library_path, &audio_collection_path).unwrap();
                    }
                }
            });
        }
        println!("stash'd em kid")
    }
}
