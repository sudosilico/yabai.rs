# yabai.rs

A Rust library for communicating with the [yabai](https://github.com/koekeishiya/yabai) tiling window manager.

It sends client commands directly to the yabai server via UnixStream sockets, acting as a Rust equivalent of `yabai -m`.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
yabai = "0.1.0"
```

or using `cargo add`:

```sh
cargo add yabai
```

## Sending Commands

You can send commands in a `yabai -m`-like fashion, using a string:

```rust
yabai::send("--focus space 2");
```

Alternatively, you can use the `Command` enum:

```rust
let command = yabai::Command::FocusSpace(yabai::SpaceOption::Recent);
yabai::send_command(command)?;
```

## Responses

The `send` and `send_command` functions both return a `Result<Option<String>>`, with the `String` containing a possible JSON response from the yabai server:



```rust
let displays = yabai::query_displays()?;
```
