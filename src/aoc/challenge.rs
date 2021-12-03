use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

use serde::{Deserialize, Serialize};

use crate::Result;

#[derive(Deserialize, Serialize)]
pub struct Challenge {
    #[serde(default = "default_challenge_day")]
    pub day: u8,
    #[serde(default = "default_challenge_parts")]
    pub parts: u8,
    pub example_data: String,
    pub data: String,
}

fn default_challenge_day() -> u8 {
    0
}

fn default_challenge_parts() -> u8 {
    1
}

pub fn get_challenge(day: u8) -> Result<Challenge> {
    let challenges_file = File::open("./data/challenges.json")?;
    let challenges_map: HashMap<String, Challenge> =
        serde_json::from_reader(BufReader::new(challenges_file))?;
    match challenges_map.get(&format!("{}", day)) {
        Some(challenge) => Ok(Challenge {
            day: if challenge.day == 0 {
                day
            } else {
                challenge.day
            },
            parts: challenge.parts,
            example_data: challenge.example_data.clone(),
            data: challenge.data.clone(),
        }),
        None => Err(Box::new(crate::Error::new(&format!(
            "Challenge #{} not found in challenges.json!",
            day
        )))),
    }
}
