use rand::prelude::*;

use rand_chacha::ChaCha12Rng;
use serde::{Serialize, Deserialize};
use uuid::Uuid;


#[derive(Serialize, Deserialize, Debug)]
pub struct WobbleOptions {
    pub seed: u64,
    pub repl_char_chance: f32,
    pub repl_char: char,
}

impl Default for WobbleOptions {
    fn default() -> Self {
        Self {
            seed: 32,
            repl_char_chance: 0.05,
            repl_char: '_'
        }
    }
}

pub fn wobble(s: &str, options: &WobbleOptions) -> String {
    let mut rng = ChaCha12Rng::seed_from_u64(options.seed);
    s.chars().fold(String::new(), 
        |mut acc, new | {
            if rng.gen::<f32>() < options.repl_char_chance {
                acc.push(options.repl_char);
            } else if rng.gen() {
                acc.extend(new.to_uppercase());
            } else {
                acc.extend(new.to_lowercase());
            }
            acc
        }
    )
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WobbleRequest {
    #[serde(default = "Uuid::new_v4")]
    pub id: Uuid,
    #[serde(default)]
    pub options: WobbleOptions,
    pub input: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WobbleResponse {
    pub id: Uuid,
    pub output: String,
}

pub fn wobble_api(req: &WobbleRequest) -> WobbleResponse {
    WobbleResponse {
        id: req.id,
        output: wobble(&req.input, &req.options),
    }
}

#[test]
fn test_wobble() {
    let opts = WobbleOptions::default();
    insta::assert_snapshot!(wobble("Goodbye World!", &opts), @"gooDbye w_Rld!");
}

#[test]
fn test_api() {
    insta::glob!("snapshot_inputs/*.json", |path| {
        let req: WobbleRequest = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
        insta::assert_json_snapshot!(wobble_api(&req), {
            ".id" => "[uuid]"
        })
    }

    )
    
}


