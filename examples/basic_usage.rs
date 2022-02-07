use audrey::Reader;
use coqui_stt::Model;
use dasp_interpolate::linear::Linear;
use dasp_signal::interpolate::Converter;
use dasp_signal::{from_iter, Signal};
use std::env::args;
use std::fs::File;
use std::path::Path;
use std::time::Instant;

// this example was mostly borrowed from the original Rust DeepSpeech implementation
// https://github.com/RustAudio/deepspeech-rs/blob/master/examples/client_simple.rs

fn main() {
    let model_dir_str = args().nth(1).expect("Please specify model dir");
    let audio_file_path = args()
        .nth(2)
        .expect("Please specify an audio file to run STT on");
    let dir_path = Path::new(&model_dir_str);
    let mut model_name: Box<Path> = dir_path.join("output_graph.pb").into_boxed_path();
    let mut scorer_name: Option<Box<Path>> = None;
    // search for model in model directory
    for file in dir_path
        .read_dir()
        .expect("Specified model dir is not a dir")
    {
        if let Ok(f) = file {
            let file_path = f.path();
            if file_path.is_file() {
                if let Some(ext) = file_path.extension() {
                    if ext == "pb" || ext == "pbmm" || ext == "tflite" {
                        model_name = file_path.into_boxed_path();
                    } else if ext == "scorer" {
                        scorer_name = Some(file_path.into_boxed_path());
                    }
                }
            }
        }
    }

    let mut m = Model::new(model_name.to_str().expect("invalid utf-8 found in path")).unwrap();
    // enable external scorer if found in the model folder
    if let Some(scorer) = scorer_name {
        let scorer = scorer.to_str().expect("invalid utf-8 found in path");
        println!("Using external scorer `{}`", scorer);
        m.enable_external_scorer(scorer).unwrap();
    }

    let audio_file = File::open(audio_file_path).unwrap();
    let mut reader = Reader::new(audio_file).unwrap();
    let desc = reader.description();
    // input audio must be mono and usually at 16KHz, but this depends on the model
    assert_eq!(
        1,
        desc.channel_count(),
        "The channel count is required to be one, at least for now"
    );

    let src_sample_rate = desc.sample_rate();
    let dest_sample_rate = m.get_sample_rate() as u32;
    // Obtain the buffer of samples
    let audio_buf: Vec<_> = if src_sample_rate == dest_sample_rate {
        reader.samples().map(|s| s.unwrap()).collect()
    } else {
        // We need to interpolate to the target sample rate
        let interpolator = Linear::new([0i16], [0]);
        let conv = Converter::from_hz_to_hz(
            from_iter(reader.samples::<i16>().map(|s| [s.unwrap()])),
            interpolator,
            src_sample_rate as f64,
            dest_sample_rate as f64,
        );
        conv.until_exhausted().map(|v| v[0]).collect()
    };

    let st = Instant::now();

    // Run the speech to text algorithm
    let result = m.speech_to_text(&audio_buf).unwrap();

    let et = Instant::now();
    let tt = et.duration_since(st);

    // Output the result
    println!("{}", result);
    println!("took {}ns", tt.as_nanos());
}
