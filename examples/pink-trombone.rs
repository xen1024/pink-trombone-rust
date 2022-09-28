use std::sync::{Arc, Mutex};
use tracing::{debug, error, info, trace, warn};

use pink_trombone::{ThreadRng, PinkTrombone, Glottis, Tract, TractShaper};
use rand::Rng;
use rodio::{OutputStream, Source};

use serde::{Deserialize, Serialize};
use serde_big_array::big_array;
use schemars::{schema_for, JsonSchema};

big_array! { BigArray; N }

//#[derive(Clone)]
struct PinkTromboneSource {
//    trombone: Arc<Mutex<PinkTrombone>>,
    trombone: PinkTrombone,
    buffer_pos: usize,
    buffer: [f64; 512],
}

impl PinkTromboneSource {
    pub fn new(trombone: PinkTrombone) -> PinkTromboneSource {
        let buffer = [0_f64; 512];
        PinkTromboneSource {
//            trombone: Arc::new(Mutex::new(trombone)),
            trombone: trombone,
            buffer_pos: buffer.len(),
            buffer,
        }
    }
}

impl Iterator for PinkTromboneSource {
    type Item = f32;
    fn next(&mut self) -> Option<f32> {
        if self.buffer_pos == self.buffer.len() {
//            self.trombone.lock().unwrap().synthesize(&mut self.buffer);
            self.trombone.synthesize(&mut self.buffer);
            self.buffer_pos = 0;
        }
        let result = self.buffer[self.buffer_pos];
        assert!(result.abs() <= 1.0);
        self.buffer_pos += 1;
        Some(result as f32)
    }
}

impl Source for PinkTromboneSource {
    fn current_frame_len(&self) -> Option<usize> {
        Some(512)
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
//        self.trombone.lock().unwrap().sample_rate()
        self.trombone.sample_rate()
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}

const SAMPLE_RATE: u32 = 48000;

// io test data [


#[derive(JsonSchema, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InputOutputBuffer {
    parameter_buf: Vec<f64>,
    mutated_buf: Vec<f64>,
}

use std::sync::MutexGuard;

fn generate_trombone(tones: u64, rounds: u64, round_sample_count: u64, filename: &str) {

    println!("generate {tones:?} {rounds:?} {round_sample_count:?} - {filename:?}");

    let seed = rand::thread_rng().gen();
    let mut rng = ThreadRng::new();

    let mut trombone = PinkTrombone::new(SAMPLE_RATE, &mut rng, seed);

    trombone.shaper.tract.glottis.random = Some(rng);

//    std::fs::write(output_path_trombone0,serde_json::to_string_pretty(&trombone).unwrap()).unwrap();

    let mut source = PinkTromboneSource::new(trombone);

    let mut samples: Vec<f64> = Vec::new();

    for tone in 0..tones {

//        {
//            let mut src = source.trombone.lock().unwrap();
//            src.set_musical_note(tone as f64);
//        }

        source.trombone.set_musical_note(tone as f64);

        for round in 0..rounds {
            for i in 0..round_sample_count {
                let sample = source.next();
                samples.push(sample.unwrap() as f64);

//                println!("{tone:?} {round:?} {i:?} {sample:?}")
            }
        }
    }
    
//    let mut trombone = source.trombone;
//    let mut src = source.trombone.lock().unwrap();

    source.trombone.samples = Some(samples);

    std::fs::write(filename,serde_json::to_string_pretty(&source.trombone).unwrap()).unwrap();
}

