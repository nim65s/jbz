use zellij_tile::prelude::*;

use std::collections::HashMap;

#[derive(Default)]
struct State {
    mode_log: HashMap<String, usize>,
    tabs: Vec<String>,
    build_runs: usize,
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self) {
        subscribe(&[EventType::ModeUpdate, EventType::TabUpdate, EventType::Key]);
    }

    fn update(&mut self, event: Event) -> bool {
        let mut should_render = false;
        match event {
            Event::ModeUpdate(mode_info) => {
                let mode = format!("{:?}", mode_info.mode);
                let count = self.mode_log.entry(mode).or_insert(0);
                *count += 1;
                should_render = true;
            }
            Event::TabUpdate(tab_info) => {
                self.tabs = tab_info.iter().map(|t| t.name.clone()).collect();
                should_render = true;
            }
            Event::Key(Key::Char('b')) => {
                self.build_runs += 1;
                open_command_pane_floating("cargo", vec!["build"]);
            }
            _ => (),
        };
        should_render
    }

    fn render(&mut self, rows: usize, cols: usize) {
        println!();
        println!("I have {rows} rows and {cols} columns");
        println!();
        println!("Modes:");
        for (mode, count) in &self.mode_log {
            println!("{mode} -> Changed {count} times");
        }
        println!();
        println!("Current Tabs: {}", self.tabs.join(", "));
        println!();
        if self.build_runs > 0 {
            println!("Ran tests {} times!", self.build_runs);
        }
    }
}
