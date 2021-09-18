# Virtual Assistant Voice Gateway - Audio Transcoder library

---
Virtual Assistant Voice Gateway - Audio Transcoder library based on [LAME](https://en.wikipedia.org/wiki/LAME) API. In-memory single-purpose MP3 to WAV converter.

For WAV sample rate conversion (22 Khz -> 8 Khz) we are using [Dasp](https://crates.io/crates/dasp) library

## Basic approach
This app is based on [lame-sys](https://crates.io/crates/lame-sys) crate which provides necessary FFI bindings. It also includes built-in compilable Lame version.

To glue it all together following C++ [sample](https://github.com/zyfu0000/lameHelper) has been used.

Minor change had to be done to include decoding capability (see [here](https://github.com/gypified/libmp3lame/blob/master/INSTALL), directive #HAVE_MPGLIB)

Following enhancements to original lame-sys build script were done to compile decoding
```
...
            .file("lame-3.99.5/mpglib/common.c")
            .file("lame-3.99.5/mpglib/dct64_i386.c")
            .file("lame-3.99.5/mpglib/decode_i386.c")
            .file("lame-3.99.5/mpglib/interface.c")
            .file("lame-3.99.5/mpglib/layer1.c")
            .file("lame-3.99.5/mpglib/layer2.c")
            .file("lame-3.99.5/mpglib/layer3.c")
            .file("lame-3.99.5/mpglib/tabinit.c")
...
            .include("lame-3.99.5/mpglib")
...
            .define("HAVE_MPGLIB", Some("1"));
...
```
## Docker
```
docker build -t audio-transcoder-lame .
docker run -it --rm --name audio-transcoder-lame-app audio-transcoder-lame
docker cp mp3-to-wav-app:/usr/src/audio-transcoder-lame/examples/sampledata/hello_rust_tts_24.wav /tmp/hello_rust_tts_24.wav
docker cp mp3-to-wav-app:/usr/src/audio-transcoder-lame/examples/sampledata/hello_rust_tts_8.wav /tmp/hello_rust_tts_8.wav
```

## TO BE DONE
- [x] Initial implementation of conversion and resampling:
- [ ] Make the app to work on windows. Getting error libmp3_to_wav-eeb18f30da307548.rlib(quantize.o) : error LNK2019: unresolved external symbol init_xrpow_core_sse referenced in function init_xrpow_core_init
- [ ] Implement Sinc interpolation or remove respective Dasp features