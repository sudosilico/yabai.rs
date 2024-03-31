# yabai.rs

A Rust library for communicating with the [yabai](https://github.com/koekeishiya/yabai) tiling window manager's [ipc socket](https://github.com/koekeishiya/yabai/wiki/Commands#message-passing-interface), acting as a Rust equivalent of the `yabai -m` command.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
yabai = "0.5.0"
```

or using `cargo add`:

```sh
cargo add yabai
```

## Sending Commands

You can send commands in a `yabai -m` like fashion, using a string:

```rust
yabai::send("--focus space 2");
```

Alternatively, you can use the `Command` enum for more strongly-typed inputs:

```rust
let command = yabai::Command::FocusSpace { option: yabai::SpaceOption::Recent };

yabai::send_command(command)?;
```

## Queries

Displays, spaces, and windows can all be queried:

```rust
let displays = yabai::query_displays()?; // Vec<DisplayInfo>

let spaces = yabai::query_spaces()?; // Vec<SpaceInfo>

let windows = yabai::query_windows()?; // Vec<WindowInfo>
```
