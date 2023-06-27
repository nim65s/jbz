use zellij_tile::prelude::*;

#[derive(Default)]
struct State {
    build_runs: usize,
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self) {
        subscribe(&[EventType::Key]);
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
        if self.build_runs > 0 {
            println!("Ran tests {} times!", self.build_runs);
        }
    }
}
