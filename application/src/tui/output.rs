use crossterm::style::{Color, SetForegroundColor, ResetColor, Print};
use crossterm::execute;
use std::io::{self, Write, stdout};

pub struct OutputRenderer;

impl OutputRenderer {
    pub fn print_token(token: &str) {
        print!("{}", token);
        let _ = io::stdout().flush();
    }

    pub fn print_done() {
        println!();
    }

    pub fn print_error(msg: &str) {
        let _ = execute!(
            stdout(),
            SetForegroundColor(Color::Red),
            Print("error: "),
            Print(msg),
            Print("\n"),
            ResetColor,
        );
    }

    pub fn print_system(msg: &str) {
        let _ = execute!(
            stdout(),
            SetForegroundColor(Color::DarkGrey),
            Print(msg),
            Print("\n"),
            ResetColor,
        );
    }
}