fn generate_test_data() {

    let output_path = "test_data.json";
    let output_path_glottis0 = "glottis0.json";
    let output_path_glottis1 = "glottis1.json";
    
    let output_path_trombone0 = "trombone0.json";
    let output_path_trombone0_1 = "trombone0_1.json";
    let output_path_trombone0_2 = "trombone0_2.json";
    let output_path_trombone0_3 = "trombone0_3.json";
    let output_path_trombone0_4 = "trombone0_4.json";
    let output_path_trombone0_5 = "trombone0_5.json";
    let output_path_trombone0_6 = "trombone0_6.json";
    let output_path_trombone0_7 = "trombone0_7.json";
    let output_path_trombone0_big = "trombone0_big.json";


    let output_path_tract_shape0 = "tract_shape0.json";
    let output_path_tract0 = "tract0.json";


    // GLOTTIS

    let seed = rand::thread_rng().gen();
    let mut rng = ThreadRng::new();
    let sample_rate = SAMPLE_RATE;
    let mut glottis = Glottis::new(sample_rate, &mut rng, seed);

    glottis.random = Some(rng);

    std::fs::write(output_path_glottis1,serde_json::to_string_pretty(&glottis).unwrap(),).unwrap();

    // TROMBONE

    generate_trombone(0, 0, 0, output_path_trombone0);

    generate_trombone(1, 1, 0, output_path_trombone0_2);

    generate_trombone(1, 1, 1, output_path_trombone0_3);

    generate_trombone(1, 1, 32, output_path_trombone0_4);

    generate_trombone(1, 1, 512, output_path_trombone0_5);

    generate_trombone(2, 1, 512, output_path_trombone0_6);

    generate_trombone(2, 8, 512, output_path_trombone0_7);

    generate_trombone(24, 8, 512, output_path_trombone0_1);

//    generate_trombone(24, 128, 512, output_path_trombone0_big);
    generate_trombone(24, 64, 512, output_path_trombone0_big);

    /*
    // TROMBONE
    let mut rng = ThreadRng::new();

    let mut trombone = PinkTrombone::new(SAMPLE_RATE, &mut rng, seed);

    trombone.shaper.tract.glottis.random = Some(rng);

    std::fs::write(output_path_trombone0,serde_json::to_string_pretty(&trombone).unwrap()).unwrap();

    let mut source = PinkTromboneSource::new(trombone);

    let mut samples: Vec<f64> = Vec::new();

    for tone in 0..24 {

//        {
//            let mut src = source.trombone.lock().unwrap();
//            src.set_musical_note(tone as f64);
//        }

        source.trombone.set_musical_note(tone as f64);

        for round in 0..8 {
            for i in 0..512 {
                let sample = source.next();
                samples.push(sample.unwrap() as f64);

                println!("{tone:?} {round:?} {i:?} {sample:?}")
            }
        }
    }
    
//    let mut trombone = source.trombone;
//    let mut src = source.trombone.lock().unwrap();

    source.trombone.samples = Some(samples);

    std::fs::write(output_path_trombone0_1,serde_json::to_string_pretty(&source.trombone).unwrap()).unwrap();
*/
/*
    std::fs::write(output_path_tract_shape0,serde_json::to_string_pretty(&trombone.shaper).unwrap()).unwrap();
    std::fs::write(output_path_tract0,serde_json::to_string_pretty(&trombone.shaper.tract).unwrap()).unwrap();
    std::fs::write(output_path_glottis0,serde_json::to_string_pretty(&trombone.shaper.tract.glottis).unwrap()).unwrap();

    let iodata = InputOutputBuffer {
        parameter_buf: vec![1.0],
        mutated_buf: vec![1.0],
    };

    std::fs::write(output_path, serde_json::to_string_pretty(&iodata).unwrap(),).unwrap();

*/

    trace!("DONE")
}

//#[cfg(feature = "jsonse")]
fn generate_schemas() {

//    let schema = schema_for!(InputOutputBuffer);
//    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
//    std::fs::write(output_path_glottis_schema0,serde_json::to_string_pretty(&schema).unwrap()).unwrap();

    if cfg!(feature = "jsonse") {
/*
        let output_path_glottis_schema0 = "glottis0.schema.json";

        let schema = schema_for!(Glottis);
        std::fs::write(output_path_glottis_schema0,serde_json::to_string_pretty(&schema).unwrap()).unwrap();

        let output_path_trombone_schema0 = "trombone.schema.json";
        let output_path_tract_schema0 = "tract.schema.json";
        let output_path_tractshaper_schema0 = "tract-shaper.schema.json";

        let schema = schema_for!(PinkTrombone);
        std::fs::write(output_path_trombone_schema0,serde_json::to_string_pretty(&schema).unwrap()).unwrap();

        let schema = schema_for!(Tract);
        std::fs::write(output_path_tract_schema0,serde_json::to_string_pretty(&schema).unwrap()).unwrap();

        let schema = schema_for!(TractShaper);
        std::fs::write(output_path_tractshaper_schema0,serde_json::to_string_pretty(&schema).unwrap()).unwrap();
*/
    }

    trace!("DONE")
}

// io test data ]

fn main() {

    generate_schemas();
    generate_test_data();

    let mut random = ThreadRng::new();
    let seed = rand::thread_rng().gen();
    let trombone = PinkTrombone::new(SAMPLE_RATE, &mut random, seed);
    let source = PinkTromboneSource::new(trombone);
/*
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    stream_handle.play_raw(source.clone()).unwrap();
*/
/*
    for tone in 0..24 {
        {
            let mut src = source.trombone.lock().unwrap();
            src.set_musical_note(tone as f64);
        }
        std::thread::sleep(std::time::Duration::from_millis(300));
    }

    for tone in (0..23).rev() {
        {
            let mut src = source.trombone.lock().unwrap();
            src.set_musical_note(tone as f64);
        }
        std::thread::sleep(std::time::Duration::from_millis(300));
    }
    */
}
