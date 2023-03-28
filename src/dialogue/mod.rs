use std::{collections::HashMap, path::PathBuf};

use bevy::prelude::*;
use bevy_proto::ProtoComponent;
use serde::{Deserialize, Serialize};

mod parser;

type ParticipantID = usize;

#[derive(Default, Debug)]
pub struct Participant {
    pub name: String,
    pub short_name: Option<String>,
    // .. other stuff
}

impl From<String> for Participant {
    fn from(value: String) -> Self {
        Participant {
            name: value, // TODO: infer from db
            short_name: None, // TODO: infer from db
        }
    }
}

#[derive(Default, Debug)]
pub struct Line {
    pub author: ParticipantID,
    pub text: String,
}

#[derive(Default, Debug)]
pub enum DialogueTree {
    #[default]
    Empty,
    List(Vec<Line>),
}

#[derive(Default, Debug)]
#[derive(Component)]
pub struct Dialogue {
    pub exchanges: Vec<DialogueTree>,
    pub participants: Vec<Participant>,
    pub curr_exchange: usize,
    pub curr_line: usize,
}

impl Dialogue {
    pub fn new(filename: PathBuf) -> Self {
        let mut file;
        if let Ok(f) = std::fs::File::open(&filename) {
            file = std::io::BufReader::new(f);
        } else {
            log::error!("Failed to open dialogue file {:?}", filename);
            return Dialogue::default()
        }

        if let Some(d) = parser::parse_dialogue_file(&mut file) {
            d
        } else {
            log::error!("Failed to parse dialogue file {:?}", filename);
            Dialogue::default()
        }
    }
}