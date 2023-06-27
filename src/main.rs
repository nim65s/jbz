use std::process::Command;
use zellij_tile::prelude::*;

#[derive(Default)]
struct State {
    build_runs: usize,
    commands: Vec<String>,
}

register_plugin!(State);

fn just_commands() -> Vec<String> {
    let output = Command::new("just").arg("-l").output().unwrap();
    String::from_utf8_lossy(&output.stdout)
        .split('\n')
        .skip(1)
        .map(|s| s.trim().to_string())
        .collect()
}

impl ZellijPlugin for State {
    fn load(&mut self) {
        subscribe(&[EventType::Key]);
        self.commands = just_commands()
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
