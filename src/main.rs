use std::fs::File;
use std::io::{BufReader, Read};

use regex::Regex;
use zellij_tile::prelude::*;

#[derive(Default)]
struct State {
    build_runs: usize,
    commands: Vec<String>,
}

register_plugin!(State);

fn just_commands() -> Vec<String> {
    let file = File::open("/host/.justfile").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    Regex::new(r"\n.*:\n")
        .unwrap()
        .find_iter(&contents)
        .filter_map(|cmd| cmd.as_str().trim().strip_suffix(':'))
        .map(|cmd| cmd.to_string())
        .collect()
}

impl ZellijPlugin for State {
    fn load(&mut self) {
        subscribe(&[EventType::Key]);
        self.commands = just_commands();
        for cmd in &self.commands {
            open_command_pane("just", vec![cmd]);
        }
    }

    fn update(&mut self, event: Event) -> bool {
        let should_render = false;
        if let Event::Key(Key::Char('b')) = event {
            self.build_runs += 1;
            open_command_pane_floating("cargo", vec!["build"]);
        }
        should_render
    }

    fn render(&mut self, rows: usize, cols: usize) {
        println!();
        println!("I have {rows} rows and {cols} columns");
        println!();
        println!("here are the available commands:");
        for cmd in &self.commands {
            println!(" - {cmd}");
        }
        println!();
        if self.build_runs > 0 {
            println!("Ran tests {} times!", self.build_runs);
        }
    }
}
