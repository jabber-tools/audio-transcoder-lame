///
/// https://rust-embedded.github.io/book/interoperability/c-with-rust.html
/// https://doc.rust-lang.org/1.30.0/book/first-edition/raw-pointers.html#ffi
/// https://locka99.gitbooks.io/a-guide-to-porting-c-to-rust/content/features_of_rust/types.html
///
use base64;
use dasp;
use dasp::Signal;
use std::fmt::Debug;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use std::result;

use crate::lame_ffi::{
    hip_decode1_headers, hip_decode_exit, hip_decode_init, lame_close, lame_init,
    lame_set_decode_only, mp3data_struct,
};

pub enum ResampleInterpolation {
    Linear = 1,
    Sinc = 2, // Whittaker-Shannon interpolation
}

pub struct ResampleSpec {
    pub source_hz: f64,
    pub target_hz: f64,
    pub interpolation: ResampleInterpolation,
}

/// Specifies properties of the audio data.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WavSpec {
    /// The number of channels.
    pub channels: u16,

    /// The number of samples per second.
    ///
    /// A common value is 44100, this is 44.1 kHz which is used for CD audio.
    pub sample_rate: u32,

    /// The number of bits per sample.
    ///
    /// A common value is 16 bits per sample, which is used for CD audio.
    pub bits_per_sample: u16,

    /// Whether the wav's samples are float or integer values.
    pub sample_format: SampleFormat,
}

/// Specifies whether a sample is stored as an "IEEE Float" or an integer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SampleFormat {
    /// Wave files with the `WAVE_FORMAT_IEEE_FLOAT` format tag store samples as floating point
    /// values.
    ///
    /// Values are normally in the range [-1.0, 1.0].
    Float,
    /// Wave files with the `WAVE_FORMAT_PCM` format tag store samples as integer values.
    Int,
}

const PCM_SIZE: usize = 4096;
const MP3_SIZE: usize = 4096;

#[derive(Debug)]
pub struct Error {
    pub message: String,
}

impl Error {
    pub fn new(message: &str) -> Self {
        Error {
            message: message.into(),
        }
    }

    // TBD: wrap this in closure so that we can provide caller name conveniently
    // then replace 'Error occurred' with 'Error occurred in xxx'
    pub fn to_error<D>(error: D) -> Self
    where
        D: Debug,
    {
        Error::new(&format!("Error occurred: {:#?}", error))
    }
}

pub type Result<T> = result::Result<T, Error>;

// PCM/WAV buffer. Here we construct wav content from MP3
// Cursor is used instead of file. It implements Read, Write and Seek
// which is all we need.
pub type PcmBuffer = Cursor<Vec<u8>>;

fn write_le_u16(pcm: &mut PcmBuffer, x: u16) -> std::io::Result<()> {
    let mut buf = [0u8; 2];
    buf[0] = (x & 0xff) as u8;
    buf[1] = (x >> 8) as u8;
    pcm.write_all(&buf)
}

fn write_le_u32(pcm: &mut PcmBuffer, x: u32) -> std::io::Result<()> {
    let mut buf = [0u8; 4];
    buf[0] = ((x >> 00) & 0xff) as u8;
    buf[1] = ((x >> 08) & 0xff) as u8;
    buf[2] = ((x >> 16) & 0xff) as u8;
    buf[3] = ((x >> 24) & 0xff) as u8;
    pcm.write_all(&buf)
}

/// sample rate convertor using linear interpolation. see details here:
/// https://alpha-ii.com/Info/AudioInt.html#:~:text=Audio%20interpolation%20is%20a%20method,better%20than%20it%20really%20is.&text=As%20a%20sound%20wave%20enters,but%20also%20use%20more%20memory.
/// should support stereo (interleaved data) as well but this was not tested (our use case works with mono only)!
pub fn resample_linear_interpolation(
    samples: Vec<u8>,
    source_hz: f64,
    target_hz: f64,
    channel_count: u32,
) -> Vec<u8> {
    let interpolator = dasp::interpolate::linear::Linear::new(samples[0], samples[1]);

    let resampled = match channel_count {
        1u32 => {
            let source_signal = dasp::signal::from_iter(samples.into_iter());
            let converter = dasp::signal::interpolate::Converter::from_hz_to_hz(
                source_signal,
                interpolator,
                source_hz,
                target_hz,
            );
            converter.until_exhausted().collect()
        }
        _ => {
            // INTERLEAVED SAMPLES WERE NOT TESTED!!!
            let source_signal = dasp::signal::from_interleaved_samples_iter(samples.into_iter());
            let converter = dasp::signal::interpolate::Converter::from_hz_to_hz(
                source_signal,
                interpolator,
                source_hz,
                target_hz,
            );
            converter.until_exhausted().collect()
        }
    };

    resampled
}

