use rodio::{source::Source, Decoder, OutputStream, Sink};
use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
    sync::mpsc::{channel, Sender},
    thread::{spawn, JoinHandle},
};

use engine::Component;

#[derive(Clone)]
pub enum Message {
    Quit,
    SetMusicVolume(f64),
    SetEffectVolume(f64),
    StopMusic,
    PlayMusic(PathBuf),
    PlayEffect(PathBuf),
}

#[derive(Component, Clone)]
pub struct SoundPlayer {
    sender: Sender<Message>,
}

impl SoundPlayer {
    pub fn new(sender: Sender<Message>) -> Self {
        Self { sender }
    }

    pub fn quit(&mut self) {
        self.sender.send(Message::Quit).unwrap()
    }

    pub fn set_music_volume(&mut self, volume: f64) {
        self.sender.send(Message::SetMusicVolume(volume)).unwrap()
    }

    pub fn set_effect_volume(&mut self, volume: f64) {
        self.sender.send(Message::SetEffectVolume(volume)).unwrap()
    }

    pub fn stop_music(&mut self) {
        self.sender.send(Message::StopMusic).unwrap()
    }

    pub fn play_music<P: AsRef<Path>>(&mut self, path: P) {
        self.sender
            .send(Message::PlayMusic(path.as_ref().to_path_buf()))
            .unwrap()
    }

    pub fn play_effect<P: AsRef<Path>>(&mut self, path: P) {
        self.sender
            .send(Message::PlayEffect(path.as_ref().to_path_buf()))
            .unwrap()
    }
}

pub fn sound_player() -> (SoundPlayer, JoinHandle<()>) {
    let (sender, receiver) = channel::<Message>();
    let join_handle = spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let music_sink = Sink::try_new(&stream_handle).unwrap();
        let effect_sink = Sink::try_new(&stream_handle).unwrap();

        loop {
            let Ok(message) = receiver.recv() else {
                break;
            };
            match message {
                Message::Quit => break,

                Message::SetMusicVolume(volume) => {
                    music_sink.set_volume(volume as f32);
                }
                Message::SetEffectVolume(volume) => {
                    music_sink.set_volume(volume as f32);
                }
                Message::StopMusic => {
                    music_sink.clear();
                }
                Message::PlayMusic(path) => {
                    let file = BufReader::new(File::open(path).unwrap());
                    let source = Decoder::new(file).unwrap();
                    music_sink.clear();
                    music_sink.play();
                    music_sink.append(source.convert_samples::<f32>().repeat_infinite());
                }
                Message::PlayEffect(path) => {
                    let file = BufReader::new(File::open(path).unwrap());
                    let source = Decoder::new(file).unwrap();
                    effect_sink.clear();
                    effect_sink.play();
                    effect_sink.append(source.convert_samples::<f32>());
                }
            }
        }
    });
    (SoundPlayer::new(sender), join_handle)
}
