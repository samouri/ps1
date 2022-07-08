//! Tenderize your prompt.
//!
//! This binary only supports modern terminals with full RGB color support.

#![allow(dead_code)]
#![allow(deprecated)]

use {
    crate::colors::*,
    std::{
        env,
        io::{Error, ErrorKind},
        path::PathBuf,
        process::Command,
    },
};

const LAMBDA: &str = "λ";

fn main() -> std::io::Result<()> {
    let home: PathBuf = get_home_dir()?;
    let cwd: PathBuf = env::current_dir()?;
    let cwd: PathBuf = match cwd.strip_prefix(home) {
        Ok(stripped) => PathBuf::from("~").join(stripped),
        Err(_) => cwd,
    };
    let branch = get_branch();

    let mut ps1: Vec<String> = Vec::new();
    ps1.push(cyan(cwd.to_string_lossy().to_string()));
    if !branch.is_empty() {
        ps1.push(grey(branch))
    }
    ps1.push(pink(LAMBDA.to_owned()));

    print!("{} ", ps1.join(" "));
    Ok(())
}

// Returns the current user's home directory.
fn get_home_dir() -> Result<PathBuf, Error> {
    // TODO: support Windows (std::env::home_dir only works on some OSes).
    let custom_error = Error::new(ErrorKind::Other, "Could not retrieve home directory");
    std::env::home_dir().ok_or(custom_error)
}

fn get_branch() -> String {
    Command::new("git")
        .args(["branch", "--show-current"])
        .output()
        .map_or(String::new(), |output| {
            String::from_utf8_lossy(&output.stdout).trim().into()
        })
}

mod colors {
    // RGB Tuple
    #[derive(Debug)]
    pub struct Color(u8, u8, u8);

    pub fn cyan(msg: String) -> String {
        colorize(Color(118, 196, 250), msg)
    }
    pub fn grey(msg: String) -> String {
        colorize(Color(108, 108, 108), msg)
    }
    pub fn pink(msg: String) -> String {
        colorize(Color(237, 115, 190), msg)
    }

    pub fn colorize(col: Color, msg: String) -> String {
        let Color(red, green, blue) = col;
        // TODO: is reset diff on windows?
        let reset_esc = "%{\x1b[0m%}";
        let color_esc = format!("%{{\x1b[38;2;{red};{green};{blue}m%}}");
        format!("{color_esc}{msg}{reset_esc}")
    }
}