/// see http://www.topherlee.com/software/pcm-tut-wavformat.html
///
/// Positions   Sample Value         Description
/// 1 - 4       "RIFF"               Marks the file as a riff file. Characters are each 1. byte long.
/// 5 - 8       File size (integer)  Size of the overall file - 8 bytes, in bytes (32-bit integer). Typically, you'd fill this in after creation.
/// 9 -12       "WAVE"               File Type Header. For our purposes, it always equals "WAVE".
/// 13-16       "fmt "               Format chunk marker. Includes trailing null
/// 17-20       16                   Length of format data as listed above
/// 21-22       1                    Type of format (1 is PCM) - 2 byte integer
/// 23-24       2                    Number of Channels - 2 byte integer
/// 25-28       44100                Sample Rate - 32 bit integer. Common values are 44100 (CD), 48000 (DAT). Sample Rate = Number of Samples per second, or Hertz.
/// 29-32       176400               (Sample Rate * BitsPerSample * Channels) / 8.
/// 33-34       4                    (BitsPerSample * Channels) / 8.1 - 8 bit mono2 - 8 bit stereo/16 bit mono4 - 16 bit stereo
/// 35-36       16                   Bits per sample
/// 37-40       "data"               "data" chunk header. Marks the beginning of the data section.
/// 41-44       File size (data)     Size of the data section, i.e. file size - 44 bytes header.
///
/// The header integers are all in Least significant byte order, so the two byte channel information 0x01 0x00 are actually 0x00001 e.g. mono.
/// See also https://stackoverflow.com/questions/28137559/can-someone-explain-wavwave-file-headers
///
fn write_wav_header(pcm_buffer: &mut PcmBuffer, spec: &WavSpec) -> Result<()> {
    pcm_buffer
        .write_all("RIFF".as_bytes())
        .map_err(Error::to_error)?;

    // Size of the overall file - 8 bytes [ in bytes ]
    // Skip 4 bytes that will be filled with the file size afterwards.
    write_le_u32(pcm_buffer, 0).map_err(Error::to_error)?;

    pcm_buffer
        .write_all("WAVE".as_bytes())
        .map_err(Error::to_error)?;
    pcm_buffer
        .write_all("fmt ".as_bytes())
        .map_err(Error::to_error)?;

    // initially let's use PCMWAVEFORMAT, not WAVEFORMATEXTENSIBLE
    // length of PCM format declaration area, for PCMWAVEFORMAT it is 16
    write_le_u32(pcm_buffer, 16).map_err(Error::to_error)?;

    // is PCM? WAVE_FORMAT_PCM => 1, WAVE_FORMAT_IEEE_FLOAT -> 3
    // WAVE_FORMAT_IEEE_FLOAT allows 32 bits per sample
    write_le_u16(pcm_buffer, 1).map_err(Error::to_error)?;

    // Number of Channels
    write_le_u16(pcm_buffer, spec.channels).map_err(Error::to_error)?;

    // sample frequency/rate
    write_le_u32(pcm_buffer, spec.sample_rate).map_err(Error::to_error)?;

    // average bytes per second
    let bytes_per_sample: u16 = (spec.bits_per_sample + 7) / 8;
    let bytes_per_sec = spec.sample_rate * bytes_per_sample as u32 * spec.channels as u32;
    write_le_u32(pcm_buffer, bytes_per_sec).map_err(Error::to_error)?;

    // BlockAlign, i.e. bytes per sample time (channels * (spec.bits_per_sample + 7) / 8)
    write_le_u16(pcm_buffer, spec.channels * bytes_per_sample).map_err(Error::to_error)?;

    // bits per sample
    write_le_u16(pcm_buffer, spec.bits_per_sample).map_err(Error::to_error)?;

    // Finally the header of the "data" chunk.
    pcm_buffer
        .write_all("data".as_bytes())
        .map_err(Error::to_error)?;

    // length in bytes of raw PCM data
    // The number of bytes
    // that this will take is not known at this point. The 0 will
    // be overwritten later.
    write_le_u32(pcm_buffer, 0).map_err(Error::to_error)?;
    Ok(())
}

