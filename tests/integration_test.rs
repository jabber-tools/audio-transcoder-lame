use audio_transcoder_lame::convertor::{decode, ResampleInterpolation, ResampleSpec};

#[test]
fn decode_test() {
    // keeping the data here like this is not very meaningful but the purpose of the test is only to prove the lib is working after every commit (as CI pipeline will run it )
    let mp3_content = "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McgcA7CHlci///////NExH0h+xJcAZKgAflc0J8oFQih5NGsUdFLACJGZ7iPfMvPXg6rR8u1m2UtC3NF1lsXUTQId4IFb3snNqc81C7dxp+U1zk6lNEwqSsHmZAusRcYsmUVgPykU0SEvhLq//NExEggSm6gAYVgALb98/QicW1hk+pJ4nHkJFQ16pZhh70Ts20cDQGLpZQqX68VI4jc7PC7/PVcCEHyq/Inyc9cVbOMCtUuZjxwAxLodQ8+KpOVnll67uBWEoQB1Y8S//NExBkWKiqsAYFAAKsN64IeccLh2Mv3CURUTm3JZVQWOFySWtYelO1PaBdoLFqH/ZLUXlm8jxg8eUxfKW7QdYoAwbOFkMFNiv3oF1K8lDDgozZAAGAqFQZmpiXJHrOK//NExBMVcfqUAYNYAOcYdSduSatJkiaKkghHNoqlEdpSqU3klJJRxOPJGzocbDue1PQoNUnXqR7f8mmrjaeah0KHod/x3H/paR6RaiWp/6YuoR0DKBlCQi4nTATyte1///NExBAWETIIAc9IAPV7Fy9e11/a1ImtVZgRCqBETahQwCwAiVYVCpFMhAKAE0QgiSwRNKoZRQoULNIkWIhUiAoKnRKCoKhoqGvlgaGHpY8Ig7/LB2oAwCgMCoHA+CgD//NExAoOoGkIAEpMKYKB4PlipwuIBxAEWUWUeZZC+vpqqqqrppppqqqq////pppoqqqqpp//////qqrppppMQU1FMy45OS41qqqqqqqqqqqqqqqqqqqqqqqqokwbYdcc//NExCIAAANIAAAAAL+JuS9yT5pmmaZ1qMVisVhgEAQDBJk9QCsn3znNGjQCgKAgGCRAQAgAMAYGw2FwTAGBsNiRj1lz//gxkECAkFbe+aNHPb2oeoZ//lzXRo0DEChw//NExHUAAANIAAAAAB8EIgBD+Xydb4YqwTErA1SWerogwmxomidIZQsqoSFEia24pIhVHNQoYoUKFZEia9SRIkOLNXlIiIKilChQoYxVksi2kSKQUBoSgIcEn1gFIlBU//NExKwAAANIAAAAAGAqCpUFcRf8sr9FAFwfY9BLzgQZ4nUfq5YG8EJjBxEUVcWgkRBhIGQThakiIoCPi82aNeLyppymvKmpOLLg0eqRSEhUVFSOxISZSAhcVMmQkKmT//NExP8Y0ZoIAHpSlCCwuGTMWFTP6v8VboFP60xBTUUzLjk5LjVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVTEFNRTMu//NExO4UqR4YAHpScDk5LjVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVV//NExO4VwQWoAHmMcFVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVV//NExKwAAANIAAAAAFVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVV";
    let wav_expected = "UklGRia0AABXQVZFZm10IBAAAAABAAEAQB8AAIC7AAACABAAZGF0YQK0AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA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AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAEAAAAAAAAAAAAAAAAAAAD/AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAgAIABQACQAAAAgACwD+//7//P/9//j/+f/8//r/+P/8//z//P/9//7//wADAAMAAgD9//7/AP/8//v//v//AP7/+f/6//3/+v/7//z//P/8//z//P/9/wAAAAABAAIAAgACAAQABAAFAAQABAAGAAMABQAEAAQAAwAEAAMABAADAAIAAwADAAEAAwACAAIAAgABAAEAAQACAAAAAAAAAAAA/////wD//////////wD////////+//7//////////////////////////////////////////wABAAAAAAAAAP///wABAAAAAAAAAAAAAAAAAAAAAAABAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAP//AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";

    let resample_spec = ResampleSpec {
        source_hz: 24_000f64,
        target_hz: 8_000f64,
        interpolation: ResampleInterpolation::Linear,
    };

    let result_8khz = decode(&mp3_content, Some(resample_spec)).unwrap();
    assert_eq!(result_8khz, wav_expected);
}
