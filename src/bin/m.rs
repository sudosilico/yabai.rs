use std::env;

fn main() -> anyhow::Result<()> {
    let args = env::args().collect::<Vec<String>>();
    let command = args[1..].join(" ");

    let result = yabai::send(&command)?;

    if let Some(result) = result {
        println!("{}", result);
    }

    Ok(())
}
