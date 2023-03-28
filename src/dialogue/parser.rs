use std::{io::{BufReader, BufRead}, collections::HashMap};

use bevy_inspector_egui::egui::TextBuffer;

use super::{Dialogue, ParticipantID, Participant, DialogueTree, Line};

#[derive(Default)]
enum DialogueToken {
    #[default]
    None,
    Participants(Vec<String>),
    ExchangeLabel(String),
    Line(ParticipantID, String)
}

fn tokenize(reader: &mut BufReader<std::fs::File>) -> Vec<DialogueToken> {
    let mut tokens = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line { 
            if line.starts_with("participants") {
                let mut toks = line.split(':');
                toks.next();
                if let Some(toks) = toks.next() {
                    let participants = toks.split(',').map(|tok| tok.to_string()).collect();
                    tokens.push(DialogueToken::Participants(participants));
                }
            } else if line.ends_with(':') {
                let mut toks = line.split(':');
                let label = toks.next();
                assert_eq!(toks.next(), Some(""));
                if let Some(label) = label {
                    tokens.push(DialogueToken::ExchangeLabel(label.to_string()));
                }
            } else if line.starts_with("[") {
                let mut toks = line.split('[');
                toks.next(); // drop the '['

                let mut toks = toks.next().unwrap().split(']');
                
                let id_str = toks.next();
                assert_ne!(id_str, None);
                let id = id_str.unwrap().parse::<usize>();
                assert!(!id.is_err());
                let id = id.unwrap();

                let line = toks.next();
                assert_ne!(line, None);

                tokens.push(DialogueToken::Line(id, line.unwrap().to_string()));
            } else if line.starts_with("*") {
                let mut line = line.split('*');
                line.next();
                let line = line.next().unwrap();
                if !line.is_empty() {
                    tokens.push(DialogueToken::Line(1, line.to_string()));
                }
            } else if line.starts_with("-") {
                let mut line = line.split('-');
                line.next();
                let line = line.next().unwrap();
                if !line.is_empty() {
                    tokens.push(DialogueToken::Line(0, line.to_string()));
                }
            }
        }   
    }

    tokens
}

pub(crate) fn parse_dialogue_file(file: &mut BufReader<std::fs::File>) -> Option<Dialogue> {
    let mut res = Dialogue::default();

    let mut tokens = tokenize(file);

    let mut last_label: String = String::new();
    let mut last_exchange = Vec::new();

    // TODO: will be used when resolving jump-to labels
    let mut labels: HashMap<String, usize> = HashMap::new();

    for token in tokens.iter_mut() {
        match token {
        DialogueToken::None => {
            return None
        },
        DialogueToken::Participants(ps) => {
            for name in ps.iter_mut() {
                res.participants.push(Participant::from(name.take()));
            }
        },
        DialogueToken::ExchangeLabel(label) => {
            if !last_exchange.is_empty() {
                assert!(!last_label.is_empty());
                assert!(labels.contains_key(&last_label));

                if let Some(tree) = res.exchanges.get_mut(*labels.get(&last_label).unwrap()) {
                    *tree = DialogueTree::List(last_exchange);
                    last_exchange = Vec::new();
                }
            }

            last_label = label.clone();
            res.exchanges.push(DialogueTree::Empty);
            labels.insert(label.take(), res.exchanges.len() - 1);
        },
        DialogueToken::Line(id, line) => {
            last_exchange.push(Line { text: line.take(), author: *id });
        },
        }
    }

    if !last_exchange.is_empty() {
        assert!(!last_label.is_empty());
        assert!(labels.contains_key(&last_label));

        if let Some(tree) = res.exchanges.get_mut(*labels.get(&last_label).unwrap()) {
            *tree = DialogueTree::List(last_exchange);
        }
    }

    Some(res)
}