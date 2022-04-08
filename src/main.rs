use rand::seq::SliceRandom;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::{read_dir, read_to_string, File};
use std::io::BufReader;
use std::process::exit;

fn pick_song() -> String {
    // pick a random song from a vec of files, defaults to /etc/bootplay.d
    let song_path: String = match read_to_string("/etc/bootplay.conf") {
        Ok(path) => path,
        Err(_) => String::from("/etc/bootplay.d/"),
    };

    let mut music: Vec<String> = vec![];
    for file in read_dir(&song_path).unwrap() {
        match file {
            Ok(file) => {
                let path = file.path().to_string_lossy().into_owned();
                let _ = &music.push(path);
            }
            Err(e) => {
                eprintln!("{e}")
            }
        };
    }

    if music.is_empty() {
        eprintln!("Could not find any music");
        exit(1)
    }

    let song_sel: Vec<&String> = music.choose_multiple(&mut rand::thread_rng(), 1).collect();
    let song = match song_sel.get(0) {
        Some(a) => a.to_owned().to_owned(),
        None => {
            eprintln!("how did we get here");
            exit(1);
        }
    };
    return song;
}

fn play_song(path: String) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let file = BufReader::new(File::open(path).unwrap());

    let source = Decoder::new(file).unwrap();

    sink.append(source);

    sink.sleep_until_end();
}

fn main() {
    println!("bootplay starting up");
    let song = pick_song();
    println!("using {song}");
    play_song(song);
}
