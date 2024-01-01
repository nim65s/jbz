use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufReader, Read};

use regex::Regex;
use zellij_tile::prelude::*;

#[derive(Default)]
struct State {
    loaded: bool,
}

register_plugin!(State);

fn just_commands() -> Vec<String> {
    // let output = Command::new("just").arg("-l").output().unwrap();
    // ↑ won't work in wasi, let's find another way
    let file = File::open("/host/.justfile").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    // regex is another way, which kinda work here, but might not be optimal
    // ref. https://github.com/casey/just/issues/365#issuecomment-1610357375
    Regex::new(r"\n.*:\n")
        .unwrap()
        .find_iter(&contents)
        .filter_map(|cmd| cmd.as_str().trim().strip_suffix(':'))
        .map(|cmd| cmd.to_string())
        .collect()
}

impl ZellijPlugin for State {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        request_permission(&[PermissionType::RunCommands]);
        hide_self();
    }

    fn update(&mut self, _: Event) -> bool {
        false
    }

    fn render(&mut self, _rows: usize, _cols: usize) {
        if !self.loaded {
            // This used to be in load(), but we can't run commands in load() anymore
            self.loaded = true;

            for cmd in &just_commands() {
                open_command_pane(CommandToRun {
                    path: "bacon".into(),
                    args: vec!["just".to_owned(), "--".to_owned(), cmd.to_owned()],
                    cwd: None,
                });
            }
        }
    }
}
