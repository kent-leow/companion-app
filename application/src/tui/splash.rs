use crossterm::style::{Color, SetForegroundColor, ResetColor, Print};
use crossterm::execute;
use std::io::stdout;

const ALIEN: &str = r#"
    ⠀⠀⠀⣀⣤⣴⣶⣶⣶⣦⣤⣀⠀⠀⠀
    ⠀⣠⣾⣿⣿⣿⣿⣿⣿⣿⣿⣷⣄⠀
    ⣴⣿⣿⣿⡿⠟⠛⠛⠟⢿⣿⣿⣿⣦
    ⣿⣿⡟⠁⠀⣠⣤⣤⡀⠀⠙⢿⣿⣿
    ⣿⡟⠀⠀⣾⣿⣿⣿⣷⡀⠀⠀⢻⣿
    ⣿⠀⠀⠀⠛⢿⣿⡿⠟⠃⠀⠀⠀⣿
    ⣿⣆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣰⣿
    ⠻⣿⣷⣄⡀⠀⠀⠀⠀⢀⣠⣾⣿⠟
    ⠀⠈⠻⣿⣿⣿⣶⣶⣿⣿⣿⠟⠁⠀
    ⠀⠀⠀⠀⠉⠛⠛⠛⠛⠉⠀⠀⠀⠀
"#;

pub fn show_splash() {
    let _ = execute!(
        stdout(),
        SetForegroundColor(Color::Green),
        Print(ALIEN),
        Print("\n"),
        SetForegroundColor(Color::Cyan),
        Print("  ╔══════════════════════════════╗\n"),
        Print("  ║   COMPANION v"),
        Print(env!("CARGO_PKG_VERSION")),
        Print("            ║\n"),
        Print("  ║   AI Agent • Ultra Concise   ║\n"),
        Print("  ╚══════════════════════════════╝\n"),
        Print("\n"),
        ResetColor,
    );
}
