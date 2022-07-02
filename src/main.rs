#![allow(dead_code)]
#![allow(deprecated)]

use std::{
    io::{Error, ErrorKind},
    path::PathBuf,
};

const LAMBDA: &str = "λ";

// Tenderize your prompt.
fn main() -> std::io::Result<()> {
    // TODO: swap a home prefix with ~
    let custom_error = Error::new(ErrorKind::Other, "oh no!");
    let home: String = std::env::home_dir()
        .ok_or(custom_error)?
        .to_string_lossy()
        .to_string();
    let cwd: PathBuf = std::env::current_dir()?;
    let cwd = match cwd.strip_prefix(home) {
        Ok(stripped) => format!("~/{}", stripped.to_string_lossy()),
        Err(_) => cwd.to_string_lossy().to_string(),
    };

    print!("{} {} ", cyan(cwd), LAMBDA);
    Ok(())
}

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

fn cyan(msg: String) -> String {
    colorize(
        Color {
            red: 118,
            green: 196,
            blue: 250,
        },
        msg,
    )
}
fn red(msg: String) -> String {
    colorize(
        Color {
            red: 255,
            green: 0,
            blue: 0,
        },
        msg,
    )
}
fn colorize(col: Color, msg: String) -> String {
    let Color { red, green, blue } = col;
    // TODO: is reset diff on windows?
    let reset_esc = "%{\x1b[0m%}";
    let color_esc = format!("%{{\x1b[38;2;{red};{green};{blue}m%}}");
    format!("{color_esc}{msg}{reset_esc}")
}
