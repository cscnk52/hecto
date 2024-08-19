use crossterm::event::{read, Event, KeyEvent, KeyEventKind};
use view::View;
mod terminal;
mod view;
use std::{
    env,
    io::Error,
    panic::{set_hook, take_hook},
};
use terminal::Terminal;
mod editorcommand;
use editorcommand::EditCommand;
mod statusbar;
use statusbar::StatusBar;

pub struct Editor {
    should_quit: bool,
    view: View,
    status_bar: StatusBar,
}

#[derive(Default, PartialEq, Eq, Debug)]
pub struct DocumentStatus {
    total_line: usize,
    current_line_index: usize,
    is_modify: bool,
    file_name: Option<String>,
}

impl Editor {
    pub fn new() -> Result<Self, Error> {
        let current_hook = take_hook();
        set_hook(Box::new(move |painc_info| {
            let _ = Terminal::terminate();
            current_hook(painc_info);
        }));
        Terminal::initialize()?;
        let mut view = View::new(2);
        let args: Vec<String> = env::args().collect();
        if let Some(file_name) = args.get(1) {
            view.load(file_name);
        }
        Ok(Self {
            should_quit: false,
            view,
            status_bar: StatusBar::new(1),
        })
    }

    pub fn run(&mut self) {
        loop {
            self.refresh_screen();
            if self.should_quit {
                break;
            }
            match read() {
                Ok(event) => self.evaluate_event(event),
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not read event: {err:?}");
                    }
                }
            }
            let status = self.view.get_status();
            self.status_bar.update_status(status);
        }
    }
    fn evaluate_event(&mut self, event: Event) {
        let should_process = match &event {
            Event::Key(KeyEvent { kind, .. }) => kind == &KeyEventKind::Press,
            Event::Resize(_, _) => true,
            _ => false,
        };
        if should_process {
            if let Ok(command) = EditCommand::try_from(event) {
                if matches!(command, EditCommand::Quit) {
                    self.should_quit = true;
                } else {
                    self.view.handle_command(command);
                    if let EditCommand::Resize(size) = command {
                        self.status_bar.resize(size);
                    }
                }
            }
        }
    }
    fn refresh_screen(&mut self) {
        let _ = Terminal::hide_caret();
        self.view.render();
        self.status_bar.render();
        let _ = Terminal::move_caret_to(self.view.caret_position());
        let _ = Terminal::show_caret();
        let _ = Terminal::execute();
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        let _ = Terminal::terminate();
        if self.should_quit {
            let _ = Terminal::print("GoodBye.\r\n");
        }
    }
}
