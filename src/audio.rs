use std::borrow::Cow;
use std::io::Cursor;

use rodio::{self, Decoder, Device, Sink, Source};


pub fn play_sound(speaker: &Device, audio_bytes: &Cow<'static, [u8]>) {
    let cursor = Cursor::new(audio_bytes.clone());
    let decoder = Decoder::new(cursor).unwrap();
    rodio::play_raw(speaker, decoder.convert_samples());
}


pub fn play_music(speaker: &Device, audio_bytes: &Cow<'static, [u8]>) -> Sink {
    let sink = Sink::new(speaker);
    let cursor = Cursor::new(audio_bytes.clone());
    let decoder = Decoder::new(cursor).unwrap().repeat_infinite();
    sink.append(decoder);
    sink
}