/// updates header (file size + data chunk size). data_bytes_written is size of data chunk (all samples)
/// data_bytes_written is an u32 because WAVE cannot accommodate more data.
fn update_wav_header_size(pcm_buffer: &mut PcmBuffer, data_bytes_written: u32) -> Result<()> {
    // WaveFormatExtensible: 64, PcmWaveFormat: 40
    let data_len_offset = 40; // using PcmWaveFormat

    // Size of the overall file - 8 bytes, in bytes (32-bit integer)
    // The data chunk length (4 bytes) is the last part of the header.
    let header_size = data_len_offset + 4 - 8;
    let file_size = data_bytes_written + header_size;

    pcm_buffer
        .seek(SeekFrom::Start(4))
        .map_err(Error::to_error)?;
    write_le_u32(pcm_buffer, file_size).map_err(Error::to_error)?;
    pcm_buffer
        .seek(SeekFrom::Start(data_len_offset as u64))
        .map_err(Error::to_error)?;
    write_le_u32(pcm_buffer, data_bytes_written).map_err(Error::to_error)?;

    Ok(())
}

fn update_wav_header_sample_rate(pcm_buffer: &mut PcmBuffer, sample_rate: u32) -> Result<()> {
    pcm_buffer
        .seek(SeekFrom::Start(24))
        .map_err(Error::to_error)?;
    write_le_u32(pcm_buffer, sample_rate).map_err(Error::to_error)?;
    Ok(())
}

fn calculate_wav_size(stereo: u32, sample_count: u32) -> u32 {
    let i = (16 / 8) * stereo;
    let wav_size: u32;
    if sample_count <= 0 {
        wav_size = 0;
    } else if sample_count > 0xFFFFFFD0 / i {
        wav_size = 0xFFFFFFD0;
    } else {
        wav_size = sample_count * i;
    }
    wav_size
}

