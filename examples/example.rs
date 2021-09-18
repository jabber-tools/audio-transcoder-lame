use audio_transcoder_lame::convertor::{decode, ResampleInterpolation, ResampleSpec};
use std::fs::{self, File};
use std::io::prelude::*;

fn main() {
    const SAMPLE_MP3: &str = "/usr/src/audio-transcoder-lame/examples/sampledata/hello_rust_tts";
    let file_content =
        fs::read_to_string(SAMPLE_MP3).expect("Something went wrong reading the file");

    let resample_spec = ResampleSpec {
        source_hz: 24_000f64,
        target_hz: 8_000f64,
        interpolation: ResampleInterpolation::Linear,
    };

    let result_24khz = decode(&file_content, None).unwrap();
    println!("24khz: {}", result_24khz);

    let result_8khz = decode(&file_content, Some(resample_spec)).unwrap();
    println!("8khz: {}", result_8khz);

    let mut file24 =
        File::create("/usr/src/audio-transcoder-lame/examples/sampledata/hello_rust_tts_24.wav")
            .unwrap();
    file24
        .write_all(&base64::decode(result_24khz).unwrap()[..])
        .unwrap();

    let mut file8 =
        File::create("/usr/src/audio-transcoder-lame/examples/sampledata/hello_rust_tts_8.wav")
            .unwrap();
    file8
        .write_all(&base64::decode(result_8khz).unwrap()[..])
        .unwrap(); /**/
}
