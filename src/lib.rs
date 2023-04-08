//! A Rust crate for communicating with the [yabai](https://github.com/koekeishiya/yabai) tiling window manager.
//!
//!
//! ## Examples:
//!
//! Send a command as a string:
//! ```
//! yabai::send("--focus space 2");
//!```
//!
//! Send a command using the `yabai::Command` type:
//! ```
//! let command: yabai::Command = yabai::Command::FocusSpace(yabai::SpaceOption::Recent);
//! yabai::send_command(command)?;
//! ```
//!
//! Query yabai for display information:
//! ```
//! let displays = yabai::query_displays()?;
//! ```
//!
mod commands;
mod errors;

pub use commands::*;
pub use errors::*;

use anyhow::anyhow;
use byteorder::{LittleEndian, WriteBytesExt};
use lazy_static::lazy_static;
use std::{
    io::{Read, Write},
    os::unix::net::UnixStream,
    path::PathBuf,
};

lazy_static! {
    static ref SOCKET_PATH: PathBuf = {
        PathBuf::from(format!(
            "/tmp/yabai_{}.socket",
            std::env::var("USER").unwrap()
        ))
    };
}

/// Send a command to yabai as a string of space-separated arguments.
///
/// See the [yabai documentation](https://github.com/koekeishiya/yabai/wiki/Commands#message-passing-interface) for more information.
///
/// Example:
///
/// ```
/// yabai::send_str("space --focus 2")?;
/// ```
pub fn send(message: &str) -> anyhow::Result<Option<String>> {
    send_raw(&format!(
        "{}\0\0",
        message.trim().split(' ').collect::<Vec<&str>>().join("\0")
    ))
}

fn send_raw(command: &str) -> anyhow::Result<Option<String>> {
    let mut buffer = Vec::new();
    let mut stream = UnixStream::connect(SOCKET_PATH.as_path())?;

    stream.write_u32::<LittleEndian>(command.len() as u32)?;
    stream.write_all(command.as_bytes())?;

    let bytes = stream.read_to_end(&mut buffer)?;

    if buffer[0] == 0x07 {
        let rest = buffer[1..].to_vec();
        let error_message = String::from_utf8(rest)?;

        let error = YabaiError::CommandError {
            command: command.to_string(),
            message: error_message,
        };

        return Err(anyhow!(error));
    }

    if bytes > 0 {
        return Ok(Some(String::from_utf8(buffer)?));
    }

    Ok(None)
}

/// Send a `yabai::Command` to yabai.
///
/// Example:
///
/// ```
/// let command = yabai::Command::FocusSpace(yabai::SpaceOption::Recent);
/// yabai::send_command(command)?;
/// ```
pub fn send_command(command: Command) -> anyhow::Result<Option<String>> {
    let result = match command {
        Command::FocusSpace(option) => match option {
            FocusSpaceOption::Space(space) => send(&format!("space --focus {}", space))?,
            named_option => send(&format!("space --focus {named_option}"))?,
        },
        Command::RotateSpace(rotation) => send(&format!("space --rotate {}", rotation))?,
        Command::BalanceSpace => send("space --balance")?,
        Command::MoveActiveWindowToSpace(space) => send(&format!("window --space {}", space))?,
        Command::FocusWindow(window) => send(&format!("window --focus {}", window))?,
        Command::FocusWindowDirection(dir) => send(&format!("window --focus {}", dir))?,
        Command::SwapWindowDirection(dir) => send(&format!("window --swap {}", dir))?,
        Command::WarpWindowDirection(warp) => send(&format!("window --warp {}", warp))?,
        Command::ToggleWindowFloating => send("window --toggle float")?,
        Command::ToggleZoomFullscreen => send("window --toggle zoom-fullscreen")?,
    };

    Ok(result)
}

/// Queries yabai for information about all spaces.
pub fn query_spaces() -> anyhow::Result<Vec<SpaceInfo>> {
    let result = send("query --spaces")?;

    match result {
        Some(str) => Ok(serde_json::from_str::<Vec<SpaceInfo>>(&str)?),
        None => Err(anyhow!("No result from yabai query --spaces")),
    }
}

/// Queries yabai for information about all displays.
pub fn query_displays() -> anyhow::Result<Vec<DisplayInfo>> {
    let result = send("query --displays")?;

    match result {
        Some(str) => Ok(serde_json::from_str::<Vec<DisplayInfo>>(&str)?),
        None => Err(anyhow!("No result from yabai query --displays")),
    }
}

/// Queries yabai for information about all windows.
pub fn query_windows() -> anyhow::Result<Vec<WindowInfo>> {
    let result = send("query --windows")?;

    match result {
        Some(str) => Ok(serde_json::from_str::<Vec<WindowInfo>>(&str)?),
        None => Err(anyhow!("No result from yabai query --windows")),
    }
}