/// decodes MP3 provided as base64 string to wav format.
/// Wav is then encoded as base 64 string as well.
/// If resample spec is provided wave data chunk will be resampled accordingly
pub fn decode(mp3_str: &str, resample_spec: Option<ResampleSpec>) -> Result<String> {
    let mut pcm_l: [i16; PCM_SIZE] = [0; PCM_SIZE];
    let mut pcm_r: [i16; PCM_SIZE] = [0; PCM_SIZE];

    let mut mp3_buffer: [u8; MP3_SIZE] = [0; MP3_SIZE];

    let mut pcm_buffer_header: PcmBuffer = Cursor::new(Vec::new());

    // data chunk of wav. we are initially buffering data here so that we
    // can potentially resample. At the end we append pcm_buffer_data to pcm_buffer_header
    let mut pcm_buffer_data: PcmBuffer = Cursor::new(Vec::new());

    // total sample count, used to calculate wav size
    let mut sample_count: u32 = 0;

    let mut mp3_is_stereo: u32 = 1; // 1 -mono, 2 - stereo. assuming mono by default

    unsafe {
        //println!("lame_init");
        let lame = lame_init();

        //println!("lame_set_decode_only");
        lame_set_decode_only(lame, 1);

        //println!("hip_decode_init");
        let hip = hip_decode_init();

        let mut mp3data: mp3data_struct = mp3data_struct::default();

        //println!("base64::decode");
        let b_vec = base64::decode(mp3_str).map_err(Error::to_error)?;
        let mut b = &b_vec[..];

        let mut n_channels: i32 = -1;

        //println!("processing mp3 bytes");
        while let Ok(bytes_read) = b.read(&mut mp3_buffer) {
            //println!("bytes_read {:#?}", bytes_read);
            if bytes_read == 0 {
                //println!("bytes_read = 0 -> break");
                break;
            }

            let mut mp3_len = bytes_read;
            loop
            /* loop until samples > 0 */
            {
                //println!("hip_decode1_headers");
                // https://doc.rust-lang.org/1.30.0/book/first-edition/ffi.html
                let samples = hip_decode1_headers(
                    hip,
                    mp3_buffer.as_mut_ptr(),
                    mp3_len,
                    pcm_l.as_mut_ptr(),
                    pcm_r.as_mut_ptr(),
                    &mut mp3data as *mut mp3data_struct,
                );
                //println!("hip_decode1_headers samples#: {}", samples);
                sample_count = sample_count + samples as u32;

                if mp3data.header_parsed == 1
                /* header is parsed */
                {
                    if n_channels < 0
                    /* reading for the first time */
                    {
                        let spec = WavSpec {
                            channels: mp3data.stereo as u16,
                            sample_rate: mp3data.samplerate as u32,
                            bits_per_sample: 16, // hardcoded to 16 in original algorithm
                            sample_format: SampleFormat::Int,
                        };
                        //println!("write_wav_header");
                        write_wav_header(&mut pcm_buffer_header, &spec)?;
                    }

                    n_channels = mp3data.stereo;
                    mp3_is_stereo = mp3data.stereo as u32;
                }

                if samples > 0 && mp3data.header_parsed != 1 {
                    // return Err(Error::new("WARNING: lame decode error occurred!"))
                    //println!("WARNING: lame decode error occurred!");
                }

                if samples > 0 {
                    //println!("writing samples");
                    for i in 0..samples {
                        write_le_u16(&mut pcm_buffer_data, pcm_l[i as usize] as u16)
                            .map_err(Error::to_error)?;
                        if mp3data.stereo == 2 {
                            write_le_u16(&mut pcm_buffer_data, pcm_r[i as usize] as u16)
                                .map_err(Error::to_error)?;
                        }
                    }
                }
                mp3_len = 0;

                if samples <= 0 {
                    break;
                }
                //println!("reading more samples");
            }
        }

        //println!("hip_decode_exit");
        hip_decode_exit(hip);

        //println!("lame_close");
        lame_close(lame);
    }

    // vector of bytes representing wav header
    let mut header_vec: Vec<u8>;

    if let Some(sample_rate_conv_spec) = resample_spec {
        // resampling (quick & dirty for now)
        //println!("resampling");
        let data_vec = pcm_buffer_data.into_inner();

        // for now only linear interpolation is supported, Sinc will silently fallback to linear as well.
        let mut resampled_data_vec = match sample_rate_conv_spec.interpolation {
            ResampleInterpolation::Linear | ResampleInterpolation::Sinc => {
                resample_linear_interpolation(
                    data_vec,
                    sample_rate_conv_spec.source_hz,
                    sample_rate_conv_spec.target_hz,
                    mp3_is_stereo,
                )
            }
        };

        let wav_size_resampled = calculate_wav_size(mp3_is_stereo, resampled_data_vec.len() as u32);

        //println!("write_wav_header");
        update_wav_header_sample_rate(
            &mut pcm_buffer_header,
            sample_rate_conv_spec.target_hz as u32,
        )
        .map_err(Error::to_error)?;
        update_wav_header_size(&mut pcm_buffer_header, wav_size_resampled)
            .map_err(Error::to_error)?;

        header_vec = pcm_buffer_header.into_inner();
        header_vec.append(&mut resampled_data_vec);
    } else
    /* no resampling */
    {
        //println!("calculating size");
        let wav_size = calculate_wav_size(mp3_is_stereo, sample_count);

        //println!("update_wav_header");
        update_wav_header_size(&mut pcm_buffer_header, wav_size).map_err(Error::to_error)?;

        //println!("no resampling");
        header_vec = pcm_buffer_header.into_inner();
        let mut data_vec = pcm_buffer_data.into_inner();
        header_vec.append(&mut data_vec);
    }

    //println!("base64::encode");
    Ok(base64::encode(header_vec))
}
